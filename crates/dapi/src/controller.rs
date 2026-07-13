use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{StatusCode, header::{HeaderMap as Headers, HeaderValue}};
use serde_json::Value;
use serenity::all::{InteractionId, ReactionType};

use crate::{AnyId, ChannelId, CommandId, GuildId, RuleId, UserId, WebhookId, dhttp::{self, HttpError}, types::{CreateEmbed, ModifyChannelPosition}};

pub enum DiscordProviderContext {
    Guild(GuildId),
    User(UserId),
    None,
}

#[derive(Debug, Clone, Copy)]
/// Sent in ``superuser_can_manage_guild_commands`` to provide context on the operation being attempted, allowing for more granular control over what endpoints can be used
/// directly
pub enum SuperUserDiscordCommandOp<'a> {
    CreateCommand(&'a str),
    FinalizeCreateCommand,
    DeleteCommand(CommandId),
    ModifyCommand(CommandId),
    GetCommands,
    GetCommand(CommandId),
}

#[derive(Debug)]
pub struct SuperUserMessageTransform {
    pub embeds: Vec<CreateEmbed>,
    pub content: Option<String>,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SuperUserMessageTransformFlags: u8 {
        const NONE = 0;
        const IS_EDIT = 1 << 0;

        // interaction
        const IS_CREATE_FOLLOWUP_RESPONSE = 1 << 1;
        const IS_CREATE_INTERACTION_RESPONSE = 1 << 2;
        const IS_EDIT_FOLLOWUP_RESPONSE = 1 << 3;
        const IS_EDIT_ORIGINAL_INTERACTION_RESPONSE = 1 << 4;
    }
}

impl SuperUserMessageTransformFlags {
    /// Returns true if the message being transformed is an interaction response (including followups), which can be used to apply different transformations to interaction responses vs regular messages
    pub fn is_interaction_response(self) -> bool {
        self.intersects(
            Self::IS_CREATE_FOLLOWUP_RESPONSE
            | Self::IS_CREATE_INTERACTION_RESPONSE
            | Self::IS_EDIT_FOLLOWUP_RESPONSE
            | Self::IS_EDIT_ORIGINAL_INTERACTION_RESPONSE
        )
    }
}

