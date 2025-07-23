mod structs;
mod types;
mod validators;
use std::rc::Rc;

//use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::{KhronosContext, Limitations};
use crate::traits::discordprovider::DiscordProvider;
use crate::utils::{serenity_backports, serenity_utils};
use crate::{primitives::lazy::Lazy, TemplateContext};
use mluau::prelude::*;
use serenity::all::{Mentionable, ParseIdError};
use structs::{
    CreateAutoModerationRuleOptions, DeleteAutoModerationRuleOptions, EditAutoModerationRuleOptions,
};
use mlua_scheduler::LuaSchedulerAsyncUserData;


const MAX_NICKNAME_LENGTH: usize = 32;

#[derive(Clone)]
/// An action executor is used to execute actions such as kick/ban/timeout from Lua
/// templates
pub struct DiscordActionExecutor<T: KhronosContext> {
    context: T,
    limitations: Rc<Limitations>,
    discord_provider: T::DiscordProvider,
}

// @userdata DiscordActionExecutor
//
// Executes actions on discord
impl<T: KhronosContext> DiscordActionExecutor<T> {
    pub fn check_reason(&self, reason: &str) -> LuaResult<()> {
        if reason.len() > 512 {
            return Err(LuaError::external("Reason is too long"));
        } else if reason.is_empty() {
            return Err(LuaError::external("Reason is empty"));
        }

        Ok(())
    }

    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.limitations.has_cap(&format!("discord:{action}")) {
            return Err(LuaError::runtime(format!(
                "Discord action `{action}` not allowed in this template context",
            )));
        }

