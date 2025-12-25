use crate::core::typesext::MultiOption;
use std::cmp::Ordering;

use dapi::types::{
    CreateAutoModRule, CreateChannel, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInvite, CreateMessage, EditMessage, EditAutoModRule,
    EditMember, EditRole, EditWebhookMessage, FollowAnnouncementChannelData, ReactionType, CreateWebhook,
    EditWebhook, ExecuteWebhook
};

// Channel

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateChannelOptions {
    pub reason: String,
    pub data: CreateChannel,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteChannelOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub reason: String,
}

// Message

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMessagesOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub target: Option<MessagePagination>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMessageOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateMessageOptions {
    pub channel_id: serenity::all::GenericChannelId, // Channel *must* be in the same guild
    pub data: CreateMessage,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateReactionOptions {
    pub channel_id: serenity::all::GenericChannelId, // Channel *must* be in the same guild
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteOwnReactionOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteUserReactionOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
    pub user_id: serenity::all::UserId,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReactionTypeEnum {
    Normal,
    Burst
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetReactionsOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
    pub r#type: Option<ReactionTypeEnum>,
    pub after: Option<serenity::all::UserId>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteAllReactionsForEmojiOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditMessageOptions {
    pub channel_id: serenity::all::GenericChannelId, // Channel *must* be in the same guild
    pub message_id: serenity::all::MessageId,
    pub data: EditMessage,
}

// Interactions


#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateCommandOptions {
    pub data: CreateCommand,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateCommandsOptions {
    pub data: Vec<CreateCommand>,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateInteractionResponseOptions {
    pub interaction_id: serenity::all::InteractionId,
    pub interaction_token: String,
    pub data: CreateInteractionResponse,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditInteractionResponseOptions {
    pub interaction_token: String,
    pub data: EditWebhookMessage,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct GetFollowupMessageOptions {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateFollowupMessageOptions {
    pub interaction_token: String,
    pub data: CreateInteractionResponseFollowup,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditFollowupMessageOptions {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
    pub data: EditWebhookMessage,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteFollowupMessageOptions {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
}

/// In Luau { type: "After" | "Around" | "Before", id: MessageId }
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
pub enum MessagePagination {
    After { id: serenity::all::MessageId },
    Around { id: serenity::all::MessageId },
    Before { id: serenity::all::MessageId },
}

impl MessagePagination {
    pub fn to_serenity(self) -> serenity::all::MessagePagination {
        match self {
            Self::After { id } => serenity::all::MessagePagination::After(id),
            Self::Around { id } => serenity::all::MessagePagination::Around(id),
            Self::Before { id } => serenity::all::MessagePagination::Before(id),
        }
    }
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateAutoModerationRuleOptions {
    pub reason: String,
    pub data: CreateAutoModRule,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditAutoModerationRuleOptions {
    pub rule_id: serenity::all::RuleId,
    pub reason: String,
    pub data: EditAutoModRule,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteAutoModerationRuleOptions {
    pub rule_id: serenity::all::RuleId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditChannelPermissionsOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub target_id: serenity::all::TargetId,
    pub allow: MultiOption<serenity::all::Permissions>,
    pub deny: MultiOption<serenity::all::Permissions>,
    #[serde(rename = "type")]
    pub kind: u8,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateChannelInviteOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: CreateInvite,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteChannelPermissionOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub overwrite_id: serenity::all::TargetId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FollowAnnouncementChannel {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: FollowAnnouncementChannelData,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddGuildMemberRoleOptions {
    pub user_id: serenity::all::UserId,
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildMemberRoleOptions {
    pub user_id: serenity::all::UserId,
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildMemberOptions {
    pub user_id: serenity::all::UserId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildBansOptions {
    pub limit: Option<serenity::nonmax::NonMaxU16>,
    pub before: Option<serenity::all::UserId>,
    pub after: Option<serenity::all::UserId>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildBanOptions {
    pub user_id: serenity::all::UserId,
    pub reason: String,
    pub delete_message_seconds: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyChannelPosition {
    pub id: serenity::all::GenericChannelId,
    pub position: u16,
    pub lock_permissions: Option<bool>,
    pub parent_id: Option<serenity::all::GenericChannelId>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildMembersOptions {
    pub limit: Option<serenity::nonmax::NonMaxU16>,
    pub after: Option<serenity::all::UserId>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SearchGuildMembersOptions {
    pub query: String,
    pub limit: Option<serenity::nonmax::NonMaxU16>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildMemberOptions {
    pub user_id: serenity::all::UserId,
    pub reason: String,
    pub data: EditMember,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildBanOptions {
    pub user_id: serenity::all::UserId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildRoleOptions {
    pub reason: String,
    pub data: EditRole,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyRolePositionOptions {
    pub data: Vec<ModifyRolePosition>,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyRolePosition {
    pub id: serenity::all::RoleId,
    pub position: i16,
}

impl PartialEq<serenity::all::Role> for ModifyRolePosition {
    fn eq(&self, other: &serenity::all::Role) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<serenity::all::Role> for ModifyRolePosition {
    fn partial_cmp(&self, other: &serenity::all::Role) -> Option<Ordering> {
        if self.position == other.position {
            Some(self.id.cmp(&other.id))
        } else {
            Some(self.position.cmp(&other.position))
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditGuildRoleOptions {
    pub role_id: serenity::all::RoleId,
    pub reason: String,
    pub data: EditRole,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteGuildRoleOptions {
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetInviteOptions {
    pub code: String,
    pub with_counts: Option<bool>,     // default to false
    pub with_expiration: Option<bool>, // default to false
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInviteOptions {
    pub code: String,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiraidFusedMemberSingle {
    pub member: serenity::all::Member,
    pub resolved_perms: serenity::all::Permissions,
}

/// A fused member contains both a member, the guild and the resolved permissions of
/// the member in the guild. This is useful for operations that require both the member and the guild context, such as permission checks.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiraidFusedMember {
    pub guild: serenity::all::PartialGuild,
    pub members: Vec<AntiraidFusedMemberSingle>,
}

// Webhooks

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateWebhookOptions {
    pub channel_id: serenity::all::GenericChannelId,
    pub reason: String,
    pub data: CreateWebhook,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditWebhookOptions {
    pub webhook_id: serenity::all::WebhookId,
    pub reason: String,
    pub data: EditWebhook,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteWebhookOptions {
    pub webhook_id: serenity::all::WebhookId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecuteWebhookOptions {
    pub webhook_id: serenity::all::WebhookId,
    pub webhook_token: String,
    pub thread_id: Option<serenity::all::ThreadId>,
    pub reason: String,
    pub data: ExecuteWebhook,
}