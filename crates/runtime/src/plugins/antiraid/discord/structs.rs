use crate::plugins::antiraid::typesext::MultiOption;
use std::cmp::Ordering;

use super::types::{
    CreateAutoModRule, CreateCommand, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateMessage, EditAutoModRule, CreateChannel, EditChannel, EditMember, EditGuild, EditRole,
    CreateInvite
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetAuditLogOptions {
    pub action_type: Option<serenity::all::audit_log::Action>,
    pub user_id: Option<serenity::all::UserId>,
    pub before: Option<serenity::all::AuditLogEntryId>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

impl Default for GetAuditLogOptions {
    fn default() -> Self {
        Self {
            action_type: Some(serenity::all::audit_log::Action::GuildUpdate),
            user_id: Some(serenity::all::UserId::default()),
            before: Some(serenity::all::AuditLogEntryId::default()),
            limit: Some(serenity::nonmax::NonMaxU8::default()),
        }
    }
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct GetChannelOptions {
    pub channel_id: serenity::all::ChannelId,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateChannelOptions {
    pub reason: String,
    pub data: CreateChannel,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditChannelOptions {
    pub channel_id: serenity::all::ChannelId,
    pub reason: String,
    pub data: EditChannel,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteChannelOptions {
    pub channel_id: serenity::all::ChannelId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMessagesOptions {
    pub channel_id: serenity::all::ChannelId,
    pub target: Option<MessagePagination>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMessageOptions {
    pub channel_id: serenity::all::ChannelId,
    pub message_id: serenity::all::MessageId,
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateMessageOptions {
    pub channel_id: serenity::all::ChannelId, // Channel *must* be in the same guild
    pub data: CreateMessage,
}

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
pub struct CreateFollowupMessageOptions {
    pub interaction_token: String,
    pub data: CreateInteractionResponseFollowup,
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
pub struct GetAutoModerationRuleOptions {
    pub rule_id: serenity::all::RuleId,
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
    pub channel_id: serenity::all::ChannelId,
    pub target_id: serenity::all::TargetId,
    pub allow: MultiOption<serenity::all::Permissions>,
    pub deny: MultiOption<serenity::all::Permissions>,
    #[serde(rename = "type")]
    pub kind: u8,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateChannelInviteOptions {
    pub channel_id: serenity::all::ChannelId,
    pub data: CreateInvite,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteChannelPermissionOptions {
    pub channel_id: serenity::all::ChannelId,
    pub overwrite_id: serenity::all::TargetId,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildOptions {
    pub data: EditGuild,
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
    pub id: serenity::all::ChannelId,
    pub position: u16,
    pub lock_permissions: Option<bool>,
    pub parent_id: Option<serenity::all::ChannelId>,
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
pub struct AntiRaidCheckPermissionsOptions {
    pub user_id: serenity::all::UserId,
    pub needed_permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsAndHierarchyOptions {
    pub user_id: serenity::all::UserId,
    pub target_id: serenity::all::UserId,
    pub needed_permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsResponse {
    pub partial_guild: serenity::all::PartialGuild,
    pub member: serenity::all::Member,
    pub permissions: serenity::all::Permissions
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissionsOptions {
    pub user_id: serenity::all::UserId,
    pub channel_id: serenity::all::ChannelId,
    pub needed_permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissionsResponse {
    pub partial_guild: serenity::all::PartialGuild,
    pub channel: serenity::all::GuildChannel,
    pub member: serenity::all::Member,
    pub permissions: serenity::all::Permissions
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
pub struct GetInviteOptions {
    pub code: String,
    pub with_counts: Option<bool>, // default to false
    pub with_expiration: Option<bool>, // default to false
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,    
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInviteOptions {
    pub code: String,
    pub reason: String,
}