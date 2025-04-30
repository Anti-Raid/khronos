use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::traits::kvprovider::KVProvider;
use crate::TemplateContextRef;
use mlua::prelude::*;
use crate::utils::khronos_value::KhronosValue;
use crate::to_struct;

use crate::lua_promise;
use crate::utils::executorscope::ExecutorScope;

/// An kv executor is used to execute key-value ops from Lua
/// templates
#[derive(Clone)]
pub struct KvExecutor<T: KhronosContext> {
    context: T,
    kv_scope: String,
    kv_provider: T::KVProvider,
}

to_struct!(
    /// Represents a full record complete with metadata
    pub struct KvRecord {
        pub key: String,
        pub value: KhronosValue,
        pub exists: bool,
        pub created_at: Option<chrono::DateTime<chrono::Utc>>,
        pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    }
);

impl From<crate::traits::ir::kv::KvRecord> for KvRecord {
    fn from(record: crate::traits::ir::kv::KvRecord) -> Self {
        KvRecord {
            key: record.key,
            exists: true,
            value: record.value,
            created_at: record.created_at,
            last_updated_at: record.last_updated_at,
        }
    }
}

impl From<KvRecord> for crate::traits::ir::kv::KvRecord {
    fn from(record: KvRecord) -> Self {
        crate::traits::ir::kv::KvRecord {
            key: record.key,
            value: record.value,
            created_at: record.created_at,
            last_updated_at: record.last_updated_at,
        }
    }
}

impl KvRecord {
    fn default() -> KvRecord {
        KvRecord {
            key: "".to_string(),
            value: KhronosValue::Null,
            exists: false,
            created_at: None,
            last_updated_at: None,
        }
    }
}

impl<T: KhronosContext> KvExecutor<T> {
    pub fn check_list_scopes(&self) -> Result<(), crate::Error> {
        if !self
            .context
            .has_cap("kv.meta:list_scopes") // KV:* means all KV operations are allowed
        {
            return Err(
                "The kv.meta:list_scopes capability is required to list scopes in this template context"
                .into()
            );
        }

        Ok(())
    }

    pub fn check_keys(&self) -> Result<(), crate::Error> {
        if !self
            .context
            .has_cap(&format!("kv.meta:{}:keys", self.kv_scope)) // kv:{scope}:meta:list_keys means that the action can be performed on any key
        {
            return Err(
                format!(
                    "The kv.meta:{}:keys capability is required to list keys in this scope for this template context",
                    self.kv_scope
                )
                .into()
            );
        }

        Ok(())
    }

    pub fn check(&self, action: String, key: String) -> Result<(), crate::Error> {
        if !self
        .context
        .has_cap(&format!("kv.{}:*", self.kv_scope)) // KV:* means all KV operations are allowed
        && !self
        .context
        .has_cap(&format!("kv.{}:{}:*", self.kv_scope, action)) // kv:{action}:* means that the action can be performed on any key
        && !self
        .context
        .has_cap(&format!("kv.{}:{}:{}", self.kv_scope, action, key)) // kv:{action}:{key} means that the action can only be performed on said key
        && !self
        .context
        .has_cap(&format!("kv.{}:*:{}", self.kv_scope, key))
        && self.kv_scope != "unscoped"
        // kv:*:{key} means that any action can be performed on said key
        {
            return Err(format!(
                "KV operation `{}` not allowed in this template context for key '{}' in scope '{}'",
                action, key,
                self.kv_scope
            )
            .into());
        }

        if self.kv_scope == "unscoped" 
        && !self
        .context
        .has_cap("kv:*") // KV:* means all KV operations are allowed
        && !self
        .context
        .has_cap(&format!("kv:{}:*", action)) // kv:{action}:* means that the action can be performed on any key
        && !self
        .context
        .has_cap(&format!("kv:{}:{}", action, key)) // kv:{action}:{key} means that the action can only be performed on said key
        && !self
        .context
        .has_cap(&format!("kv:*:{}", key))
        // kv:*:{key} means that any action can be performed on said key
        {
            return Err(format!(
                "KV operation `{}` not allowed in this template context for key '{}'",
                action, key
            )
            .into());
        }

        self.kv_provider.attempt_action(&action)?; // Check rate limits

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for KvExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "KvExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Type, |_, _this, _: ()| Ok("KvExecutor"));
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, _: ()| Ok("KvExecutor"));

        methods.add_method("list_scopes", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_list_scopes()
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let scopes = this.kv_provider.list_scopes().await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(scopes)
            }))
        });

        methods.add_method("find", |_, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                this.check("find".to_string(), key.clone())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let records = this.kv_provider.find(key).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                    .into_iter()
                    .map(|k| {
                        KvRecord {
                            key: k.key,
                            value: k.value,
                            exists: true,
                            created_at: k.created_at,
                            last_updated_at: k.last_updated_at,
                        }
                    })
                    .collect::<Vec<KvRecord>>();

                Ok::<KhronosValue, LuaError>(records.into())
            }))
        });

        methods.add_method("exists", |_, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                this.check("exists".to_string(), key.clone())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let exists = this.kv_provider.exists(key).await
                .map_err(|e| LuaError::external(e.to_string()))?;
                Ok(exists)
            }))
        });

        methods.add_method("get", |_, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                log::info!("Starting get operation");

                this.check("get".to_string(), key.clone())
                .map_err(|e| {
                    LuaError::runtime(e.to_string())
                })?;

                log::info!("Getting key: {}", key);

                let record = this.kv_provider.get(key).await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                match record {
                    // Return None and true if record was found but value is null
                    Some(rec) => {
                        Ok((Some(rec.value), true))
                    },
                    // Return None and 0 if record was not found
                    None => Ok((None, false)),
                }
            }))
        });

        methods.add_method("getrecord", |_, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                this.check("get".to_string(), key.clone())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let record = this.kv_provider.get(key).await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                let record = match record {
                    Some(rec) => KvRecord {
                        key: rec.key,
                        value: rec.value,
                        exists: true,
                        created_at: rec.created_at,
                        last_updated_at: rec.last_updated_at,
                    },
                    None => KvRecord::default(),
                };

                Ok::<KhronosValue, LuaError>(record.into())
            }))
        });

        methods.add_method("keys", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_keys()
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let keys = this.kv_provider.keys().await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(keys)
            }))
        });

        methods.add_method("set", |_, this, (key, value): (String, LuaValue)| {
            Ok(lua_promise!(this, key, value, |lua, this, key, value|, {
                this.check("set".to_string(), key.clone())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let value = KhronosValue::from_lua(value, &lua)?;

                this.kv_provider.set(key, value).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;
                Ok(())
            }))
        });

        methods.add_method("delete", |_lua, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                this.check("delete".to_string(), key.clone())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.kv_provider.delete(key).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<KvExecutor<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    // Methods
                    "list_scopes",
                    "find",
                    "exists",
                    "get",
                    "getrecord",
                    "keys",
                    "set",
                    "delete",
                ],
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(
            |_, (token, scope, kv_scope): (TemplateContextRef<T>, Option<String>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let kv_scope = kv_scope.unwrap_or("unscoped".to_string());
                let Some(kv_provider) = token.context.kv_provider(scope, &kv_scope) else {
                    return Err(LuaError::external(
                        "The key-value plugin is not supported in this context",
                    ));
                };
                let executor = KvExecutor {
                    context: token.context.clone(),
                    kv_provider,
                    kv_scope
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
