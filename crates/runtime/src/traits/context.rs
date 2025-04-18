use super::{
    discordprovider::DiscordProvider, kvprovider::KVProvider, lockdownprovider::LockdownProvider,
    pageprovider::PageProvider, userinfoprovider::UserInfoProvider, scheduledexecprovider::ScheduledExecProvider,
    datastoreprovider::DataStoreProvider
};
use crate::utils::executorscope::ExecutorScope;

pub trait KhronosContext: 'static + Clone + Sized {
    type Data: serde::Serialize;
    type KVProvider: KVProvider;
    type DiscordProvider: DiscordProvider;
    type LockdownDataStore: lockdowns::LockdownDataStore + Clone;
    type LockdownProvider: LockdownProvider<Self::LockdownDataStore>;
    type UserInfoProvider: UserInfoProvider;
    type PageProvider: PageProvider;
    type ScheduledExecProvider: ScheduledExecProvider;
    type DataStoreProvider: DataStoreProvider;

    /// Returns context-specific data that will be exposed in context.data
    fn data(&self) -> Self::Data;

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

    /// Returns the runtime shareable data for the current context
    fn runtime_shareable_data(&self) -> crate::rt::RuntimeShareableData;

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
}
