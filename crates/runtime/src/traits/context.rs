use super::{
    datastoreprovider::DataStoreProvider, discordprovider::DiscordProvider, kvprovider::KVProvider,
    lockdownprovider::LockdownProvider, pageprovider::PageProvider,
    scheduledexecprovider::ScheduledExecProvider, userinfoprovider::UserInfoProvider,
    objectstorageprovider::ObjectStorageProvider
};
use crate::utils::executorscope::ExecutorScope;

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
    type ScheduledExecProvider: ScheduledExecProvider;
    type DataStoreProvider: DataStoreProvider;
    type ObjectStorageProvider: ObjectStorageProvider;

    /// Returns context-specific data that will be exposed in context.data
    fn data(&self) -> &ScriptData;

    /// Returns the allowed capabilities for the current context
    fn allowed_caps(&self) -> &[String];

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
    fn guild_id(&self) -> Option<serenity::all::GuildId>;

    /// Returns the owner guild ID of the current context, if any
    ///
    /// In a shop template, this would be the guild ID that owns the template on the shop,
    /// and in a normal guild template, this would be the guild ID that owns the template on the guild.
    ///
    /// In local development, both owner_guild_id and guild_id will be the same, unless configured otherwise.
    fn owner_guild_id(&self) -> Option<serenity::all::GuildId>;

    /// Returns the templates name
    fn template_name(&self) -> String;

    /// Returns the current Discord user, if any
    fn current_user(&self) -> Option<serenity::all::CurrentUser>;

    /// Returns a key-value provider with the given scope
    fn kv_provider(&self, scope: ExecutorScope, kv_scope: &str) -> Option<Self::KVProvider>;

    /// Returns a Discord provider with the given scope
    /// This is used to interact with Discord API
    fn discord_provider(&self, scope: ExecutorScope) -> Option<Self::DiscordProvider>;

    /// Returns a Lockdown provider with the given scope
    fn lockdown_provider(&self, scope: ExecutorScope) -> Option<Self::LockdownProvider>;

    /// Returns a UserInfo provider with the given scope
    fn userinfo_provider(&self, scope: ExecutorScope) -> Option<Self::UserInfoProvider>;

    /// Returns a Page provider with the given scope
    fn page_provider(&self, scope: ExecutorScope) -> Option<Self::PageProvider>;

    /// Returns a ScheduledExec provider
    fn scheduled_exec_provider(&self) -> Option<Self::ScheduledExecProvider>;

    /// Returns a DataStore provider
    fn datastore_provider(&self, scope: ExecutorScope) -> Option<Self::DataStoreProvider>;

    /// Returns a ObjectStorage provider
    fn objectstorage_provider(&self, scope: ExecutorScope) -> Option<Self::ObjectStorageProvider>;

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }
}
