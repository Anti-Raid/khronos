use crate::{ApiReq, antiraid_check_channel_permissions::AntiRaidCheckChannelPermissions, antiraid_check_permissions::{AntiRaidCheckPermissions, AntiRaidCheckPermissionsAndHierarchy}, antiraid_get_fused_member::AntiRaidGetFusedMember, api::{auditlogs::GetAuditLog, automoderation::{get_auto_moderation_rule::GetAutoModerationRule, list_auto_moderation_rules::ListAutoModerationRules}, channels::edit_channel::EditChannel, guilds::modify_guild::ModifyGuild}, context::DiscordContext, controller::DiscordProvider};

// ($name:ident { $($variant:ident($ty:ty) = $api_name:literal),* $(,)? }) => {

#[cfg(feature = "luau")]
pub trait APIUserData: mluau::UserData + 'static {
    type DiscordProvider: DiscordProvider;
    fn check_action(&self, lua: &mluau::Lua, action: &str) -> mluau::Result<()>;
    fn controller(&self) -> &DiscordContext<Self::DiscordProvider>;
    fn map_response<T: serde::Serialize + 'static>(&self, lua: &mluau::Lua, resp: T) -> mluau::Result<mluau::Value> {
        use mluau::LuaSerdeExt;
        let value = lua.to_value(&resp)?;
        Ok(value)
    }
}

macro_rules! api_list_enum {
    ($name:ident { $($variant:ident($ty:ty) = $api_name:literal,)* }) => {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $(
                $variant($ty),
            )*
        }

        impl $name {
            pub async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<serde_json::Value, crate::Error> {
                match self {
                    $(
                        $name::$variant(req) => {
                            let resp = req.execute(this).await?;
                            Ok(serde_json::to_value(resp)?)
                        }
                    )*
                }
            }

            pub fn api_name(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant(_) => $api_name,
                    )*
                }
            }
        }

        impl $name {
            pub fn from_name_and_data(name: &str, data: serde_json::Value) -> Result<Self, crate::Error> {
                match name {
                    $(
                        $api_name => {
                            let req: $ty = serde_json::from_value(data)?;
                            Ok($name::$variant(req))
                        }
                    )*
                    _ => Err(format!("internal error: unknown API name: {}", name).into()),
                }
            }

            #[cfg(feature = "luau")]
            pub fn add_luau_methods<T: APIUserData, M: mluau::UserDataMethods<T>>(methods: &mut M) {
                use mlua_scheduler::LuaSchedulerAsyncUserData;
                use mluau::LuaSerdeExt;
                use crate::exec_api;
                $(
                    methods.add_scheduler_async_method($api_name, async |lua, this, data: mluau::Value| {
                        this.check_action(&lua, $api_name)?;

                        let data: $ty = lua.from_value(data)?;

                        let resp = exec_api(
                            &this.controller(), 
                            data,
                        )
                        .await
                        .map_err(|e| mluau::Error::external(e.to_string()))?;
                        
                        this.map_response(&lua, resp)
                    });
                )*
            }
        }
    };
}

