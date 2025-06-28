use super::LUA_SERIALIZE_OPTIONS;
use crate::primitives::create_userdata_iterator_with_dyn_fields;
use crate::traits::context::{KhronosContext, Limitations};
use crate::traits::datastoreprovider::{DataStoreImpl, DataStoreMethod, DataStoreProvider};
use crate::utils::khronos_value::KhronosValue;
use crate::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct DataStore<T: KhronosContext> {
    executor: DataStoreExecutor<T>,
    ds_impl: Rc<dyn DataStoreImpl>,
    method_cache: Rc<RefCell<HashMap<String, LuaValue>>>,
}

impl<T: KhronosContext> DataStore<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if self.ds_impl.need_caps(&action) {
            if !self
                .executor
                .limitations
                .has_cap(&format!("datastore:{}", self.ds_impl.name()))
                && !self.executor.limitations.has_cap(&format!(
                    "datastore:{}:{}",
                    self.ds_impl.name(),
                    action
                ))
            {
                return Err(LuaError::runtime(format!(
                    "Datastore action is not allowed in this template context: data store: {}, action: {}",
                    self.ds_impl.name(),
                    action
                )));
            }
        }

        self.executor
            .datastore_provider
            .attempt_action(&self.ds_impl.name(), &action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for DataStore<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.ds_impl.name()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("need_caps", |_, this, op: String| {
            Ok(this.ds_impl.need_caps(&op))
        });

        methods.add_method("methods", |lua, this, _: ()| {
            Ok(lua.to_value_with(&this.ds_impl.methods(), LUA_SERIALIZE_OPTIONS)?)
        });

        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: LuaValue| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(None);
                }
            };

            let mut methods_cache = this
                .method_cache
                .try_borrow_mut()
                .map_err(|_| LuaError::external("Failed to borrow method cache"))?;

            match methods_cache.get(&key) {
                Some(cached_method) => {
                    return Ok(Some(cached_method.clone()));
                }
                None => {
                    if let Some(method_impl) = this.ds_impl.get_method(key.clone()) {
                        let this_ref = this.clone();
                        let key_ref = key.clone();

                        let method = {
                            match method_impl {
                                DataStoreMethod::Async(method_impl) => lua
                                    .create_scheduler_async_function(
                                        move |lua, data: LuaMultiValue| {
                                            let this_ref = this_ref.clone();
                                            let key_ref = key_ref.clone();
                                            let method_impl = method_impl.clone();

                                            async move {
                                                let mut args = Vec::with_capacity(data.len());
                                                for value in data {
                                                    args.push(KhronosValue::from_lua(value, &lua)?);
                                                }

                                                this_ref.check_action(key_ref.clone())?;

                                                let result =
                                                    (method_impl)(args).await.map_err(|e| {
                                                        LuaError::external(e.to_string())
                                                    })?;
                                                Ok(result)
                                            }
                                        },
                                    )?,
                                DataStoreMethod::Sync(method_impl) => {
                                    lua.create_function(move |lua, data: LuaMultiValue| {
                                        let mut args = Vec::with_capacity(data.len());
                                        for value in data {
                                            args.push(KhronosValue::from_lua(value, &lua)?);
                                        }

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (method_impl)(args)
                                            .map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result.into_lua(lua)?)
                                    })?
                                }
                            }
                        };

                        let method = LuaValue::Function(method);

                        methods_cache.insert(key.to_string(), method.clone());
                        return Ok(Some(method));
                    } else {
                        return Ok(None);
                    }
                }
            }
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<DataStore<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            let mut base = vec![
                // Fields
                "name".to_string(),
                // Methods
                "needed_caps".to_string(),
                "methods".to_string(),
            ];

            if let Ok(ds) = ud.borrow::<DataStore<T>>() {
                base.extend(ds.ds_impl.methods());
            }

            create_userdata_iterator_with_dyn_fields(lua, ud, base)
        });
    }
}

#[derive(Clone)]
pub struct DataStoreExecutor<T: KhronosContext> {
    limitations: Rc<Limitations>,
    datastore_provider: T::DataStoreProvider,
    known_datastores: Rc<RefCell<HashMap<String, LuaValue>>>,
}

impl<T: KhronosContext> LuaUserData for DataStoreExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "DataStoreExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: LuaValue| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(None);
                }
            };

            {
                let map = this
                    .known_datastores
                    .try_borrow()
                    .map_err(|_| LuaError::external("Failed to borrow custom datastores"))?;
                if let Some(ds) = map.get(key.as_str()) {
                    return Ok(Some(ds.clone()));
                }
            }

            let Some(ds_impl) = this.datastore_provider.get_builtin_data_store(&key) else {
                return Ok(None);
            };

            let ds = DataStore {
                executor: this.clone(),
                ds_impl,
                method_cache: Rc::new(RefCell::new(HashMap::new())),
            };

            let ds = ds.into_lua(lua)?;
            this.known_datastores
                .try_borrow_mut()
                .map_err(|_| LuaError::external("Failed to borrow custom datastores"))?
                .insert(key.to_string(), ds.clone());

            Ok(Some(ds))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<DataStoreExecutor<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            let datastores = {
                let this = ud
                    .borrow::<DataStoreExecutor<T>>()
                    .map_err(|_| LuaError::external("Invalid userdata type"))?;

                this.datastore_provider.public_builtin_data_stores()
            };

            create_userdata_iterator_with_dyn_fields(lua, ud, datastores)
        });
    }
}

pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaValue> {
    let Some(datastore_provider) = token.context.datastore_provider() else {
        return Err(LuaError::external(
            "The datastore plugin is not supported in this context",
        ));
    };

    let executor = DataStoreExecutor::<T> {
        limitations: token.limitations.clone(),
        datastore_provider,
        known_datastores: Rc::new(RefCell::new(HashMap::new())),
    }
    .into_lua(lua)?;

    Ok(executor)
}
