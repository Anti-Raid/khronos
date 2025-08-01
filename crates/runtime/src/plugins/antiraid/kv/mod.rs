use std::rc::Rc;

use crate::core::datetime::DateTimeRef;
use crate::to_struct;
use crate::traits::context::{KhronosContext, Limitations};
use crate::traits::kvprovider::KVProvider;
use crate::utils::khronos_value::KhronosValue;
use crate::TemplateContext;
use chrono::Utc;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// An kv executor is used to execute key-value ops from Lua
/// templates
pub struct KvExecutor<T: KhronosContext> {
    limitations: Rc<Limitations>,
    kv_provider: T::KVProvider,
}

to_struct!(
    /// Represents a result of a set operation in the key-value store
    pub struct SetResult {
        pub exists: bool, // If true, the key already existed
        pub id: String,   // The ID of the record
    }
);

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
        pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
        pub resume: bool,
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
            expires_at: record.expires_at,
            resume: record.resume,
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
            expires_at: record.expires_at,
            resume: record.resume,
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
            expires_at: None,
            resume: false,
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
        action: String,
        key: String,
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
            self.kv_provider.attempt_action(scopes, &action)?; // Check rate limits
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
                self.kv_provider.attempt_action(scopes, &action)?; // Check rate limits
                return Ok(()); // allow if any scope succeeds
            }
        }

        Err(format!(
            "KV operation `{action}` not allowed in this template context for key '{key}'",
        )
        .into())
    }

    pub fn id_check(&self, action: &str) -> Result<(), crate::Error> {
        if !self.limitations.has_cap("kv.meta:id_ops") {
            return Err(
                "The kv.meta:id_ops capability is required to use ID-taking methods".into(),
            );
        }

        self.kv_provider.attempt_action(&[], action)?; // Check rate limits

        Ok(())
    }

    pub fn validate_expiry(
        &self,
        expiry: Option<DateTimeRef>,
    ) -> LuaResult<Option<chrono::DateTime<Utc>>> {
        match expiry {
            Some(dt) => {
                let dt = dt.dt.with_timezone(&chrono_tz::Tz::UTC).to_utc();

                if dt <= chrono::Utc::now() {
                    return Err(LuaError::external("Expiry time must be in the future"));
                }

                Ok(Some(dt))
            }
            None => Ok(None),
        }
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
                        expires_at: k.expires_at,
                        resume: k.resume,
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
            async move |_, this, (key, scopes): (String, Vec<String>)| {
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
            async move |_, this, (key, scopes): (String, Vec<String>)| {
                this.check(&scopes, "get".to_string(), key.clone())
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

        methods.add_scheduler_async_method("getbyid", async move |_, this, id: String| {
            this.id_check("getbyid")
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let record = this
                .kv_provider
                .get_by_id(id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            match record {
                // Return None and true if record was found but value is null
                Some(rec) => Ok((Some(rec.value), true)),
                // Return None and 0 if record was not found
                None => Ok((None, false)),
            }
        });

        methods.add_scheduler_async_method(
            "getrecord",
            async move |_, this, (key, scopes): (String, Vec<String>)| {
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
                        expires_at: rec.expires_at,
                        resume: rec.resume,
                    },
                    None => KvRecord::default(),
                };

                let v: KhronosValue = record
                    .try_into()
                    .map_err(|x: crate::Error| LuaError::external(x.to_string()))?;

                Ok(v)
            },
        );

        methods.add_scheduler_async_method("getrecordbyid", async move |_, this, id: String| {
            this.id_check("getbyid")
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let record = this
                .kv_provider
                .get_by_id(id)
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
                    expires_at: rec.expires_at,
                    resume: rec.resume,
                },
                None => KvRecord::default(),
            };

            let v: KhronosValue = record
                .try_into()
                .map_err(|x: crate::Error| LuaError::external(x.to_string()))?;

            Ok(v)
        });

        methods.add_scheduler_async_method("keys", async move |_, this, scopes: Vec<String>| {
            this.check_keys(&scopes)
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let keys = this
                .kv_provider
                .keys(&scopes)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(keys)
        });

        methods.add_scheduler_async_method(
            "set",
            async move |lua,
                        this,
                        (key, value, scopes, expires_at, resume): (
                String,
                LuaValue,
                Vec<String>,
                Option<DateTimeRef>,
                Option<bool>,
            )| {
                if scopes.is_empty() {
                    return Err(LuaError::external(
                        "Setting keys without a scope is not allowed".to_string(),
                    ));
                }
                this.check(&scopes, "set".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let value = KhronosValue::from_lua(value, &lua)?;
                let expires_at = this.validate_expiry(expires_at)?;

                let (exists, id) = this
                    .kv_provider
                    .set(&scopes, key, value, expires_at, resume.unwrap_or(false))
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let rec = SetResult { exists, id };

                let rec: KhronosValue = rec
                    .try_into()
                    .map_err(|x: crate::Error| LuaError::external(x.to_string()))?;

                Ok(rec)
            },
        );

        methods.add_scheduler_async_method(
            "setbyid",
            async move |lua,
                        this,
                        (id, value, expires_at, resume): (
                String,
                LuaValue,
                Option<DateTimeRef>,
                Option<bool>,
            )| {
                this.id_check("setbyid")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let value = KhronosValue::from_lua(value, &lua)?;
                let expires_at = this.validate_expiry(expires_at)?;

                this
                    .kv_provider
                    .set_by_id(id, value, expires_at, resume.unwrap_or(false))
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "setexpiry",
            async move |_lua,
                        this,
                        (key, scopes, expires_at): (
                String,
                Vec<String>,
                Option<DateTimeRef>,
            )| {
                this.check(&scopes, "setexpiry".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let expires_at = this.validate_expiry(expires_at)?;

                this.kv_provider
                    .set_expiry(&scopes, key, expires_at)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;
                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "setexpirybyid",
            async move |_lua, this, (id, expires_at): (String, Option<DateTimeRef>)| {
                this.id_check("setexpirybyid")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let expires_at = this.validate_expiry(expires_at)?;

                this.kv_provider
                    .set_expiry_by_id(id, expires_at)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;
                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "delete",
            async move |_, this, (key, scopes): (String, Vec<String>)| {
                this.check(&scopes, "delete".to_string(), key.clone())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.kv_provider
                    .delete(&scopes, key)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method("deletebyid", async move |_, this, id: String| {
            this.id_check("deletebyid")
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            this.kv_provider
                .delete_by_id(id)
                .await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            Ok(())
        });
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
