use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::header::{HeaderMap as Headers, HeaderValue};
use serenity::all::{InteractionId, ReactionType};

/// A discord provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait DiscordProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Http client
    fn serenity_http(&self) -> &serenity::http::Http;

    /// Returns the guild ID
    fn guild_id(&self) -> serenity::all::GuildId;

    // Audit Log

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
            .map_err(|e| format!("Failed to fetch audit logs: {e}").into())
    }

    // Auto Moderation

    async fn list_auto_moderation_rules(
        &self,
    ) -> Result<Vec<serenity::all::AutoModRule>, crate::Error> {
        self.serenity_http()
            .get_automod_rules(self.guild_id())
            .await
            .map_err(|e| format!("Failed to fetch automod rules: {e}").into())
    }

    async fn get_auto_moderation_rule(
        &self,
        rule_id: serenity::all::RuleId,
    ) -> Result<serenity::all::AutoModRule, crate::Error> {
        self.serenity_http()
            .get_automod_rule(self.guild_id(), rule_id)
            .await
            .map_err(|e| format!("Failed to fetch automod rule: {e}").into())
    }

    async fn create_auto_moderation_rule(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::AutoModRule, crate::Error> {
        self.serenity_http()
            .create_automod_rule(self.guild_id(), &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to create automod rule: {e}").into())
    }

    async fn edit_auto_moderation_rule(
        &self,
        rule_id: serenity::all::RuleId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::AutoModRule, crate::Error> {
        self.serenity_http()
            .edit_automod_rule(self.guild_id(), rule_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit automod rule: {e}").into())
    }

    async fn delete_auto_moderation_rule(
        &self,
        rule_id: serenity::all::RuleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_automod_rule(self.guild_id(), rule_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete automod rule: {e}").into())
    }

    // Channel

    /// Fetches a channel from the guild.
    ///
    /// This should return an error if the channel does not exist
    /// or does not belong to the guild
    async fn get_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
    ) -> Result<serenity::all::Channel, crate::Error> {
        let chan = self.serenity_http().get_channel(channel_id).await?;

        let Some(guild_id) = chan.guild_id() else {
            return Err(format!("Channel {channel_id} does not belong to a guild").into());
        };

        if guild_id != self.guild_id() {
            return Err(format!("Channel {channel_id} does not belong to the guild").into());
        }

        Ok(chan)
    }

    async fn edit_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, crate::Error> {
        let chan = self
            .serenity_http()
            .edit_channel(channel_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel: {e}"))?;

        Ok(chan)
    }

    async fn delete_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, crate::Error> {
        let chan = self
            .serenity_http()
            .delete_channel(channel_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete channel: {e}"))?;

        Ok(chan)
    }

    async fn edit_channel_permissions(
        &self,
        channel_id: serenity::all::GenericChannelId,
        target_id: serenity::all::TargetId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .create_permission(channel_id.expect_channel(), target_id, &data, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel permissions: {e}").into())
    }

    async fn get_channel_invites(
        &self,
        channel_id: serenity::all::GenericChannelId,
    ) -> Result<Vec<serenity::all::RichInvite>, crate::Error> {
        self.serenity_http()
            .get_channel_invites(channel_id.expect_channel())
            .await
            .map_err(|e| format!("Failed to get channel invites: {e}").into())
    }

    async fn create_channel_invite(
        &self,
        channel_id: serenity::all::GenericChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::RichInvite, crate::Error> {
        self.serenity_http()
            .create_invite(channel_id.expect_channel(), &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to create channel invite: {e}").into())
    }

    async fn delete_channel_permission(
        &self,
        channel_id: serenity::all::GenericChannelId,
        target_id: serenity::all::TargetId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_permission(channel_id.expect_channel(), target_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete channel permission: {e}").into())
    }

    async fn follow_announcement_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::FollowedChannel, crate::Error> {
        let headers = if let Some(reason) = audit_log_reason {
            Some(reason_into_header(reason)?)
        } else {
            None
        };

        Ok(self
            .serenity_http()
            .fire(
                serenity::all::Request::new(
                    serenity::all::Route::ChannelFollowNews { channel_id: channel_id.expect_channel() },
                    serenity::all::LightMethod::Post,
                )
                .body(Some(serde_json::to_vec(&map)?))
                .headers(headers),
            )
            .await
            .map_err(|e| format!("Failed to follow announcement channel: {e}"))?)
    }

    // Guild

    /// Fetches the target guild.
    ///
    /// This should return an error if the guild does not exist
    async fn get_guild(&self) -> Result<serenity::all::PartialGuild, crate::Error> {
        self.serenity_http()
            .get_guild_with_counts(self.guild_id())
            .await
            .map_err(|e| format!("Failed to fetch guild: {e}").into())
    }

    /// Fetches a guild preview
    async fn get_guild_preview(&self) -> Result<serenity::all::GuildPreview, crate::Error> {
        self.serenity_http()
            .get_guild_preview(self.guild_id())
            .await
            .map_err(|e| format!("Failed to fetch guild preview: {e}").into())
    }

    // Modify Guild
    async fn modify_guild(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::PartialGuild, crate::Error> {
        self.serenity_http()
            .edit_guild(self.guild_id(), &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to modify guild: {e}").into())
    }

    // Delete guild will not be implemented as we can't really use it

    /// Gets all guild channels
    async fn get_guild_channels(&self) -> Result<Vec<serenity::all::GuildChannel>, crate::Error> {
        Ok(self
            .serenity_http()
            .get_channels(self.guild_id())
            .await
            .map_err(|e| format!("Failed to fetch guild channels: {e:?}"))?
            .into_iter()
            .collect::<Vec<_>>())
    }

    /// Create a guild channel
    async fn create_guild_channel(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::GuildChannel, crate::Error> {
        self.serenity_http()
            .create_channel(self.guild_id(), &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to create guild channel: {e}").into())
    }

    /// Modify Guild Channel Positions
    async fn modify_guild_channel_positions(
        &self,
        map: impl Iterator<Item: serde::Serialize>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .edit_guild_channel_positions(self.guild_id(), map)
            .await
            .map_err(|e| format!("Failed to modify guild channel positions: {e}").into())
    }

    /// List Active Guild Threads
    async fn list_active_guild_threads(&self) -> Result<serenity::all::ThreadsData, crate::Error> {
        self.serenity_http()
            .get_guild_active_threads(self.guild_id())
            .await
            .map_err(|e| format!("Failed to list active threads: {e}").into())
    }

    /// Returns a member from the guild.
    ///
    /// This should return a Ok(None) if the member does not exist
    async fn get_guild_member(
        &self,
        user_id: serenity::all::UserId,
    ) -> Result<Option<serenity::all::Member>, crate::Error> {
        match self
            .serenity_http()
            .get_member(self.guild_id(), user_id)
            .await
        {
            Ok(member) => Ok(Some(member)),
            Err(serenity::all::Error::Http(serenity::all::HttpError::UnsuccessfulRequest(e))) => {
                if e.status_code == serenity::all::StatusCode::NOT_FOUND {
                    Ok(None)
                } else {
                    Err(format!("Failed to fetch member: {e:?}").into())
                }
            }
            Err(e) => Err(format!("Failed to fetch member: {e:?}").into()),
        }
    }

    /// List guild members
    async fn list_guild_members(
        &self,
        limit: Option<serenity::nonmax::NonMaxU16>,
        after: Option<serenity::all::UserId>,
    ) -> Result<Vec<serenity::all::Member>, crate::Error> {
        self.serenity_http()
            .get_guild_members(self.guild_id(), limit, after)
            .await
            .map_err(|e| format!("Failed to list guild members: {e}").into())
    }

    /// Search Guild Members
    async fn search_guild_members(
        &self,
        query: &str,
        limit: Option<serenity::nonmax::NonMaxU16>,
    ) -> Result<Vec<serenity::all::Member>, crate::Error> {
        self.serenity_http()
            .search_guild_members(self.guild_id(), query, limit)
            .await
            .map_err(|e| format!("Failed to search guild members: {e}").into())
    }

    // Add Guild Member is intentionally not supported as it needs OAuth2 to work
    // and has security implications

    /// Modify Guild Member
    async fn modify_guild_member(
        &self,
        user_id: serenity::all::UserId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Member, crate::Error> {
        self.serenity_http()
            .edit_member(self.guild_id(), user_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to modify guild member: {e}").into())
    }

    // Modify Current Member and Modify Current Member Nick are intentionally not supported due to our current self-modification position

    async fn add_guild_member_role(
        &self,
        user_id: serenity::all::UserId,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .add_member_role(self.guild_id(), user_id, role_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to add role to member: {e}").into())
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
            .map_err(|e| format!("Failed to remove role from member: {e}").into())
    }

    async fn remove_guild_member(
        &self,
        user_id: serenity::all::UserId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .kick_member(self.guild_id(), user_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to remove member: {e}").into())
    }

    async fn get_guild_bans(
        &self,
        target: Option<serenity::all::UserPagination>,
        limit: Option<serenity::nonmax::NonMaxU16>,
    ) -> Result<Vec<serenity::all::Ban>, crate::Error> {
        self.serenity_http()
            .get_bans(self.guild_id(), target, limit)
            .await
            .map_err(|e| format!("Failed to get guild bans: {e}").into())
    }

    async fn get_guild_ban(
        &self,
        user_id: serenity::all::UserId,
    ) -> Result<Option<serenity::all::Ban>, crate::Error> {
        match self
            .serenity_http()
            .fire(serenity::all::Request::new(
                serenity::all::Route::GuildBan {
                    guild_id: self.guild_id(),
                    user_id,
                },
                serenity::all::LightMethod::Get,
            ))
            .await
        {
            Ok(v) => Ok(Some(v)),
            Err(serenity::all::Error::Http(serenity::all::HttpError::UnsuccessfulRequest(e))) => {
                if e.status_code == serenity::all::StatusCode::NOT_FOUND {
                    Ok(None)
                } else {
                    Err(format!("Failed to get guild ban: {e:?}").into())
                }
            }
            Err(e) => Err(format!("Failed to get guild ban: {e:?}").into()),
        }
    }

    async fn create_guild_ban(
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
                    .map_err(|e| format!("Failed to convert ban duration to days: {e}"))?,
                reason,
            )
            .await
            .map_err(|e| format!("Failed to ban user: {e}").into())
    }

    async fn remove_guild_ban(
        &self,
        user_id: serenity::all::UserId,
        reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .remove_ban(self.guild_id(), user_id, reason)
            .await
            .map_err(|e| format!("Failed to unban user: {e}").into())
    }

    // Bulk Guild Ban is intentionally super-disabled (both Khronos + infra wide endpoint ban)
    // due to severe possibility of damage

    async fn get_guild_roles(
        &self,
    ) -> Result<extract_map::ExtractMap<serenity::all::RoleId, serenity::all::Role>, crate::Error>
    {
        self.serenity_http()
            .get_guild_roles(self.guild_id())
            .await
            .map_err(|e| format!("Failed to get guild roles: {e}").into())
    }

    async fn get_guild_role(
        &self,
        role_id: serenity::all::RoleId,
    ) -> Result<serenity::all::Role, crate::Error> {
        self.serenity_http()
            .get_guild_role(self.guild_id(), role_id)
            .await
            .map_err(|e| format!("Failed to get guild role: {e}").into())
    }

    async fn create_guild_role(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Role, crate::Error> {
        self.serenity_http()
            .create_role(self.guild_id(), &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to create guild role: {e}").into())
    }

    async fn modify_guild_role_positions(
        &self,
        map: impl Iterator<Item: serde::Serialize>,
        audit_log_reason: Option<&str>,
    ) -> Result<Vec<serenity::all::Role>, crate::Error> {
        self.serenity_http()
            .edit_role_positions(self.guild_id(), map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to modify guild role positions: {e}").into())
    }

    async fn modify_guild_role(
        &self,
        role_id: serenity::all::RoleId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Role, crate::Error> {
        self.serenity_http()
            .edit_role(self.guild_id(), role_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to modify guild role: {e}").into())
    }

    async fn delete_guild_role(
        &self,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_role(self.guild_id(), role_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to modify guild role: {e}").into())
    }

    // Invites

    /// Gets an invite, this can be overrided to add stuff like caching invite codes etc
    async fn get_invite(
        &self,
        code: &str,
        member_counts: bool,
        expiration: bool,
        event_id: Option<serenity::all::ScheduledEventId>,
    ) -> Result<serenity::all::Invite, crate::Error> {
        self.serenity_http()
            .get_invite(code, member_counts, expiration, event_id)
            .await
            .map_err(|e| format!("Failed to get invite: {e}").into())
    }

    async fn delete_invite(
        &self,
        code: &str,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Invite, crate::Error> {
        self.serenity_http()
            .delete_invite(code, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete invite: {e}").into())
    }

    // Messages

    async fn get_channel_messages(
        &self,
        channel_id: serenity::all::GenericChannelId,
        target: Option<serenity::all::MessagePagination>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Vec<serenity::all::Message>, crate::Error> {
        self.serenity_http()
            .get_messages(channel_id, target, limit)
            .await
            .map_err(|e| format!("Failed to get messages: {e}").into())
    }

    async fn get_channel_message(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, crate::Error> {
        self.serenity_http()
            .get_message(channel_id, message_id)
            .await
            .map_err(|e| format!("Failed to get message: {e}").into())
    }

    async fn create_message(
        &self,
        channel_id: serenity::all::GenericChannelId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        self.serenity_http()
            .send_message(channel_id, files, &data)
            .await
            .map_err(|e| format!("Failed to send message: {e}").into())
    }

    async fn crosspost_message(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, crate::Error> {
        self.serenity_http()
            .crosspost_message(channel_id.expect_channel(), message_id)
            .await
            .map_err(|e| format!("Failed to crosspost message: {e}").into())
    }

    async fn create_reaction(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .create_reaction(channel_id, message_id, reaction)
            .await
            .map_err(|e| format!("Failed to create reaction: {e}").into())
    }

    async fn delete_own_reaction(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_reaction_me(channel_id, message_id, reaction)
            .await
            .map_err(|e| format!("Failed to delete own reaction: {e}").into())
    }

    async fn delete_user_reaction(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        user_id: serenity::all::UserId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_reaction(channel_id, message_id, user_id, reaction)
            .await
            .map_err(|e| format!("Failed to delete reaction: {e}").into())
    }

    async fn get_reactions(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
        is_burst: Option<bool>,
        after: Option<serenity::all::UserId>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Vec<serenity::all::User>, crate::Error> {
        let mut params= vec![];

        let after = after.map(|x| x.to_string());
        if let Some(ref after_str) = after {
            let after_str = after_str.as_str();
            params.push(("after", after_str));
        }

        let limit = limit.map(|x| x.to_string());
        if let Some(ref limit) = limit {
            let limit_str = limit.as_str();
            params.push(("limit", limit_str));
        }

        if let Some(burst) = is_burst {
            if burst {
                params.push(("type", "1"));
            } else {
                params.push(("type", "0"));
            }
        }

        Ok(self
            .serenity_http()
            .fire(
                serenity::all::Request::new(
                    serenity::all::Route::ChannelMessageReactionEmoji { channel_id, message_id, reaction: &reaction.as_data() },
                    serenity::all::LightMethod::Get,
                )
                .params(&params),
            )
            .await
            .map_err(|e| format!("Failed to get reactions: {e}"))?)
    }

    async fn delete_all_reactions(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_message_reactions(channel_id, message_id)
            .await
            .map_err(|e| format!("Failed to delete all reactions: {e}").into())
    }

    async fn delete_all_reactions_for_emoji(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_message_reaction_emoji(channel_id, message_id, reaction)
            .await
            .map_err(|e| format!("Failed to delete all reactions for emoji: {e}").into())
    }

    async fn edit_message(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        self.serenity_http()
            .edit_message(channel_id, message_id, &data, files)
            .await
            .map_err(|e| format!("Failed to send message: {e}").into())
    }

    async fn delete_message(
        &self,
        channel_id: serenity::all::GenericChannelId,
        message_id: serenity::all::MessageId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_message(channel_id, message_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete message: {e}").into())
    }

    async fn bulk_delete_messages(
        &self,
        channel_id: serenity::all::GenericChannelId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_messages(channel_id, &data, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to bulk delete messages: {e}").into())
    }

    // Interactions

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
            .map_err(|e| format!("Failed to create interaction response: {e}").into())
    }

    async fn get_original_interaction_response(
        &self,
        interaction_token: &str,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        self.serenity_http()
            .get_original_interaction_response(interaction_token)
            .await
            .map_err(|e| format!("Failed to get original interaction response: {e}").into())
    }

    // https://discord.com/developers/docs/interactions/receiving-and-responding#edit-original-interaction-response
    async fn edit_original_interaction_response(
        &self,
        interaction_token: &str,
        map: impl serde::Serialize,
        new_files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        self.serenity_http()
            .edit_original_interaction_response(interaction_token, &map, new_files)
            .await
            .map_err(|e| format!("Failed to edit original interaction response: {e}").into())
    }

    async fn delete_original_interaction_response(
        &self,
        interaction_token: &str,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_original_interaction_response(interaction_token)
            .await
            .map_err(|e| format!("Failed to delete original interaction response: {e}").into())
    }

    async fn get_followup_message(
        &self,
        interaction_token: &str,
        message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, crate::Error> {
        self.serenity_http()
            .get_followup_message(interaction_token, message_id)
            .await
            .map_err(|e| format!("Failed to get interaction followup: {e}").into())
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
            .map_err(|e| format!("Failed to create interaction followup: {e}").into())
    }

    async fn edit_followup_message(
        &self,
        interaction_token: &str,
        message_id: serenity::all::MessageId,
        map: impl serde::Serialize,
        new_files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<serenity::all::Message, crate::Error> {
        self.serenity_http()
            .edit_followup_message(interaction_token, message_id, &map, new_files)
            .await
            .map_err(|e| format!("Failed to edit interaction followup: {e}").into())
    }

    async fn delete_followup_message(
        &self,
        interaction_token: &str,
        message_id: serenity::all::MessageId,
    ) -> Result<(), crate::Error> {
        self.serenity_http()
            .delete_followup_message(interaction_token, message_id)
            .await
            .map_err(|e| format!("Failed to delete interaction followup: {e}").into())
    }

    // Uncategorized (for now)

    async fn get_guild_commands(&self) -> Result<Vec<serenity::all::Command>, crate::Error> {
        self.serenity_http()
            .get_guild_commands(self.guild_id())
            .await
            .map_err(|e| format!("Failed to get guild commands: {e}").into())
    }

    async fn get_guild_command(
        &self,
        command_id: serenity::all::CommandId,
    ) -> Result<serenity::all::Command, crate::Error> {
        self.serenity_http()
            .get_guild_command(self.guild_id(), command_id)
            .await
            .map_err(|e| format!("Failed to get guild command: {e}").into())
    }

    async fn create_guild_command(
        &self,
        map: impl serde::Serialize,
    ) -> Result<serenity::all::Command, crate::Error> {
        self.serenity_http()
            .create_guild_command(self.guild_id(), &map)
            .await
            .map_err(|e| format!("Failed to create guild command: {e}").into())
    }

    async fn create_guild_commands(
        &self,
        map: impl serde::Serialize,
    ) -> Result<Vec<serenity::all::Command>, crate::Error> {
        self.serenity_http()
            .create_guild_commands(self.guild_id(), &map)
            .await
            .map_err(|e| format!("Failed to create guild commands: {e}").into())
    }
}

fn reason_into_header(reason: &str) -> Result<Headers, crate::Error> {
    let mut headers = Headers::new();

    // "The X-Audit-Log-Reason header supports 1-512 URL-encoded UTF-8 characters."
    // https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object
    let header_value =
        match std::borrow::Cow::from(utf8_percent_encode(reason, NON_ALPHANUMERIC)) {
            std::borrow::Cow::Borrowed(value) => HeaderValue::from_str(value),
            std::borrow::Cow::Owned(value) => HeaderValue::try_from(value),
        }
        .map_err(|_| format!("Failed to convert audit log reason to header value: {reason:?}"))?;

    headers.insert("X-Audit-Log-Reason", header_value);
    Ok(headers)
}
