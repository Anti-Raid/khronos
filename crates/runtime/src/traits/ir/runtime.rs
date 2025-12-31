use mluau::prelude::*;

use crate::core::datetime::DateTime;

// Tenant State
#[derive(Debug, Clone)]
pub struct TenantState {
    pub events: Vec<String>,
    pub banned: bool,
    pub data: serde_json::Value,
}

impl FromLua for TenantState {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "TenantState".to_string(),
                    message: Some("expected a table".to_string()),
                })
            }
        };

        let events: Vec<String> = table.get("events")?;
        let banned: bool = table.get("banned")?;
        let data: LuaValue = table.get("data")?;
        let data = lua.from_value(data)?;

        Ok(TenantState {
            events,
            banned,
            data,
        })
    }
}

impl IntoLua for TenantState {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        table.set("events", self.events)?;
        table.set("banned", self.banned)?;
        table.set("data", lua.to_value(&self.data)?)?;

        // Note that we do not set tenant state to readonly as we may want to mutate it

        Ok(LuaValue::Table(table))
    }
}

pub struct RuntimeStats {
    pub total_cached_guilds: u64,
    pub total_guilds: u64,
    pub total_users: u64,
    pub last_started_at: chrono::DateTime<chrono::Utc>,
}

impl IntoLua for RuntimeStats {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        table.set("total_cached_guilds", self.total_cached_guilds)?;
        table.set("total_guilds", self.total_guilds)?;
        table.set("total_users", self.total_users)?;
        table.set("last_started_at", DateTime::from_utc(self.last_started_at))?;

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}

pub struct RuntimeLinks {
    pub support_server: String,
    pub api_url: String,
    pub frontend_url: String,
    pub docs_url: String,
}

impl IntoLua for RuntimeLinks {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        table.set("support_server", self.support_server)?;
        table.set("api_url", self.api_url)?;
        table.set("frontend_url", self.frontend_url)?;
        table.set("docs_url", self.docs_url)?;

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}