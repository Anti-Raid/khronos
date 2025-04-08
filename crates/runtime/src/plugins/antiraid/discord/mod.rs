mod structs;
mod types;
mod validators;

use crate::lua_promise;
use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::traits::discordprovider::DiscordProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::utils::{serenity_backports, serenity_utils};
use crate::{plugins::antiraid::lazy::Lazy, TemplateContextRef};
use mlua::prelude::*;
use serenity::all::Mentionable;
use structs::{
    CreateAutoModerationRuleOptions, DeleteAutoModerationRuleOptions, EditAutoModerationRuleOptions,
};

const MAX_NICKNAME_LENGTH: usize = 32;

#[derive(Clone)]
/// An action executor is used to execute actions such as kick/ban/timeout from Lua
/// templates
pub struct DiscordActionExecutor<T: KhronosContext> {
    context: T,
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
        if !self.context.has_cap(&format!("discord:{}", action)) {
            return Err(LuaError::runtime(format!(
                "Discord action `{}` not allowed in this template context",
                action
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
    ) -> LuaResult<(serenity::all::PartialGuild, serenity::all::Member, serenity::all::Permissions)> {
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
    ) -> LuaResult<(serenity::all::PartialGuild, serenity::all::Member, serenity::all::Permissions)> {
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
                "User does not have the required permissions: {:?}: {}",
                needed_permissions, user_id
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
                    "User does not have a higher role than the target user: {}",
                    user_id.mention()
                ))
            })?;

        if higher_id != member.user.id {
            return Err(LuaError::runtime(format!(
                "User does not have a higher role than the target user: {}",
                user_id.mention()
            )));
        }

        Ok((guild, target_member, member_perms))
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

        // Audit Log

