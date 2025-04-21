use super::LUA_SERIALIZE_OPTIONS;
use mlua::prelude::*;
use crate::lua_promise;
use std::rc::Rc;
use crate::primitives::create_userdata_iterator_with_dyn_fields;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::traits::context::KhronosContext;
use crate::traits::datastoreprovider::{DataStoreImpl, DataStoreProvider, DataStoreMethod};
use crate::traits::ir::{Filters, DataStoreValue, DataStoreValueMap};
use crate::{plugins::antiraid::lazy::Lazy, TemplateContextRef};
use crate::utils::executorscope::ExecutorScope;

#[derive(Clone)]
pub struct DataStore<T: KhronosContext> {
    executor: DataStoreExecutor<T>,
    ds_impl: Rc<dyn DataStoreImpl>,
    method_cache: Rc<RefCell<HashMap<String, LuaValue>>>,
    columns_cache: Rc<RefCell<Option<LuaValue>>>
}

impl<T: KhronosContext> DataStore<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.executor.context.has_cap(&format!("datastore:{}", self.ds_impl.name())) && !self.executor.context.has_cap(&format!("datastore:{}:{}", self.ds_impl.name(), action)) {
            return Err(LuaError::runtime(format!(
                "Datastore action is not allowed in this template context: data store: {}, action: {}",
                self.ds_impl.name(),
                action
            )));
        }

        self.executor
            .datastore_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for DataStore<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.ds_impl.name()));
        fields.add_field_method_get("table_name", |lua, this| lua.to_value_with(&this.ds_impl.table_name(), LUA_SERIALIZE_OPTIONS));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("column_names", |lua, this, ()| {
            Ok(
                lua.to_value_with(&this.ds_impl.column_names(), LUA_SERIALIZE_OPTIONS)?
            )
        });

        methods.add_method("columns", |lua, this, ()| {
            // Check for cached serialized data
            let mut cached_data = this
                .columns_cache
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(v) = cached_data.as_ref() {
                return Ok(v.clone());
            }

            let v = lua.to_value_with(&this.ds_impl.columns(), LUA_SERIALIZE_OPTIONS)?;

            *cached_data = Some(v.clone());

            Ok(v)
        });

        methods.add_method("filters_sql", |lua, this, filters: LuaValue| {
            let filters: Filters = lua.from_value(filters)?;
            let (sql, filter_fields) = this.ds_impl.filters_sql(filters);
            Ok((sql, Lazy::new(filter_fields)))
        });

        methods.add_method("validate_data_against_columns", |lua, this, data: LuaValue| {
            let validate_data_resp = this.ds_impl.validate_data_against_columns(&lua, &data);
            Ok(validate_data_resp)
        });

        methods.add_method("methods", |lua, this, _: ()| {
            Ok(
                lua.to_value_with(&this.ds_impl.methods(), LUA_SERIALIZE_OPTIONS)?
            ) 
        });

        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: LuaValue| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(None);
                }
            };

            if let Some(method_impl) = this.ds_impl.get_method(key.clone()) {
                let mut methods_cache = this.method_cache.try_borrow_mut().map_err(|_| LuaError::external("Failed to borrow method cache"))?;
                let Some(cached_method) = methods_cache.get(&key) else {
                    let this_ref = this.clone();
                    let key_ref = key.clone();
                    let method = lua.create_function(
                        move |_lua, data: LuaMultiValue| {
                            let method_impl = method_impl.clone();
                            match method_impl {
                                DataStoreMethod::NoArgs(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, func, |_lua, this_ref, key_ref, func|, {
                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)().await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::Filters(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("Filters not provided"));
                                        };

                                        let filters: Filters = lua.from_value(value)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(filters).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::Value(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValue not provided"));
                                        };

                                        let value = DataStoreValue::from_lua(value, &lua)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(value).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::Map(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValueMap not provided"));
                                        };

                                        let map = DataStoreValueMap::from_lua(value, &lua)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(map).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::FiltersAndValue(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("Filters and DataStoreValue not provided"));
                                        };

                                        let filters: Filters = lua.from_value(value)?;

                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValue not provided"));
                                        };

                                        let value = DataStoreValue::from_lua(value, &lua)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(filters, value).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::FiltersAndMap(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("Filters and DataStoreValueMap not provided"));
                                        };

                                        let filters: Filters = lua.from_value(value)?;

                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValueMap not provided"));
                                        };

                                        let map = DataStoreValueMap::from_lua(value, &lua)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(filters, map).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::ValueAndMap(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValue and DataStoreValueMap not provided"));
                                        };

                                        let ds_value = DataStoreValue::from_lua(value, &lua)?;

                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValueMap not provided"));
                                        };

                                        let map = DataStoreValueMap::from_lua(value, &lua)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(ds_value, map).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                                DataStoreMethod::FiltersAndValueAndMap(func) => {
                                    Ok(lua_promise!(this_ref, key_ref, data, func, |lua, this_ref, key_ref, data, func|, {
                                        let mut data = data;
                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("Filters and DataStoreValue and DataStoreValueMap not provided"));
                                        };

                                        let filters: Filters = lua.from_value(value)?;

                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValue not provided"));
                                        };

                                        let ds_value = DataStoreValue::from_lua(value, &lua)?;

                                        let Some(value) = data.pop_front() else {
                                            return Err(LuaError::external("DataStoreValueMap not provided"));
                                        };

                                        let map = DataStoreValueMap::from_lua(value, &lua)?;

                                        this_ref.check_action(key_ref.clone())?;
                                        let result = (func)(filters, ds_value, map).await.map_err(|e| LuaError::external(e.to_string()))?;
                                        Ok(result)
                                    }))
                                },
                            }
                        },
                    )?;

                    let method = LuaValue::Function(method);

                    methods_cache.insert(key.to_string(), method.clone());
                    return Ok(Some(method));
                };

                return Ok(Some(cached_method.clone()));
            }

            Ok(None)
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<DataStore<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            let mut base = vec![
                // Fields
                "name".to_string(),
                "table_name".to_string(),
                // Methods
                "column_names".to_string(),
                "list".to_string(),
                "columns".to_string(),
                "filters_sql".to_string(),
                "validate_data_against_columns".to_string(),
                "methods".to_string(),
            ];

            if let Ok(ds) = ud.borrow::<DataStore<T>>() {
                base.extend(ds.ds_impl.methods());
            }

            create_userdata_iterator_with_dyn_fields(
                lua,
                ud,
                base,
            )
        });
    }
}

