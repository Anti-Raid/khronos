use crate::{TemplateContext, to_struct};

use super::{
    httpclientprovider::HTTPClientProvider, httpserverprovider::HTTPServerProvider, 
    kvprovider::KVProvider, objectstorageprovider::ObjectStorageProvider,
};
use dapi::controller::DiscordProvider;
use bitflags::bitflags;
use mluau::prelude::*;

bitflags! {
    /// The tflags (template compatibility flags) to be passed to.
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
    pub struct TFlags: u8 {
        const EXPERIMENTAL_LUAUFUSION_SUPPORT = 1 << 2; // Enable LuauFusion support (proxy bridge to Javascript (and potentially other language))

        // Privileged flags (require elevated permissions to use)
        const DAPI_DESTRUCTIVE_CHANNEL_OPERATIONS = 1 << 3; // Allow destructive channel operations via dapi 
        const DAPI_DESTRUCTIVE_ROLE_OPERATIONS = 1 << 4; // Allow destructive role operations via dapi
        const DAPI_DESTRUCTIVE_WEBHOOK_OPERATIONS = 1 << 5; // Allow destructive webhook operations via dapi
        const DAPI_DESTRUCTIVE_GLOBAL = 1 << 6; // Allow all destructive operations via dapi
    }
}

impl TFlags {
    /// Given a list of strings, returns the corresponding TFlags
    pub fn from_strs(flags: &[String], allow_restricted: bool) -> Result<Self, crate::Error> {
        let mut tflags = TFlags::empty();
        for flag in flags {
            let f = Self::from_name(flag.as_str())
                .ok_or_else(|| format!("Unknown tflag: {flag}"))?;
            tflags |= f;
        }

        tflags.is_valid()?;

        if !allow_restricted && (tflags.is_experimental() || tflags.is_privileged()) {
            return Err("At least one of the specified tflags is experimental or privileged and as such cannot be used in this context".into());
        }

        Ok(tflags)
    }

    /// Returns true if the specific tflag combination is valid
    pub fn is_valid(&self) -> Result<(), crate::Error> {
        Ok(())
    }

    /// Returns true if the flag is experimental
    pub fn is_experimental(&self) -> bool {
        self.contains(TFlags::EXPERIMENTAL_LUAUFUSION_SUPPORT)
    }

    /// Returns true if the flag is privileged
    pub fn is_privileged(&self) -> bool {
        self.intersects(
            TFlags::DAPI_DESTRUCTIVE_CHANNEL_OPERATIONS
                | TFlags::DAPI_DESTRUCTIVE_ROLE_OPERATIONS
                | TFlags::DAPI_DESTRUCTIVE_WEBHOOK_OPERATIONS
                | TFlags::DAPI_DESTRUCTIVE_GLOBAL,
        )
    }
}

to_struct!(
    /// Represents the data to be passed into ctx:with()
    pub struct KhronosValueWith {
        pub ext_data: Option<ExtContextData>,
        pub capabilities: Vec<String>,
        pub tflags: Option<Vec<String>>,
    }
);

/// Represents a result of a set operation in the key-value store
pub struct Limitations {
    pub capabilities: Vec<String>,
    pub tflags: TFlags,
}

impl Limitations {
    /// Returns a new limitations instance with the given capabilities
    pub fn new(capabilities: Vec<String>, tflags: TFlags) -> Self {
        Self { capabilities, tflags }
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

to_struct! {
    pub struct ExtContextData {
        pub template_name: String,
        pub events: Vec<String>,
    }
}

pub trait KhronosContext: 'static + Clone + Sized {
    type KVProvider: KVProvider;
    type DiscordProvider: DiscordProvider;
    type ObjectStorageProvider: ObjectStorageProvider;
    type HTTPClientProvider: HTTPClientProvider;
    type HTTPServerProvider: HTTPServerProvider;

    /// Returns the (outer) limitations for the context
    ///
    /// Note that subcontexts may have subsets of these limitations (e.g. with ctx:with)
    /// to further limit the capabilities available within sections of a script/shared core
    /// between scripts
    ///
    /// Note: TemplateContext will auto-cache Limitations and use it.
    fn limitations(&self) -> Limitations;

    /// Returns the initial extended context data    
    fn ext_data(&self) -> ExtContextData;

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

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }

    /// Returns any additional plugins to bind to the context
    fn extra_plugins() -> indexmap::IndexMap<String, Box<dyn Fn(&Lua, &TemplateContext<Self>) -> LuaResult<LuaValue>>> {
        indexmap::IndexMap::new()
    }
}
