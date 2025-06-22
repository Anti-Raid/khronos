use crate::primitives::create_userdata_iterator_with_fields;
use crate::to_struct;
use crate::traits::context::KhronosContext;
use crate::traits::kvprovider::KVProvider;
use crate::utils::khronos_value::KhronosValue;
use crate::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsyncUserData;

/// An kv executor is used to execute key-value ops from Lua
/// templates
pub struct KvExecutor<T: KhronosContext> {
    context: T,
    kv_provider: T::KVProvider,
    unscoped_allowed: bool, // If false, the executor will not allow unscoped operations
}

to_struct!(
    /// Represents a full record complete with metadata
    pub struct KvRecord {
        pub id: String,
        pub key: String,
        pub value: KhronosValue,
        pub scopes: Vec<String>,
        pub exists: bool,
        pub created_at: Option<chrono::DateTime<chrono::Utc>>,
        pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    }
);

impl From<crate::traits::ir::kv::KvRecord> for KvRecord {
    fn from(record: crate::traits::ir::kv::KvRecord) -> Self {
        KvRecord {
            id: record.id,
            key: record.key,
            exists: true,
            value: record.value,
            scopes: record.scopes,
            created_at: record.created_at,
            last_updated_at: record.last_updated_at,
        }
    }
}

impl From<KvRecord> for crate::traits::ir::kv::KvRecord {
    fn from(record: KvRecord) -> Self {
        crate::traits::ir::kv::KvRecord {
            id: record.id,
            key: record.key,
            value: record.value,
            scopes: record.scopes,
            created_at: record.created_at,
            last_updated_at: record.last_updated_at,
        }
    }
}

impl KvRecord {
    fn default() -> KvRecord {
        KvRecord {
            id: "".to_string(),
            key: "".to_string(),
            value: KhronosValue::Null,
            scopes: vec![],
            exists: false,
            created_at: None,
            last_updated_at: None,
        }
    }
}

impl<T: KhronosContext> KvExecutor<T> {
    pub fn check_list_scopes(&self) -> Result<(), crate::Error> {
        if !self.context.has_cap("kv.meta:list_scopes")
        // KV:* means all KV operations are allowed
        {
            return Err(
                "The kv.meta:list_scopes capability is required to list scopes in this template context"
                .into()
            );
        }

        Ok(())
    }

    pub fn check_keys(&self, scopes: &[String]) -> Result<(), crate::Error> {
        if scopes.is_empty() && !self.unscoped_allowed {
            return Err("Unscoped operations are not allowed in this executor".into());
        }

        if self.context.has_cap("kv.meta:keys") {
            return Ok(());
        }

        for scope in scopes {
            if !self.context.has_cap(&format!("kv.meta[{}]:keys", scope))
            // kv.meta[{scope}]:keys means that the action can be performed on any key
            {
                return Err(
                    format!(
                        "Either kv.meta[{}]:keys or kv.meta:keys capability is required to list keys in this template context for the specified scopes ({}).",
                        scope,
                        scopes.join(", ")
                    )
                    .into()
                );
            }
        }

        Ok(())
    }

    pub fn check(
        &self,
        scopes: &[String],
        action: String,
        key: String,
    ) -> Result<(), crate::Error> {
        if scopes.is_empty() && !self.unscoped_allowed {
            return Err("Unscoped operations are not allowed in this executor".into());
        }

        if self.context.has_cap("kv:*") // KV:* means all KV operations are allowed
        || self.context.has_cap(&format!("kv:{}:*", action)) // kv:{action} means that the action can be performed on any key
        ||  self
            .context
            .has_cap(&format!("kv:{}:{}", action, key))
        // kv:{action}:{key} means that the action can only be performed on said key
        {
            return Ok(()); // No need to check scopes if the action is allowed globally or for the specific key
        }

        for scope in scopes {
            if !self
            .context
            .has_cap(&format!("kv[{}]:*", scope)) // kv[{scopes}]:* means that the scope can be performed on any key
            && !self
                .context
                .has_cap(&format!("kv[{}]:{}", scope, action)) // kv[{scopes}]:{action} means that the action can be performed on any key in the scope
            && !self
                .context
                .has_cap(&format!("kv[{}]:{}:{}", scope, action, key))
            // kv[{scopes}]:{action}:{key} means that the action can only be performed on said key in the scope
            {
                return Err(format!(
                    "KV operation `{}` not allowed in this template context for key '{}' in scope '{}'",
                    action,
                    key,
                    scope
                )
                .into());
            }
        }

        self.kv_provider.attempt_action(&scopes, &action)?; // Check rate limits

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for KvExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "KvExecutor");

