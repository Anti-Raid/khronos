use crate::{TemplateContext, primitives::event::CreateEvent, traits::runtimeprovider::RuntimeProvider};

use super::{
    httpclientprovider::HTTPClientProvider, httpserverprovider::HTTPServerProvider, 
    kvprovider::KVProvider, objectstorageprovider::ObjectStorageProvider,
};
use dapi::controller::DiscordProvider;
use mluau::prelude::*;

/// Represents the data to be passed into ctx:with()
#[derive(serde::Serialize, serde::Deserialize)]
pub struct KhronosValueWith {
    pub capabilities: Vec<String>,
    pub event: Option<CreateEvent>,
}

/// Represents a result of a set operation in the key-value store
pub struct Limitations {
    pub capabilities: Vec<String>,
}

impl Limitations {
    /// Returns a new limitations instance with the given capabilities
    pub fn new(capabilities: Vec<String>) -> Self {
        Self { capabilities }
    }

    /// Returns Ok(()) if `other` is a subset of `self`, otherwise returns an error
    pub fn subset_of(&self, other: &Self) -> Result<(), String> {
        for cap in &self.capabilities {
            if !other.has_cap(cap) {
                return Err(format!("Missing capability: {cap}. A context can only be limited into a set of limitations that are strictly a subset of itself"));
            }
        }
        Ok(())
    }

    /// Checks if the limitations allow a specific capability
    pub fn has_cap(&self, cap: &str) -> bool {
        self.capabilities.iter().any(|c| c == cap || c == "*")
    }

    /// Checks if the limitations allow any of a set of capabilities
    pub fn has_any_cap(&self, caps: &[String]) -> bool {
        self.capabilities
            .iter()
            .any(|c| caps.iter().any(|cap| c == cap || c == "*"))
    }
}

pub trait KhronosContext: 'static + Clone + Sized {
    type KVProvider: KVProvider;
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

    /// Returns the guild ID of the current context, if any
    fn guild_id(&self) -> Option<serenity::all::GuildId>;

    /// Returns a key-value provider
    fn kv_provider(&self) -> Option<Self::KVProvider>;

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
