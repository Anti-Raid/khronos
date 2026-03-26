use crate::core::datetime::DateTime;
use crate::traits::context::{KhronosContext};
use crate::traits::kvprovider::KVProvider;
use crate::utils::khronos_value::KhronosValue;
use crate::TemplateContext;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// An kv executor is used to execute key-value ops from Lua
/// templates
pub struct KvExecutor<T: KhronosContext> {
    kv_provider: T::KVProvider,
}

/// Represents a full record complete with metadata
pub struct KvRecord {
    pub id: String,
    pub key: String,
    pub value: KhronosValue,
    pub scope: String,
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
        table.set("scope", self.scope)?;
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
            scope: record.scope,
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
            scope: record.scope,
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
            scope: "".to_string(),
            exists: false,
            created_at: None,
            last_updated_at: None,
        }
    }
}

impl<T: KhronosContext> KvExecutor<T> {
    pub fn check(
        &self,
        action: &str,
    ) -> Result<(), crate::Error> {
        self.kv_provider.attempt_action(action)?;
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

        methods.add_scheduler_async_method("list_scopes", async move |_, this, _g: ()| {
            this.check("list_scopes")
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
            async move |_, this, (key, scope): (String, String)| {
                this.check("find")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let records = this
                    .kv_provider
                    .find(scope, key)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?
                    .into_iter()
                    .map(|k| KvRecord {
                        id: k.id,
                        key: k.key,
                        value: k.value,
                        scope: k.scope,
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
            async move |_, this, (key, scope): (String, String)| {
                this.check("get")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let record = this
                    .kv_provider
                    .get(scope, key)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                let record = match record {
                    Some(rec) => KvRecord {
                        id: rec.id,
                        key: rec.key,
                        value: rec.value,
                        scope: rec.scope,
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
                        (key, value, scope): (
                String,
                LuaValue,
                String,
            )| {
                this.check("set")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let value = KhronosValue::from_lua(value, &lua)?;

                this
                    .kv_provider
                    .set(scope, key, value)
                    .await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "delete",
            async move |_, this, (key, scope): (String, String)| {
                this.check("delete")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.kv_provider
                    .delete(scope, key)
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
        kv_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}
