use serenity::nonmax::NonMaxU8;

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
    async fn guild(&self) -> Result<serenity::all::PartialGuild, crate::Error>;

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
        user_id: Option<serenity::all::UserId>,
        before: Option<serenity::all::AuditLogEntryId>,
        limit: Option<NonMaxU8>,
    ) -> Result<serenity::all::AuditLogs, crate::Error>;

    /// Retrieves all auto moderation rules in a guild.
    async fn get_automod_rules(
        &self,
    ) -> Result<Vec<serenity::model::guild::automod::Rule>, crate::Error>;

    /// Retrieves an auto moderation rule in a guild.
    async fn get_automod_rule(
        &self,
        rule_id: serenity::all::RuleId,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error>;

    /// Edits a discord channel
    async fn edit_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::GuildChannel, crate::Error>;

    /// Deletes a discord channel
    async fn delete_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, crate::Error>;

    /// Creates a ban for a user
    async fn create_member_ban(
        &self,
        user_id: serenity::all::UserId,
        delete_message_seconds: u32,
        reason: Option<&str>,
    ) -> Result<(), crate::Error>;

    /// Kicks a member from the guild
    async fn kick_member(
        &self,
        user_id: serenity::all::UserId,
        reason: Option<&str>,
    ) -> Result<(), crate::Error>;

    /// Edits a member on the guild
    async fn edit_member(
        &self,
        user_id: serenity::all::UserId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Member, crate::Error>;

    /// Sends a message
    async fn send_message(
        &self,
        channel_id: serenity::all::ChannelId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, crate::Error>;

    /// Creates an interaction response
    async fn create_interaction_response(
        &self,
        interaction_id: serenity::all::InteractionId,
        interaction_token: &str,
        response: impl serde::Serialize,
        files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<(), crate::Error>;

    /// Gets the original interaction response
    async fn get_original_interaction_response(
        &self,
        interaction_token: &str,
    ) -> Result<serenity::model::channel::Message, crate::Error>;

    /// Returns the guilds commands
    async fn get_guild_commands(&self) -> Result<Vec<serenity::all::Command>, crate::Error>;

    /// Returns a guild command by id
    async fn get_guild_command(
        &self,
        command_id: serenity::all::CommandId,
    ) -> Result<serenity::all::Command, crate::Error>;

    /// Creates a guild command
    async fn create_guild_command(
        &self,
        map: impl serde::Serialize,
    ) -> Result<serenity::all::Command, crate::Error>;
}
