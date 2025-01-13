use crate::traits::context::KhronosContext;
use crate::traits::kvprovider::KVProvider;
use crate::TemplateContextRef;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::lua_promise;
use crate::utils::executorscope::ExecutorScope;

/// An kv executor is used to execute key-value ops from Lua
/// templates
#[derive(Clone)]
pub struct KvExecutor<T: KVProvider> {
    /// The guild ID to execute the operation on
    ///
    /// This can be either ThisGuild or OwnerGuild
    guild_id: Option<serenity::all::GuildId>,
    /// The origin guild id
    origin_guild_id: Option<serenity::all::GuildId>,
    scope: ExecutorScope,
    kv_provider: T,
}

/// Represents a full record complete with metadata
#[derive(Serialize, Deserialize)]
pub struct KvRecord {
    pub key: String,
    pub value: serde_json::Value,
    pub exists: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl KvRecord {
    fn default() -> KvRecord {
        KvRecord {
            key: "".to_string(),
            value: serde_json::Value::Null,
            exists: false,
            created_at: None,
            last_updated_at: None,
        }
    }
}

impl<T: KVProvider> LuaUserData for KvExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("guild_id", |lua, this| {
            let value = lua.to_value(&this.guild_id)?;
            Ok(value)
        });
        fields.add_field_method_get("origin_guild_id", |lua, this| {
            let value = lua.to_value(&this.origin_guild_id)?;
            Ok(value)
        });
        fields.add_field_method_get("scope", |_, this| Ok(this.scope.to_string()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("find", |_, this, key: String| {
            Ok(lua_promise!(this, key, |lua, this, key|, {
                let records = this.kv_provider.find(key).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                    .into_iter()
                    .map(|k| {
                        KvRecord {
                            key: k.key,
                            value: k.value,
                            exists: k.exists,
                            created_at: k.created_at,
                            last_updated_at: k.last_updated_at,
                        }
                    })
                    .collect::<Vec<KvRecord>>();

                let records: LuaValue = lua.to_value(&records)?;

                Ok(records)
            }))
        });

        methods.add_method("exists", |_, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                let exists = this.kv_provider.exists(key).await
                .map_err(|e| LuaError::external(e.to_string()))?;
                Ok(exists)
            }))
        });

        methods.add_method("get", |_, this, key: String| {
            Ok(lua_promise!(this, key, |lua, this, key|, {
                let record = this.kv_provider.get(key).await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                match record {
                    // Return None and true if record was found but value is null
                    Some(rec) => {
                        let value: LuaValue = lua.to_value(&rec.value)?;
                        Ok((Some(value), true))
                    },
                    // Return None and 0 if record was not found
                    None => Ok((None, false)),
                }
            }))
        });

        methods.add_method("getrecord", |_, this, key: String| {
            Ok(lua_promise!(this, key, |lua, this, key|, {
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

                let record: LuaValue = lua.to_value(&record)?;
                Ok(record)
            }))
        });

        methods.add_method("set", |_, this, (key, value): (String, LuaValue)| {
            Ok(lua_promise!(this, key, value, |lua, this, key, value|, {
                this.kv_provider.set(key, lua.from_value(value)?).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;
                Ok(())
            }))
        });

        methods.add_method("delete", |_lua, this, key: String| {
            Ok(lua_promise!(this, key, |_lua, this, key|, {
                this.kv_provider.delete(key).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(())
            }))
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
                let Some((guild_id, kv_provider)) = token.context.kv_executor(scope) else {
                    return Err(LuaError::external("KV provider not found"));
                };
                let executor = KvExecutor {
                    origin_guild_id: token.context.guild_id(),
                    guild_id,
                    scope,
                    kv_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
