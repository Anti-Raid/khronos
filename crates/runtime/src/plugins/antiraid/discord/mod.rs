mod structs;
mod validators;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::LazyLock;

//use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::{KhronosContext, Limitations};
use dapi::antiraid_check_channel_permissions::{AntiRaidCheckChannelPermissions, AntiRaidCheckChannelPermissionsOptions};
use dapi::antiraid_check_permissions::{AntiRaidCheckPermissions, AntiRaidCheckPermissionsAndHierarchy, AntiRaidCheckPermissionsAndHierarchyOptions, AntiRaidCheckPermissionsOptions};
use dapi::antiraid_get_fused_member::AntiRaidGetFusedMember;
use dapi::api::auditlogs::{GetAuditLog, GetAuditLogOptions};
use dapi::api::channels::edit_channel::EditChannel;
use dapi::api::guilds::modify_guild::ModifyGuild;
use dapi::context::DiscordContext;
use dapi::controller::DiscordProvider;
use crate::utils::{serenity_backports, serenity_utils};
use crate::{primitives::lazy::Lazy, TemplateContext};
use mluau::prelude::*;
use serenity::all::{Mentionable, ParseIdError, UserId};
use structs::{
    CreateAutoModerationRuleOptions, DeleteAutoModerationRuleOptions, EditAutoModerationRuleOptions,
};
use mlua_scheduler::LuaSchedulerAsyncUserData;

#[derive(Debug, Clone)]
struct BulkOpLimit {
    /// Maximum number of operations that can be performed in a bulk operation
    pub max_ops: usize,
    /// Minimum wait period between bulk operations
    pub min_wait: std::time::Duration,
}

static BULK_OP_LIMITS: LazyLock<HashMap<String, BulkOpLimit>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("create_message".to_string(), BulkOpLimit {
        max_ops: 1,
        min_wait: std::time::Duration::from_millis(1200),
    });
    map.insert("create_guild_role".to_string(), BulkOpLimit {
        max_ops: 1,
        min_wait: std::time::Duration::from_millis(2200),
    });
    map.insert("modify_guild_role".to_string(), BulkOpLimit {
        max_ops: 1,
        min_wait: std::time::Duration::from_millis(2000),
    });
    map.insert("delete_guild_role".to_string(), BulkOpLimit {
        max_ops: 1,
        min_wait: std::time::Duration::from_millis(2000),
    });
    map
});

const MAX_NICKNAME_LENGTH: usize = 32;
const MINIMUM_BULK_OP_WAIT_PERIOD: std::time::Duration = std::time::Duration::from_millis(2500);
const DEFAULT_BULK_OP_MAX_OPS: usize = 1;
const DEFAULT_BULK_OP_MAX_WAIT: std::time::Duration = std::time::Duration::from_secs(10);

pub struct BulkOpData {
    /// Number of operations that has been performed before a call to antiraid_bulk_op_wait
    pub op_performed: RefCell<usize>,
    /// Total number of operations that can be performed before a call to antiraid_bulk_op_wait
    pub max_ops: usize,
    /// When the bulk operation was last waited on
    pub last_waited: RefCell<std::time::Instant>,
    /// The wait period for the bulk operation
    pub min_wait: std::time::Duration,
    /// What action this bulk operation can be used for
    pub action: Option<String>,
}

#[derive(Clone)]
/// An action executor is used to execute actions such as kick/ban/timeout from Lua
/// templates
pub struct DiscordActionExecutor<T: KhronosContext> {
    context: T,
    limitations: Rc<Limitations>,
    discord_provider: T::DiscordProvider,
    discord_controller: dapi::context::DiscordContext<T::DiscordProvider>,
    bulk_op: Option<Rc<BulkOpData>>,
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

