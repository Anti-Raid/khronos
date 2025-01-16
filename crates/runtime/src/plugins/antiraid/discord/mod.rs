mod builders;
mod structs;
mod validators;

use crate::lua_promise;
use crate::traits::context::KhronosContext;
use crate::traits::discordprovider::DiscordProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::utils::serenity_backports;
use crate::{plugins::antiraid::lazy::Lazy, TemplateContextRef};
use mlua::prelude::*;
use serenity::all::Mentionable;

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
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("discord:{}", action)) {
            return Err(LuaError::runtime(
                "Discord action not allowed in this template context",
            ));
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
    ) -> LuaResult<()> {
        // Get the guild
        let guild = self
            .discord_provider
            .guild()
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

        let Some(member) = self
            .discord_provider
            .member(user_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?
        else {
            return Err(LuaError::runtime("Bot user not found in guild"));
        }; // Get the bot user

        if !serenity_backports::member_permissions(&guild, &member).contains(needed_permissions) {
            return Err(LuaError::WithContext {
                context: needed_permissions.to_string(),
                cause: LuaError::runtime("Bot does not have the required permissions").into(),
            });
        }

        Ok(())
    }

    pub async fn check_permissions_and_hierarchy(
        &self,
        user_id: serenity::all::UserId,
        target_id: serenity::all::UserId,
        needed_permissions: serenity::all::Permissions,
    ) -> LuaResult<()> {
        let guild = self
            .discord_provider
            .guild()
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

        let Some(member) = self
            .discord_provider
            .member(user_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?
        else {
            return Err(LuaError::runtime(format!(
                "User not found in guild: {}",
                user_id.mention()
            )));
        }; // Get the bot user

        if !serenity_backports::member_permissions(&guild, &member).contains(needed_permissions) {
            return Err(LuaError::runtime(format!(
                "User does not have the required permissions: {:?}: {}",
                needed_permissions, user_id
            )));
        }

        let Some(target_member) = self
            .discord_provider
            .member(target_id)
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

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for DiscordActionExecutor<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Type, |_, _, (): ()| {
            Ok("DiscordActionExecutor")
        });
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

        // Auto Moderation, not yet finished and hence not documented yet
        // get_automod_rules
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
                    .get_automod_rules()
                    .await
                    .map_err(|x| LuaError::external(x.to_string()))?;

                Ok(Lazy::new(rules))
            }))
        });

        // Not yet documented, not yet stable
        methods.add_method("get_auto_moderation_rule", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let rule_id: serenity::all::RuleId = lua.from_value(data)?;

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
                    .get_automod_rule(rule_id)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(rule))
            }))
        });

        // Not yet documented, not yet stable
        /*methods.add_method("create_auto_moderation_rule", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                #[derive(serde::Serialize, serde::Deserialize)]
                pub struct CreateAutoModerationRuleOptions {
                    name: String,
                    reason: String,
                    event_type: serenity::all::AutomodEventType,
                    trigger: serenity::all::Trigger,
                    actions: Vec<serenity::all::automod::Action>,
                    enabled: Option<bool>,
                    exempt_roles: Option<Vec<serenity::all::RoleId>>,
                    exempt_channels: Option<Vec<serenity::all::ChannelId>>,
                }

                let data: CreateAutoModerationRuleOptions = lua.from_value(data)?;

                this.check_action("create_auto_moderation_rule".to_string())
                    .map_err(LuaError::external)?;

                let bot_userid = this.serenity_context.cache.current_user().id;

                this.check_permissions(bot_userid, serenity::all::Permissions::MANAGE_GUILD)
                    .await
                    .map_err(LuaError::external)?;

                let mut rule = serenity::all::EditAutoModRule::new();
                rule = rule
                    .name(data.name)
                    .event_type(data.event_type)
                    .trigger(data.trigger)
                    .actions(data.actions);

                if let Some(enabled) = data.enabled {
                    rule = rule.enabled(enabled);
                }

                if let Some(exempt_roles) = data.exempt_roles {
                    if exempt_roles.len() > 20 {
                        return Err(LuaError::external(
                            "A maximum of 20 exempt_roles can be provided",
                        ));
                    }

                    rule = rule.exempt_roles(exempt_roles);
                }

                if let Some(exempt_channels) = data.exempt_channels {
                    if exempt_channels.len() > 50 {
                        return Err(LuaError::external(
                            "A maximum of 50 exempt_channels can be provided",
                        ));
                    }

                    rule = rule.exempt_channels(exempt_channels);
                }

                let rule = this
                    .serenity_context
                    .http
                    .create_automod_rule(this.guild_id, &rule, Some(data.reason.as_str()))
                    .await
                    .map_err(LuaError::external)?;

                Ok(Lazy::new(rule))
            }))
        });

        methods.add_method(
            "edit_auto_moderation_rule",
            |lua, this, data: LuaValue| {
                Ok(lua_promise!(this, data, |lua, this, data|, {
                    #[derive(serde::Serialize, serde::Deserialize)]
                    pub struct EditAutoModerationRuleOptions {
                        rule_id: serenity::all::RuleId,
                        reason: String,
                        name: Option<String>,
                        event_type: Option<serenity::all::AutomodEventType>,
                        trigger_metadata: Option<serenity::all::TriggerMetadata>,
                        actions: Vec<serenity::all::automod::Action>,
                        enabled: Option<bool>,
                        exempt_roles: Option<Vec<serenity::all::RoleId>>,
                        exempt_channels: Option<Vec<serenity::all::ChannelId>>,
                    }

                    let data: EditAutoModerationRuleOptions = lua.from_value(data)?;

                    this.check_action("edit_auto_moderation_rule".to_string())
                        .map_err(LuaError::external)?;

                    let bot_userid = this.serenity_context.cache.current_user().id;

                    this.check_permissions(bot_userid, serenity::all::Permissions::MANAGE_GUILD)
                        .await
                        .map_err(LuaError::external)?;

                    let mut rule = serenity::all::EditAutoModRule::new();

                    if let Some(name) = data.name {
                        rule = rule.name(name);
                    }

                    if let Some(event_type) = data.event_type {
                        rule = rule.event_type(event_type);
                    }

                    if let Some(trigger_metadata) = data.trigger_metadata {
                        rule = rule.trigger(trigger)
                    }

                    rule = rule
                        .name(data.name)
                        .event_type(data.event_type)
                        .trigger(data.trigger)
                        .actions(data.actions);

                    if let Some(enabled) = data.enabled {
                        rule = rule.enabled(enabled);
                    }

                    if let Some(exempt_roles) = data.exempt_roles {
                        if exempt_roles.len() > 20 {
                            return Err(LuaError::external(
                                "A maximum of 20 exempt_roles can be provided",
                            ));
                        }

                        rule = rule.exempt_roles(exempt_roles);
                    }

                    if let Some(exempt_channels) = data.exempt_channels {
                        if exempt_channels.len() > 50 {
                            return Err(LuaError::external(
                                "A maximum of 50 exempt_channels can be provided",
                            ));
                        }

                        rule = rule.exempt_channels(exempt_channels);
                    }

                    let rule = this
                        .serenity_context
                        .http
                        .create_automod_rule(this.guild_id, &rule, Some(data.reason.as_str()))
                        .await
                        .map_err(LuaError::external)?;

                    let v = lua.to_value(&rule)?;

                    Ok(v)
                }))
            },
        );*/

        // Channel

        // Should be documented
        methods.add_method("get_channel", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::GetChannelOptions>(data)?;

                this.check_action("get_channel".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.guild_channel(data.channel_id).await
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

                let guild_channel = this.discord_provider.guild_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                let Some(bot_member) = this.discord_provider.member(bot_user.id).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                else {
                    return Err(LuaError::runtime("Bot user not found in guild"));
                };

                let guild = this.discord_provider.guild().await
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

                // TODO: Handle permission overwrites permissions

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
            }))
        });

        // Should be documented
        methods.add_method("delete_channel", |_, this, channel_id: LuaValue| {
            Ok(lua_promise!(this, channel_id, |lua, this, channel_id|, {
                let data = lua.from_value::<structs::DeleteChannelOptions>(channel_id)?;

                this.check_action("delete_channel".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.guild_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                let Some(bot_member) = this.discord_provider.member(bot_user.id).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                else {
                    return Err(LuaError::runtime("Bot user not found in guild"));
                };

                let guild = this.discord_provider.guild().await
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

                let channel = this
                    .discord_provider
                    .delete_channel(data.channel_id, Some(data.reason.as_str()))
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(channel))
            }))
        });

        // Ban/Kick/Timeout, not yet documented as it is not yet stable
        methods.add_method("create_guild_ban", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                /// A ban action
                #[derive(serde::Serialize, serde::Deserialize)]
                pub struct BanAction {
                    user_id: serenity::all::UserId,
                    reason: String,
                    delete_message_seconds: Option<u32>,
                }

                let data = lua.from_value::<BanAction>(data)?;

                this.check_action("ban".to_string())
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

                if data.reason.len() > 128 || data.reason.is_empty() {
                    return Err(LuaError::external(
                        "Reason must be less than 128 characters and not empty",
                    ));
                }

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

        // Ban/Kick/Timeout, not yet documented as it is not yet stable
        methods.add_method("kick", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                /// A kick action
                #[derive(serde::Serialize, serde::Deserialize)]
                pub struct KickAction {
                    user_id: serenity::all::UserId,
                    reason: String,
                }

                let data = lua.from_value::<KickAction>(data)?;

                this.check_action("kick".to_string())
                    .map_err(LuaError::external)?;

                if data.reason.len() > 128 || data.reason.is_empty() {
                    return Err(LuaError::external(
                        "Reason must be less than 128 characters and not empty",
                    ));
                }

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

                this.discord_provider
                    .kick_member(data.user_id, Some(data.reason.as_str()))
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        // Ban/Kick/Timeout, not yet documented as it is not yet stable
        methods.add_method("timeout", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                /// A timeout action
                #[derive(serde::Serialize, serde::Deserialize)]
                pub struct TimeoutAction {
                    user_id: serenity::all::UserId,
                    reason: String,
                    duration_seconds: u64,
                }

                let data = lua.from_value::<TimeoutAction>(data)?;

                this.check_action("timeout".to_string())
                    .map_err(LuaError::external)?;

                if data.reason.len() > 128 || data.reason.is_empty() {
                    return Err(LuaError::external(
                        "Reason must be less than 128 characters and not empty",
                    ));
                }

                if data.duration_seconds > 60 * 60 * 24 * 28 {
                    return Err(LuaError::external(
                        "Timeout duration must be less than 28 days",
                    ));
                }

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                this.check_permissions_and_hierarchy(
                    bot_user.id,
                    data.user_id,
                    serenity::all::Permissions::MODERATE_MEMBERS,
                )
                .await
                .map_err(LuaError::external)?;

                let communication_disabled_until =
                    chrono::Utc::now() + std::time::Duration::from_secs(data.duration_seconds);

                let member = this.discord_provider
                    .edit_member(
                        data.user_id,
                        serenity::all::EditMember::new()
                            .disable_communication_until(communication_disabled_until.into()),
                        Some(data.reason.as_str())
                    )
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(member))
            }))
        });

        // Should be documented
        methods.add_method("get_messages", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::GetMessagesOptions>(data)?;

                this.check_action("get_messages".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.guild_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                let Some(bot_member) = this.discord_provider.member(bot_user.id).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                else {
                    return Err(LuaError::runtime("Bot user not found in guild"));
                };

                let guild = this.discord_provider.guild().await
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

        // Should be documented (get_message)
        methods.add_method("get_message", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let data = lua.from_value::<structs::GetMessageOptions>(data)?;

                this.check_action("get_message".to_string())
                    .map_err(LuaError::external)?;

                // Perform required checks
                let guild_channel = this.discord_provider.guild_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                let Some(bot_member) = this.discord_provider.member(bot_user.id).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                else {
                    return Err(LuaError::runtime("Bot user not found in guild"));
                };

                let guild = this.discord_provider.guild().await
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
                let guild_channel = this.discord_provider.guild_channel(data.channel_id).await
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let Some(bot_user) = this.context.current_user() else {
                    return Err(LuaError::runtime("Internal error: Current user not found"));
                };

                let Some(bot_member) = this.discord_provider.member(bot_user.id).await
                    .map_err(|e| LuaError::external(e.to_string()))?
                else {
                    return Err(LuaError::runtime("Bot user not found in guild"));
                };

                let guild = this.discord_provider.guild().await
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

                let resp = this.discord_provider
                    .create_guild_command(&data.data)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(Lazy::new(resp))
            }))
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
