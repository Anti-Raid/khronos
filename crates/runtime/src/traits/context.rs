use crate::to_struct;

use super::{
    datastoreprovider::DataStoreProvider, discordprovider::DiscordProvider,
    httpclientprovider::HTTPClientProvider, httpserverprovider::HTTPServerProvider, 
    kvprovider::KVProvider, objectstorageprovider::ObjectStorageProvider,
};
use bitflags::bitflags;

bitflags! {
    /// The compatibility flags for the script context.
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
    pub struct CompatibilityFlags: u8 {}
}

/// Extra data about the script context
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ScriptData {
    /// The guild ID the script is running in, if any
    pub guild_id: Option<serenity::all::GuildId>,
    /// The name of the template
    pub name: String,
    /// The description of the template
    pub description: Option<String>,
    /// The name of the template as it appears on the template shop listing
    pub shop_name: Option<String>,
    /// The owner of the template on the template shop
    pub shop_owner: Option<serenity::all::GuildId>,
    /// The events that this template listens to
    pub events: Vec<String>,
    /// The channel to send errors to
    pub error_channel: Option<serenity::all::GenericChannelId>,
    /// The language of the template
    pub lang: String,
    /// The allowed capabilities the template has access to
    pub allowed_caps: Vec<String>,
    /// The compatibility flags for the script context
    pub compatibility_flags: CompatibilityFlags,
    /// The user who created the template
    pub created_by: Option<serenity::all::UserId>,
    /// The time the template was created
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The user who last updated the template
    pub updated_by: Option<serenity::all::UserId>,
    /// The time the template was last updated
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

to_struct!(
    /// Represents a result of a set operation in the key-value store
    pub struct Limitations {
        pub capabilities: Vec<String>,
    }
);

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
    type DataStoreProvider: DataStoreProvider;
    type ObjectStorageProvider: ObjectStorageProvider;
    type HTTPClientProvider: HTTPClientProvider;
    type HTTPServerProvider: HTTPServerProvider;

    /// Returns context-specific data that will be exposed in context.data
    fn data(&self) -> &ScriptData;

    /// Returns the (outer) limitations for the context
    ///
    /// Note that subcontexts may have subsets of these limitations (e.g. with ctx:withlimits)
    /// to further limit the capabilities available within sections of a script/shared core
    /// between scripts
    ///
    /// Note: TemplateContext will auto-cache Limitations and use it.
    fn limitations(&self) -> Limitations;

    /// Returns the compatibility flags for the current context
    fn compatibility_flags(&self) -> CompatibilityFlags {
        self.data().compatibility_flags
    }

    /// Returns the guild ID of the current context, if any
    fn guild_id(&self) -> Option<serenity::all::GuildId> {
        self.data().guild_id
    }

    /// Returns the owner guild ID of the current context, if any
    ///
    /// In a shop template, this would be the guild ID that owns the template on the shop,
    /// and in a normal guild template, this would be the guild ID that owns the template on the guild.
    ///
    /// In local development, both owner_guild_id and guild_id will be the same, unless configured otherwise.
    fn owner_guild_id(&self) -> Option<serenity::all::GuildId> {
        self.data().shop_owner
    }

    /// Returns the templates name
    fn template_name(&self) -> String;

    /// Returns the current Discord user, if any
    fn current_user(&self) -> Option<serenity::all::CurrentUser>;

    /// Returns a key-value provider
    fn kv_provider(&self) -> Option<Self::KVProvider>;

    /// Returns a Discord provider
    ///
    /// This is used to interact with Discord API
    fn discord_provider(&self) -> Option<Self::DiscordProvider>;

    /// Returns a DataStore provider
    fn datastore_provider(&self) -> Option<Self::DataStoreProvider>;

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
}
