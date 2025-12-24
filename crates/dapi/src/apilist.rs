use crate::{ApiReq, antiraid_check_channel_permissions::AntiRaidCheckChannelPermissions, antiraid_check_permissions::{AntiRaidCheckPermissions, AntiRaidCheckPermissionsAndHierarchy}, antiraid_get_fused_member::AntiRaidGetFusedMember, api::{auditlogs::GetAuditLog, automoderation::{get_auto_moderation_rule::GetAutoModerationRule, list_auto_moderation_rules::ListAutoModerationRules}, channels::edit_channel::EditChannel, guilds::modify_guild::ModifyGuild}, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum API {
    // Antiraid specific
    AntiRaidCheckChannelPermissions(AntiRaidCheckChannelPermissions),
    AntiRaidCheckPermissions(AntiRaidCheckPermissions),
    AntiRaidCheckPermissionsAndHierarchy(AntiRaidCheckPermissionsAndHierarchy),
    AntiRaidGetFusedMember(AntiRaidGetFusedMember),

    // Audit logs
    GetAuditLog(GetAuditLog),

    // Auto Moderation
    GetAutoModerationRule(GetAutoModerationRule),
    ListAutoModerationRules(ListAutoModerationRules),
    CreateAutoModerationRule(crate::api::automoderation::create_auto_moderation_rule::CreateAutoModerationRule),
    EditAutoModerationRule(crate::api::automoderation::edit_auto_moderation_rule::EditAutoModerationRule),
    DeleteAutoModerationRule(crate::api::automoderation::delete_auto_moderation_rule::DeleteAutoModerationRule),

    // Channels
    EditChannel(EditChannel),
    GetChannel(crate::api::channels::get_channel::GetChannel),
    DeleteChannel(crate::api::channels::delete_channel::DeleteChannel),
    EditChannelPermissions(crate::api::channels::edit_channel_permissions::EditChannelPermissions),
    GetChannelInvites(crate::api::channels::get_channel_invites::GetChannelInvites),
    CreateChannelInvite(crate::api::channels::create_channel_invite::CreateChannelInvite),
    DeleteChannelPermission(crate::api::channels::delete_channel_permission::DeleteChannelPermission),
    FollowAnnouncementChannel(crate::api::channels::follow_announcement_channel::FollowAnnouncementChannel),

    // Guilds
    ModifyGuild(ModifyGuild),
    GetGuild(crate::api::guilds::get_guild::GetGuild),
    GetGuildPreview(crate::api::guilds::get_guild_preview::GetGuildPreview),
    GetGuildChannels(crate::api::guilds::get_guild_channels::GetGuildChannels),
    CreateGuildChannel(crate::api::guilds::create_guild_channel::CreateGuildChannel),
    ModifyGuildChannelPositions(crate::api::guilds::modify_guild_channel_positions::ModifyGuildChannelPositions),
    ListActiveGuildThreads(crate::api::guilds::list_active_guild_threads::ListActiveGuildThreads),
    GetGuildMember(crate::api::guilds::get_guild_member::GetGuildMember),
    ListGuildMembers(crate::api::guilds::list_guild_members::ListGuildMembers),
    SearchGuildMembers(crate::api::guilds::search_guild_members::SearchGuildMembers),
    ModifyGuildMember(crate::api::guilds::modify_guild_member::ModifyGuildMember),
    AddGuildMemberRole(crate::api::guilds::add_guild_member_role::AddGuildMemberRole),
    RemoveGuildMemberRole(crate::api::guilds::remove_guild_member_role::RemoveGuildMemberRole),
    RemoveGuildMember(crate::api::guilds::remove_guild_member::RemoveGuildMember),
    GetGuildBans(crate::api::guilds::get_guild_bans::GetGuildBans),
    GetGuildBan(crate::api::guilds::get_guild_ban::GetGuildBan),
    CreateGuildBan(crate::api::guilds::create_guild_ban::CreateGuildBan),
    RemoveGuildBan(crate::api::guilds::remove_guild_ban::RemoveGuildBan),
    GetGuildRoles(crate::api::guilds::get_guild_roles::GetGuildRoles),
    GetGuildRole(crate::api::guilds::get_guild_role::GetGuildRole),
    CreateGuildRole(crate::api::guilds::create_guild_role::CreateGuildRole),
    ModifyGuildRolePositions(crate::api::guilds::modify_guild_role_positions::ModifyGuildRolePositions),
    ModifyGuildRole(crate::api::guilds::modify_guild_role::ModifyGuildRole),
    DeleteGuildRole(crate::api::guilds::delete_guild_role::DeleteGuildRole),

    // Invites
    GetInvite(crate::api::invites::get_invite::GetInvite),
    DeleteInvite(crate::api::invites::delete_invite::DeleteInvite),

    // Messages
    GetChannelMessages(crate::api::messages::get_channel_messages::GetChannelMessages),
    GetChannelMessage(crate::api::messages::get_channel_message::GetChannelMessage),
    CreateMessage(crate::api::messages::create_message::CreateMessageRequest),
    CrosspostMessage(crate::api::messages::crosspost_message::CrosspostMessage),
    EditMessage(crate::api::messages::edit_message::EditMessageRequest),
    DeleteMessage(crate::api::messages::delete_message::DeleteMessage),
    BulkDeleteMessages(crate::api::messages::bulk_delete_messages::BulkDeleteMessages),

    // Reactions
    CreateReaction(crate::api::reactions::create_reaction::CreateReaction),
    DeleteOwnReaction(crate::api::reactions::delete_own_reaction::DeleteOwnReaction),
    DeleteUserReaction(crate::api::reactions::delete_user_reaction::DeleteUserReaction),
    GetReactions(crate::api::reactions::get_reactions::GetReactions),
    DeleteAllReactions(crate::api::reactions::delete_all_reactions::DeleteAllReactions),
    DeleteAllReactionsForEmoji(crate::api::reactions::delete_all_reactions_for_emoji::DeleteAllReactionsForEmoji),

    // Interactions
    CreateInteractionResponse(crate::api::interactions::create_interaction_response::CreateInteractionResponseRequest),
    GetOriginalInteractionResponse(crate::api::interactions::get_original_interaction_response::GetOriginalInteractionResponse),
    EditOriginalInteractionResponse(crate::api::interactions::edit_original_interaction_response::EditOriginalInteractionResponse),
    DeleteOriginalInteractionResponse(crate::api::interactions::delete_original_interaction_response::DeleteOriginalInteractionResponse),
    CreateFollowupMessage(crate::api::interactions::create_followup_message::CreateFollowupMessage),
    GetFollowupMessage(crate::api::interactions::get_followup_message::GetFollowupMessage),
    EditFollowupMessage(crate::api::interactions::edit_followup_message::EditFollowupMessage),
    DeleteFollowupMessage(crate::api::interactions::delete_followup_message::DeleteFollowupMessage),

    // Commands
    GetGuildCommand(crate::api::commands::get_guild_command::GetGuildCommand),
    GetGuildCommands(crate::api::commands::get_guild_commands::GetGuildCommands),
    CreateGuildCommand(crate::api::commands::create_guild_command::CreateGuildCommand),
    CreateGuildCommands(crate::api::commands::create_guild_commands::CreateGuildCommands),

    // Webhooks
    CreateWebhook(crate::api::webhooks::create_webhook::CreateWebhook),
    GetChannelWebhooks(crate::api::webhooks::get_channel_webhooks::GetChannelWebhooks),
    GetGuildWebhooks(crate::api::webhooks::get_guild_webhooks::GetGuildWebhooks),
    GetWebhook(crate::api::webhooks::get_webhook::GetWebhook),
    ModifyWebhook(crate::api::webhooks::modify_webhook::ModifyWebhook),
    DeleteWebhook(crate::api::webhooks::delete_webhook::DeleteWebhook),
    ExecuteWebhook(crate::api::webhooks::execute_webhook::ExecuteWebhookRequest),
}

macro_rules! jsonify {
    ($e:expr) => {
        serde_json::to_value($e)?
    };
}

macro_rules! op {
    ($this:expr, $req:expr) => {
        {
            let resp = $req.execute($this).await?;
            Ok(jsonify!(resp))
        }
    };
}

impl ApiReq for API {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        match self {
            API::AntiRaidCheckChannelPermissions(req) => op!(this, req),
            API::AntiRaidCheckPermissions(req) => op!(this, req),
            API::AntiRaidCheckPermissionsAndHierarchy(req) => op!(this, req),
            API::AntiRaidGetFusedMember(req) => op!(this, req),
            API::GetAuditLog(req) => op!(this, req),
            API::GetAutoModerationRule(req) => op!(this, req),
            API::ListAutoModerationRules(req) => op!(this, req),
            API::CreateAutoModerationRule(req) => op!(this, req),
            API::EditAutoModerationRule(req) => op!(this, req),
            API::DeleteAutoModerationRule(req) => op!(this, req),
            API::EditChannel(req) => op!(this, req),
            API::GetChannel(req) => op!(this, req),
            API::DeleteChannel(req) => op!(this, req),
            API::EditChannelPermissions(req) => op!(this, req),
            API::GetChannelInvites(req) => op!(this, req),
            API::CreateChannelInvite(req) => op!(this, req),
            API::DeleteChannelPermission(req) => op!(this, req),
            API::FollowAnnouncementChannel(req) => op!(this, req),
            API::ModifyGuild(req) => op!(this, req),
            API::GetGuild(req) => op!(this, req),
            API::GetGuildPreview(req) => op!(this, req),
            API::GetGuildChannels(req) => op!(this, req),
            API::CreateGuildChannel(req) => op!(this, req),
            API::ModifyGuildChannelPositions(req) => op!(this, req),
            API::ListActiveGuildThreads(req) => op!(this, req),
            API::GetGuildMember(req) => op!(this, req),
            API::ListGuildMembers(req) => op!(this, req),
            API::SearchGuildMembers(req) => op!(this, req),
            API::ModifyGuildMember(req) => op!(this, req),
            API::AddGuildMemberRole(req) => op!(this, req),
            API::RemoveGuildMemberRole(req) => op!(this, req),
            API::RemoveGuildMember(req) => op!(this, req),
            API::GetGuildBans(req) => op!(this, req),
            API::GetGuildBan(req) => op!(this, req),
            API::CreateGuildBan(req) => op!(this, req),
            API::RemoveGuildBan(req) => op!(this, req),
            API::GetGuildRoles(req) => op!(this, req),
            API::GetGuildRole(req) => op!(this, req),
            API::CreateGuildRole(req) => op!(this, req),
            API::ModifyGuildRolePositions(req) => op!(this, req),
            API::ModifyGuildRole(req) => op!(this, req),
            API::DeleteGuildRole(req) => op!(this, req),
            API::GetInvite(req) => op!(this, req),
            API::DeleteInvite(req) => op!(this, req),
            API::GetChannelMessages(req) => op!(this, req),
            API::GetChannelMessage(req) => op!(this, req),
            API::CreateMessage(req) => op!(this, req),
            API::CrosspostMessage(req) => op!(this, req),
            API::EditMessage(req) => op!(this, req),
            API::DeleteMessage(req) => op!(this, req),
            API::BulkDeleteMessages(req) => op!(this, req),
            API::CreateReaction(req) => op!(this, req),
            API::DeleteOwnReaction(req) => op!(this, req),
            API::DeleteUserReaction(req) => op!(this, req),
            API::GetReactions(req) => op!(this, req),
            API::DeleteAllReactions(req) => op!(this, req),
            API::DeleteAllReactionsForEmoji(req) => op!(this, req),
            API::CreateInteractionResponse(req) => op!(this, req),
            API::GetOriginalInteractionResponse(req) => op!(this, req),
            API::EditOriginalInteractionResponse(req) => op!(this, req),
            API::DeleteOriginalInteractionResponse(req) => op!(this, req),
            API::CreateFollowupMessage(req) => op!(this, req),
            API::GetFollowupMessage(req) => op!(this, req),
            API::EditFollowupMessage(req) => op!(this, req),
            API::DeleteFollowupMessage(req) => op!(this, req),
            API::GetGuildCommand(req) => op!(this, req),
            API::GetGuildCommands(req) => op!(this, req),
            API::CreateGuildCommand(req) => op!(this, req),
            API::CreateGuildCommands(req) => op!(this, req),
            API::CreateWebhook(req) => op!(this, req),
            API::GetChannelWebhooks(req) => op!(this, req),
            API::GetGuildWebhooks(req) => op!(this, req),
            API::GetWebhook(req) => op!(this, req),
            API::ModifyWebhook(req) => op!(this, req),
            API::DeleteWebhook(req) => op!(this, req),
            API::ExecuteWebhook(req) => op!(this, req),
        }
    }

    fn to_apilist(self) -> crate::apilist::API {
        self
    }
}