        // Should be documented
        methods.add_method("get_audit_logs", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Auto Moderation

        // Should be documented.
        methods.add_method("list_auto_moderation_rules", |_, this, _: ()| {
            Ok(lua_promise!(this, |_lua, this|, {
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
            }))
        });

        // Should be documented.
        methods.add_method("get_auto_moderation_rule", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented.
        methods.add_method("create_auto_moderation_rule", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented.
        methods.add_method("edit_auto_moderation_rule", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data: EditAutoModerationRuleOptions = lua.from_value(data)?;

                this.check_action("edit_auto_moderation_rule".to_string())
                    .map_err(LuaError::external)?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                    .await
                    .map_err(LuaError::external)?;

                data.data.validate().map_err(LuaError::external)?;

                this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

                let rule = this
                    .discord_provider
                    .edit_auto_moderation_rule(data.rule_id, &data.data, Some(data.reason.as_str()))
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(rule))
            }))
        });

        // Should be documented.
        methods.add_method("delete_auto_moderation_rule", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data: DeleteAutoModerationRuleOptions = lua.from_value(data)?;

                this.check_action("delete_auto_moderation_rule".to_string())
                    .map_err(LuaError::external)?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                this.check_permissions(bot_user.id, serenity::all::Permissions::MANAGE_GUILD)
                    .await
                    .map_err(LuaError::external)?;

                    this.check_reason(&data.reason)
                    .map_err(LuaError::external)?;

                this
                    .discord_provider
                    .delete_auto_moderation_rule(data.rule_id, Some(data.reason.as_str()))
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        // Channel

        // Should be documented
        methods.add_method("get_channel", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::GetChannelOptions>(data)?;

                this.check_action("get_channel".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                Ok(Lazy::new(guild_channel))
            }))
        });

        // Should be documented
        methods.add_method("edit_channel", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::EditChannelOptions>(data)?;

                this.check_action("edit_channel".to_string())
                    .map_err(LuaError::external)?;

                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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

                let perms = guild
                .user_permissions_in(&guild_channel, &bot_member);

                match guild_channel.kind {
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
                    if !guild
                        .user_permissions_in(&guild_channel, &bot_member)
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

                this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

                let channel = this
                    .discord_provider
                    .edit_channel(data.channel_id, &data.data, Some(data.reason.as_str()))
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(channel))
            }))
        });

        // Should be documented
        methods.add_method("delete_channel", |_, this, channel_id: LuaValue| {
            Ok(lua_promise!(this, channel_id, |lua, this, channel_id|, {
                let data = lua.from_value::<structs::DeleteChannelOptions>(channel_id)?;

                this.check_action("delete_channel".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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

                match guild_channel.kind {
                    serenity::all::ChannelType::PublicThread | serenity::all::ChannelType::PrivateThread => {
                        // Check if the bot has permissions to manage threads
                        if !guild
                            .user_permissions_in(&guild_channel, &bot_member)
                            .manage_threads()
                        {
                            return Err(LuaError::external(
                                "Bot does not have permission to manage this thread",
                            ));
                        }
                    },
                    _ => {
                        // Check if the bot has permissions to manage channels
                        if !guild
                            .user_permissions_in(&guild_channel, &bot_member)
                            .manage_channels()
                        {
                            return Err(LuaError::external(
                                "Bot does not have permission to manage this channel",
                            ));
                        }
                    }
                }

                this.check_reason(&data.reason)
                    .map_err(LuaError::external)?;

                let channel = this
                    .discord_provider
                    .delete_channel(data.channel_id, Some(data.reason.as_str()))
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(channel))
            }))
        });

        // Should be documented.
        methods.add_method("edit_channel_permissions", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::EditChannelPermissionsOptions>(data)?;

                this.check_action("edit_channel_permissions".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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

                let resolved = guild
                .user_permissions_in(&guild_channel, &bot_member);

                if !resolved
                    .manage_roles()
                {
                    return Err(LuaError::external(
                        "Bot does not have permission to manage roles",
                    ));
                }

                if let Some(allow_permissions) = data.allow.as_inner_ref() {
                    for perm in allow_permissions.iter() {
                        if !resolved.contains(perm) {
                            return Err(LuaError::external(
                                format!("Bot does not have permission to deny: {:?}", perm),
                            ));
                        }
                    }
                }

                if let Some(deny_permissions) = data.deny.as_inner_ref() {
                    for perm in deny_permissions.iter() {
                        if !resolved.contains(perm) {
                            return Err(LuaError::external(
                                format!("Bot does not have permission to deny: {:?}", perm),
                            ));
                        }
                    }
                }

                this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

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
            }))
        });

        // Guild

        // Should be documented
        methods.add_method("get_guild", |_, this, _: ()| {
            Ok(lua_promise!(this, |_lua, this|, {
                this.check_action("get_guild".to_string())
                    .map_err(LuaError::external)?;

                let guild = this.discord_provider
                    .get_guild()
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(guild))
            }))
        });

        // Should be documented
        methods.add_method("get_guild_preview", |_, this, _: ()| {
            Ok(lua_promise!(this, |_lua, this|, {
                this.check_action("get_guild_preview".to_string())
                    .map_err(LuaError::external)?;

                let guild_preview = this.discord_provider
                    .get_guild_preview()
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(guild_preview))
            }))
        });

        // Modify guild is intentionally skipped for now. TODO: Add later
        // Delete guild will not be implemented as we can't really use it

        // Should be documented
        methods.add_method("get_guild_channels", |_, this, _: ()| {
            Ok(lua_promise!(this, |_lua, this|, {
                this.check_action("get_guild_channels".to_string())
                    .map_err(LuaError::external)?;

                let chans = this.discord_provider
                    .get_guild_channels()
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(chans))
            }))
        });

        // Should be documented
        methods.add_method("create_guild_channel", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented
        methods.add_method("modify_guild_channel_positions", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented
        methods.add_method("list_active_guild_threads", |_, this, _: ()| {
            Ok(lua_promise!(this, |_lua, this|, {
                this.check_action("list_active_guild_threads".to_string())
                    .map_err(LuaError::external)?;

                let data = this.discord_provider
                    .list_active_guild_threads()
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(data))
            }))
        });

        // Should be documented
        methods.add_method("get_guild_member", |_, this, user_id: String| {
            Ok(lua_promise!(this, user_id, |_lua, this, user_id|, {
                let user_id = user_id.parse()
                .map_err(LuaError::external)?;

                this.check_action("get_guild_member".to_string())
                    .map_err(LuaError::external)?;

                let data = this.discord_provider
                    .get_guild_member(user_id)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(data))
            }))
        });

        // Should be documented
        methods.add_method("list_guild_members", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented
        methods.add_method("search_guild_members", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        methods.add_method("modify_guild_member", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let mut data = lua.from_value::<structs::ModifyGuildMemberOptions>(data)?;

                this.check_action("modify_guild_member".to_string())
                    .map_err(LuaError::external)?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                let mut needed_perms = serenity::all::Permissions::empty();

                if let Some(ref nick) = data.data.nick {
                    if nick.is_empty() {
                        return Err(LuaError::external("Nickname cannot be empty string if provided"));
                    }

                    if nick.len() > MAX_NICKNAME_LENGTH {
                        return Err(LuaError::external(
                            format!("Nickname must be less than {} characters", MAX_NICKNAME_LENGTH),
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

                let (guild, member, perms) = this.check_permissions_and_hierarchy(
                    bot_user.id,
                    data.user_id,
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
            }))
        });

        // Should be documented
        methods.add_method("add_guild_member_role", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented
        methods.add_method("remove_guild_member_role", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        // Should be documented
        methods.add_method("remove_guild_member", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });      

        // Should be documented
        methods.add_method("get_guild_ban", |_, this, user_id: String| {
            Ok(lua_promise!(this, user_id, |_lua, this, user_id|, {
                let user_id = user_id.parse::<serenity::all::UserId>()
                    .map_err(|e| LuaError::external(format!("Error while parsing user id: {}", e)))?;

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
            }))
        });     

        // Should be documented
        methods.add_method("get_guild_bans", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });        

        // Should be documented
        methods.add_method("create_guild_ban", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
                    .create_member_ban(
                        data.user_id,
                        delete_message_seconds,
                        Some(data.reason.as_str()),
                    )
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        // Should be documented
        methods.add_method("get_messages", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::GetMessagesOptions>(data)?;

                this.check_action("get_messages".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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

                // Check if the bot has permissions to send messages in the given channel
                if !guild
                    .user_permissions_in(&guild_channel, &bot_member)
                    .view_channel()
                {
                    return Err(LuaError::external(
                        "Bot does not have permission to send messages in the given channel",
                    ));
                }

                if guild_channel.kind == serenity::all::ChannelType::Voice && !guild
                .user_permissions_in(&guild_channel, &bot_member)
                .connect() {
                    return Err(LuaError::external(
                        "Bot does not have permission to connect to the given voice channel",
                    ));
                }

                let msg = this.discord_provider
                    .get_messages(data.channel_id, data.target.map(|x| x.to_serenity()), data.limit)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(msg))
            }))
        });

        // Should be documented
        methods.add_method("get_guild_roles", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_action("get_guild_roles".to_string())
                    .map_err(LuaError::external)?;

                let roles = this.discord_provider
                    .get_guild_roles()
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(roles))
            }))
        });                

        // Should be documented (get_message)
        methods.add_method("get_message", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::GetMessageOptions>(data)?;

                this.check_action("get_message".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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

                // Check if the bot has permissions to send messages in the given channel
                if !guild
                    .user_permissions_in(&guild_channel, &bot_member)
                    .view_channel()
                {
                    return Err(LuaError::external(
                        "Bot does not have permission to send messages in the given channel",
                    ));
                }

                if guild_channel.kind == serenity::all::ChannelType::Voice && !guild
                .user_permissions_in(&guild_channel, &bot_member)
                .connect() {
                    return Err(LuaError::external(
                        "Bot does not have permission to connect to the given voice channel",
                    ));
                }

                let msg = this.discord_provider
                    .get_message(data.channel_id, data.message_id)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(msg))
            }))
        });

        // Should be documented
        methods.add_method("create_message", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::CreateMessageOptions>(data)?;

                validators::validate_message(&data.data)
                    .map_err(|x| LuaError::external(x.to_string()))?;

                this.check_action("create_message".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.get_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

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

                // Check if the bot has permissions to send messages in the given channel
                if !guild
                    .user_permissions_in(&guild_channel, &bot_member)
                    .send_messages()
                {
                    return Err(LuaError::external(
                        "Bot does not have permission to send messages in the given channel",
                    ));
                }

                let files = if let Some(ref attachments) = data.data.attachments {
                    attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
                } else {
                    Vec::new()
                };

                let msg = this.discord_provider
                    .create_message(guild_channel.id, files, &data.data)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(msg))
            }))
        });

        // Interactions
        methods.add_method("create_interaction_response", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::CreateInteractionResponseOptions>(data)?;

                this.check_action("create_interaction_response".to_string())
                    .map_err(LuaError::external)?;

                let files = data.data.take_files().map_err(|e| LuaError::external(e.to_string()))?;

                this.discord_provider
                    .create_interaction_response(data.interaction_id, &data.interaction_token, &data.data, files)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        methods.add_method("create_followup_message", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::CreateFollowupMessageOptions>(data)?;

                this.check_action("create_followup_message".to_string())
                    .map_err(LuaError::external)?;

                let files = if let Some(ref attachments) = data.data.attachments {
                    attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
                } else {
                    Vec::new()
                };


                this.discord_provider
                    .create_followup_message(&data.interaction_token, &data.data, files)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        methods.add_method(
            "get_original_interaction_response",
            |_, this, interaction_token: String| {
                Ok(
                    lua_promise!(this, interaction_token, |_lua, this, interaction_token|, {
                        this.check_action("get_original_interaction_response".to_string())
                            .map_err(LuaError::external)?;

                        let resp = this.discord_provider
                            .get_original_interaction_response(&interaction_token)
                            .await
                            .map_err(|e| LuaError::external(e.to_string()))?;

                        Ok(Lazy::new(resp))
                    }),
                )
            },
        );

        methods.add_method("get_guild_command", |_, this, cmd_id: String| {
            Ok(lua_promise!(this, cmd_id, |_lua, this, cmd_id|, {
                let command_id: serenity::all::CommandId = cmd_id.parse().map_err(|e| {
                    LuaError::external(format!("Invalid command id: {}", e))
                })?;
                this.check_action("get_guild_command".to_string())
                    .map_err(LuaError::external)?;

                let resp = this.discord_provider
                    .get_guild_command(command_id)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(resp))
            }))
        });

        methods.add_method("get_guild_commands", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_action("get_guild_commands".to_string())
                    .map_err(LuaError::external)?;

                let resp = this.discord_provider
                    .get_guild_commands()
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(resp))
            }))
        });

        methods.add_method("create_guild_command", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        methods.add_method("create_guild_commands", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
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
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<DiscordActionExecutor<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    // Methods
                    "get_audit_logs",
                    "list_auto_moderation_rules",
                    "get_auto_moderation_rule",
                    "create_auto_moderation_rule",
                    "edit_auto_moderation_rule",
                    "delete_auto_moderation_rule",
                    "get_channel",
                    "edit_channel",
                    "delete_channel",
                    "edit_channel_permissions",
                    "add_guild_member_role",
                    "remove_guild_member_role",
                    "remove_guild_member",
                    "get_guild_bans",
                    "get_guild_roles",
                    "create_guild_ban",
                    //"timeout", (Not yet stable)
                    "get_messages",
                    "get_message",
                    "create_message",
                    "create_interaction_response",
                    "create_followup_message",
                    "get_original_interaction_response",
                    "get_guild_command",
                    "get_guild_commands",
                    "create_guild_command",
                    "create_guild_commands",
                ],
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(
            |_, (token, scope): (TemplateContextRef<T>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let Some(discord_provider) = token.context.discord_provider(scope) else {
                    return Err(LuaError::external(
                        "The discord plugin is not supported in this context",
                    ));
                };

                let executor = DiscordActionExecutor {
                    context: token.context.clone(),
                    discord_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
