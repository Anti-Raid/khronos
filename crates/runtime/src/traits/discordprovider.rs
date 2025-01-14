use nonmax::NonMaxU8;
use serenity::all::*;

/// A discord provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait DiscordProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Fetches the target guild.
    ///
    /// This should return an error if the guild does not exist
    async fn guild(&self) -> Result<PartialGuild, crate::Error>;

    /// Returns a member from the guild.
    ///
    /// This should return a Ok(None) if the member does not exist
    async fn member(
        &self,
        user_id: serenity::all::UserId,
    ) -> Result<Option<serenity::all::Member>, crate::Error>;

    /// Fetches a channel from the guild.
    ///
    /// This should return an error if the channel does not exist
    /// or does not belong to the guild
    async fn guild_channel(
        &self,
        channel_id: serenity::all::ChannelId,
    ) -> Result<serenity::all::GuildChannel, crate::Error>;

    /// Returns the audit logs for the guild.
    async fn get_audit_logs(
        &self,
        action_type: Option<serenity::all::audit_log::Action>,
        user_id: Option<UserId>,
        before: Option<AuditLogEntryId>,
        limit: Option<NonMaxU8>,
    ) -> Result<AuditLogs, crate::Error>;

    /// Retrieves all auto moderation rules in a guild.
    async fn get_automod_rules(&self) -> Result<Vec<serenity::model::guild::automod::Rule>>;

    /// Retrieves an auto moderation rule in a guild.
    async fn get_automod_rule(
        &self,
        rule_id: serenity::all::RuleId,
    ) -> Result<serenity::model::guild::automod::Rule>;

    /// Edits a discord channel
    async fn edit_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel>;

    /// Deletes a discord channel
    async fn delete_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<()>;

    /// Creates a ban for a user
    async fn create_member_ban(
        &self,
        user_id: serenity::all::UserId,
        delete_message_seconds: u32,
        reason: Option<&str>,
    ) -> Result<()>;
}
