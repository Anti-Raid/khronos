use std::rc::Rc;

use crate::core::datetime::DateTime;
use crate::traits::context::{KhronosContext, Limitations};
use crate::traits::kvprovider::KVProvider;
use crate::utils::khronos_value::KhronosValue;
use crate::TemplateContext;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// An kv executor is used to execute key-value ops from Lua
/// templates
pub struct KvExecutor<T: KhronosContext> {
    limitations: Rc<Limitations>,
    kv_provider: T::KVProvider,
}

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

impl IntoLua for KvRecord {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("id", self.id)?;
        table.set("key", self.key)?;
        table.set("value", self.value)?;
        table.set("scopes", self.scopes)?;
        table.set("exists", self.exists)?;
        table.set("created_at", match self.created_at {
            Some(dt) => DateTime::from_utc(dt).into_lua(lua)?,
            None => LuaValue::Nil,
        })?;
        table.set("last_updated_at", match self.last_updated_at {
            Some(dt) => DateTime::from_utc(dt).into_lua(lua)?,
            None => LuaValue::Nil,
        })?;
        table.set_readonly(true); // We want KvRecords to be immutable
        Ok(LuaValue::Table(table))
    }
}

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
        if !self.limitations.has_cap("kv.meta:list_scopes")
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
        if scopes.is_empty() {
            return Err("Unscoped operations are not allowed".into());
        }

        if self.limitations.has_cap("kv.meta:keys") {
            return Ok(());
        }

        for scope in scopes {
            if !self.limitations.has_cap(&format!("kv.meta[{scope}]:keys"))
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
        action: &str,
        key: &str,
    ) -> Result<(), crate::Error> {
        if scopes.is_empty() {
            return Err("Unscoped operations are not allowed".into());
        }

        if self.limitations.has_cap("kv:*") // KV:* means all KV operations are allowed
        || self.limitations.has_cap(&format!("kv:{action}:*")) // kv:{action} means that the action can be performed on any key
        ||  self
            .limitations
            .has_cap(&format!("kv:{action}:{key}"))
        // kv:{action}:{key} means that the action can only be performed on said key
        {
            self.kv_provider.attempt_action(scopes, action)?; // Check rate limits
            return Ok(()); // No need to check scopes if the action is allowed globally or for the specific key
        }

        for scope in scopes {
            if self
            .limitations
            .has_cap(&format!("kv[{scope}]:*")) // kv[{scopes}]:* means that the action can be performed on any key in the scope
            || self
                .limitations
                .has_cap(&format!("kv[{scope}]:{action}")) // kv[{scopes}]:{action} means that the action can be performed on any key in the scope
            || self
                .limitations
                .has_cap(&format!("kv[{scope}]:{action}:{key}"))
            // kv[{scopes}]:{action}:{key} means that the action can only be performed on said key in the scope
            {
                self.kv_provider.attempt_action(scopes, action)?; // Check rate limits
                return Ok(()); // allow if any scope succeeds
            }
        }

        Err(format!(
            "KV operation `{action}` not allowed in this template context for key '{key}'",
        )
        .into())
    }
}

impl<T: KhronosContext> LuaUserData for KvExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "KvExecutor");
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
            async move |_, this, (key, scopes): (String, Vec<String>)| {
                this.check(&scopes, "find", &key)
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

                Ok(records)
            },
        );

        methods.add_scheduler_async_method(
            "get",
            async move |_, this, (key, scopes): (String, Vec<String>)| {
                this.check(&scopes, "get", &key)
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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
            async move |_, this, (key, scopes): (String, Vec<String>)| {
                this.check(&scopes, "get", &key)
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

                Ok(record)
            },
        );

        methods.add_scheduler_async_method(
            "set",
            async move |lua,
                        this,
                        (key, value, scopes): (
                String,
                LuaValue,
                Vec<String>,
            )| {
                if scopes.is_empty() {
                    return Err(LuaError::external(
                        "Setting keys without a scope is not allowed".to_string(),
                    ));
                }
                this.check(&scopes, "set", &key)
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let value = KhronosValue::from_lua(value, &lua)?;

                this
                    .kv_provider
                    .set(&scopes, key, value)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "delete",
            async move |_, this, (key, scopes): (String, Vec<String>)| {
                this.check(&scopes, "delete", &key)
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.kv_provider
                    .delete(&scopes, key)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            },
        );
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
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
    let executor = KvExecutor::<T> {
        limitations: token.limitations.clone(),
        kv_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}
