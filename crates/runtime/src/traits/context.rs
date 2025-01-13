use super::kvprovider::KVProvider;
use crate::utils::executorscope::ExecutorScope;

pub trait KhronosContext: 'static + Clone {
    type Data: serde::Serialize;
    type KVProvider: KVProvider;

    /// Returns context-specific data that will be exposed in context.data
    fn data(&self) -> Self::Data;

    /// Returns the guild ID of the current context, if any
    fn guild_id(&self) -> Option<serenity::all::GuildId>;

    /// Returns the current Discord user, if any
    fn current_user(&self) -> Option<impl AsRef<serenity::all::User>>;

    /// Returns a key-value executor and the guild id it runs on (if any) as a tuple pair
    /// given scope.
    fn kv_executor(
        &self,
        scope: ExecutorScope,
    ) -> Option<(Option<serenity::all::GuildId>, Self::KVProvider)>;
}

/*
Data can contain: template: Template, guild_id: Option<GuildId>, current_discord_user: Option<User>,
*/
