use std::collections::HashSet;

use crate::{TemplateContext, primitives::event::CreateEvent, traits::runtimeprovider::RuntimeProvider};

use super::{
    httpclientprovider::HTTPClientProvider, httpserverprovider::HTTPServerProvider, 
    kvprovider::KVProvider, objectstorageprovider::ObjectStorageProvider,
    globalkvprovider::GlobalKVProvider
};
use dapi::controller::DiscordProvider;
use mluau::prelude::*;

/// Represents the data to be passed into ctx:with()
pub struct KhronosValueWith {
    pub limitations: Limitations,
    pub event: Option<CreateEvent>,
}

impl FromLua for KhronosValueWith {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => return Err(LuaError::FromLuaConversionError { from: value.type_name(), to: "KhronosValueWith".to_string(), message: Some("Expected a table".to_string()) }),
        };

        let limitations: Limitations = table.get("limitations")?;
        let event: Option<CreateEvent> = table.get("event")?;

        Ok(KhronosValueWith { limitations, event })
    }
}

/// Represents a result of a set operation in the key-value store
pub struct Limitations {
    pub capabilities: HashSet<String>,
    pub reserved_key_scopes: HashSet<String>,
}

impl Limitations {
    /// Returns a new limitations instance with the given capabilities
    pub fn new(capabilities: HashSet<String>, reserved_key_scopes: HashSet<String>) -> Self {
        Self { capabilities, reserved_key_scopes }
    }

    /// Returns Ok(()) if `other` is a subset of `self`, otherwise returns an error
    pub fn subset_of(&self, other: &Self) -> Result<(), String> {
        for cap in &self.capabilities {
            if !other.has_cap(cap) {
                return Err(format!("Missing capability: {cap}. A context can only be limited into a set of limitations that are strictly a subset of itself"));
            }
        }
        for scope in &other.reserved_key_scopes {
            if !self.has_reserved_key_scope(scope) {
                return Err(format!("Missing reserved key scope: {scope}. A context can only be limited into a set of limitations that are strictly a subset of itself"));
            }
        }
        Ok(())
    }

    /// Checks if the limitations has a specific reserved scope (cannot be interacted with in key-value api)
    pub fn has_reserved_key_scope(&self, scope: &str) -> bool {
        self.reserved_key_scopes.contains(scope)
    }

    /// Checks if the limitations has any of a set of reserved scopes (cannot be interacted with in key-value api)
    pub fn has_any_reserved_key_scope(&self, scopes: &[String]) -> bool {
        for scope in scopes {
            if self.has_reserved_key_scope(scope) {
                return true;
            }
        }
        false
    }

    /// Checks if the limitations allow a specific capability
    pub fn has_cap(&self, cap: &str) -> bool {
        self.capabilities.contains(cap) || self.capabilities.contains("*")
    }

    /// Checks if the limitations allow any of a set of capabilities
    pub fn has_any_cap(&self, caps: &[String]) -> bool {
        for cap in caps {
            if self.has_cap(cap) {
                return true;
            }
        }
        false
    }
}

impl FromLua for Limitations {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        Limitations::from_lua_impl(value)
    }
}

impl Limitations {
    pub fn from_lua_impl(value: LuaValue) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => return Err(LuaError::FromLuaConversionError { from: value.type_name(), to: "Limitations".to_string(), message: Some("Expected a table".to_string()) }),
        };

        let capabilities: Vec<String> = table.get("capabilities")?;
        let reserved_key_scopes: Vec<String> = table.get("reserved_key_scopes")?;

        Ok(Limitations::new(HashSet::from_iter(capabilities), HashSet::from_iter(reserved_key_scopes)))
    }

    pub fn into_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;

        let capabilities_table = lua.create_table()?;
        for cap in &self.capabilities {
            capabilities_table.raw_push(cap.as_str())?;
        }
        table.set("capabilities", capabilities_table)?;

        let reserved_key_scopes_table = lua.create_table()?;
        for scope in &self.reserved_key_scopes {
            reserved_key_scopes_table.raw_push(scope.as_str())?;
        }
        table.set("reserved_key_scopes", reserved_key_scopes_table)?;

        table.set_readonly(true);

        Ok(LuaValue::Table(table))
    }
}

pub trait KhronosContext: 'static + Clone + Sized {
    type KVProvider: KVProvider;
    type GlobalKVProvider: GlobalKVProvider;
    type DiscordProvider: DiscordProvider;
    type ObjectStorageProvider: ObjectStorageProvider;
    type HTTPClientProvider: HTTPClientProvider;
    type HTTPServerProvider: HTTPServerProvider;
    type RuntimeProvider: RuntimeProvider;

    /// Returns the (outer) limitations for the context
    ///
    /// Note that subcontexts may have subsets of these limitations (e.g. with ctx:with)
    /// to further limit the capabilities available within sections of a script/shared core
    /// between scripts
    ///
    /// Note: TemplateContext will auto-cache Limitations and use it.
    fn limitations(&self) -> Limitations;

    /// Returns a key-value provider
    fn kv_provider(&self) -> Option<Self::KVProvider>;

    /// Returns a global key-value provider
    fn global_kv_provider(&self) -> Option<Self::GlobalKVProvider>;

    /// Returns a Discord provider
    ///
    /// This is used to interact with Discord API
    fn discord_provider(&self) -> Option<Self::DiscordProvider>;

    /// Returns a ObjectStorage provider
    fn objectstorage_provider(&self) -> Option<Self::ObjectStorageProvider>;

    /// Returns a HTTP client provider
    fn httpclient_provider(&self) -> Option<Self::HTTPClientProvider>;

    /// Returns a HTTP server provider
    fn httpserver_provider(&self) -> Option<Self::HTTPServerProvider>;

    /// Returns a runtime provider
    fn runtime_provider(&self) -> Option<Self::RuntimeProvider>;

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }

    /// Returns any additional plugins to bind to the context
    fn extra_plugins() -> indexmap::IndexMap<String, Box<dyn Fn(&Lua, &TemplateContext<Self>) -> LuaResult<LuaValue>>> {
        indexmap::IndexMap::new()
    }
}
