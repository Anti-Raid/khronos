use super::{
    datastoreprovider::DataStoreProvider, discordprovider::DiscordProvider, kvprovider::KVProvider,
    lockdownprovider::LockdownProvider, objectstorageprovider::ObjectStorageProvider,
    pageprovider::PageProvider, userinfoprovider::UserInfoProvider,
};
use bitflags::bitflags;

bitflags! {
    /// The compatibility flags for the script context.
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
    pub struct CompatibilityFlags: u8 {
        /// Whether or not to allow unscoped key-value operations by default.
        ///
        /// Older scripts without per-operation scopes do not have scopes set, so this flag allows them to run without modification.
        const ALLOW_UNSCOPED_KV = 1 << 0;
    }
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
    pub error_channel: Option<serenity::all::ChannelId>,
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

pub trait KhronosContext: 'static + Clone + Sized {
    type KVProvider: KVProvider;
    type DiscordProvider: DiscordProvider;
    type LockdownDataStore: lockdowns::LockdownDataStore + Clone;
    type LockdownProvider: LockdownProvider<Self::LockdownDataStore>;
    type UserInfoProvider: UserInfoProvider;
    type PageProvider: PageProvider;
    type DataStoreProvider: DataStoreProvider;
    type ObjectStorageProvider: ObjectStorageProvider;

    /// Returns context-specific data that will be exposed in context.data
    fn data(&self) -> &ScriptData;

    /// Returns the allowed capabilities for the current context
    fn allowed_caps(&self) -> &[String] {
        &self.data().allowed_caps
    }

    /// Returns the compatibility flags for the current context
    fn compatibility_flags(&self) -> CompatibilityFlags {
        self.data().compatibility_flags
    }

    /// Returns if the current context has a specific capability
    fn has_cap(&self, cap: &str) -> bool {
        for allowed_cap in self.allowed_caps() {
            if allowed_cap == cap || allowed_cap == "*" {
                return true;
            }
        }

        false
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

    /// Returns a Lockdown provider
    fn lockdown_provider(&self) -> Option<Self::LockdownProvider>;

    /// Returns a UserInfo provider
    fn userinfo_provider(&self) -> Option<Self::UserInfoProvider>;

    /// Returns a Page provider
    fn page_provider(&self) -> Option<Self::PageProvider>;

    /// Returns a DataStore provider
    fn datastore_provider(&self) -> Option<Self::DataStoreProvider>;

    /// Returns a ObjectStorage provider
    fn objectstorage_provider(&self) -> Option<Self::ObjectStorageProvider>;

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }
}
