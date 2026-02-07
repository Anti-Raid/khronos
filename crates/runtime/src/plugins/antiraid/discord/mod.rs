use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::LazyLock;

use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
//use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::{KhronosContext, Limitations};
use dapi::{apilist::MapResponseMetadata, context::DiscordContext};
use dapi::controller::{DiscordProvider, DiscordProviderContext};
use crate::{primitives::lazy::Lazy, TemplateContext};
use mluau::prelude::*;
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

    pub fn check_action_impl(&self, _lua: &Lua, action: &str) -> LuaResult<()> {
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
}

impl<T: KhronosContext> dapi::apilist::APIUserData for DiscordActionExecutor<T> {
    type DiscordProvider = T::DiscordProvider;

    fn check_action(&self, lua: &mluau::Lua, action: &str) -> mluau::Result<()> {
        self.check_action_impl(lua, action)
    }

    fn controller(&self) -> &DiscordContext<Self::DiscordProvider> {
        &self.discord_controller
    }

    fn map_response<TT: serde::Serialize + 'static>(&self, lua: &mluau::Lua, _action: &str, mrm: MapResponseMetadata, resp: TT) -> mluau::Result<mluau::Value> {
        if mrm.is_primitive_response {
            let v = lua.to_value_with(&resp, LUA_SERIALIZE_OPTIONS)?;
            return Ok(v);
        }
        
        let ud = lua.create_userdata(Lazy::new(resp))?;
        Ok(mluau::Value::UserData(ud))
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
            this.check_action_impl(&lua, "antiraid_bulk_op")
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
                this.check_action_impl(&lua, "antiraid_bulk_op_wait")
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

        methods.add_method("antiraid_check_reason", |_, this, reason: String| {
            Ok(this.check_reason(&reason))
        });

        // Checks the guild_id of the discord executor
        methods.add_method("guild_id", |_lua, this, _: ()| {
            match this.discord_controller.controller().context() {
                DiscordProviderContext::Guild(guild_id) => Ok(Some(guild_id.to_string())),
                _ => Ok(None)
            }
        });

        // Checks the user_id of the discord executor
        methods.add_method("user_id", |_lua, this, _: ()| {
            match this.discord_controller.controller().context() {
                DiscordProviderContext::User(user_id) => Ok(Some(user_id.to_string())),
                _ => Ok(None)
            }
        });

        dapi::apilist::API::add_luau_methods::<DiscordActionExecutor<T>, _>(methods);

        // Get/Edit/Delete webhook message is currently not supported
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