#[allow(async_fn_in_trait)] 
pub trait DiscordProvider: 'static + Clone {
    fn dhttp(&self) -> &dhttp::Client;

    /// Returns the guild ID
    fn context(&self) -> DiscordProviderContext;

    /// Either returns the guild ID or returns an error if it is not available
    fn guild_context(&self) -> Result<GuildId, crate::Error> {
        match self.context() {
            DiscordProviderContext::Guild(guild_id) => Ok(guild_id),
            _ => Err("Guild ID is not available in the current context".into()),
        }
    }

    // Superuser Moderation

    /// Returns if commands can be created/deleted/modified at a per-guild level
    /// 
    /// This can be used to block guild commands in templates without explicit permission
    /// from AntiRaid
    fn superuser_can_manage_guild_commands<'a>(&self, _req: SuperUserDiscordCommandOp<'a>) -> bool {
        false // Disabled by default, needs to be enabled explicitly
    }

    /// Applies any transformations to a message before it is sent as a response to a command, such as appending disclaimers or modifying embeds.
    fn superuser_transform_message_before_send(&self, msg: SuperUserMessageTransform, _flags: SuperUserMessageTransformFlags) -> Result<SuperUserMessageTransform, crate::Error> {
        Ok(msg)
    }

    // Audit Log

    /// Returns the audit logs for the guild.
    async fn get_audit_logs(
        &self,
        action_type: Option<u16>,
        user_id: Option<UserId>,
        before: Option<AnyId>,
        limit: Option<u8>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetAuditLogs {
                guild_id: self.guild_context()?,
                action_type,
                user_id,
                before,
                limit,
            })
            .await
            .map_err(|e| format!("Failed to fetch audit logs: {e}").into())
    }

    // Auto Moderation

    async fn list_auto_moderation_rules(
        &self,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetAutomodRules {
                guild_id: self.guild_context()?,
            })
            .await
            .map_err(|e| format!("Failed to fetch automod rules: {e}").into())
    }

    async fn get_auto_moderation_rule(
        &self,
        rule_id: RuleId,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetAutomodRule {
                guild_id: self.guild_context()?,
                rule_id,
            })
            .await
            .map_err(|e| format!("Failed to fetch automod rule: {e}").into())
    }

    async fn create_auto_moderation_rule(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreateAutomodRule {
                guild_id: self.guild_context()?,
                map: serde_json::to_vec(&map)?,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to create automod rule: {e}").into())
    }

    async fn edit_auto_moderation_rule(
        &self,
        rule_id: RuleId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditAutomodRule {
                guild_id: self.guild_context()?,
                rule_id,
                map: serde_json::to_vec(&map)?,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to edit automod rule: {e}").into())
    }

    async fn delete_auto_moderation_rule(
        &self,
        rule_id: RuleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_fire(crate::dhttp::HttpCall::DeleteAutomodRule {
                guild_id: self.guild_context()?,
                rule_id,
                audit_log_reason,
            })
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
        channel_id: ChannelId,
    ) -> Result<Value, crate::Error> {
        let chan = self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetChannel {
                channel_id,
            }).await?;

        let Some(Value::String(guild_id)) = chan.get("guild_id") else {
            return Err(format!("Channel {channel_id} does not belong to a guild").into());
        };

        if guild_id != &self.guild_context()?.to_string() {
            return Err(format!("Channel {channel_id} does not belong to the guild").into());
        }

        Ok(chan)
    }

    async fn edit_channel(
        &self,
        channel_id: ChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditChannel {
                channel_id,
                map: serde_json::to_vec(&map)?,
                audit_log_reason
            }).await?;

        let chan = self
            .serenity_http()
            .edit_channel(channel_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel: {e}"))?;

        Ok(chan)
    }

    async fn delete_channel(
        &self,
        channel_id: ChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        let chan = self
            .serenity_http()
            .delete_channel(channel_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete channel: {e}"))?;

        Ok(chan)
    }

    async fn edit_channel_permissions(
        &self,
        channel_id: ChannelId,
        target_id: serenity::all::TargetId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreatePermission {
                channel_id: channel_id.expect_channel(),
                target_id,
                map: serde_json::to_vec(&data)?,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to edit channel permissions: {e}").into())
    }

    async fn get_channel_invites(
        &self,
        channel_id: ChannelId,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetChannelInvites {
                channel_id: channel_id.expect_channel(),
            })
            .await
            .map_err(|e| format!("Failed to get channel invites: {e}").into())
    }

    async fn create_channel_invite(
        &self,
        channel_id: ChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreateInvite {
                channel_id: channel_id.expect_channel(),
                map: serde_json::to_vec(&map).unwrap(),
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to create channel invite: {e}").into())
    }

    async fn delete_channel_permission(
        &self,
        channel_id: ChannelId,
        target_id: serenity::all::TargetId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeletePermission {
                channel_id: channel_id.expect_channel(),
                target_id,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to delete channel permission: {e}").into())
    }

    // Guild

    /// Fetches the target guild.
    ///
    /// This should return an error if the guild does not exist
    async fn get_guild(&self) -> Result<Value, crate::Error> {
        self.serenity_http()
            .get_guild_with_counts(self.guild_context()?)
            .await
            .map_err(|e| format!("Failed to fetch guild: {e}").into())
    }

    /// Fetches a guild preview
    async fn get_guild_preview(&self) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetGuildPreview {
                guild_id: self.guild_context()?,
            })
            .await
            .map_err(|e| format!("Failed to fetch guild preview: {e}").into())
    }

    // Modify Guild
    async fn modify_guild(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditGuild {
                guild_id: self.guild_context()?,
                map: serde_json::to_vec(&map).unwrap(),
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to modify guild: {e}").into())
    }

    // Delete guild will not be implemented as we can't really use it

    /// Gets all guild channels
    async fn get_guild_channels(&self) -> Result<Value, crate::Error> {
        Ok(self
            .serenity_http()
            .get_channels(self.guild_context()?)
            .await
            .map_err(|e| format!("Failed to fetch guild channels: {e:?}"))?)
    }

    /// Create a guild channel
    async fn create_guild_channel(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreateChannel {
                guild_id: self.guild_context()?,
                map: serde_json::to_vec(&map)?,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to create guild channel: {e}").into())
    }

    /// Modify Guild Channel Positions
    async fn modify_guild_channel_positions(
        &self,
        map: &[ModifyChannelPosition],
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_fire(crate::dhttp::HttpCall::EditGuildChannelPositions {
                guild_id: self.guild_context()?,
                value: map,
            })
            .await
            .map_err(|e| format!("Failed to modify guild channel positions: {e}").into())
    }

    /// List Active Guild Threads
    async fn list_active_guild_threads(&self) -> Result<Value, crate::Error> {
        self.serenity_http()
            .get_guild_active_threads(self.guild_context()?)
            .await
            .map_err(|e| format!("Failed to list active threads: {e}").into())
    }

    /// Returns a member from the guild.
    ///
    /// This should return a Ok(Value::Null) if the member does not exist
    async fn get_guild_member(
        &self,
        user_id: UserId,
    ) -> Result<Value, crate::Error> {
        match self
            .dhttp()
            .call_json(dhttp::HttpCall::GetGuildMember { guild_id: self.guild_context()?, user_id })
            .await
        {
            Ok(member) => Ok(member),
            Err(HttpError::UnsuccessfulRequest(e)) => {
                if e.status_code == StatusCode::NOT_FOUND {
                    Ok(Value::Null)
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
        after: Option<UserId>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetGuildMembers {
                guild_id: self.guild_context()?,
                limit,
                after,
            })
            .await
            .map_err(|e| format!("Failed to list guild members: {e}").into())
    }

    /// Search Guild Members
    async fn search_guild_members(
        &self,
        query: &str,
        limit: Option<serenity::nonmax::NonMaxU16>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::SearchGuildMembers {
                guild_id: self.guild_context()?,
                query,
                limit,
            })
            .await
            .map_err(|e| format!("Failed to search guild members: {e}").into())
    }

    // Add Guild Member is intentionally not supported as it needs OAuth2 to work
    // and has security implications

    /// Modify Guild Member
    async fn modify_guild_member(
        &self,
        user_id: UserId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditMember {
                guild_id: self.guild_context()?,
                user_id,
                map: serde_json::to_vec(&map)?,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to modify guild member: {e}").into())
    }

    // Modify Current Member and Modify Current Member Nick are intentionally not supported due to our current self-modification position

    async fn add_guild_member_role(
        &self,
        user_id: UserId,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::AddMemberRole {
                guild_id: self.guild_context()?,
                user_id,
                role_id,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to add role to member: {e}").into())
    }

    async fn remove_guild_member_role(
        &self,
        user_id: UserId,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::RemoveMemberRole {
                guild_id: self.guild_context()?,
                user_id,
                role_id,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to remove role from member: {e}").into())
    }

    async fn remove_guild_member(
        &self,
        user_id: UserId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::KickMember {
                guild_id: self.guild_context()?,
                user_id,
                reason: audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to remove member: {e}").into())
    }

    async fn get_guild_bans(
        &self,
        target: Option<serenity::all::UserPagination>,
        limit: Option<serenity::nonmax::NonMaxU16>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetBans {
                guild_id: self.guild_context()?,
                target,
                limit,
            })
            .await
            .map_err(|e| format!("Failed to get guild bans: {e}").into())
    }

    async fn get_guild_ban(
        &self,
        user_id: UserId,
    ) -> Result<Value, crate::Error> {
        match self
            .serenity_http()
            .fire(serenity::all::Request::new(
                serenity::all::Route::GuildBan {
                    guild_id: self.guild_context()?,
                    user_id,
                },
                serenity::all::LightMethod::Get,
            ))
            .await
        {
            Ok(v) => Ok(v),
            Err(serenity::all::Error::Http(serenity::all::HttpError::UnsuccessfulRequest(e))) => {
                if e.status_code == serenity::all::StatusCode::NOT_FOUND {
                    Ok(Value::Null)
                } else {
                    Err(format!("Failed to get guild ban: {e:?}").into())
                }
            }
            Err(e) => Err(format!("Failed to get guild ban: {e:?}").into()),
        }
    }

    async fn create_guild_ban(
        &self,
        user_id: UserId,
        delete_message_seconds: u32,
        reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::BanUser {
                guild_id: self.guild_context()?,
                user_id,
                delete_message_seconds: (delete_message_seconds / 86400)
                    .try_into()
                    .map_err(|e| format!("Failed to convert ban duration to days: {e}"))?,
                reason: reason.map(|x| x.to_string()),
            })
            .await
            .map_err(|e| format!("Failed to ban user: {e}").into())
    }

    async fn remove_guild_ban(
        &self,
        user_id: UserId,
        reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::RemoveBan {
                guild_id: self.guild_context()?,
                user_id,
                audit_log_reason: reason: reason.map(|x| x.to_string()),
            })
            .await
            .map_err(|e| format!("Failed to unban user: {e}").into())
    }

    // Bulk Guild Ban is intentionally super-disabled (both Khronos + infra wide endpoint ban)
    // due to severe possibility of damage

    async fn get_guild_roles(
        &self,
    ) -> Result<Value, crate::Error>
    {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetGuildRoles {
                guild_id: self.guild_context()?,
            })
            .await
            .map_err(|e| format!("Failed to get guild roles: {e}").into())
    }

    async fn get_guild_role(
        &self,
        role_id: serenity::all::RoleId,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetGuildRole {
                guild_id: self.guild_context()?,
                role_id,
            })
            .await
            .map_err(|e| format!("Failed to get guild role: {e}").into())
    }

    async fn create_guild_role(
        &self,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreateRole {
                guild_id: self.guild_context()?,
                body: &map,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to create guild role: {e}").into())
    }

    async fn modify_guild_role_positions(
        &self,
        map: impl Iterator<Item: serde::Serialize>,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditRolePositions {
                guild_id: self.guild_context()?,
                positions: map,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to modify guild role positions: {e}").into())
    }

    async fn modify_guild_role(
        &self,
        role_id: serenity::all::RoleId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditRole {
                guild_id: self.guild_context()?,
                role_id,
                map: serde_json::to_vec(&map)?,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to modify guild role: {e}").into())
    }

    async fn delete_guild_role(
        &self,
        role_id: serenity::all::RoleId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_fire(crate::dhttp::HttpCall::DeleteRole {
                guild_id: self.guild_context()?,
                role_id,
                audit_log_reason,
            })
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
    ) -> Result<Value, crate::Error> {
        self.serenity_http()
            .get_invite(code, member_counts, expiration, event_id)
            .await
            .map_err(|e| format!("Failed to get invite: {e}").into())
    }

    async fn delete_invite(
        &self,
        code: &str,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeleteInvite {
                code,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to delete invite: {e}").into())
    }

    // Messages

    async fn get_channel_messages(
        &self,
        channel_id: ChannelId,
        target: Option<serenity::all::MessagePagination>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetMessages {
                channel_id,
                target,
                limit,
            })
            .await
            .map_err(|e| format!("Failed to get messages: {e}").into())
    }

    async fn get_channel_message(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::GetMessage {
                channel_id,
                message_id,
            })
            .await
            .map_err(|e| format!("Failed to get message: {e}").into())
    }

    async fn create_message(
        &self,
        channel_id: ChannelId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<Value, crate::Error> {
        self.serenity_http()
            .send_message(channel_id, files, &data)
            .await
            .map_err(|e| format!("Failed to send message: {e}").into())
    }

    async fn create_reaction(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreateReaction {
                channel_id,
                message_id,
                reaction_type: reaction,
            })
            .await
            .map_err(|e| format!("Failed to create reaction: {e}").into())
    }

    async fn delete_own_reaction(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeleteReactionMe {
                channel_id,
                message_id,
                reaction_type: reaction,
            })
            .await
            .map_err(|e| format!("Failed to delete own reaction: {e}").into())
    }

    async fn delete_user_reaction(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        user_id: UserId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeleteReaction {
                channel_id,
                message_id,
                user_id,
                reaction_type: reaction,
            })
            .await
            .map_err(|e| format!("Failed to delete reaction: {e}").into())
    }

    async fn get_reactions(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
        is_burst: Option<bool>,
        after: Option<UserId>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Value, crate::Error> {
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
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeleteMessageReactions {
                channel_id,
                message_id,
            })
            .await
            .map_err(|e| format!("Failed to delete all reactions: {e}").into())
    }

    async fn delete_all_reactions_for_emoji(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        reaction: &ReactionType,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeleteMessageReactionEmoji {
                channel_id,
                message_id,
                reaction_type: reaction,
            })
            .await
            .map_err(|e| format!("Failed to delete all reactions for emoji: {e}").into())
    }

    async fn edit_message(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<Value, crate::Error> {
        self.serenity_http()
            .edit_message(channel_id, message_id, &data, files)
            .await
            .map_err(|e| format!("Failed to send message: {e}").into())
    }

    async fn delete_message(
        &self,
        channel_id: ChannelId,
        message_id: serenity::all::MessageId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::DeleteMessage {
                channel_id,
                message_id,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to delete message: {e}").into())
    }

    async fn bulk_delete_messages(
        &self,
        channel_id: ChannelId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_fire(crate::dhttp::HttpCall::DeleteMessages {
                channel_id,
                map: serde_json::to_vec(&data)?,
                audit_log_reason,
            })
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
    ) -> Result<Value, crate::Error> {
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
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditOriginalInteractionResponse {
                application_id: interaction_token,
                interaction_token: &map,
                map: new_files,
            })
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
    ) -> Result<Value, crate::Error> {
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
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::CreateFollowupMessage {
                application_id: interaction_token,
                interaction_token: &response,
                map: files,
            })
            .await
            .map_err(|e| format!("Failed to create interaction followup: {e}").into())
    }

    async fn edit_followup_message(
        &self,
        interaction_token: &str,
        message_id: serenity::all::MessageId,
        map: impl serde::Serialize,
        new_files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<Value, crate::Error> {
        self.dhttp()
            .call_json(crate::dhttp::HttpCall::EditFollowupMessage {
                application_id: interaction_token,
                interaction_token: message_id,
                message_id: &map,
                map: new_files,
            })
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

    // Webhooks (all methods outside of deleting is currently not supported due to security risks)
    async fn delete_webhook(
        &self,
        webhook_id: WebhookId,
        audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.dhttp()
            .call_fire(crate::dhttp::HttpCall::DeleteWebhook {
                webhook_id,
                audit_log_reason,
            })
            .await
            .map_err(|e| format!("Failed to delete webhook: {e}").into())
    }

    // Delete webhook with token is intentionally not supported for security reasons
    // Get/Edit/Delete webhook message is intentionally not supported due to lack of use cases and security concerns

    // Uncategorized (for now)

    async fn get_guild_commands(&self) -> Result<Value, crate::Error> {
        if !self.superuser_can_manage_guild_commands(crate::controller::SuperUserDiscordCommandOp::GetCommands) {
            return Err("Guild commands are not enabled for this controller".into());
        }

        self.serenity_http()
            .get_guild_commands(self.guild_context()?)
            .await
            .map_err(|e| format!("Failed to get guild commands: {e}").into())
    }

    async fn get_guild_command(
        &self,
        command_id: serenity::all::CommandId,
    ) -> Result<Value, crate::Error> {
        if !self.superuser_can_manage_guild_commands(crate::controller::SuperUserDiscordCommandOp::GetCommand(command_id)) {
            return Err("Guild commands are not enabled for this controller".into());
        }

        self.serenity_http()
            .get_guild_command(self.guild_context()?, command_id)
            .await
            .map_err(|e| format!("Failed to get guild command: {e}").into())
    }

    async fn create_guild_command(
        &self,
        map: impl serde::Serialize,
    ) -> Result<Value, crate::Error> {
        if !self.superuser_can_manage_guild_commands(crate::controller::SuperUserDiscordCommandOp::FinalizeCreateCommand) {
            return Err("Guild commands are not enabled for this controller".into());
        }

        self.serenity_http()
            .create_guild_command(self.guild_context()?, &map)
            .await
            .map_err(|e| format!("Failed to create guild command: {e}").into())
    }

    async fn create_guild_commands(
        &self,
        map: impl serde::Serialize,
    ) -> Result<Value, crate::Error> {
        if !self.superuser_can_manage_guild_commands(crate::controller::SuperUserDiscordCommandOp::FinalizeCreateCommand) {
            return Err("Guild commands are not enabled for this controller".into());
        }

        self.serenity_http()
            .create_guild_commands(self.guild_context()?, &map)
            .await
            .map_err(|e| format!("Failed to create guild commands: {e}").into())
    }
}