api_list_enum!(API {
    // Antiraid specific
    AntiRaidCheckChannelPermissions(AntiRaidCheckChannelPermissions) = "antiraid_check_channel_permissions",
    AntiRaidCheckPermissions(AntiRaidCheckPermissions) = "antiraid_check_permissions",
    AntiRaidCheckPermissionsAndHierarchy(AntiRaidCheckPermissionsAndHierarchy) = "antiraid_check_permissions_and_hierarchy",
    AntiRaidGetFusedMember(AntiRaidGetFusedMember) = "antiraid_get_fused_member",

    // Audit logs
    GetAuditLog(GetAuditLog) = "get_audit_log",

    // Auto Moderation
    GetAutoModerationRule(GetAutoModerationRule) = "get_auto_moderation_rule",
    ListAutoModerationRules(ListAutoModerationRules) = "list_auto_moderation_rules",
    CreateAutoModerationRule(crate::api::automoderation::create_auto_moderation_rule::CreateAutoModerationRule) = "create_auto_moderation_rule",
    EditAutoModerationRule(crate::api::automoderation::edit_auto_moderation_rule::EditAutoModerationRule) = "edit_auto_moderation_rule",
    DeleteAutoModerationRule(crate::api::automoderation::delete_auto_moderation_rule::DeleteAutoModerationRule) = "delete_auto_moderation_rule",

    // Channels
    EditChannel(EditChannel) = "edit_channel",
    GetChannel(crate::api::channels::get_channel::GetChannel) = "get_channel",
    DeleteChannel(crate::api::channels::delete_channel::DeleteChannel) = "delete_channel",
    EditChannelPermissions(crate::api::channels::edit_channel_permissions::EditChannelPermissions) = "edit_channel_permissions",
    // GetChannelInvites(crate::api::channels::get_channel_invites::GetChannelInvites),
    CreateChannelInvite(crate::api::channels::create_channel_invite::CreateChannelInvite) = "create_channel_invite",
    DeleteChannelPermission(crate::api::channels::delete_channel_permission::DeleteChannelPermission) = "delete_channel_permission",
    // FollowAnnouncementChannel(crate::api::channels::follow_announcement_channel::FollowAnnouncementChannel),

    // Guilds
    ModifyGuild(ModifyGuild) = "modify_guild",
    GetGuild(crate::api::guilds::get_guild::GetGuild) = "get_guild",
    GetGuildPreview(crate::api::guilds::get_guild_preview::GetGuildPreview) = "get_guild_preview",
    GetGuildChannels(crate::api::guilds::get_guild_channels::GetGuildChannels) = "get_guild_channels",
    CreateGuildChannel(crate::api::guilds::create_guild_channel::CreateGuildChannel) = "create_guild_channel",
    ModifyGuildChannelPositions(crate::api::guilds::modify_guild_channel_positions::ModifyGuildChannelPositions) = "modify_guild_channel_positions",
    ListActiveGuildThreads(crate::api::guilds::list_active_guild_threads::ListActiveGuildThreads) = "list_active_guild_threads",
    GetGuildMember(crate::api::guilds::get_guild_member::GetGuildMember) = "get_guild_member",
    ListGuildMembers(crate::api::guilds::list_guild_members::ListGuildMembers) = "list_guild_members",
    SearchGuildMembers(crate::api::guilds::search_guild_members::SearchGuildMembers) = "search_guild_members",
    ModifyGuildMember(crate::api::guilds::modify_guild_member::ModifyGuildMember) = "modify_guild_member",
    AddGuildMemberRole(crate::api::guilds::add_guild_member_role::AddGuildMemberRole) = "add_guild_member_role",
    RemoveGuildMemberRole(crate::api::guilds::remove_guild_member_role::RemoveGuildMemberRole) = "remove_guild_member_role",
    RemoveGuildMember(crate::api::guilds::remove_guild_member::RemoveGuildMember) = "remove_guild_member",
    GetGuildBans(crate::api::guilds::get_guild_bans::GetGuildBans) = "get_guild_bans",
    GetGuildBan(crate::api::guilds::get_guild_ban::GetGuildBan) = "get_guild_ban",
    CreateGuildBan(crate::api::guilds::create_guild_ban::CreateGuildBan) = "create_guild_ban",
    RemoveGuildBan(crate::api::guilds::remove_guild_ban::RemoveGuildBan) = "remove_guild_ban",
    GetGuildRoles(crate::api::guilds::get_guild_roles::GetGuildRoles) = "get_guild_roles",
    GetGuildRole(crate::api::guilds::get_guild_role::GetGuildRole) = "get_guild_role",
    CreateGuildRole(crate::api::guilds::create_guild_role::CreateGuildRole) = "create_guild_role",
    ModifyGuildRolePositions(crate::api::guilds::modify_guild_role_positions::ModifyGuildRolePositions) = "modify_guild_role_positions",
    ModifyGuildRole(crate::api::guilds::modify_guild_role::ModifyGuildRole) = "modify_guild_role",
    DeleteGuildRole(crate::api::guilds::delete_guild_role::DeleteGuildRole) = "delete_guild_role",

    // Invites
    GetInvite(crate::api::invites::get_invite::GetInvite) = "get_invite",
    DeleteInvite(crate::api::invites::delete_invite::DeleteInvite) = "delete_invite",

    // Messages
    GetChannelMessages(crate::api::messages::get_channel_messages::GetChannelMessages) = "get_channel_messages",
    GetChannelMessage(crate::api::messages::get_channel_message::GetChannelMessage) = "get_channel_message",
    CreateMessage(crate::api::messages::create_message::CreateMessageRequest) = "create_message",
    CrosspostMessage(crate::api::messages::crosspost_message::CrosspostMessage) = "crosspost_message",
    EditMessage(crate::api::messages::edit_message::EditMessageRequest) = "edit_message",
    DeleteMessage(crate::api::messages::delete_message::DeleteMessage) = "delete_message",
    BulkDeleteMessages(crate::api::messages::bulk_delete_messages::BulkDeleteMessages) = "bulk_delete_messages",

    // Reactions
    CreateReaction(crate::api::messages::create_reaction::CreateReaction) = "create_reaction",
    DeleteOwnReaction(crate::api::messages::delete_own_reaction::DeleteOwnReaction) = "delete_own_reaction",
    DeleteUserReaction(crate::api::messages::delete_user_reaction::DeleteUserReaction) = "delete_user_reaction",
    GetReactions(crate::api::messages::get_reactions::GetReactions) = "get_reactions",
    DeleteAllReactions(crate::api::messages::delete_all_reactions::DeleteAllReactions) = "delete_all_reactions",
    DeleteAllReactionsForEmoji(crate::api::messages::delete_all_reactions_for_emoji::DeleteAllReactionsForEmoji) = "delete_all_reactions_for_emoji",

    // Interactions
    CreateInteractionResponse(crate::api::interactions::create_interaction_response::CreateInteractionResponseRequest) = "create_interaction_response",
    GetOriginalInteractionResponse(crate::api::interactions::get_original_interaction_response::GetOriginalInteractionResponse) = "get_original_interaction_response",
    EditOriginalInteractionResponse(crate::api::interactions::edit_original_interaction_response::EditOriginalInteractionResponse) = "edit_original_interaction_response",
    DeleteOriginalInteractionResponse(crate::api::interactions::delete_original_interaction_response::DeleteOriginalInteractionResponse) = "delete_original_interaction_response",
    CreateFollowupMessage(crate::api::interactions::create_followup_message::CreateFollowupMessage) = "create_followup_message",
    GetFollowupMessage(crate::api::interactions::get_followup_message::GetFollowupMessage) = "get_followup_message",
    EditFollowupMessage(crate::api::interactions::edit_followup_message::EditFollowupMessage) = "edit_followup_message",
    DeleteFollowupMessage(crate::api::interactions::delete_followup_message::DeleteFollowupMessage) = "delete_followup_message",

    // Commands
    GetGuildCommand(crate::api::commands::get_guild_command::GetGuildCommand) = "get_guild_command",
    GetGuildCommands(crate::api::commands::get_guild_commands::GetGuildCommands) = "get_guild_commands",
    CreateGuildCommand(crate::api::commands::create_guild_command::CreateGuildCommand) = "create_guild_command",
    CreateGuildCommands(crate::api::commands::create_guild_commands::CreateGuildCommands) = "create_guild_commands",

    // Webhooks
    CreateWebhook(crate::api::webhooks::create_webhook::CreateWebhookRequest) = "create_webhook",
    GetChannelWebhooks(crate::api::webhooks::get_channel_webhooks::GetChannelWebhooks) = "get_channel_webhooks",
    GetGuildWebhooks(crate::api::webhooks::get_guild_webhooks::GetGuildWebhooks) = "get_guild_webhooks",
    GetWebhook(crate::api::webhooks::get_webhook::GetWebhook) = "get_webhook",
    ModifyWebhook(crate::api::webhooks::modify_webhook::ModifyWebhook) = "modify_webhook",
    DeleteWebhook(crate::api::webhooks::delete_webhook::DeleteWebhook) = "delete_webhook",
    ExecuteWebhook(crate::api::webhooks::execute_webhook::ExecuteWebhookRequest) = "execute_webhook",
});
