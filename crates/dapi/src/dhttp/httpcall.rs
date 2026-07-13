#![allow(clippy::missing_errors_doc)]

use std::borrow::Cow;

use super::routing::Route;
use super::{
    GuildPagination, HttpError, MessagePagination, UserPagination,
};
use crate::types::{ModifyChannelPosition, ModifyRolePosition};
use crate::{
    AnyId, ApplicationId, ChannelId, CommandId, EmojiId, GuildId, InteractionId, MessageId, RoleId, RuleId, UserId, WebhookId
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::header::{HeaderMap as Headers, HeaderValue};
use std::result::Result as StdResult;

type Result<T, E = HttpError> = StdResult<T, E>;
pub type ResultJson = Result<serde_json::Value>;

pub enum HttpCall<'a> {
    AddMemberRole {
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
        audit_log_reason: Option<&'a str>,
    },
    BanUser {
        guild_id: GuildId,
        user_id: UserId,
        delete_message_seconds: u32,
        reason: Option<&'a str>,
    },
    BroadcastTyping {
        channel_id: ChannelId,
    },
    CreateChannel {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    CreateStageInstance {
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    CreateEmoji {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    CreateApplicationEmoji {
        application_id: ApplicationId,
        map: Vec<u8>,
    },
    CreateFollowupMessage {
        application_id: ApplicationId,
        interaction_token: &'a str,
        map: Vec<u8>,
    },
    CreateGlobalCommand {
        application_id: ApplicationId,
        map: Vec<u8>,
    },
    CreateGlobalCommands {
        application_id: ApplicationId,
        map: Vec<u8>,
    },
    CreateGuildCommands {
        application_id: ApplicationId,
        guild_id: GuildId,
        map: Vec<u8>,
    },
    CreateGuildCommand {
        application_id: ApplicationId,
        guild_id: GuildId,
        map: Vec<u8>,
    },

    CreateInteractionResponse {
        interaction_id: InteractionId,
        interaction_token: &'a str,
        map: Vec<u8>,
    },
    CreateInvite {
        channel_id: ChannelId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    CreatePermission {
        channel_id: ChannelId,
        target_id: AnyId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    CreateReaction {
        channel_id: ChannelId,
        message_id: MessageId,
        reaction_type: &'a str,
    },
    CreateRole {
        guild_id: GuildId,
        body: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    DeleteChannel {
        channel_id: ChannelId,
        audit_log_reason: Option<&'a str>,
    },
    DeleteStageInstance {
        channel_id: ChannelId,
        audit_log_reason: Option<&'a str>,
    },
    DeleteEmoji {
        guild_id: GuildId,
        emoji_id: EmojiId,
        audit_log_reason: Option<&'a str>,
    },
    DeleteApplicationEmoji {
        application_id: ApplicationId,
        emoji_id: EmojiId,
    },
    DeleteFollowupMessage {
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
    },
    DeleteGlobalCommand {
        application_id: ApplicationId,
        command_id: CommandId,
    },
    DeleteGuildCommand {
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    },
    DeleteInvite {
        code: &'a str,
        audit_log_reason: Option<&'a str>,
    },
    DeleteMessage {
        channel_id: ChannelId,
        message_id: MessageId,
        audit_log_reason: Option<&'a str>,
    },
    DeleteMessages {
        channel_id: ChannelId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    DeleteMessageReactions {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    DeleteMessageReactionEmoji {
        channel_id: ChannelId,
        message_id: MessageId,
        reaction_type: &'a str,
    },
    DeleteOriginalInteractionResponse {
        application_id: ApplicationId,
        interaction_token: &'a str,
    },
    DeletePermission {
        channel_id: ChannelId,
        target_id: AnyId,
        audit_log_reason: Option<&'a str>,
    },
    DeleteReaction {
        channel_id: ChannelId,
        message_id: MessageId,
        user_id: UserId,
        reaction_type: &'a str,
    },
    DeleteReactionMe {
        channel_id: ChannelId,
        message_id: MessageId,
        reaction_type: &'a str,
    },
    DeleteRole {
        guild_id: GuildId,
        role_id: RoleId,
        audit_log_reason: Option<&'a str>,
    },
    DeleteWebhook {
        webhook_id: WebhookId,
        audit_log_reason: Option<&'a str>,
    },
    EditChannel {
        channel_id: ChannelId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditStageInstance {
        channel_id: ChannelId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditEmoji {
        guild_id: GuildId,
        emoji_id: EmojiId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditApplicationEmoji {
        application_id: ApplicationId,
        emoji_id: EmojiId,
        map: Vec<u8>,
    },
    EditFollowupMessage {
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
        map: Vec<u8>,
    },
    GetFollowupMessage {
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
    },
    EditGlobalCommand {
        application_id: ApplicationId,
        command_id: CommandId,
        map: Vec<u8>,
    },
    EditGuild {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditGuildCommand {
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
        map: Vec<u8>,
    },
    EditGuildCommandPermissions {
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
        map: Vec<u8>,
    },
    EditGuildChannelPositions {
        guild_id: GuildId,
        value: &'a [ModifyChannelPosition],
    },
    ListActiveGuildThreads {
        guild_id: GuildId,
    },
    EditGuildWidget {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditGuildWelcomeScreen {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditMember {
        guild_id: GuildId,
        user_id: UserId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditMessage {
        channel_id: ChannelId,
        message_id: MessageId,
        map: Vec<u8>,
    },
    EditMemberMe {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditCurrentMember {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    FollowNewsChannel {
        news_channel_id: ChannelId,
        map: Vec<u8>,
    },
    GetOriginalInteractionResponse {
        application_id: ApplicationId,
        interaction_token: &'a str,
    },
    EditOriginalInteractionResponse {
        application_id: ApplicationId,
        interaction_token: &'a str,
        map: Vec<u8>,
    },
    EditProfile {
        map: Vec<u8>,
    },
    EditRole {
        guild_id: GuildId,
        role_id: RoleId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditRolePositions {
        guild_id: GuildId,
        positions: &'a [ModifyRolePosition],
        audit_log_reason: Option<&'a str>,
    },
    GetBans {
        guild_id: GuildId,
        target: Option<UserPagination>,
        limit: Option<u16>,
    },
    GetBan {
        guild_id: GuildId,
        user_id: UserId,
    },
    GetAuditLogs {
        guild_id: GuildId,
        action_type: Option<u16>,
        user_id: Option<UserId>,
        before: Option<AnyId>,
        limit: Option<u8>,
    },
    GetAutomodRules {
        guild_id: GuildId,
    },
    GetAutomodRule {
        guild_id: GuildId,
        rule_id: RuleId,
    },
    CreateAutomodRule {
        guild_id: GuildId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    EditAutomodRule {
        guild_id: GuildId,
        rule_id: RuleId,
        map: Vec<u8>,
        audit_log_reason: Option<&'a str>,
    },
    DeleteAutomodRule {
        guild_id: GuildId,
        rule_id: RuleId,
        audit_log_reason: Option<&'a str>,
    },
    GetChannelInvites {
        channel_id: ChannelId,
    },
    GetChannel {
        channel_id: ChannelId,
    },
    GetChannels {
        guild_id: GuildId,
    },
    GetStageInstance {
        channel_id: ChannelId,
    },
    GetCurrentApplicationInfo,
    GetCurrentUser,
    GetEmojis {
        guild_id: GuildId,
    },
    GetEmoji {
        guild_id: GuildId,
        emoji_id: EmojiId,
    },
    GetApplicationEmojis {
        application_id: ApplicationId,
    },
    GetApplicationEmoji {
        application_id: ApplicationId,
        emoji_id: EmojiId,
    },
    GetGlobalCommands {
        application_id: ApplicationId,
    },
    GetGlobalCommandsWithLocalizations {
        application_id: ApplicationId,
    },
    GetGlobalCommand {
        application_id: ApplicationId,
        command_id: CommandId,
    },
    GetGuild {
        guild_id: GuildId,
        with_counts: bool,
    },
    GetGuildMember {
        guild_id: GuildId,
        user_id: UserId,
    },
    GetGuildCommands {
        application_id: ApplicationId,
        guild_id: GuildId,
    },
    GetGuildCommandsWithLocalizations {
        application_id: ApplicationId,
        guild_id: GuildId,
    },
    GetGuildCommand {
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    },
    GetGuildWidget {
        guild_id: GuildId,
    },
    GetGuildPreview {
        guild_id: GuildId,
    },
    GetGuildWelcomeScreen {
        guild_id: GuildId,
    },
    GetGuildInvites {
        guild_id: GuildId,
    },
    GetGuildVanityUrl {
        guild_id: GuildId,
    },
    GetGuildMembers {
        guild_id: GuildId,
        limit: Option<u16>,
        after: Option<UserId>,
    },
    GetGuildRole {
        guild_id: GuildId,
        role_id: RoleId,
    },
    GetGuildRoles {
        guild_id: GuildId,
    },
    GetCurrentUserGuilds {
        target: Option<GuildPagination>,
        limit: Option<u8>,
        with_counts: bool,
    },
    GetCurrentUserGuildMember {
        guild_id: GuildId,
    },
    GetInvite {
        code: &'a str,
        member_counts: bool,
    },
    GetMember {
        guild_id: GuildId,
        user_id: UserId,
    },
    GetMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    GetMessages {
        channel_id: ChannelId,
        target: Option<MessagePagination>,
        limit: Option<u8>,
    },
    GetPins {
        channel_id: ChannelId,
    },
    GetSkus {
        application_id: ApplicationId,
    },
    GetUser {
        user_id: UserId,
    },
    KickMember {
        guild_id: GuildId,
        user_id: UserId,
        reason: Option<&'a str>,
    },
    CreateChannelMessage {
        channel_id: ChannelId,
        map: Vec<u8>,
    },
    PinMessage {
        channel_id: ChannelId,
        message_id: MessageId,
        audit_log_reason: Option<&'a str>,
    },
    RemoveBan {
        guild_id: GuildId,
        user_id: UserId,
        audit_log_reason: Option<&'a str>,
    },
    RemoveMemberRole {
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
        audit_log_reason: Option<&'a str>,
    },
    SearchGuildMembers {
        guild_id: GuildId,
        query: &'a str,
        limit: Option<u16>,
    },
    StartGuildPrune {
        guild_id: GuildId,
        days: u8,
        audit_log_reason: Option<&'a str>,
    },
    UnpinMessage {
        channel_id: ChannelId,
        message_id: MessageId,
        audit_log_reason: Option<&'a str>,
    },
}

pub struct CustomRoute {
    pub url: String,
    pub body: Option<Vec<u8>>,
    pub headers: Option<Headers>,
    pub method: reqwest::Method,
}

impl<'a> HttpCall<'a> {
    pub fn into_url_and_body(self) -> CustomRoute {
        match self {
            Self::AddMemberRole {
                guild_id,
                user_id,
                role_id,
                audit_log_reason,
            } => {
                let route = Route::GuildMemberRole {
                    guild_id,
                    role_id,
                    user_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PUT }
            }
            Self::BanUser {
                guild_id,
                user_id,
                delete_message_seconds,
                reason,
            } => {
                let route = Route::GuildBan { guild_id, user_id };
                let mut path = route.path();
                path.push_str(&format!(
                    "?delete_message_seconds={}",
                    delete_message_seconds
                ));
                CustomRoute { url: path, body: None, headers: reason.as_deref().map(reason_into_header), method: reqwest::Method::PUT }
            }
            Self::BroadcastTyping { channel_id } => {
                let route = Route::ChannelTyping { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::POST }
            }
            Self::CreateChannel {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildChannels { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::CreateStageInstance {
                map,
                audit_log_reason,
            } => {
                let route = Route::StageInstances;
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::CreateEmoji {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildEmojis { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::CreateApplicationEmoji {
                application_id, map } => {
                let route = Route::Emojis {
                    application_id: application_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }
            Self::CreateFollowupMessage {
                application_id,
                interaction_token,
                map,
            } => {
                let route = Route::WebhookFollowupMessages {
                    application_id: application_id,
                    token: interaction_token,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }
            Self::CreateGlobalCommand {
                application_id, map } => {
                let route = Route::Commands {
                    application_id: application_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }
            Self::CreateGlobalCommands {
                application_id, map } => {
                let route = Route::Commands {
                    application_id: application_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PUT }
            }
            Self::CreateGuildCommands {
                application_id, guild_id, map } => {
                let route = Route::GuildCommands {
                    application_id: application_id,
                    guild_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PUT }
            }
            Self::CreateGuildCommand {
                application_id, guild_id, map } => {
                let route = Route::GuildCommands {
                    application_id: application_id,
                    guild_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }

            Self::CreateInteractionResponse {
                interaction_id,
                interaction_token,
                map,
            } => {
                let route = Route::InteractionResponse {
                    interaction_id,
                    token: interaction_token,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }
            Self::CreateInvite {
                channel_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::ChannelInvites { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::CreatePermission {
                channel_id,
                target_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::ChannelPermission {
                    channel_id,
                    target_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::CreateReaction {
                channel_id,
                message_id,
                reaction_type,
            } => {
                let route = Route::ChannelMessageReactionMe {
                    channel_id,
                    message_id,
                    reaction: reaction_type,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::PUT }
            }
            Self::CreateRole {
                guild_id,
                body,
                audit_log_reason,
            } => {
                let route = Route::GuildRoles { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(body), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::DeleteChannel {
                channel_id,
                audit_log_reason,
            } => {
                let route = Route::Channel { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteStageInstance {
                channel_id,
                audit_log_reason,
            } => {
                let route = Route::StageInstance { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteEmoji {
                guild_id,
                emoji_id,
                audit_log_reason,
            } => {
                let route = Route::GuildEmoji { guild_id, emoji_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteApplicationEmoji {
                application_id, emoji_id } => {
                let route = Route::Emoji {
                    application_id: application_id,
                    emoji_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteFollowupMessage {
                application_id,
                interaction_token,
                message_id,
            } => {
                let route = Route::WebhookFollowupMessage {
                    application_id: application_id,
                    token: interaction_token,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteGlobalCommand {
                application_id, command_id } => {
                let route = Route::Command {
                    application_id: application_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteGuildCommand {
                application_id,
                guild_id,
                command_id,
            } => {
                let route = Route::GuildCommand {
                    application_id: application_id,
                    guild_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteInvite {
                code,
                audit_log_reason,
            } => {
                let route = Route::Invite { code };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteMessage {
                channel_id,
                message_id,
                audit_log_reason,
            } => {
                let route = Route::ChannelMessage {
                    channel_id,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteMessages {
                channel_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::ChannelMessagesBulkDelete { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteMessageReactions {
                channel_id,
                message_id,
            } => {
                let route = Route::ChannelMessageReactions {
                    channel_id,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteMessageReactionEmoji {
                channel_id,
                message_id,
                reaction_type,
            } => {
                let route = Route::ChannelMessageReactionEmoji {
                    channel_id,
                    message_id,
                    reaction: &reaction_type,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteOriginalInteractionResponse {
                application_id, interaction_token } => {
                let route = Route::WebhookOriginalInteractionResponse {
                    application_id: application_id,
                    token: interaction_token,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeletePermission {
                channel_id,
                target_id,
                audit_log_reason,
            } => {
                let route = Route::ChannelPermission {
                    channel_id,
                    target_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteReaction {
                channel_id,
                message_id,
                user_id,
                reaction_type,
            } => {
                let route = Route::ChannelMessageReaction {
                    channel_id,
                    message_id,
                    user_id,
                    reaction: &reaction_type,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteReactionMe {
                channel_id,
                message_id,
                reaction_type,
            } => {
                let route = Route::ChannelMessageReactionMe {
                    channel_id,
                    message_id,
                    reaction: &reaction_type,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::DELETE }
            }
            Self::DeleteRole {
                guild_id,
                role_id,
                audit_log_reason,
            } => {
                let route = Route::GuildRole { guild_id, role_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::DeleteWebhook {
                webhook_id,
                audit_log_reason,
            } => {
                let route = Route::Webhook { webhook_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::EditChannel {
                channel_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::Channel { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditStageInstance {
                channel_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::StageInstance { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditEmoji {
                guild_id,
                emoji_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildEmoji { guild_id, emoji_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditApplicationEmoji {
                application_id, emoji_id, map } => {
                let route = Route::Emoji {
                    application_id: application_id,
                    emoji_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditFollowupMessage {
                application_id,
                interaction_token,
                message_id,
                map,
            } => {
                let route = Route::WebhookFollowupMessage {
                    application_id: application_id,
                    token: interaction_token,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::GetFollowupMessage {
                application_id,
                interaction_token,
                message_id,
            } => {
                let route = Route::WebhookFollowupMessage {
                    application_id: application_id,
                    token: interaction_token,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::EditGlobalCommand {
                application_id, command_id, map } => {
                let route = Route::Command {
                    application_id: application_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditGuild {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::Guild { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditGuildCommand {
                application_id,
                guild_id,
                command_id,
                map,
            } => {
                let route = Route::GuildCommand {
                    application_id: application_id,
                    guild_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditGuildCommandPermissions {
                application_id,
                guild_id,
                command_id,
                map,
            } => {
                let route = Route::GuildCommandPermissions {
                    application_id: application_id,
                    guild_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditGuildChannelPositions { guild_id, value } => {
                let route = Route::GuildChannels { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(serde_json::to_vec(value).unwrap()), headers: None, method: reqwest::Method::PATCH }
            }
            Self::ListActiveGuildThreads { guild_id } => {
                let route = Route::GuildThreadsActive { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditGuildWidget {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildWidget { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditGuildWelcomeScreen {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildWelcomeScreen { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditMember {
                guild_id,
                user_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildMember { guild_id, user_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditMessage {
                channel_id,
                message_id,
                map,
            } => {
                let route = Route::ChannelMessage {
                    channel_id,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditMemberMe {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildMemberMe { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditCurrentMember {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildMemberMe { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::FollowNewsChannel {
                news_channel_id,
                map,
            } => {
                let route = Route::ChannelFollowNews {
                    channel_id: news_channel_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }
            Self::GetOriginalInteractionResponse {
                application_id, interaction_token } => {
                let route = Route::WebhookOriginalInteractionResponse {
                    application_id: application_id,
                    token: interaction_token,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::EditOriginalInteractionResponse {
                application_id,
                interaction_token,
                map,
            } => {
                let route = Route::WebhookOriginalInteractionResponse {
                    application_id: application_id,
                    token: interaction_token,
                };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditProfile { map } => {
                let route = Route::UserMe;
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::PATCH }
            }
            Self::EditRole {
                guild_id,
                role_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildRole { guild_id, role_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::EditRolePositions {
                guild_id,
                positions,
                audit_log_reason,
            } => {
                let route = Route::GuildRoles { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(serde_json::to_vec(positions).unwrap()), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::GetBans {
                guild_id,
                target,
                limit,
            } => {
                let route = Route::GuildBans { guild_id };
                let mut path = route.path();
                let mut params = Vec::new();
                if let Some(target) = target {
                    let (name, id) = match target {
                        UserPagination::After(id) => ("after", id),
                        UserPagination::Before(id) => ("before", id),
                    };

                    params.push(format!("{name}={id}"));
                }
                if let Some(limit) = limit {
                    params.push(format!("limit={limit}"))
                }
                if !params.is_empty() {
                    path.push_str("?");
                    path.push_str(&params.join("&"));
                }
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetBan { guild_id, user_id } => {
                let route = Route::GuildBan { guild_id, user_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetAuditLogs {
                guild_id,
                action_type,
                user_id,
                before,
                limit,
            } => {
                let route = Route::GuildAuditLogs { guild_id };
                let mut path = route.path();
                let mut params = Vec::new();

                if let Some(action_type) = action_type {
                    params.push(format!("action_type={action_type}"));
                }
                if let Some(user_id) = user_id {
                    params.push(format!("user_id={user_id}"))
                }
                if let Some(before) = before {
                    params.push(format!("before={before}"))
                }
                if let Some(limit) = limit {
                    params.push(format!("limit={limit}"))
                }

                if !params.is_empty() {
                    path.push_str("?");
                    path.push_str(&params.join("&"));
                }

                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetAutomodRules { guild_id } => {
                let route = Route::GuildAutomodRules { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetAutomodRule { guild_id, rule_id } => {
                let route = Route::GuildAutomodRule { guild_id, rule_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::CreateAutomodRule {
                guild_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildAutomodRules { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::EditAutomodRule {
                guild_id,
                rule_id,
                map,
                audit_log_reason,
            } => {
                let route = Route::GuildAutomodRule { guild_id, rule_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PATCH }
            }
            Self::DeleteAutomodRule {
                guild_id,
                rule_id,
                audit_log_reason,
            } => {
                let route = Route::GuildAutomodRule { guild_id, rule_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::GetChannelInvites { channel_id } => {
                let route = Route::ChannelInvites { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetChannel { channel_id } => {
                let route = Route::Channel { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetChannels { guild_id } => {
                let route = Route::GuildChannels { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetStageInstance { channel_id } => {
                let route = Route::StageInstance { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetCurrentApplicationInfo => {
                let route = Route::OAuth2ApplicationCurrent;
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetCurrentUser => {
                let route = Route::UserMe;
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetEmojis { guild_id } => {
                let route = Route::GuildEmojis { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetEmoji { guild_id, emoji_id } => {
                let route = Route::GuildEmoji { guild_id, emoji_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetApplicationEmojis { application_id } => {
                let route = Route::Emojis {
                    application_id: application_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetApplicationEmoji {
                application_id, emoji_id } => {
                let route = Route::Emoji {
                    application_id: application_id,
                    emoji_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGlobalCommands { application_id } => {
                let route = Route::Commands {
                    application_id: application_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGlobalCommandsWithLocalizations { application_id } => {
                let route = Route::Commands {
                    application_id: application_id,
                };
                let mut path = route.path();
                path.push_str("?with_localizations=true");
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGlobalCommand {
                application_id, command_id } => {
                let route = Route::Command {
                    application_id: application_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuild { guild_id, with_counts } => {
                let route = Route::Guild { guild_id };
                let mut path = route.path();
                if with_counts {
                    path.push_str("?with_counts=true");
                }
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildMember { guild_id, user_id } => {
                let route = Route::GuildMember { guild_id, user_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildCommands {
                application_id, guild_id } => {
                let route = Route::GuildCommands {
                    application_id: application_id,
                    guild_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildCommandsWithLocalizations {
                application_id, guild_id } => {
                let route = Route::GuildCommands {
                    application_id: application_id,
                    guild_id,
                };
                let mut path = route.path();
                path.push_str("?with_localizations=true");
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildCommand {
                application_id,
                guild_id,
                command_id,
            } => {
                let route = Route::GuildCommand {
                    application_id: application_id,
                    guild_id,
                    command_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildWidget { guild_id } => {
                let route = Route::GuildWidget { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildPreview { guild_id } => {
                let route = Route::GuildPreview { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildWelcomeScreen { guild_id } => {
                let route = Route::GuildWelcomeScreen { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildInvites { guild_id } => {
                let route = Route::GuildInvites { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildVanityUrl { guild_id } => {
                let route = Route::GuildVanityUrl { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildMembers {
                guild_id,
                limit,
                after,
            } => {
                let route = Route::GuildMembers { guild_id };
                let mut path = route.path();
                let mut params = Vec::new();
                if let Some(limit) = limit {
                    params.push(format!("limit={limit}"))
                }
                if let Some(after) = after {
                    params.push(format!("after={after}"));
                }
                if !params.is_empty() {
                    path.push_str("?");
                    path.push_str(&params.join("&"));
                }
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildRole { guild_id, role_id } => {
                let route = Route::GuildRole { guild_id, role_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetGuildRoles { guild_id } => {
                let route = Route::GuildRoles { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetCurrentUserGuilds { target, limit, with_counts } => {
                let route = Route::UserMeGuilds;
                let mut path = route.path();
                let mut params = Vec::new();
                if let Some(target) = target {
                    let (name, id) = match target {
                        GuildPagination::After(id) => ("after", id),
                        GuildPagination::Before(id) => ("before", id),
                    };

                    params.push(format!("{name}={id}"));
                }
                if let Some(limit) = limit {
                    params.push(format!("limit={limit}"))
                }
                if with_counts {
                    params.push(format!("with_counts=true"))
                }
                if !params.is_empty() {
                    path.push_str("?");
                    path.push_str(&params.join("&"));
                }
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetCurrentUserGuildMember { guild_id } => {
                let route = Route::UserMeGuildMember { guild_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetInvite {
                code,
                member_counts,
            } => {
                let route = Route::Invite { code };
                let mut path = route.path();
                
                if member_counts {
                    path.push_str("?with_counts=true");
                }
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetMember { guild_id, user_id } => {
                let route = Route::GuildMember { guild_id, user_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetMessage {
                channel_id,
                message_id,
            } => {
                let route = Route::ChannelMessage {
                    channel_id,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetMessages {
                channel_id,
                target,
                limit,
            } => {
                let route = Route::ChannelMessages { channel_id };
                let mut path = route.path();
                let mut params = Vec::new();
                if let Some(target) = target {
                    let (name, id) = match target {
                        MessagePagination::After(id) => ("after", id),
                        MessagePagination::Around(id) => ("around", id),
                        MessagePagination::Before(id) => ("before", id),
                    };

                    params.push(format!("{name}={id}"));
                }
                if let Some(limit) = limit {
                    params.push(format!("limit={limit}"))
                }
                if !params.is_empty() {
                    path.push_str("?");
                    path.push_str(&params.join("&"));
                }
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetPins { channel_id } => {
                let route = Route::ChannelPins { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetSkus { application_id } => {
                let route = Route::Skus {
                    application_id: application_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::GetUser { user_id } => {
                let route = Route::User { user_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::KickMember {
                guild_id,
                user_id,
                reason,
            } => {
                let route = Route::GuildMember { guild_id, user_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::CreateChannelMessage {
                channel_id,
                map,
            } => {
                let route = Route::ChannelMessages { channel_id };
                let path = route.path();
                CustomRoute { url: path, body: Some(map), headers: None, method: reqwest::Method::POST }
            }
            Self::PinMessage {
                channel_id,
                message_id,
                audit_log_reason,
            } => {
                let route = Route::ChannelPin {
                    channel_id,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::PUT }
            }
            Self::RemoveBan {
                guild_id,
                user_id,
                audit_log_reason,
            } => {
                let route = Route::GuildBan { guild_id, user_id };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::RemoveMemberRole {
                guild_id,
                user_id,
                role_id,
                audit_log_reason,
            } => {
                let route = Route::GuildMemberRole {
                    guild_id,
                    user_id,
                    role_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
            Self::SearchGuildMembers {
                guild_id,
                query,
                limit,
            } => {
                let route = Route::GuildMembersSearch { guild_id, query, limit: limit.unwrap_or(1) };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: None, method: reqwest::Method::GET }
            }
            Self::StartGuildPrune {
                guild_id,
                days,
                audit_log_reason,
            } => {
                let route = Route::GuildPrune { guild_id };
                let mut path = route.path();
                path.push_str(&format!("?days={}", days));
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::POST }
            }
            Self::UnpinMessage {
                channel_id,
                message_id,
                audit_log_reason,
            } => {
                let route = Route::ChannelPin {
                    channel_id,
                    message_id,
                };
                let path = route.path();
                CustomRoute { url: path, body: None, headers: audit_log_reason.as_deref().map(reason_into_header), method: reqwest::Method::DELETE }
            }
        }
    }
}

fn reason_into_header(reason: &str) -> Headers {
    let mut headers = Headers::new();

    // "The X-Audit-Log-Reason header supports 1-512 URL-encoded UTF-8 characters."
    // https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object
    let header_value = match Cow::from(utf8_percent_encode(reason, NON_ALPHANUMERIC)) {
        Cow::Borrowed(value) => HeaderValue::from_str(value),
        Cow::Owned(value) => HeaderValue::try_from(value),
    }
    .expect("Invalid header value even after percent encode");

    headers.insert("X-Audit-Log-Reason", header_value);
    headers
}