        fields.add_field_method_get("unscoped_allowed", |_, this| {
            if !this.context.has_cap("kv.meta:unscoped_allowed") {
                return Err(LuaError::external(
                    "The kv.meta:unscoped_allowed capability is required to access this field",
                ));
            }

            Ok(this.unscoped_allowed)
        });

        fields.add_field_method_set("unscoped_allowed", |_, this, value: bool| {
            if !this.context.has_cap("kv.meta:unscoped_allowed:set") {
                return Err(LuaError::external(
                    "The kv.meta:unscoped_allowed:set capability is required to set this field",
                ));
            }

            this.unscoped_allowed = value;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Type, |_, _this, _: ()| Ok("KvExecutor"));
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, _: ()| Ok("KvExecutor"));

        methods.add_scheduler_async_method("list_scopes", async move |_, this, _g: ()| {
            this.check_list_scopes()
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let scopes = this
                .kv_provider
                .list_scopes()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(scopes)
        });

        methods.add_scheduler_async_method(
            "find",
            async move |_, this, (key, scopes): (String, Option<Vec<String>>)| {
                let scopes = scopes.unwrap_or_default();
                this.check(&scopes, "find".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let records = this
                    .kv_provider
                    .find(&scopes, key)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?
                    .into_iter()
                    .map(|k| KvRecord {
                        id: k.id,
                        key: k.key,
                        value: k.value,
                        scopes: k.scopes,
                        exists: true,
                        created_at: k.created_at,
                        last_updated_at: k.last_updated_at,
                    })
                    .collect::<Vec<KvRecord>>();

                let v: KhronosValue = records
                    .try_into()
                    .map_err(|x: crate::Error| LuaError::external(x.to_string()))?;

                Ok(v)
            },
        );

        methods.add_scheduler_async_method(
            "exists",
            async move |_, this, (key, scopes): (String, Option<Vec<String>>)| {
                let scopes = scopes.unwrap_or_default();
                this.check(&scopes, "exists".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let exists = this
                    .kv_provider
                    .exists(&scopes, key)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;
                Ok(exists)
            },
        );

        methods.add_scheduler_async_method(
            "get",
            async move |_, this, (key, scopes): (String, Option<Vec<String>>)| {
                let scopes = scopes.unwrap_or_default();
                log::info!("Starting get operation");

                this.check(&scopes, "get".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                log::info!("Getting key: {}", key);

                let record = this
                    .kv_provider
                    .get(&scopes, key)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                match record {
                    // Return None and true if record was found but value is null
                    Some(rec) => Ok((Some(rec.value), true)),
                    // Return None and 0 if record was not found
                    None => Ok((None, false)),
                }
            },
        );

        methods.add_scheduler_async_method(
            "getrecord",
            async move |_, this, (key, scopes): (String, Option<Vec<String>>)| {
                let scopes = scopes.unwrap_or_default();
                this.check(&scopes, "get".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let record = this
                    .kv_provider
                    .get(&scopes, key)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                let record = match record {
                    Some(rec) => KvRecord {
                        id: rec.id,
                        key: rec.key,
                        value: rec.value,
                        scopes: rec.scopes,
                        exists: true,
                        created_at: rec.created_at,
                        last_updated_at: rec.last_updated_at,
                    },
                    None => KvRecord::default(),
                };

                let v: KhronosValue = record
                    .try_into()
                    .map_err(|x: crate::Error| LuaError::external(x.to_string()))?;

                Ok(v)
            },
        );

        methods.add_scheduler_async_method(
            "keys",
            async move |_, this, scopes: Option<Vec<String>>| {
                let scopes = scopes.unwrap_or_default();
                this.check_keys(&scopes)
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let keys = this
                    .kv_provider
                    .keys(&scopes)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(keys)
            },
        );

        methods.add_scheduler_async_method(
            "set",
            async move |lua,
                        this,
                        (key, value, scopes): (String, LuaValue, Option<Vec<String>>)| {
                let scopes = scopes.unwrap_or_default();
                this.check(&scopes, "set".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let value = KhronosValue::from_lua(value, &lua)?;

                this.kv_provider
                    .set(&scopes, key, value)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;
                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "delete",
            async move |_, this, (key, scopes): (String, Option<Vec<String>>)| {
                let scopes = scopes.unwrap_or_default();
                this.check(&scopes, "delete".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.kv_provider
                    .delete(&scopes, key)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            },
        );

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

pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaValue> {
    let Some(kv_provider) = token.context.kv_provider() else {
        return Err(LuaError::external(
            "The key-value plugin is not supported in this context",
        ));
    };
    let executor = KvExecutor {
        context: token.context.clone(),
        kv_provider,
        unscoped_allowed: token
            .context
            .compatibility_flags()
            .contains(crate::traits::context::CompatibilityFlags::ALLOW_UNSCOPED_KV),
    }
    .into_lua(lua)?;

    Ok(executor)
}
