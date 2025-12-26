use std::collections::HashMap;

use mluau::prelude::*;
use serenity::all::{GuildId, UserId};

use crate::{core::{datetime::DateTime, typesext::Vfs}, primitives::lazy::Lazy};

/**
 * TODO:
--- Runtime plugin provides basic hooks into the AntiRaid template-worker runtime
export type Plugin = {
    --- @yields
    ---
    --- Lists all templates
    listtemplates: (self: Plugin) -> { Template },

    --- @yields
    ---
    --- Gets a template by name
    gettemplate: (self: Plugin, id: string) -> Template?,

    --- @yields
    ---
    --- Creates an existing template
    createtemplate: (self: Plugin, template: CreateTemplate) -> nil,

    --- @yields
    ---
    --- Creates an existing template
    updatetemplate: (self: Plugin, id: string, template: CreateTemplate) -> nil,

    --- @yields
    ---
    --- Deletes a template by name
    deletetemplate: (self: Plugin, id: string) -> nil,

    --- @yields
    ---
    --- Fetches the TenantState or returns a suitable default
    gettenantstate: (self: Plugin) -> TenantState,

    --- @yields
    ---
    --- Sets the TenantState
    settenantstate: (self: Plugin, state: TenantState) -> (),

    --- @yields
    ---
    --- Returns the statistics of the bot.
    stats: (self: Plugin) -> {
        total_cached_guilds: number,
        total_guilds: number,
        total_users: number,
        last_started_at: datetime.DateTime,
    },

    --- Returns various important links
    --- of the bot
    links: (self: Plugin) -> {
        support_server: string,
        api_url: string,
        frontend_url: string,
        docs_url: string,
    },

    --- Returns the list of events the bot can dispatch
    event_list: (self: Plugin) -> {string},
}

return {}
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateOwner {
    User { id: UserId },
    Guild { id: GuildId },
}

impl IntoLua for TemplateOwner {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        match self {
            TemplateOwner::User { id } => {
                table.set("type", "user")?;
                table.set("id", id.to_string())?;
            }
            TemplateOwner::Guild { id } => {
                table.set("type", "guild")?;
                table.set("id", id.to_string())?;
            }
        }

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSource {
    Builtins,
    Shop { shop_listing: String },
    Custom {
        name: String,
        language: String,
        content: HashMap<String, String>,
    },
}

impl IntoLua for TemplateSource {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        match self {
            TemplateSource::Builtins => {
                table.set("type", "builtins")?;
            }
            TemplateSource::Shop { shop_listing } => {
                table.set("type", "shop")?;
                table.set("shop_listing", shop_listing)?;
            }
            TemplateSource::Custom {
                name,
                language,
                content,
            } => {
                table.set("type", "custom")?;
                table.set("name", name)?;
                table.set("language", language)?;
                table.set("content", Lazy::new(content))?;
            }
        }

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone)]
pub struct Template {
    pub id: String,
    pub owner: TemplateOwner,
    pub source: TemplateSource,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    pub allowed_caps: Vec<String>,
    pub vfs: Vfs,
    pub paused: bool,
}

impl IntoLua for Template {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        table.set("id", self.id)?;
        table.set("owner", self.owner.into_lua(lua)?)?;
        table.set("source", self.source.into_lua(lua)?)?;
        table.set("created_at", DateTime::from_utc(self.created_at))?;
        table.set("last_updated_at", DateTime::from_utc(self.last_updated_at))?;
        table.set("allowed_caps", self.allowed_caps)?;
        table.set("vfs", self.vfs)?;
        table.set("paused", self.paused)?;

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateTemplateSource {
    Builtins,
    Shop { shop_listing: String },
    Custom {
        name: String,
        language: String,
        content: HashMap<String, String>,
    },
}

impl FromLua for CreateTemplateSource {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "CreateTemplateSource".to_string(),
                    message: Some("expected a table".to_string()),
                })
            }
        };

        let source_type: String = table.get("type")?;

        match source_type.as_str() {
            "builtins" => Ok(CreateTemplateSource::Builtins),
            "shop" => {
                let shop_listing: String = table.get("shop_listing")?;
                Ok(CreateTemplateSource::Shop { shop_listing })
            }
            "custom" => {
                let name: String = table.get("name")?;
                let language: String = table.get("language")?;
                let content: LuaValue = table.get("content")?;
                let content: HashMap<String, String> = lua.from_value(content)?;
                Ok(CreateTemplateSource::Custom {
                    name,
                    language,
                    content,
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: "string",
                to: "CreateTemplateSource".to_string(),
                message: Some(format!("unknown template source type: {}", source_type)),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTemplate {
    pub source: CreateTemplateSource,
    pub allowed_caps: Vec<String>,
    pub paused: bool,
}

impl FromLua for CreateTemplate {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "CreateTemplate".to_string(),
                    message: Some("expected a table".to_string()),
                })
            }
        };

        let source_value: LuaValue = table.get("source")?;
        let source: CreateTemplateSource = CreateTemplateSource::from_lua(source_value, lua)?;

        let allowed_caps: Vec<String> = table.get("allowed_caps")?;
        let paused: bool = table.get("paused")?;

        Ok(CreateTemplate {
            source,
            allowed_caps,
            paused,
        })
    }
}

// Tenant State
#[derive(Debug, Clone)]
pub struct TenantState {
    pub events: Vec<String>,
    pub banned: bool,
    pub flags: u32,
    pub startup_events: bool,
}

impl FromLua for TenantState {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
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
        let flags: u32 = table.get("flags")?;
        let startup_events: bool = table.get("startup_events")?;

        Ok(TenantState {
            events,
            banned,
            flags,
            startup_events,
        })
    }
}

impl IntoLua for TenantState {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        table.set("events", self.events)?;
        table.set("banned", self.banned)?;
        table.set("flags", self.flags)?;
        table.set("startup_events", self.startup_events)?;

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}