#[derive(Clone)]
pub struct DataStoreExecutor<T: KhronosContext> {
    context: T,
    datastore_provider: T::DataStoreProvider,
    custom_datastores: Rc<RefCell<HashMap<String, DataStore<T>>>>,
}

impl<T: KhronosContext> LuaUserData for DataStoreExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "DataStoreExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::NewIndex, |_, this, (key, value): (LuaValue, LuaValue)| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(());
                }
            };

            let ds = match value {
                LuaValue::UserData(ds) => {
                    if !ds.is::<DataStore<T>>() {
                        return Ok(());
                    }

                    ds.borrow::<DataStore<T>>()?
                }
                _ => {
                    return Ok(());
                }
            };

            this.custom_datastores.try_borrow_mut()
            .map_err(|_| LuaError::external("Failed to borrow custom datastores"))?
            .insert(key.to_string(), ds.clone());

            Ok(())
        });

        methods.add_meta_method(LuaMetaMethod::Index, |_, this, key: LuaValue| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(None);
                }
            };

            {
                let map = this.custom_datastores.try_borrow().map_err(|_| LuaError::external("Failed to borrow custom datastores"))?;
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
                columns_cache: Rc::new(RefCell::new(None))
            };

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

            create_userdata_iterator_with_dyn_fields(
                lua,
                ud,
                datastores,
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(
            |_, (token, scope): (TemplateContextRef<T>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let Some(datastore_provider) = token.context.datastore_provider(scope) else {
                    return Err(LuaError::external(
                        "The datastore plugin is not supported in this context",
                    ));
                };

                let executor = DataStoreExecutor {
                    context: token.context.clone(),
                    datastore_provider,
                    custom_datastores: Rc::new(RefCell::new(HashMap::new())),
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

