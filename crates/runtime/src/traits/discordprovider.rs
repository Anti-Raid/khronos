use serenity::all::InteractionId;

/// A discord provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait DiscordProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    // Base stuff

    /// Returns the guild ID
    fn guild_id(&self) -> serenity::all::GuildId;

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

    /// Http client
    fn serenity_http(&self) -> &serenity::http::Http;

    // Pre-provided stuff that can be overridden

    /// Returns the audit logs for the guild.
    async fn get_audit_logs(
        &self,
        action_type: Option<serenity::all::audit_log::Action>,
        user_id: Option<serenity::model::prelude::UserId>,
        before: Option<serenity::model::prelude::AuditLogEntryId>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<serenity::model::prelude::AuditLogs, crate::Error> {
        self.serenity_http()
            .get_audit_logs(self.guild_id(), action_type, user_id, before, limit)
            .await
            .map_err(|e| format!("Failed to fetch audit logs: {}", e).into())
    }

    async fn list_auto_moderation_rules(
        &self,
    ) -> Result<Vec<serenity::model::guild::automod::Rule>, crate::Error> {
        self.serenity_http()
            .get_automod_rules(self.guild_id())
            .await
            .map_err(|e| format!("Failed to fetch automod rules: {}", e).into())
    }

    async fn get_auto_moderation_rule(
        &self,
        rule_id: serenity::all::RuleId,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error> {
        self.serenity_http()
            .get_automod_rule(self.guild_id(), rule_id)
            .await
            .map_err(|e| format!("Failed to fetch automod rule: {}", e).into())
    }

    async fn create_auto_moderation_rule(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error> {
        self.serenity_http()
            .create_automod_rule(self.guild_id(), &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to create automod rule: {}", e).into())
    }

    async fn edit_auto_moderation_rule(
        &self,
        rule_id: serenity::all::RuleId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error> {
        self.serenity_http()
            .edit_automod_rule(self.guild_id(), rule_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit automod rule: {}", e).into())
    }

    async fn delete_auto_moderation_rule(
        &self,
        rule_id: serenity::all::RuleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_automod_rule(self.guild_id(), rule_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete automod rule: {}", e).into())
    }

    async fn edit_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::GuildChannel, crate::Error> {
        let chan = self
            .serenity_http()
            .edit_channel(channel_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel: {}", e))?;

        Ok(chan)
    }

    async fn delete_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, crate::Error> {
        let chan = self
            .serenity_http()
            .delete_channel(channel_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete channel: {}", e))?;

        Ok(chan)
    }

    async fn edit_channel_permissions(
        &self,
        channel_id: serenity::all::ChannelId,
        target_id: serenity::all::TargetId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .create_permission(channel_id, target_id, &data, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel permissions: {}", e).into())
    }

    async fn add_guild_member_role(
        &self,
        user_id: serenity::all::UserId,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .add_member_role(self.guild_id(), user_id, role_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to add role to member: {}", e).into())
    }

    async fn remove_guild_member_role(
        &self,
        user_id: serenity::all::UserId,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .remove_member_role(self.guild_id(), user_id, role_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to remove role from member: {}", e).into())
    }

    async fn remove_guild_member(
        &self,
        user_id: serenity::all::UserId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .kick_member(self.guild_id(), user_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to remove member: {}", e).into())
    }

    async fn get_guild_bans(
        &self,
        target: Option<serenity::all::UserPagination>,
        limit: Option<serenity::nonmax::NonMaxU16>,
    ) -> Result<Vec<serenity::all::Ban>, crate::Error> {
        self.serenity_http()
            .get_bans(self.guild_id(), target, limit)
            .await
            .map_err(|e| format!("Failed to get guild bans: {}", e).into())
    }

    async fn create_member_ban(
        &self,
        user_id: serenity::all::UserId,
        delete_message_seconds: u32,
        reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .ban_user(
                self.guild_id(),
                user_id,
                (delete_message_seconds / 86400)
                    .try_into()
                    .map_err(|e| format!("Failed to convert ban duration to days: {}", e))?,
                reason,
            )
            .await
            .map_err(|e| format!("Failed to ban user: {}", e).into())
    }

    async fn kick_member(
        &self,
        user_id: serenity::all::UserId,
        reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .kick_member(self.guild_id(), user_id, reason)
            .await
            .map_err(|e| format!("Failed to kick user: {}", e).into())
    }

    async fn edit_member(
        &self,
        user_id: serenity::all::UserId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Member, crate::Error> {
        self.serenity_http()
            .edit_member(self.guild_id(), user_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit member: {}", e).into())
    }

    async fn create_message(
        &self,
        channel_id: serenity::all::ChannelId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        self.serenity_http()
            .send_message(channel_id, files, &data)
            .await
            .map_err(|e| format!("Failed to send message: {}", e).into())
    }

    async fn create_interaction_response(
        &self,
        interaction_id: InteractionId,
        interaction_token: &str,
        response: impl serde::Serialize,
        files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .create_interaction_response(interaction_id, interaction_token, &response, files)
            .await
            .map_err(|e| format!("Failed to create interaction response: {}", e).into())
    }

    async fn create_followup_message(
        &self,
        interaction_token: &str,
        response: impl serde::Serialize,
        files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<serenity::all::Message, crate::Error> {
        self.serenity_http()
            .create_followup_message(interaction_token, &response, files)
            .await
            .map_err(|e| format!("Failed to create interaction followup: {}", e).into())
    }

    async fn get_original_interaction_response(
        &self,
        interaction_token: &str,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        self.serenity_http()
            .get_original_interaction_response(interaction_token)
            .await
            .map_err(|e| format!("Failed to get original interaction response: {}", e).into())
    }

    async fn get_guild_commands(
        &self,
    ) -> Result<Vec<serenity::all::Command>, crate::Error> {
        self.serenity_http()
            .get_guild_commands(self.guild_id())
            .await
            .map_err(|e| format!("Failed to get guild commands: {}", e).into())
    }

    async fn get_guild_command(
        &self,
        command_id: serenity::all::CommandId,
    ) -> Result<serenity::all::Command, crate::Error> {
        self.serenity_http()
            .get_guild_command(self.guild_id(), command_id)
            .await
            .map_err(|e| format!("Failed to get guild command: {}", e).into())
    }

    async fn create_guild_command(
        &self,
        map: impl serde::Serialize,
    ) -> Result<serenity::all::Command, crate::Error> {
        self.serenity_http()
            .create_guild_command(self.guild_id(), &map)
            .await
            .map_err(|e| format!("Failed to create guild command: {}", e).into())
    }

    async fn create_guild_commands(
        &self,
        map: impl serde::Serialize,
    ) -> Result<Vec<serenity::all::Command>, crate::Error> {
        self.serenity_http()
            .create_guild_commands(self.guild_id(), &map)
            .await
            .map_err(|e| format!("Failed to create guild commands: {}", e).into())
    }

    async fn get_guild_roles(
        &self,
    ) -> Result<
        extract_map::ExtractMap<serenity::all::RoleId, serenity::all::Role>,
        crate::Error,
    > {
        self.serenity_http()
            .get_guild_roles(self.guild_id())
            .await
            .map_err(|e| format!("Failed to get guild roles: {}", e).into())
    }

    async fn get_messages(
        &self,
        channel_id: serenity::all::ChannelId,
        target: Option<serenity::all::MessagePagination>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Vec<serenity::all::Message>, crate::Error> {
        self.serenity_http()
            .get_messages(channel_id, target, limit)
            .await
            .map_err(|e| format!("Failed to get messages: {}", e).into())
    }

    async fn get_message(
        &self,
        channel_id: serenity::all::ChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, crate::Error> {
        self.serenity_http()
            .get_message(channel_id, message_id)
            .await
            .map_err(|e| format!("Failed to get message: {}", e).into())
    }
}