    pub fn check_action(&self, _lua: &Lua, action: String) -> LuaResult<()> {
        if !self.limitations.has_cap(&format!("discord:{action}")) {
            return Err(LuaError::runtime(format!(
                "Discord action `{action}` not allowed in this template context",
            )));
        }

        if let Some(bulk_op) = &self.bulk_op {
            if action != "antiraid_bulk_op_wait" {
                if let Some(ref b_action) = bulk_op.action {
                    if b_action != &action {
                        return Err(LuaError::runtime(format!(
                            "Bulk operation action mismatch: expected `{}`, got `{}`",
                            b_action, action
                        )));
                    }
                }

                // Check expiry
                if *bulk_op.last_waited.try_borrow().map_err(LuaError::runtime)? + DEFAULT_BULK_OP_MAX_WAIT < std::time::Instant::now() {
                    return Err(LuaError::runtime("Bulk operation maximum wait period has passed"));
                }

                if *bulk_op.op_performed.try_borrow().map_err(LuaError::external)? >= bulk_op.max_ops {
                    return Err(LuaError::runtime(format!(
                        "Bulk operation limit reached: {action}. A call to `antiraid_bulk_op_wait` is required before performing more operations",
                    )));
                }

                *bulk_op.op_performed.try_borrow_mut().map_err(LuaError::external)? += 1; // Increment the op performed counter
                return Ok(()); // No GCRA/attempt_action check needed
            }
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
        let guild_json = self
            .discord_provider
            .get_guild()
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

        let guild: serenity::all::PartialGuild = serde_json::from_value(guild_json)
            .map_err(|e| LuaError::external(e.to_string()))?;

        let member_json = self
            .discord_provider
            .get_guild_member(user_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?; // Get the bot user

        if member_json.is_null() {
            return Err(LuaError::runtime(format!(
                "User not found in guild: {}",
                user_id.mention()
            )));
        }

        let member: serenity::all::Member = serde_json::from_value(member_json)
            .map_err(|e| LuaError::external(e.to_string()))?;

        let member_perms = serenity_backports::member_permissions(&guild, &member);

        if !member_perms.contains(needed_permissions) {
            return Err(LuaError::WithContext {
                context: needed_permissions.to_string(),
                cause: LuaError::runtime("Bot does not have the required permissions").into(),
            });
        }

        Ok((guild, member, member_perms))
    }

    pub async fn get_fused_member(&self, user_ids: Vec<UserId>) -> LuaResult<structs::AntiraidFusedMember> {
        // Fetch the partial guild *once*
        let partial_guild_json = self.discord_provider
            .get_guild()
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

        let partial_guild: serenity::all::PartialGuild = serde_json::from_value(partial_guild_json)
            .map_err(|e| LuaError::external(e.to_string()))?;

        let mut member_and_resolved_perms = Vec::with_capacity(user_ids.len());

        for id in user_ids {
            let member_json = self.discord_provider
            .get_guild_member(id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            if member_json.is_null() {
                return Err(LuaError::runtime(format!(
                    "User not found in guild: {}",
                    id.mention()
                )));
            }

            let member: serenity::all::Member = serde_json::from_value(member_json)
                .map_err(|e| LuaError::external(e.to_string()))?;

            let resolved_perms = serenity_backports::member_permissions(&partial_guild, &member);

            member_and_resolved_perms.push(structs::AntiraidFusedMemberSingle {
                member,
                resolved_perms,
            });
        }
        
        Ok(structs::AntiraidFusedMember {
            guild: partial_guild,
            members: member_and_resolved_perms,
        })
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

        // Bulk operation support
        methods.add_method("antiraid_bulk_op", |lua, this, action: Option<String>| {
            this.check_action(&lua, "antiraid_bulk_op".to_string())
                .map_err(LuaError::external)?;

            if this.bulk_op.is_some() {
                return Err(LuaError::runtime("Cannot start a bulk operation if the DiscordActionExecutor itself is setup for bulk operations"));
            }

            let bulk_limits = if let Some(action) = action.as_ref() {
                match BULK_OP_LIMITS.get(action.as_str()) {
                    Some(limit) => limit.clone(),
                    None => {
                        BulkOpLimit {
                            max_ops: DEFAULT_BULK_OP_MAX_OPS,
                            min_wait: MINIMUM_BULK_OP_WAIT_PERIOD,
                        }
                    }
                }
            } else {
                BulkOpLimit {
                    max_ops: DEFAULT_BULK_OP_MAX_OPS,
                    min_wait: MINIMUM_BULK_OP_WAIT_PERIOD,
                }   
            };

            let bulk_op = Rc::new(BulkOpData {
                action: action.clone(),
                op_performed: RefCell::new(0), // Any op which calls check_action will increment this
                max_ops: bulk_limits.max_ops, // Default max ops
                min_wait: bulk_limits.min_wait, // Default min wait
                last_waited: RefCell::new(std::time::Instant::now()), // Last waited time
            });

            let executor = DiscordActionExecutor {
                context: this.context.clone(),
                limitations: this.limitations.clone(),
                discord_provider: this.discord_provider.clone(),
                discord_controller: this.discord_controller.clone(),
                bulk_op: Some(bulk_op)
            };

            Ok(executor)
        });

        methods.add_scheduler_async_method(
            "antiraid_bulk_op_wait",
            async move |lua, this, _: ()| {
                this.check_action(&lua, "antiraid_bulk_op_wait".to_string())
                .map_err(LuaError::external)?;

                let Some(bulk_op) = &this.bulk_op else {
                    return Err(LuaError::runtime("This DiscordActionExecutor is not set up for bulk operations"));
                };

                // Check max ops performed
                if *bulk_op.op_performed.try_borrow().map_err(LuaError::runtime)? < *bulk_op.op_performed.try_borrow().map_err(LuaError::runtime)? {
                    return Ok(()); // No-op if the user can still perform operations
                }

                // Check expiry
                if *bulk_op.last_waited.try_borrow().map_err(LuaError::runtime)? + DEFAULT_BULK_OP_MAX_WAIT < std::time::Instant::now() {
                    return Err(LuaError::runtime("antiraid_bulk_op_wait called after the maximum wait period"));
                }

                if *bulk_op.op_performed.try_borrow().map_err(|_| LuaError::runtime("Failed to borrow op_performed"))? == 0 {
                    return Ok(()); // No-op if no operation was performed
                }

                // Wait for the enforced wait period + random jitter between 10 and 500 millis
                let wait_period = bulk_op.min_wait + std::time::Duration::from_millis(
                    rand::random_range(10..500),
                );

                tokio::time::sleep(wait_period).await;

                if *bulk_op.op_performed.try_borrow().map_err(|_| LuaError::runtime("Failed to borrow op_performed"))? == 0 {
                    return Err(LuaError::runtime("antiraid_bulk_op_wait cannot be called concurrently during/across a wait period"));
                }

                *bulk_op.op_performed.try_borrow_mut().map_err(|_| LuaError::runtime("Failed to borrow op_performed"))? = 0; // Reset the op_performed counter
                *bulk_op.last_waited.try_borrow_mut().map_err(|_| LuaError::runtime("Failed to borrow last_waited"))? = std::time::Instant::now(); // Update the last waited time

                Ok(())
            },
        );

        // Basic helper functions
        methods.add_method("antiraid_check_reason", |_, this, reason: String| {
            Ok(this.check_reason(&reason))
        });

        // now in dapi
        methods.add_scheduler_async_method(
            "antiraid_check_permissions",
            async move |lua, this, data: LuaValue| {
                let data = lua.from_value::<AntiRaidCheckPermissionsOptions>(data)?;
                
                let resp = dapi::exec_api(
                    &this.discord_controller, 
                    AntiRaidCheckPermissions {
                        data,
                    }, 
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;
                
                Ok(Lazy::new(resp))
            },
        );

        // now in dapi
        methods.add_scheduler_async_method(
            "antiraid_check_permissions_and_hierarchy",
            async move |lua, this, data: LuaValue| {
                let data =
                    lua.from_value::<AntiRaidCheckPermissionsAndHierarchyOptions>(data)?;

                let resp = dapi::exec_api(
                    &this.discord_controller, 
                    AntiRaidCheckPermissionsAndHierarchy {
                        data,
                    }, 
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;
                
                Ok(Lazy::new(resp))
            },
        );

        // now in dapi
        methods.add_scheduler_async_method("antiraid_check_channel_permissions", async move |
            lua, 
            this,
            data: LuaValue,
        | {
            let data = lua.from_value::<AntiRaidCheckChannelPermissionsOptions>(data)?;

            let resp = dapi::exec_api(
                &this.discord_controller, 
                AntiRaidCheckChannelPermissions {
                    data,
                }, 
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;
            
            Ok(Lazy::new(resp))
        });

        // now in dapi
        methods.add_scheduler_async_method("antiraid_get_fused_member", async move |
            _lua, 
            this,
            ids: Vec<String>,
        | {
            let resp = dapi::exec_api(
                &this.discord_controller, 
                AntiRaidGetFusedMember {
                    ids,
                }, 
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;
            
            Ok(Lazy::new(resp))
        });

        // Audit Log

        // Should be documented
        // Implemented in dapi
        methods.add_scheduler_async_method("get_audit_logs", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<GetAuditLogOptions>(data)?;

            this.check_action(&lua, "get_audit_logs".to_string())
                .map_err(LuaError::external)?;

            let resp = dapi::exec_api(
                &this.discord_controller, 
                GetAuditLog {
                    data,
                }, 
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;
            
            Ok(Lazy::new(resp))
        });

        // Auto Moderation

        // Should be documented.
        methods.add_scheduler_async_method("list_auto_moderation_rules", async move |lua, this, _: ()| {
            this.check_action(&lua, "list_auto_moderation_rules".to_string())
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "get_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "create_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "edit_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "delete_auto_moderation_rule".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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
        methods.add_scheduler_async_method("get_channel", async move |lua, this, channel_id: String| {
            let channel_id: serenity::all::GenericChannelId = channel_id
                .parse()
                .map_err(|e: ParseIdError| LuaError::external(e.to_string()))?;

            this.check_action(&lua, "get_channel".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks. Note that get_channel does access control
            let channel = this.discord_provider.get_channel(channel_id).await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            Ok(Lazy::new(channel))
        });

        // Should be documented
        methods.add_scheduler_async_method("edit_channel", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<EditChannel>(data)?;

            this.check_action(&lua, "edit_channel".to_string())
                .map_err(LuaError::external)?;

            let resp = dapi::exec_api(
                &this.discord_controller, 
                data, 
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;
            
            Ok(Lazy::new(resp))
        });

        // Should be documented
        methods.add_scheduler_async_method("delete_channel", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteChannelOptions>(data)?;

            this.check_action(&lua, "delete_channel".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (_partial_guild, _bot_member, guild_channel, perms) = this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::empty())
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

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

            this.check_action(&lua, "edit_channel_permissions".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let (_partial_guild, _bot_member, _guild_channel, perms) = this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_ROLES)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

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

        methods.add_scheduler_async_method("get_channel_invites", async move |lua, this, channel_id: String| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
            .map_err(|e| LuaError::external(e.to_string()))?;

            this.check_action(&lua, "get_channel_invites".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_CHANNELS)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            let invites = this
            .discord_provider
            .get_channel_invites(channel_id)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invites))
        });

        methods.add_scheduler_async_method("create_channel_invite", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateChannelInviteOptions>(data)?;

            this.check_action(&lua, "create_channel_invite".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::CREATE_INSTANT_INVITE)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            let invite = this
            .discord_provider
            .create_channel_invite(data.channel_id, &data.data, Some(data.reason.as_str()))
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invite))
        });

        methods.add_scheduler_async_method("delete_channel_permission", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteChannelPermissionOptions>(data)?;

            this.check_action(&lua, "delete_channel_permission".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "follow_announcement_channel".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            this.discord_controller.check_channel_permissions(bot_user.id, data.data.webhook_channel_id, serenity::all::Permissions::MANAGE_WEBHOOKS)
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            let data = this
            .discord_provider
            .follow_announcement_channel(data.channel_id, data.data, Some(data.reason.as_str()))
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Guild

        // Should be documented
        methods.add_scheduler_async_method("get_guild", async move |lua, this, _: ()| {
            this.check_action(&lua, "get_guild".to_string())
            .map_err(LuaError::external)?;

            let guild = this.discord_provider
                .get_guild()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(guild))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_preview", async move |lua, this, _: ()| {
            this.check_action(&lua, "get_guild_preview".to_string())
            .map_err(LuaError::external)?;

            let guild_preview = this.discord_provider
                .get_guild_preview()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(guild_preview))
        });

        methods.add_scheduler_async_method("modify_guild", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<ModifyGuild>(data)?;

            this.check_action(&lua, "modify_guild".to_string())
                .map_err(LuaError::external)?;

            let resp = dapi::exec_api(
                &this.discord_controller, 
                data, 
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });

        // Modify guild is intentionally skipped for now. TODO: Add later
        // Delete guild will not be implemented as we can't really use it

        // Should be documented
        methods.add_scheduler_async_method("get_guild_channels", async move |lua, this, _: ()| {
            this.check_action(&lua, "get_guild_channels".to_string())
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

            this.check_action(&lua, "create_guild_channel".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

                this.check_action(&lua, "modify_guild_channel_positions".to_string())
                    .map_err(LuaError::external)?;

                let Some(bot_user) = this.discord_controller.current_user() else {
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
        methods.add_scheduler_async_method("list_active_guild_threads", async move |lua, this, _: ()| {
            this.check_action(&lua, "list_active_guild_threads".to_string())
            .map_err(LuaError::external)?;

            let data = this.discord_provider
                .list_active_guild_threads()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(data))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_member", async move |lua, this, user_id: String| {
            let user_id = user_id.parse()
            .map_err(LuaError::external)?;

            this.check_action(&lua, "get_guild_member".to_string())
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

            this.check_action(&lua, "list_guild_members".to_string())
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

            this.check_action(&lua, "search_guild_members".to_string())
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

            this.check_action(&lua, "modify_guild_member".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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
                    if crdu > (chrono::Utc::now() + chrono::Duration::days(28) + chrono::Duration::seconds(10)) {
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

            this.check_action(&lua, "add_guild_member_role".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let bot_member_json = this.discord_provider.get_guild_member(bot_user.id).await
                .map_err(|e| LuaError::external(e.to_string()))?;

            if bot_member_json.is_null() {
                return Err(LuaError::runtime("Bot user not found in guild"));
            }

            let bot_member = serde_json::from_value::<serenity::all::Member>(bot_member_json)
                .map_err(LuaError::external)?;

            let guild_json = this.discord_provider.get_guild().await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let guild = serde_json::from_value::<serenity::all::PartialGuild>(guild_json)
                .map_err(LuaError::external)?;

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

            this.check_action(&lua, "remove_guild_member_role".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            let bot_member_json = this.discord_provider.get_guild_member(bot_user.id).await
                .map_err(|e| LuaError::external(e.to_string()))?;

            if bot_member_json.is_null() {
                return Err(LuaError::runtime("Bot user not found in guild"));
            }

            let bot_member = serde_json::from_value::<serenity::all::Member>(bot_member_json)
                .map_err(LuaError::external)?;

            let guild_json = this.discord_provider.get_guild().await
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let guild = serde_json::from_value::<serenity::all::PartialGuild>(guild_json)
                .map_err(LuaError::external)?;

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

            this.check_action(&lua, "remove_guild_member".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_permissions_and_hierarchy(
                bot_user.id,
                data.user_id,
                serenity::all::Permissions::KICK_MEMBERS,
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

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

            this.check_action(&lua, "get_guild_bans".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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
        methods.add_scheduler_async_method("get_guild_ban", async move |lua, this, user_id: String| {
            let user_id = user_id.parse::<serenity::all::UserId>()
            .map_err(|e| LuaError::external(format!("Error while parsing user id: {e}")))?;

            this.check_action(&lua, "get_guild_ban".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "create_guild_ban".to_string())
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

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_permissions_and_hierarchy(
                bot_user.id,
                data.user_id,
                serenity::all::Permissions::BAN_MEMBERS,
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

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

            this.check_action(&lua, "remove_guild_ban".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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
        methods.add_scheduler_async_method("get_guild_roles", async move |lua, this, _g: ()| {
            this.check_action(&lua, "get_guild_roles".to_string())
            .map_err(LuaError::external)?;

            let roles = this.discord_provider
                .get_guild_roles()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(roles))
        });

        // Should be documented
        methods.add_scheduler_async_method("get_guild_role", async move |lua, this, role_id: String| {
            let role_id = role_id.parse::<serenity::all::RoleId>()
            .map_err(LuaError::external)?;

            this.check_action(&lua, "get_guild_role".to_string())
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

            this.check_action(&lua, "create_guild_role".to_string())
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

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "modify_guild_role_positions".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "modify_guild_role".to_string())
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

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "delete_guild_role".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
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

            this.check_action(&lua, "get_invite".to_string())
                .map_err(LuaError::external)?;

            let invite = this.discord_provider
                .get_invite(&data.code, data.with_counts.unwrap_or(false), data.with_expiration.unwrap_or(false), data.guild_scheduled_event_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(invite))
        });

        methods.add_scheduler_async_method("delete_invite", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteInviteOptions>(data)?;

            this.check_action(&lua, "delete_invite".to_string())
                .map_err(LuaError::external)?;

            this.check_reason(&data.reason)
            .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };    

            // Call get_invite to find the channel id
            let invite_json = this.discord_provider
                .get_invite(&data.code, false, false, None)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let invite = serde_json::from_value::<serenity::all::Invite>(invite_json)
                .map_err(LuaError::external)?;

            if let Some(guild) = invite.guild {
                if guild.id != this.discord_provider.guild_id() {
                    return Err(LuaError::external("Invite does not belong to the current guild"));
                }
            }

            let (_partial_guild, _bot_member, _channel, perms) = this.discord_controller.check_channel_permissions(bot_user.id, invite.channel.id.widen(), serenity::all::Permissions::empty())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

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

            this.check_action(&lua, "get_channel_messages".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // Perform required checks
            let (_, _, guild_channel, perms) = this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::VIEW_CHANNEL).await
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

            this.check_action(&lua, "get_channel_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // Perform required checks
            let (_, _, guild_channel, perms) = this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::VIEW_CHANNEL).await
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

            this.check_action(&lua, "create_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::SEND_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

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

        methods.add_scheduler_async_method("crosspost_message", async move |lua, this, (channel_id, message_id): (String, String)| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let message_id = message_id.parse::<serenity::all::MessageId>()
                .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?;

            this.check_action(&lua, "crosspost_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just send if the message author is the same as the bot user, this takes an extra API call to check. Not worth it
            this.discord_controller.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::SEND_MESSAGES | serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let msg = this.discord_provider
                .crosspost_message(channel_id, message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("create_reaction", async move |lua, this, data: LuaValue| {
            let data: structs::CreateReactionOptions = lua.from_value(data)?;

            this.check_action(&lua, "create_reaction".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::READ_MESSAGE_HISTORY | serenity::all::Permissions::ADD_REACTIONS)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .create_reaction(data.channel_id, data.message_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete_own_reaction", async move |lua, this, data: LuaValue| {
            let data: structs::DeleteOwnReactionOptions = lua.from_value(data)?;

            this.check_action(&lua, "delete_own_reaction".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::READ_MESSAGE_HISTORY | serenity::all::Permissions::ADD_REACTIONS)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .delete_own_reaction(data.channel_id, data.message_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete_user_reaction", async move |lua, this, data: LuaValue| {
            let data: structs::DeleteUserReactionOptions = lua.from_value(data)?;

            this.check_action(&lua, "delete_user_reaction".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .delete_user_reaction(data.channel_id, data.message_id, data.user_id, &data.reaction.into_serenity())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("get_reactions", async move |lua, this, data: LuaValue| {
            let data: structs::GetReactionsOptions = lua.from_value(data)?;

            this.check_action(&lua, "get_reactions".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::empty())
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

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

        methods.add_scheduler_async_method("delete_all_reactions", async move |lua, this, (channel_id, message_id): (String, String)| {
            let channel_id: serenity::all::GenericChannelId = channel_id.parse()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let message_id: serenity::all::MessageId = message_id.parse()
                .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?;

            this.check_action(&lua, "delete_all_reactions".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.discord_controller.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .delete_all_reactions(channel_id, message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete_all_reactions_for_emoji", async move |lua, this, data: LuaValue| {
            let data: structs::DeleteAllReactionsForEmojiOptions = lua.from_value(data)?;

            this.check_action(&lua, "delete_all_reactions_for_emoji".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            // While discord technically allows just read message history if the reaction already exists, this takes an extra API call to check and might not be desirable either. Not worth it
            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

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

            this.check_action(&lua, "edit_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, data.channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

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

        methods.add_scheduler_async_method("delete_message", async move |lua, this, (channel_id, message_id, reason): (String, String, String)| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let message_id = message_id.parse::<serenity::all::MessageId>()
                .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?;

            this.check_action(&lua, "delete_message".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .delete_message(channel_id, message_id, Some(reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("bulk_delete_messages", async move |lua, this, (channel_id, messages, reason): (String, Vec<String>, String)| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing channel id: {e}")))?;

            let mut message_ids = Vec::with_capacity(messages.len());
            for message_id in messages {
                message_ids.push(
                    message_id.parse::<serenity::all::MessageId>()
                        .map_err(|e| LuaError::external(format!("Error while parsing message id: {e}")))?
                );
            }

            this.check_action(&lua, "bulk_delete_messages".to_string())
                .map_err(LuaError::external)?;

            // Perform required checks
            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(bot_user.id, channel_id, serenity::all::Permissions::MANAGE_MESSAGES)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            this.discord_provider
                .bulk_delete_messages(channel_id, serde_json::json!({"messages": message_ids}), Some(reason.as_str()))
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Interactions
        methods.add_scheduler_async_method("create_interaction_response", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateInteractionResponseOptions>(data)?;

            this.check_action(&lua, "create_interaction_response".to_string())
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
            async move |lua, this, interaction_token: String| {
                this.check_action(&lua, "get_original_interaction_response".to_string())
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

            this.check_action(&lua, "edit_original_interaction_response".to_string())
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
            async move |lua, this, interaction_token: String| {
                this.check_action(&lua, "delete_original_interaction_response".to_string())
                .map_err(LuaError::external)?;

                this.discord_provider
                    .delete_original_interaction_response(&interaction_token)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method("create_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateFollowupMessageOptions>(data)?;

            this.check_action(&lua, "create_followup_message".to_string())
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

        methods.add_scheduler_async_method("get_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::GetFollowupMessageOptions>(data)?;

            this.check_action(&lua, "get_followup_message".to_string())
                .map_err(LuaError::external)?;

            let msg = this.discord_provider
                .get_followup_message(&data.interaction_token, data.message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        methods.add_scheduler_async_method("edit_followup_message", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditFollowupMessageOptions>(data)?;

            this.check_action(&lua, "edit_followup_message".to_string())
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

            this.check_action(&lua, "delete_followup_message".to_string())
                .map_err(LuaError::external)?;

            this.discord_provider
                .delete_followup_message(&data.interaction_token, data.message_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Uncategorized

        // Should be documented
        methods.add_scheduler_async_method("get_guild_command", async move |lua, this, cmd_id: String| {
            let command_id: serenity::all::CommandId = cmd_id.parse().map_err(|e| {
                LuaError::external(format!("Invalid command id: {e}"))
            })?;
            this.check_action(&lua, "get_guild_command".to_string())
                .map_err(LuaError::external)?;

            let resp = this.discord_provider
                .get_guild_command(command_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });

        methods.add_scheduler_async_method("get_guild_commands", async move |lua, this, _g: ()| {
            this.check_action(&lua, "get_guild_commands".to_string())
            .map_err(LuaError::external)?;

            let resp = this.discord_provider
                .get_guild_commands()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(resp))
        });

        methods.add_scheduler_async_method("create_guild_command", async move |lua, this, data: LuaValue| {
            this.check_action(&lua, "create_guild_command".to_string())
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
            this.check_action(&lua, "create_guild_commands".to_string())
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

        // Webhooks
        methods.add_scheduler_async_method("create_webhook", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::CreateWebhookOptions>(data)?;
            
            this.check_action(&lua, "create_webhook".to_string())
                .map_err(LuaError::external)?;
            
            this.check_reason(&data.reason)?;

            if let Some(ref avatar) = data.data.avatar {
                let format = get_format_from_image_data(avatar)
                .map_err(LuaError::external)?;

                if format != "png" && format != "jpeg" && format != "gif" {
                    return Err(LuaError::external(
                        "Icon must be a PNG, JPEG, or GIF format",
                    ));
                }
            }

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(
                bot_user.id,   
                data.channel_id,
                serenity::all::Permissions::MANAGE_WEBHOOKS,
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;
            
            let webhook = this.discord_provider
                .create_webhook(
                    data.channel_id,
                    data.data,
                    Some(data.reason.as_str())
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(webhook))
        });

        methods.add_scheduler_async_method("get_channel_webhooks", async move |lua, this, channel_id: String| {
            let channel_id = channel_id.parse::<serenity::all::GenericChannelId>()
                .map_err(|e| LuaError::external(format!("Error while parsing webhook id: {e}")))?;

            this.check_action(&lua, "get_channel_webhooks".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.discord_controller.check_channel_permissions(
                bot_user.id,   
                channel_id,
                serenity::all::Permissions::MANAGE_WEBHOOKS,
            )
            .await
            .map_err(|e| LuaError::external(e.to_string()))?;

            let webhooks = this.discord_provider
                .get_channel_webhooks(channel_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(webhooks))
        });

        methods.add_scheduler_async_method("get_guild_webhooks", async move |lua, this, _: ()| {
            this.check_action(&lua, "get_guild_webhooks".to_string())
                .map_err(LuaError::external)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(
                bot_user.id,   
                serenity::all::Permissions::MANAGE_WEBHOOKS,
            )
            .await
            .map_err(LuaError::external)?;

            let webhooks = this.discord_provider
                .get_guild_webhooks()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(webhooks))
        });

        methods.add_scheduler_async_method("get_webhook", async move |lua, this, webhook_id: String| {
            let webhook_id = webhook_id.parse::<serenity::all::WebhookId>()
                .map_err(|e| LuaError::external(format!("Error while parsing webhook id: {e}")))?;

            this.check_action(&lua, "get_webhook".to_string())
                .map_err(LuaError::external)?;

            let webhook = this.discord_provider
                .get_webhook(webhook_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            };

            if guild_id != &this.discord_provider.guild_id().to_string() {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            }

            Ok(Lazy::new(webhook))
        });

        methods.add_scheduler_async_method("modify_webhook", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::EditWebhookOptions>(data)?;
            
            this.check_action(&lua, "modify_webhook".to_string())
                .map_err(LuaError::external)?;
            
            this.check_reason(&data.reason)?;

            if let Some(ref avatar) = data.data.avatar.as_inner_ref() {
                let format = get_format_from_image_data(avatar)
                .map_err(LuaError::external)?;

                if format != "png" && format != "jpeg" && format != "gif" {
                    return Err(LuaError::external(
                        "Icon must be a PNG, JPEG, or GIF format",
                    ));
                }
            }

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            if let Some(channel_id) = data.data.channel_id {
                this.discord_controller.check_channel_permissions(
                    bot_user.id,   
                    channel_id.widen(),
                    serenity::all::Permissions::empty(),
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;
            }

            this.check_permissions(
                bot_user.id,   
                serenity::all::Permissions::MANAGE_WEBHOOKS,
            )
            .await
            .map_err(LuaError::external)?;

            let webhook = this.discord_provider
                .get_webhook(data.webhook_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            };

            if guild_id != &this.discord_provider.guild_id().to_string() {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            }
            
            let webhook = this.discord_provider
                .modify_webhook(
                    data.webhook_id,
                    data.data,
                    Some(data.reason.as_str())
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(webhook))
        });

        // Modify webhook with token is intentionally not supported due to security concerns

        methods.add_scheduler_async_method("delete_webhook", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::DeleteWebhookOptions>(data)?;
            
            this.check_action(&lua, "delete_webhook".to_string())
                .map_err(LuaError::external)?;
            
            this.check_reason(&data.reason)?;

            let Some(bot_user) = this.discord_controller.current_user() else {
                return Err(LuaError::runtime("Internal error: Current user not found"));
            };

            this.check_permissions(
                bot_user.id,   
                serenity::all::Permissions::MANAGE_WEBHOOKS,
            )
            .await
            .map_err(LuaError::external)?;


            let webhook = this.discord_provider
                .get_webhook(data.webhook_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            };

            if guild_id != &this.discord_provider.guild_id().to_string() {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            }
            
            this.discord_provider
                .delete_webhook(
                    data.webhook_id,
                    Some(data.reason.as_str())
                )
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        // Delete webhook with token is intentionally not supported due to security concerns

        methods.add_scheduler_async_method("execute_webhook", async move |lua, this, data: LuaValue| {
            let data = lua.from_value::<structs::ExecuteWebhookOptions>(data)?;

            validators::validate_webhook_execute(&data.data)
                .map_err(|x| LuaError::external(x.to_string()))?;

            this.check_action(&lua, "execute_webhook".to_string())
                .map_err(LuaError::external)?;

            // Ensure webhook exists on the same server as the guild we're in
            let webhook = this.discord_provider
                .get_webhook(data.webhook_id)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            };

            if guild_id != &this.discord_provider.guild_id().to_string() {
                return Err(LuaError::runtime("Webhook does not belong to a guild"));
            }

            let files = if let Some(ref attachments) = data.data.attachments {
                attachments.take_files().map_err(|e| LuaError::external(e.to_string()))?
            } else {
                Vec::new()
            };

            let msg = this.discord_provider
                .execute_webhook(data.webhook_id, &data.webhook_token, data.thread_id, &data.data, files)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(Lazy::new(msg))
        });

        // Get/Edit/Delete webhook message is intentionally not supported due to lack of use cases and security concerns
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
        discord_controller: DiscordContext::new(
            discord_provider.clone()
        ),
        discord_provider,
        bulk_op: None, // A call to antiraid_bulk_op in DiscordActionExecutor will set this on a new executor instance
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