        self.discord_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }

    pub async fn check_permissions(
        &self,
        user_id: serenity::all::UserId,
        needed_permissions: serenity::all::Permissions,
    ) -> LuaResult<(
        serenity::all::PartialGuild,
        serenity::all::Member,
        serenity::all::Permissions,
    )> {
        // Get the guild
        let guild = self
            .discord_provider
            .get_guild()
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

        let Some(member) = self
            .discord_provider
            .get_guild_member(user_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?
        else {
            return Err(LuaError::runtime("Bot user not found in guild"));
        }; // Get the bot user

        let member_perms = serenity_backports::member_permissions(&guild, &member);

        if !member_perms.contains(needed_permissions) {
            return Err(LuaError::WithContext {
                context: needed_permissions.to_string(),
                cause: LuaError::runtime("Bot does not have the required permissions").into(),
            });
        }

        Ok((guild, member, member_perms))
    }

    pub async fn check_permissions_and_hierarchy(
        &self,
        user_id: serenity::all::UserId,
        target_id: serenity::all::UserId,
        needed_permissions: serenity::all::Permissions,
    ) -> LuaResult<(
        serenity::all::PartialGuild,
        serenity::all::Member,
        serenity::all::Permissions,
    )> {
        let guild = self
            .discord_provider
            .get_guild()
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

        let Some(member) = self
            .discord_provider
            .get_guild_member(user_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?
        else {
            return Err(LuaError::runtime(format!(
                "User not found in guild: {}",
                user_id.mention()
            )));
        }; // Get the bot user

        let member_perms = serenity_backports::member_permissions(&guild, &member);
        if !member_perms.contains(needed_permissions) {
            return Err(LuaError::runtime(format!(
                "User does not have the required permissions: {needed_permissions:?}: user_id={user_id}, target_id={target_id}",
            )));
        }

        let Some(target_member) = self
            .discord_provider
            .get_guild_member(target_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?
        else {
            return Err(LuaError::runtime("Target user not found in guild"));
        }; // Get the target user

        let higher_id = guild
            .greater_member_hierarchy(&member, &target_member)
            .ok_or_else(|| {
                LuaError::runtime(format!(
                    "User does not have a higher role than the target user: user_id={user_id}, target_id={target_id}",
                ))
            })?;

        if higher_id != member.user.id {
            return Err(LuaError::runtime(format!(
                "User does not have a higher role than the target user: user_id={user_id}, target_id={target_id}",
            )));
        }

        Ok((guild, target_member, member_perms))
    }

    /// Returns the channel permissions
    /// 
    /// The returned GuildChannel will either be the GuildChannel or the parent GuildChannel of a thread (if the channel id is one for a thread)
    pub async fn check_channel_permissions(
        &self,
        user_id: serenity::all::UserId,
        channel_id: serenity::all::GenericChannelId,
        needed_permissions: serenity::all::Permissions,
    ) -> LuaResult<(
        serenity::all::PartialGuild,
        serenity::all::Member,
        serenity::all::GuildChannel,
        serenity::all::Permissions,
    )> {
        let channel = self
            .discord_provider
            .get_channel(channel_id)
            .await
            .map_err(|e| LuaError::runtime(e.to_string()))?;

        let Some(member) = self
            .discord_provider
            .get_guild_member(user_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?
        else {
            return Err(LuaError::runtime("Bot user not found in guild"));
        };

        let guild = self
            .discord_provider
            .get_guild()
            .await
            .map_err(|e| LuaError::runtime(e.to_string()))?;

        match channel {
            serenity::all::Channel::Private(_) => {
                return Err(LuaError::runtime("Private channels are not supported by check_channel_permissions"));
            },
            serenity::all::Channel::Guild(guild_channel) => {
                // Should never happen, but just in case
                if guild_channel.base.guild_id != guild.id {
                    return Err(LuaError::runtime("Internal Error: Guild channel does not belong to the current guild"));
                }

                let perms = guild.user_permissions_in(&guild_channel, &member);

                if !perms.contains(needed_permissions) {
                    return Err(LuaError::runtime(format!(
                        "User does not have the required permissions: {needed_permissions:?}: {user_id}",
                    )));
                }

                return Ok((guild, member, guild_channel, perms))
            }
            serenity::all::Channel::GuildThread(gt) => return self.check_channel_permissions(gt.owner_id, gt.parent_id.widen(), needed_permissions).await,
            _ => {
                return Err(LuaError::runtime("Unsupported channel type in check_channel_permissions"));
            }
        }
    }
}

impl<T: KhronosContext> LuaUserData for DiscordActionExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "DiscordActionExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, _: ()| {
            Ok("DiscordActionExecutor")
        });

        // Basic helper functions
        methods.add_method("antiraid_check_reason", |_, this, reason: String| {
            Ok(this.check_reason(&reason))
        });

        methods.add_scheduler_async_method(
            "antiraid_check_permissions",
            async move |lua, this, data: LuaValue| {
                let data = lua.from_value::<structs::AntiRaidCheckPermissionsOptions>(data)?;
                let (partial_guild, member, permissions) = this
                    .check_permissions(data.user_id, data.needed_permissions)
                    .await
                    .map_err(LuaError::external)?;

                Ok(Lazy::new(structs::AntiRaidCheckPermissionsResponse {
                    partial_guild,
                    member,
                    permissions,
                }))
            },
        );

        methods.add_scheduler_async_method(
            "antiraid_check_permissions_and_hierarchy",
            async move |lua, this, data: LuaValue| {
                let data =
                    lua.from_value::<structs::AntiRaidCheckPermissionsAndHierarchyOptions>(data)?;
                
                //println!("user_id={user_id}", user_id=data.user_id);
                let (partial_guild, member, permissions) = this
                    .check_permissions_and_hierarchy(
                        data.user_id,
                        data.target_id,
                        data.needed_permissions,
                    )
                    .await
                    .map_err(LuaError::external)?;

                Ok(Lazy::new(structs::AntiRaidCheckPermissionsResponse {
                    partial_guild,
                    member,
                    permissions,
                }))
            },
        );

        methods.add_scheduler_async_method("antiraid_check_channel_permissions", async move |
            lua, 
            this,
            data: LuaValue,
        | {
            let data = lua.from_value::<structs::AntiRaidCheckChannelPermissionsOptions>(data)?;
            let (partial_guild, member, channel, permissions) = this.check_channel_permissions(data.user_id, data.channel_id, data.needed_permissions)
                .await
                .map_err(LuaError::external)?;

            Ok(Lazy::new(
                structs::AntiRaidCheckChannelPermissionsResponse {
                    partial_guild,
                    member,
                    channel,
                    permissions,
                }
            ))
        });

        methods.add_scheduler_async_method("antiraid_get_fused_member", async move |
            _lua, 
            this,
            ids: Vec<String>,
        | {
            let mut user_ids = Vec::with_capacity(ids.len());

            for user_id in ids {
                let user_id: serenity::all::UserId = user_id
                    .parse()
                    .map_err(|e: ParseIdError| LuaError::external(e.to_string()))?;
                user_ids.push(user_id);
            }

            // Fetch the partial guild *once*
            let partial_guild = this.discord_provider
                .get_guild()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let mut member_and_resolved_perms = Vec::with_capacity(user_ids.len());

            for id in user_ids {
                let member = this.discord_provider
                .get_guild_member(id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?
                .ok_or_else(|| LuaError::runtime("Member not found"))?;

                let resolved_perms = serenity_backports::member_permissions(&partial_guild, &member);

                member_and_resolved_perms.push(structs::AntiraidFusedMemberSingle {
                    member,
                    resolved_perms,
                });
            }
            
            Ok(Lazy::new(
                structs::AntiraidFusedMember {
                    guild: partial_guild,
                    members: member_and_resolved_perms,
                }
            ))
        });

        // Audit Log

        // Should be documented
        methods.add_scheduler_async_method("get_audit_logs", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetAuditLogOptions>(data)?;

            this.check_action("get_audit_logs".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::VIEW_AUDIT_LOG)
                .await
                .map_err(LuaError::external)?;

            let logs = this
                .discord_provider
                .get_audit_logs(
                    data.action_type,
                    data.user_id,
                    data.before,
                    data.limit,
                )
                .await
                .map_err(|x| LuaError::external(x.to_string()))?;

            Ok(Lazy::new(logs))
        });

        // Auto Moderation

        // Should be documented.
        methods.add_scheduler_async_method("list_auto_moderation_rules", async move |_, this, _: ()| {
            this.check_action("list_auto_moderation_rules".to_string())
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                .await
                .map_err(LuaError::external)?;

            let rules = this
                .discord_provider
                .list_auto_moderation_rules()
                .await
                .map_err(|x| LuaError::external(x.to_string()))?;

            Ok(Lazy::new(rules))
        });

        // Should be documented.
        methods.add_scheduler_async_method("get_auto_moderation_rule", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetAutoModerationRuleOptions>(data)?;

            this.check_action("get_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                .await
                .map_err(LuaError::external)?;

            let rule = this
                .discord_provider
                .get_auto_moderation_rule(data.rule_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(rule))
        });

        // Should be documented.
        methods.add_scheduler_async_method("create_auto_moderation_rule", async move |lua, this, data: LuaValue| {
            let data: CreateAutoModerationRuleOptions = lua.from_value(data)?;

            this.check_action("create_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                .await
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

            data.data.validate().map_err(LuaError::external)?;

            let rule = this
                .discord_provider
                .create_auto_moderation_rule(&data.data, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(rule))
        });

        // Should be documented.
        methods.add_scheduler_async_method("edit_auto_moderation_rule", async move |lua, this, data: LuaValue| {
            let data: EditAutoModerationRuleOptions = lua.from_value(data)?;

            this.check_action("edit_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                .await
                .map_err(LuaError::external)?;

            data.data.validate().map_err(LuaError::external)?;

            let rule = this
                .discord_provider
                .edit_auto_moderation_rule(data.rule_id, &data.data, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(rule))
        });

        // Should be documented.
        methods.add_scheduler_async_method("delete_auto_moderation_rule", async move |lua, this, data: LuaValue| {
            let data: DeleteAutoModerationRuleOptions = lua.from_value(data)?;

            this.check_action("delete_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                .await
                .map_err(LuaError::external)?;

            this
                .discord_provider
                .delete_auto_moderation_rule(data.rule_id, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Channel

        // Should be documented
        methods.add_scheduler_async_method("get_channel", async move |_lua, this, channel_id: String| {
            let channel_id: serenity::all::GenericChannelId = channel_id
                .parse()
                .map_err(|e: ParseIdError| LuaError::external(e.to_string()))?;

            this.check_action("get_channel".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let guild_channel = this.discord_provider.get_channel(channel_id).await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            Ok(Lazy::new(guild_channel))
        });

        // Should be documented
        methods.add_scheduler_async_method("edit_channel", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditChannelOptions>(data)?;

            this.check_action("edit_channel".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;    

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (_partial_guild, _bot_member, guild_channel, perms) = this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::empty())
            .await
            .map_err(LuaError::external)?;

            match guild_channel.base.kind {
                serenity::all::ChannelType::PublicThread | serenity::all::ChannelType::PrivateThread => {
                    // Check if the bot has permissions to manage threads
                    if !perms
                        .manage_threads()
                    {
                        return Err(LuaError::external(
                            "Bot does not have permission to manage this thread",
                        ));
                    }
                },
                _ => {
                    // Check if the bot has permissions to manage channels
                    if !perms
                        .manage_channels()
                    {
                        return Err(LuaError::external(
                            "Bot does not have permission to manage this channel",
                        ));
                    }
                }
            }

            if let Some(ref topic) = data.data.topic {
                if topic.len() > 1024 {
                    return Err(LuaError::external(
                        "Topic must be less than 1024 characters",
                    ));
                }
            }

            if let Some(ref rate_limit_per_user) = data.data.rate_limit_per_user {
                if rate_limit_per_user.get() > 21600 {
                    return Err(LuaError::external(
                        "Rate limit per user must be less than 21600 seconds",
                    ));
                }
            }

            if let Some(ref permission_overwrites) = data.data.permission_overwrites {
                // Check for ManageRoles permission
                if !perms
                    .manage_roles()
                {
                    return Err(LuaError::external(
                        "Bot does not have permission to manage roles",
                    ));
                }

                for overwrite in permission_overwrites.iter() {
                    if !perms.contains(overwrite.allow) {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to allow: {:?}", overwrite.allow),
                        ));
                    } else if !perms.contains(overwrite.deny) {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to deny: {:?}", overwrite.deny),
                        ));
                    }
                }
            }

            if let Some(ref available_tags) = data.data.available_tags {
                for tag in available_tags.iter() {
                    if tag.name.len() > 20 {
                        return Err(LuaError::external(
                            "Tag name must be less than 20 characters",
                        ));
                    }
                }
            }

            if let Some(ref default_thread_rate_limit_per_user) =
                data.data.default_thread_rate_limit_per_user
            {
               if default_thread_rate_limit_per_user.get() > 21600 {
                    return Err(LuaError::external(
                        "Default thread rate limit per user must be less than 21600 seconds",
                    ));
                }
            }

            let channel = this
                .discord_provider
                .edit_channel(data.channel_id, &data.data, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(channel))
        });

        // Should be documented
        methods.add_scheduler_async_method("delete_channel", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteChannelOptions>(data)?;

            this.check_action("delete_channel".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (_partial_guild, _bot_member, guild_channel, perms) = this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::empty())
            .await
            .map_err(LuaError::external)?;

            match guild_channel.base.kind {
                serenity::all::ChannelType::PublicThread | serenity::all::ChannelType::PrivateThread => {
                    // Check if the bot has permissions to manage threads
                    if !perms
                        .manage_threads()
                    {
                        return Err(LuaError::external(
                            "Bot does not have permission to manage this thread",
                        ));
                    }
                },
                _ => {
                    // Check if the bot has permissions to manage channels
                    if !perms
                        .manage_channels()
                    {
                        return Err(LuaError::external(
                            "Bot does not have permission to manage this channel",
                        ));
                    }
                }
            }

            let channel = this
                .discord_provider
                .delete_channel(data.channel_id, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(channel))
        });

        // Should be documented.
        methods.add_scheduler_async_method("edit_channel_permissions", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditChannelPermissionsOptions>(data)?;

            this.check_action("edit_channel_permissions".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (_partial_guild, _bot_member, _guild_channel, perms) = this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_ROLES)
            .await
            .map_err(LuaError::external)?;

            if let Some(allow_permissions) = data.allow.as_inner_ref() {
                for perm in allow_permissions.iter() {
                    if !perms.contains(perm) {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to allow: {perm:?}"),
                        ));
                    }
                }
            }

            if let Some(deny_permissions) = data.deny.as_inner_ref() {
                for perm in deny_permissions.iter() {
                    if !perms.contains(perm) {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to deny: {perm:?}"),
                        ));
                    }
                }
            }

            this
                .discord_provider
                .edit_channel_permissions(
                    data.channel_id,
                    data.target_id,
                    serde_json::json!({
                        "allow": data.allow,
                        "deny": data.deny,
                        "type": data.kind,
                    }),
                    Some(data.reason.as_str())
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("get_channel_invites", async move |_, this, channel_id: String| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
            .map_err(|e| LuaError::external(e.to_string()))?;

            this.check_action("get_channel_invites".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_CHANNELS)
            .await
            .map_err(LuaError::external)?;

            let invites = this
            .discord_provider
            .get_channel_invites(channel_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invites))
        });

        methods.add_scheduler_async_method("create_channel_invite", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateChannelInviteOptions>(data)?;

            this.check_action("create_channel_invite".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::CREATE_INSTANT_INVITE)
            .await
            .map_err(LuaError::external)?;

            let invite = this
            .discord_provider
            .create_channel_invite(data.channel_id, &data.data, Some(data.reason.as_str()))
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invite))
        });

        methods.add_scheduler_async_method("delete_channel_permission", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteChannelPermissionOptions>(data)?;

            this.check_action("delete_channel_permission".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_ROLES)
            .await
            .map_err(LuaError::external)?;

            this
            .discord_provider
            .delete_channel_permission(data.channel_id, data.overwrite_id, Some(data.reason.as_str()))
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("follow_announcement_channel", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::FollowAnnouncementChannel>(data)?;

            this.check_action("follow_announcement_channel".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.check_channel_permissions(bot_user.id, data.data.webhook_channel_id, serenity::all::Permissions::MANAGE_WEBHOOKS)
            .await
            .map_err(LuaError::external)?;

            let data = this
            .discord_provider
            .follow_announcement_channel(data.channel_id, data.data, Some(data.reason.as_str()))
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Guild

        // Should be documented
        methods.add_scheduler_async_method("get_guild", async move |_, this, _: ()| {
            this.check_action("get_guild".to_string())
            .map_err(LuaError::external)?;

            let guild = this.discord_provider
                .get_guild()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(guild))
        });

        methods.add_method("antiraid_guild_member_count", move |lua, this, _: ()| {
            this.check_action("get_cached_guild".to_string())
            .map_err(LuaError::external)?;

            let guild = this.discord_provider
                .get_cached_guild(
                    lua,
                    |_, g| g.member_count
                );

            Ok(Lazy::new(guild))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_preview", async move |_, this, _: ()| {
            this.check_action("get_guild_preview".to_string())
            .map_err(LuaError::external)?;

            let guild_preview = this.discord_provider
                .get_guild_preview()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(guild_preview))
        });

        methods.add_scheduler_async_method("modify_guild", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::ModifyGuildOptions>(data)?;

            this.check_action("modify_guild".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;    

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            if let Some(ref name) = data.data.name {
                if name.len() < 2 || name.len() > 100 {
                    return Err(LuaError::external(
                        "Name must be between 2 and 100 characters",
                    ));
                }
            }

            if let Some(ref description) = data.data.description {
                if description.len() > 300 {
                    return Err(LuaError::external(
                        "Description must be less than 300 characters",
                    ));
                }
            }

            if let Some(icon) = data.data.icon.as_inner_ref() {
                let format = get_format_from_image_data(icon)
                .map_err(LuaError::external)?;

                if format != "png" && format != "jpeg" && format != "gif" {
                    return Err(LuaError::external(
                        "Icon must be a PNG, JPEG, or GIF format",
                    ));
                }
            }

            let splash_format = {
                if let Some(splash) = data.data.splash.as_inner_ref() {
                    let format = get_format_from_image_data(splash)
                    .map_err(LuaError::external)?;

                    if format != "png" && format != "jpeg" {
                        return Err(LuaError::external(
                            "Splash must be a PNG or JPEG format",
                        ));
                    }

                    Some(format)
                } else {
                    None
                }
            };

            let discovery_splash_format = {
                if let Some(discovery_splash) = data.data.discovery_splash.as_inner_ref() {
                    let format = get_format_from_image_data(discovery_splash)
                    .map_err(LuaError::external)?;

                    if format != "png" && format != "jpeg" {
                        return Err(LuaError::external(
                            "Discovery splash must be a PNG or JPEG format",
                        ));
                    }

                    Some(format)
                } else {
                    None
                }
            };

            let banner_format = {
                if let Some(banner) = data.data.banner.as_inner_ref() {
                    let format = get_format_from_image_data(banner)
                    .map_err(LuaError::external)?;

                    if format != "png" && format != "jpeg" && format != "gif" {
                        return Err(LuaError::external(
                            "Banner must be a PNG, JPEG, or GIF format",
                        ));
                    }

                    Some(format)
                } else {
                    None
                }
            };

            // TODO: Check afk_channel_id, system_channel_id, rules_channel_id, public_updates_channel_id, safety_alerts_channel_id too

            let (guild, _member, perms) = this.check_permissions(
                bot_user.id,
                serenity::all::Permissions::MANAGE_GUILD,
            )
            .await
            .map_err(LuaError::external)?;

            let mut guild_has_community = false;
            let mut guild_has_invite_splash = false;
            let mut guild_has_discoverable = false;
            let mut guild_has_banner = false;
            let mut guild_has_animated_banner = false;

            for feature in guild.features.iter() {
                match feature.as_str() {
                    "COMMUNITY" => guild_has_community = true,
                    "INVITE_SPLASH" => guild_has_invite_splash = true,
                    "DISCOVERABLE" => guild_has_discoverable = true,
                    "BANNER" => guild_has_banner = true,
                    "ANIMATED_BANNER" => guild_has_animated_banner = true,
                    _ => {}
                }
            }

            if let Some(ref features) = data.data.features {
                if (
                    (features.contains(&"COMMUNITY".into()) && !guild_has_community) || 
                    (!features.contains(&"COMMUNITY".into()) && guild_has_community)
                ) && !perms.contains(serenity::all::Permissions::ADMINISTRATOR) {
                    return Err(LuaError::external("Enabling/disabling the community feature requires the bot to have the Administrator permission"));
                }
            }

            if !guild_has_invite_splash && splash_format.is_some() {
                return Err(LuaError::external("Guild does not have the Invite Splash feature and as such cannot have an invite splash"));
            }

            if !guild_has_discoverable && discovery_splash_format.is_some() {
                return Err(LuaError::external("Guild does not have the Discoverable feature and as such cannot have a discovery splash"));
            }

            if banner_format.is_some() {
                if !guild_has_banner {
                    return Err(LuaError::external("Guild does not have the Banner feature and as such cannot have a banner"));
                }

                if !guild_has_animated_banner && banner_format == Some("gif".to_string()) {
                    return Err(LuaError::external("Guild does not have the Animated Banner feature and as such cannot have an (animated) GIF banner"));
                }
            }

            let new_guild = this.discord_provider
                .modify_guild(
                    data.data,
                    Some(data.reason.as_str()),
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(new_guild))
        });

        // Modify guild is intentionally skipped for now. TODO: Add later
        // Delete guild will not be implemented as we can't really use it

        // Should be documented
        methods.add_scheduler_async_method("get_guild_channels", async move |_, this, _: ()| {
            this.check_action("get_guild_channels".to_string())
            .map_err(LuaError::external)?;

            let chans = this.discord_provider
                .get_guild_channels()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(chans))
        });

        // Should be documented
        methods.add_scheduler_async_method("create_guild_channel", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateChannelOptions>(data)?;

            this.check_action("create_guild_channel".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };
            
            let (_, _, bot_perms) = this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_CHANNELS)
            .await
            .map_err(LuaError::external)?;

            if let Some(ref topic) = data.data.topic {
                if topic.len() > 1024 {
                    return Err(LuaError::external(
                        "Topic must be less than 1024 characters",
                    ));
                }
            }

            if let Some(ref rate_limit_per_user) = data.data.rate_limit_per_user {
                if rate_limit_per_user.get() > 21600 {
                    return Err(LuaError::external(
                        "Rate limit per user must be less than 21600 seconds",
                    ));
                }
            }

            if let Some(ref permission_overwrites) = data.data.permission_overwrites {
                // Check for ManageRoles permission
                if !bot_perms
                    .manage_roles()
                {
                    return Err(LuaError::external(
                        "Bot does not have permission to manage roles",
                    ));
                }

                for overwrite in permission_overwrites.iter() {
                    if !bot_perms.contains(overwrite.allow) {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to allow: {:?}", overwrite.allow),
                        ));
                    }
                    
                    if !bot_perms.contains(overwrite.deny) {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to deny: {:?}", overwrite.deny),
                        ));
                    }
                }
            }

            if let Some(ref available_tags) = data.data.available_tags {
                for tag in available_tags.iter() {
                    if tag.name.len() > 20 {
                        return Err(LuaError::external(
                            "Tag name must be less than 20 characters",
                        ));
                    }
                }
            }

            if let Some(ref default_thread_rate_limit_per_user) =
                data.data.default_thread_rate_limit_per_user
            {
               if default_thread_rate_limit_per_user.get() > 21600 {
                    return Err(LuaError::external(
                        "Default thread rate limit per user must be less than 21600 seconds",
                    ));
                }
            }

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let channel = this
                .discord_provider
                .create_guild_channel(&data.data, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(channel))
        });

        // Should be documented
        methods.add_scheduler_async_method(
            "modify_guild_channel_positions",
            async move |lua, this, data: LuaValue| {
                let data = lua.from_value::<Vec<structs::ModifyChannelPosition>>(data)?;

                this.check_action("modify_guild_channel_positions".to_string())
                    .map_err(LuaError::external)?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_CHANNELS)
                    .await
                    .map_err(LuaError::external)?;

                this.discord_provider
                    .modify_guild_channel_positions(data.iter())
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            },
        );

        // Should be documented
        methods.add_scheduler_async_method("list_active_guild_threads", async move |_, this, _: ()| {
            this.check_action("list_active_guild_threads".to_string())
            .map_err(LuaError::external)?;

            let data = this.discord_provider
                .list_active_guild_threads()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_member", async move |_, this, user_id: String| {
            let user_id = user_id.parse()
            .map_err(LuaError::external)?;

            this.check_action("get_guild_member".to_string())
                .map_err(LuaError::external)?;

            let data = this.discord_provider
                .get_guild_member(user_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Should be documented
        methods.add_scheduler_async_method("list_guild_members", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetGuildMembersOptions>(data)?;

            this.check_action("list_guild_members".to_string())
                .map_err(LuaError::external)?;

            if let Some(limit) = data.limit {
                if limit.get() > 1000 {
                    return Err(LuaError::external("The maximum `limit` for get_guild_members is 1000"));
                }
            }

            let data = this.discord_provider
                .list_guild_members(data.limit, data.after)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Should be documented
        methods.add_scheduler_async_method("search_guild_members", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::SearchGuildMembersOptions>(data)?;

            this.check_action("search_guild_members".to_string())
                .map_err(LuaError::external)?;

            if let Some(limit) = data.limit {
                if limit.get() > 1000 {
                    return Err(LuaError::external("The maximum `limit` for get_guild_members is 1000"));
                }
            }

            let data = this.discord_provider
                .search_guild_members(&data.query, data.limit)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Should be documented
        methods.add_scheduler_async_method("modify_guild_member", async move |lua, this, data: LuaValue| {
            let mut data = lua.from_value::<structs::ModifyGuildMemberOptions>(data)?;

            this.check_action("modify_guild_member".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // TODO: Rethink this position on not allowing self-modification
            if bot_user.id == data.user_id {
                return Err(LuaError::external("Cannot modify self"));
            }

            let mut needed_perms = serenity::all::Permissions::empty();

            if let Some(ref nick) = data.data.nick {
                if nick.is_empty() {
                    return Err(LuaError::external("Nickname cannot be empty string if provided"));
                }

                if nick.len() > MAX_NICKNAME_LENGTH {
                    return Err(LuaError::external(
                        format!("Nickname must be less than {MAX_NICKNAME_LENGTH} characters"),
                    ));
                }

                needed_perms |= serenity::all::Permissions::MANAGE_NICKNAMES;
            }

            if let Some(ref roles) = data.data.roles {
                if roles.is_empty() {
                    return Err(LuaError::external("Roles cannot be empty if provided"));
                }

                needed_perms |= serenity::all::Permissions::MANAGE_ROLES;
            }

            if let Some(mute) = data.data.mute {
                if mute {
                    needed_perms |= serenity::all::Permissions::MUTE_MEMBERS;
                }
            }

            if let Some(deaf) = data.data.deaf {
                if deaf {
                    needed_perms |= serenity::all::Permissions::DEAFEN_MEMBERS;
                }
            }

            if data.data.channel_id.is_some() {
                needed_perms |= serenity::all::Permissions::MOVE_MEMBERS;
            } // TODO: Ensure the bot has connect perms in the specific channel

            if let Some(communication_disabled_until) = *data.data.communication_disabled_until {
                if let Some(crdu) = communication_disabled_until {
                    if *crdu > (chrono::Utc::now() + chrono::Duration::days(28) + chrono::Duration::seconds(10)) {
                        return Err(LuaError::external("Communication disabled until must be less than 28 days in the future"));
                    }    
                }

                needed_perms |= serenity::all::Permissions::MODERATE_MEMBERS;
            }

            let (guild, member, perms) = this.check_permissions(
                bot_user.id,
                needed_perms,
            )
            .await
            .map_err(LuaError::external)?;

            if let Some(ref mut flags) = data.data.flags {
                if !(perms.contains(serenity::all::Permissions::MANAGE_GUILD) || perms.contains(serenity::all::Permissions::MANAGE_ROLES) || perms.contains(serenity::all::Permissions::MODERATE_MEMBERS | serenity::all::Permissions::KICK_MEMBERS | serenity::all::Permissions::BAN_MEMBERS)) {
                    return Err(LuaError::external("Modifying member flags requires either MANAGE_GUILD, MANAGE_ROLES, or (MODERATE_MEMBERS and KICK_MEMBERS and BAN_MEMBERS)"));
                }

                let mut p_flags = serenity::all::GuildMemberFlags::empty();
                if flags.contains(serenity::all::GuildMemberFlags::BYPASSES_VERIFICATION) {
                    p_flags |= serenity::all::GuildMemberFlags::BYPASSES_VERIFICATION;
                }
                
                *flags = p_flags;
            }

            // Check roles
            let bot_highest_role = serenity_utils::highest_role(&guild, &member)
                .ok_or_else(|| LuaError::runtime("Bot does not have a role"))?;

            if let Some(ref roles) = data.data.roles {
                for role in roles.iter() {
                    let Some(role) = guild.roles.get(role) else {
                        return Err(LuaError::runtime("Role not found in guild"));
                    };

                    if role >= bot_highest_role {
                        return Err(LuaError::external(
                            format!("Bot does not have permission to add the requested role to the member specified ({}, ``{}``)", role.id, role.name.replace("`", "\\`")),
                        ));
                    }
                }
            }

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let member = this.discord_provider
                .modify_guild_member(
                    data.user_id,
                    data.data,
                    Some(data.reason.as_str()),
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(member))
        });

        // Should be documented
        methods.add_scheduler_async_method("add_guild_member_role", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::AddGuildMemberRoleOptions>(data)?;

            this.check_action("add_guild_member_role".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let Some(bot_member) = this.discord_provider.get_guild_member(bot_user.id).await
                .map_err(|e| LuaError::external(e.to_string()))?
            else {
                return Err(LuaError::runtime("Bot user not found in guild"));
            };

            let guild = this.discord_provider.get_guild().await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let resolved = serenity_backports::member_permissions(&guild, &bot_member);

            if !resolved
                .manage_roles()
            {
                return Err(LuaError::external(
                    "Bot does not have permission to manage roles",
                ));
            }

            let Some(bot_highest_role) = serenity_utils::highest_role(&guild, &bot_member) else {
                return Err(LuaError::runtime("Bot does not have a role"));
            };

            let Some(role_to_add) = guild.roles.get(&data.role_id) else {
                return Err(LuaError::runtime("Role to add to member not found in guild"));
            };

            if role_to_add >= bot_highest_role {
                return Err(LuaError::external(
                    format!("Bot does not have permission to add the requested role ({}, ``{}``) to the member", role_to_add.id, role_to_add.name.replace("`", "\\`")),
                ));
            }

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.discord_provider
                .add_guild_member_role(data.user_id, data.role_id, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Should be documented
        methods.add_scheduler_async_method("remove_guild_member_role", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::RemoveGuildMemberRoleOptions>(data)?;

            this.check_action("remove_guild_member_role".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let Some(bot_member) = this.discord_provider.get_guild_member(bot_user.id).await
                .map_err(|e| LuaError::external(e.to_string()))?
            else {
                return Err(LuaError::runtime("Bot user not found in guild"));
            };

            let guild = this.discord_provider.get_guild().await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let resolved = serenity_backports::member_permissions(&guild, &bot_member);

            if !resolved
                .manage_roles()
            {
                return Err(LuaError::external(
                    "Bot does not have permission to manage roles",
                ));
            }

            let Some(bot_highest_role) = serenity_utils::highest_role(&guild, &bot_member) else {
                return Err(LuaError::runtime("Bot does not have a role"));
            };

            let Some(role_to_remove) = guild.roles.get(&data.role_id) else {
                return Err(LuaError::runtime("Role to remove from member not found in guild"));
            };

            if role_to_remove >= bot_highest_role {
                return Err(LuaError::external(
                    format!("Bot does not have permission to remove the requested role ({}, ``{}``) from the member", role_to_remove.id, role_to_remove.name.replace("`", "\\`")),
                ));
            }

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.discord_provider
                .remove_guild_member_role(data.user_id, data.role_id, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Should be documented
        methods.add_scheduler_async_method("remove_guild_member", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::RemoveGuildMemberOptions>(data)?;

            this.check_action("remove_guild_member".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions_and_hierarchy(
                bot_user.id,
                data.user_id,
                serenity::all::Permissions::KICK_MEMBERS,
            )
            .await
            .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.discord_provider
                .remove_guild_member(data.user_id, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_bans", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetGuildBansOptions>(data)?;

            this.check_action("get_guild_bans".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(bot_user.id, serenity::all::Permissions::BAN_MEMBERS)
            .await
            .map_err(LuaError::external)?;

            let mut target = None;
            if let Some(before) = data.before {
                target = Some(serenity::all::UserPagination::Before(before));
            } else if let Some(after) = data.after {
                target = Some(serenity::all::UserPagination::After(after));
            }

            if let Some(limit) = data.limit {
                if limit.get() > 1000 {
                    return Err(LuaError::external(
                        "Limit must be less than 1000",
                    ));
                }
            }

            let bans = this.discord_provider
                .get_guild_bans(target, data.limit)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(bans))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_ban", async move |_, this, user_id: String| {
            let user_id = user_id.parse::<serenity::all::UserId>()
            .map_err(|e| LuaError::external(format!("Error while parsing user id: {e}")))?;

            this.check_action("get_guild_ban".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };    

            this.check_permissions(bot_user.id, serenity::all::Permissions::BAN_MEMBERS)
            .await
            .map_err(LuaError::external)?;

            let ban = this.discord_provider
                .get_guild_ban(user_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(ban))
        });

        // Should be documented
        methods.add_scheduler_async_method("create_guild_ban", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateGuildBanOptions>(data)?;

            this.check_action("create_guild_ban".to_string())
                .map_err(LuaError::external)?;

            let delete_message_seconds = {
                if let Some(seconds) = data.delete_message_seconds {
                    if seconds > 604800 {
                        return Err(LuaError::external(
                            "Delete message seconds must be between 0 and 604800",
                        ));
                    }

                    seconds
                } else {
                    0
                }
            };

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions_and_hierarchy(
                bot_user.id,
                data.user_id,
                serenity::all::Permissions::BAN_MEMBERS,
            )
            .await
            .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.discord_provider
                .create_guild_ban(
                    data.user_id,
                    delete_message_seconds,
                    Some(data.reason.as_str()),
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Should be documented
        methods.add_scheduler_async_method("remove_guild_ban", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::RemoveGuildBanOptions>(data)?;

            this.check_action("remove_guild_ban".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.check_permissions(
                bot_user.id,
                serenity::all::Permissions::BAN_MEMBERS,
            )
            .await
            .map_err(LuaError::external)?;

            this.discord_provider
                .remove_guild_ban(
                    data.user_id,
                    Some(data.reason.as_str()),
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_roles", async move |_, this, _g: ()| {
            this.check_action("get_guild_roles".to_string())
            .map_err(LuaError::external)?;

            let roles = this.discord_provider
                .get_guild_roles()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(roles))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_role", async move |_, this, role_id: String| {
            let role_id = role_id.parse::<serenity::all::RoleId>()
            .map_err(LuaError::external)?;

            this.check_action("get_guild_role".to_string())
                .map_err(LuaError::external)?;

            let role = this.discord_provider
                .get_guild_role(role_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(role))
        });

        // Should be documented
        methods.add_scheduler_async_method("create_guild_role", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateGuildRoleOptions>(data)?;

            this.check_action("create_guild_role".to_string())
                .map_err(LuaError::external)?;

            if let Some(ref name) = data.data.name {
                if name.len() > 100 || name.is_empty() {
                    return Err(LuaError::external(
                        "Role name must be a maximum of 100 characters and not empty",
                    ));
                }
            }

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (guild, _, bot_perms) = this.check_permissions(
                bot_user.id,
                serenity::all::Permissions::MANAGE_ROLES,
            )
            .await
            .map_err(LuaError::external)?; 

            let mut guild_has_role_icons = false;
            for feature in guild.features.iter() {
                if feature.as_str() == "ROLE_ICONS" { 
                    guild_has_role_icons = true 
                }
            }
            
            if let Some(permissions) = data.data.permissions {
                if !bot_perms.contains(permissions) {
                    return Err(LuaError::external(
                        format!("Bot does not have permissions: {:?}", permissions.difference(bot_perms)),
                    ));
                }
            }

            if let Some(icon) = data.data.icon.as_inner_ref() {
                if !guild_has_role_icons {
                    return Err(LuaError::external("Guild does not have the Role Icons feature and as such cannot create a role with a role_icon field"));
                }

                let format = get_format_from_image_data(icon)
                .map_err(LuaError::external)?;

                if format != "png" && format != "jpeg" && format != "gif" {
                    return Err(LuaError::external(
                        "Icon must be a PNG, JPEG, or GIF format",
                    ));
                }
            }

            let role = this.discord_provider
                .create_guild_role(data.data, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(role))
        });

        // Should be documented
        methods.add_scheduler_async_method("modify_guild_role_positions", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::ModifyRolePositionOptions>(data)?;

            this.check_action("modify_guild_role_positions".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };    

            let (guild, member, _) = this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_ROLES)
                .await
                .map_err(LuaError::external)?;

            // Check roles
            let bot_highest_role = serenity_utils::highest_role(&guild, &member)
                .ok_or_else(|| LuaError::runtime("Bot does not have a role"))?;

            for modify_role_position in data.data.iter() {
                let Some(role) = guild.roles.get(&modify_role_position.id) else {
                    return Err(LuaError::runtime("Role not found in guild"));
                };

                // Check current
                if role >= bot_highest_role || modify_role_position >= bot_highest_role {
                    return Err(LuaError::external(
                        format!("Bot does not have permission to modify the requested role ({}, ``{}``)", role.id, role.name.replace("`", "\\`")),
                    ));
                }
            }

            let roles = this.discord_provider
                .modify_guild_role_positions(data.data.iter(), Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(roles))
        });

        // Should be documented
        methods.add_scheduler_async_method("modify_guild_role", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditGuildRoleOptions>(data)?;

            this.check_action("modify_guild_role".to_string())
                .map_err(LuaError::external)?;

            if let Some(ref name) = data.data.name {
                if name.len() > 100 || name.is_empty() {
                    return Err(LuaError::external(
                        "Role name must be a maximum of 100 characters and not empty",
                    ));
                }
            }

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (guild, member, bot_perms) = this.check_permissions(
                bot_user.id,
                serenity::all::Permissions::MANAGE_ROLES,
            )
            .await
            .map_err(LuaError::external)?; 

            let mut bot_highest_role: Option<&serenity::all::Role> = None;
            let mut mod_role: Option<&serenity::all::Role> = None;

            for role in guild.roles.iter() {
                if role.id == data.role_id {
                    mod_role = Some(role);
                }

                if (bot_highest_role.is_none() || bot_highest_role.unwrap() < role) && member.roles.contains(&role.id) {
                    bot_highest_role = Some(role);
                }
            }

            let Some(mod_role) = mod_role else {
                return Err(LuaError::runtime("The role being modified could not be found on the server"));
            };

            let Some(bot_highest_role) = bot_highest_role else {
                return Err(LuaError::runtime("The bot must have roles in order to modify a guild role"));
            };  

            if bot_highest_role <= mod_role {
                return Err(LuaError::runtime("The bot must have a role that is higher than the role it is trying to modify"));
            }

            let mut guild_has_role_icons = false;
            for feature in guild.features.iter() {
                if feature.as_str() == "ROLE_ICONS" { 
                    guild_has_role_icons = true 
                }
            }
            
            if let Some(permissions) = data.data.permissions {
                if !bot_perms.contains(permissions) {
                    return Err(LuaError::external(
                        format!("Bot does not have permissions: {:?}", permissions.difference(bot_perms)),
                    ));
                }
            }

            if let Some(icon) = data.data.icon.as_inner_ref() {
                if !guild_has_role_icons {
                    return Err(LuaError::external("Guild does not have the Role Icons feature and as such cannot create a role with a role_icon field"));
                }

                let format = get_format_from_image_data(icon)
                .map_err(LuaError::external)?;

                if format != "png" && format != "jpeg" && format != "gif" {
                    return Err(LuaError::external(
                        "Icon must be a PNG, JPEG, or GIF format",
                    ));
                }
            }

            let role = this.discord_provider
                .modify_guild_role(data.role_id, data.data, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(role))
        });

        // Should be documented
        methods.add_scheduler_async_method("delete_guild_role", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteGuildRoleOptions>(data)?;

            if data.role_id.to_string() == this.discord_provider.guild_id().to_string() {
                return Err(LuaError::runtime("Cannot remove the default @everyone role"));
            }

            this.check_action("delete_guild_role".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (guild, member, _) = this.check_permissions(
                bot_user.id,
                serenity::all::Permissions::MANAGE_ROLES,
            )
            .await
            .map_err(LuaError::external)?; 

            let mut bot_highest_role: Option<&serenity::all::Role> = None;
            let mut mod_role: Option<&serenity::all::Role> = None;

            for role in guild.roles.iter() {
                if role.id == data.role_id {
                    mod_role = Some(role);
                }

                if (bot_highest_role.is_none() || bot_highest_role.unwrap() < role) && member.roles.contains(&role.id) {
                    bot_highest_role = Some(role);
                }
            }

            let Some(mod_role) = mod_role else {
                return Err(LuaError::runtime("The role being modified could not be found on the server"));
            };

            let Some(bot_highest_role) = bot_highest_role else {
                return Err(LuaError::runtime("The bot must have roles in order to modify a guild role"));
            };  

            if bot_highest_role <= mod_role {
                return Err(LuaError::runtime("The bot must have a role that is higher than the role it is trying to modify"));
            }

            this.discord_provider
                .delete_guild_role(data.role_id, Some(data.reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Invites
        methods.add_scheduler_async_method("get_invite", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetInviteOptions>(data)?;

            this.check_action("get_invite".to_string())
                .map_err(LuaError::external)?;

            let invite = this.discord_provider
                .get_invite(&data.code, data.with_counts.unwrap_or(false), data.with_expiration.unwrap_or(false), data.guild_scheduled_event_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invite))
        });

        methods.add_scheduler_async_method("delete_invite", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteInviteOptions>(data)?;

            this.check_action("delete_invite".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };    

            // Call get_invite to find the channel id
            let invite = this.discord_provider
                .get_invite(&data.code, false, false, None)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(guild) = invite.guild {
                if guild.id != this.discord_provider.guild_id() {
                    return Err(LuaError::external("Invite does not belong to the current guild"));
                }
            }

            let (_partial_guild, _bot_member, _channel, perms) = this.check_channel_permissions(bot_user.id, invite.channel.id.widen(), serenity::all::Permissions::empty())
                .await
                .map_err(LuaError::external)?;    

            let has_perms = perms.manage_guild() || perms.manage_channels();

            if !has_perms {
                return Err(LuaError::external(
                    "Bot does not have permission to manage channels (either Manage Server globally or Manage Channels on the channel level)",
                ));
            }

            let invite = this.discord_provider
            .delete_invite(&data.code, Some(data.reason.as_str()))
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invite))
        });

        // Messages

        // Should be documented
        methods.add_scheduler_async_method("get_channel_messages", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetMessagesOptions>(data)?;

            this.check_action("get_channel_messages".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // Perform required checks
            let (_, _, guild_channel, perms) = this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::VIEW_CHANNEL).await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            if guild_channel.base.kind == serenity::all::ChannelType::Voice 
            && !perms
            .connect() {
                return Err(LuaError::external(
                    "Bot does not have permission to connect to the given voice channel",
                ));
            }

            let msg = this.discord_provider
                .get_channel_messages(data.channel_id, data.target.map(|x| x.to_serenity()), data.limit)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_channel_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetMessageOptions>(data)?;

            this.check_action("get_channel_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // Perform required checks
            let (_, _, guild_channel, perms) = this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::VIEW_CHANNEL).await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            if guild_channel.base.kind == serenity::all::ChannelType::Voice 
            && !perms
            .connect() {
                return Err(LuaError::external(
                    "Bot does not have permission to connect to the given voice channel",
                ));
            }

            let msg = this.discord_provider
                .get_channel_message(data.channel_id, data.message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        // Should be documented
        methods.add_scheduler_async_method("create_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateMessageOptions>(data)?;

            validators::validate_message(&data.data)
                .map_err(|x| LuaError::external(x.to_string()))?;

            this.check_action("create_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::SEND_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            let files = if let Some(ref attachments) = data.data.attachments {
                attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
            } else {
                Vec::new()
            };

            let msg = this.discord_provider
                .create_message(data.channel_id, files, &data.data)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("crosspost_message", async move |_lua, this, (channel_id, message_id): (String, String)| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let message_id = message_id.parse::<serenity::all::MessageId>()
                .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?;

            this.check_action("crosspost_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just send if the message author is the same as the bot user, this takes an extra API call to check. Not worth it
            this.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::SEND_MESSAGES | serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            let msg = this.discord_provider
                .crosspost_message(channel_id, message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("create_reaction", async move |lua, this, data: LuaValue| {
            let data: structs::CreateReactionOptions = lua.from_value(data)?;

            this.check_action("create_reaction".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::READ_MESSAGE_HISTORY | serenity::all::Permissions::ADD_REACTIONS)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .create_reaction(data.channel_id, data.message_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete_own_reaction", async move |lua, this, data: LuaValue| {
            let data: structs::DeleteOwnReactionOptions = lua.from_value(data)?;

            this.check_action("delete_own_reaction".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::READ_MESSAGE_HISTORY | serenity::all::Permissions::ADD_REACTIONS)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_own_reaction(data.channel_id, data.message_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete_user_reaction", async move |lua, this, data: LuaValue| {
            let data: structs::DeleteUserReactionOptions = lua.from_value(data)?;

            this.check_action("delete_user_reaction".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_user_reaction(data.channel_id, data.message_id, data.user_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("get_reactions", async move |lua, this, data: LuaValue| {
            let data: structs::GetReactionsOptions = lua.from_value(data)?;

            this.check_action("get_reactions".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::empty())
                .await
                .map_err(LuaError::external)?;

            let users = this.discord_provider
                .get_reactions(
                    data.channel_id, 
                    data.message_id,
                    &data.reaction.into_serenity(),
                    data.r#type.map(|x| {
                        matches!(x, structs::ReactionTypeEnum::Burst)
                    }),
                    data.after,
                    data.limit,
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(users))
        });

        methods.add_scheduler_async_method("delete_all_reactions", async move |_lua, this, (channel_id, message_id): (String, String)| {
            let channel_id: serenity::all::GenericChannelId = channel_id.parse()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let message_id: serenity::all::MessageId = message_id.parse()
                .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?;

            this.check_action("delete_all_reactions".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_all_reactions(channel_id, message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete_all_reactions_for_emoji", async move |lua, this, data: LuaValue| {
            let data: structs::DeleteAllReactionsForEmojiOptions = lua.from_value(data)?;

            this.check_action("delete_all_reactions_for_emoji".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_all_reactions_for_emoji(data.channel_id, data.message_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("edit_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditMessageOptions>(data)?;

            validators::validate_message_edit(&data.data)
                .map_err(|x| LuaError::external(x.to_string()))?;

            this.check_action("edit_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            let files = if let Some(ref attachments) = data.data.attachments {
                attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
            } else {
                Vec::new()
            };

            let msg = this.discord_provider
                .edit_message(data.channel_id, data.message_id, files, &data.data)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("delete_message", async move |_lua, this, (channel_id, message_id, reason): (String, String, String)| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let message_id = message_id.parse::<serenity::all::MessageId>()
                .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?;

            this.check_action("delete_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_message(channel_id, message_id, Some(reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("bulk_delete_messages", async move |_lua, this, (channel_id, messages, reason): (String, Vec<String>, String)| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let mut message_ids = Vec::with_capacity(messages.len());
            for message_id in messages {
                message_ids.push(
                    message_id.parse::<serenity::all::MessageId>()
                        .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?
                );
            }

            this.check_action("bulk_delete_messages".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.context.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(LuaError::external)?;

            this.discord_provider
                .bulk_delete_messages(channel_id, serde_json::json!({"messages": message_ids}), Some(reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Interactions
        methods.add_scheduler_async_method("create_interaction_response", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateInteractionResponseOptions>(data)?;

            this.check_action("create_interaction_response".to_string())
                .map_err(LuaError::external)?;

            let files = data.data.take_files().map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .create_interaction_response(data.interaction_id, &data.interaction_token, &data.data, files)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method(
            "get_original_interaction_response",
            async move |_, this, interaction_token: String| {
                this.check_action("get_original_interaction_response".to_string())
                .map_err(LuaError::external)?;

                let resp = this.discord_provider
                    .get_original_interaction_response(&interaction_token)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(resp))
            },
        );

        methods.add_scheduler_async_method("edit_original_interaction_response", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditInteractionResponseOptions>(data)?;

            this.check_action("edit_original_interaction_response".to_string())
                .map_err(LuaError::external)?;

            let files = if let Some(ref attachments) = data.data.attachments {
                attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
            } else {
                Vec::new()
            };


            let msg = this.discord_provider
                .edit_original_interaction_response(&data.interaction_token, &data.data, files)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method(
            "delete_original_interaction_response",
            async move |_, this, interaction_token: String| {
                this.check_action("delete_original_interaction_response".to_string())
                .map_err(LuaError::external)?;

                this.discord_provider
                    .delete_original_interaction_response(&interaction_token)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method("get_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetFollowupMessageOptions>(data)?;

            this.check_action("get_followup_message".to_string())
                .map_err(LuaError::external)?;

            let msg = this.discord_provider
                .get_followup_message(&data.interaction_token, data.message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("create_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateFollowupMessageOptions>(data)?;

            this.check_action("create_followup_message".to_string())
                .map_err(LuaError::external)?;

            let files = if let Some(ref attachments) = data.data.attachments {
                attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
            } else {
                Vec::new()
            };


            let msg = this.discord_provider
                .create_followup_message(&data.interaction_token, &data.data, files)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("edit_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditFollowupMessageOptions>(data)?;

            this.check_action("edit_followup_message".to_string())
                .map_err(LuaError::external)?;

            let files = if let Some(ref attachments) = data.data.attachments {
                attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
            } else {
                Vec::new()
            };


            let msg = this.discord_provider
                .edit_followup_message(&data.interaction_token, data.message_id, &data.data, files)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("delete_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteFollowupMessageOptions>(data)?;

            this.check_action("delete_followup_message".to_string())
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_followup_message(&data.interaction_token, data.message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Uncategorized

        // Should be documented
        methods.add_scheduler_async_method("get_guild_command", async move |_, this, cmd_id: String| {
            let command_id: serenity::all::CommandId = cmd_id.parse().map_err(|e| {
                LuaError::external(format!("Invalid command id: {e}"))
            })?;
            this.check_action("get_guild_command".to_string())
                .map_err(LuaError::external)?;

            let resp = this.discord_provider
                .get_guild_command(command_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });

        methods.add_scheduler_async_method("get_guild_commands", async move |_, this, _g: ()| {
            this.check_action("get_guild_commands".to_string())
            .map_err(LuaError::external)?;

            let resp = this.discord_provider
                .get_guild_commands()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });

        methods.add_scheduler_async_method("create_guild_command", async move |lua, this, data: LuaValue| {
            this.check_action("create_guild_command".to_string())
            .map_err(LuaError::external)?;

            let data = lua.from_value::<structs::CreateCommandOptions>(data)?;

            validators::validate_command(&data.data)
                .map_err(|x| LuaError::external(x.to_string()))?;

            let resp = this.discord_provider
                .create_guild_command(&data.data)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });

        methods.add_scheduler_async_method("create_guild_commands", async move |lua, this, data: LuaValue| {
            this.check_action("create_guild_commands".to_string())
            .map_err(LuaError::external)?;

            let data = lua.from_value::<structs::CreateCommandsOptions>(data)?;

            for cmd in &data.data {
                validators::validate_command(cmd)
                    .map_err(|x| LuaError::external(x.to_string()))?;
            }

            let resp = this.discord_provider
                .create_guild_commands(&data.data)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua, token: &TemplateContext<T>) -> LuaResult<LuaValue> {
    let Some(discord_provider) = token.context.discord_provider() else {
        return Err(LuaError::external(
            "The discord plugin is not supported in this context",
        ));
    };

    let executor = DiscordActionExecutor {
        context: token.context.clone(),
        limitations: token.limitations.clone(),
        discord_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}

fn get_format_from_image_data(data: &str) -> Result<String, LuaError> {
    if !data.starts_with("data:image/") {
        return Err(LuaError::external("Image must be a data URL"));
    }

    let Some(format) = data.split(";").next() else {
        return Err(LuaError::external("Image is not a valid data URL"));
    };

    let Some(format) = format.split("/").nth(1) else {
        return Err(LuaError::external("No format found in data URL"));
    };

    Ok(format.to_string())
}
