use crate::plugins::antiraid::typesext::MultiOption;

use super::types::{
    CreateAutoModRule, CreateCommand, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateMessage, EditAutoModRule, EditChannel,
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
