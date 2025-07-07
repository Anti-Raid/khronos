mod types;

use std::rc::Rc;

use crate::traits::context::KhronosContext;
use crate::traits::lockdownprovider::LockdownProvider;
use crate::TemplateContext;
use crate::{primitives::create_userdata_iterator_with_fields, traits::context::Limitations};
use lockdowns::LockdownSet;
use mluau::prelude::*;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use types::{CreateLockdownMode, LockdownMode};

#[derive(Clone)]
/// An lockdown executor is used to manage AntiRaid lockdowns from Lua
/// templates
pub struct LockdownExecutor<T: KhronosContext> {
    pub context: T,
    pub limitations: Rc<Limitations>,
    pub lockdown_provider: T::LockdownProvider,
}

// @userdata LockdownExecutor
//
// Executes actions on discord
impl<T: KhronosContext> LockdownExecutor<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.limitations.has_cap(&format!("lockdown:{}", action)) {
            return Err(LuaError::runtime(
                "Lockdown action is not allowed in this template context",
            ));
        }

        self.lockdown_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for LockdownExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "LockdownExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method("fetch_lockdown_set", async move |_, this, _g: ()| {
            this.check_action("fetch_lockdown_set".to_string())?;

            // Get the current lockdown set
            let lockdown_set = LockdownSet::guild(
                this.context.guild_id().ok_or_else(|| {
                    LuaError::external("This function can only be used in a guild context")
                })?,
                this.lockdown_provider.lockdown_data_store().clone(),
            )
            .await
            .map_err(|e| LuaError::external(format!("Error while fetching lockdown set: {}", e)))?;

            Ok(types::LockdownSet::<T> {
                lockdown_set,
                limitations: this.limitations.clone(),
                lockdown_provider: this.lockdown_provider.clone(),
            })
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownExecutor<T>>() {
                return Err(mluau::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Methods
                    "fetch_lockdown_set",
                ],
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    let Some(lockdown_provider) = token.context.lockdown_provider() else {
        return Err(LuaError::external(
            "The lockdown plugin is not supported in this context",
        ));
    };

    let executor = LockdownExecutor {
        context: token.context.clone(),
        limitations: token.limitations.clone(),
        lockdown_provider,
    };

    module.set("Client", executor)?;

    module.set(
        "CreateQuickServerLockdown",
        CreateLockdownMode(Box::new(lockdowns::qsl::CreateQuickServerLockdown)),
    )?;
    module.set(
        "CreateTraditionalServerLockdown",
        CreateLockdownMode(Box::new(lockdowns::tsl::CreateTraditionalServerLockdown)),
    )?;
    module.set(
        "CreateSingleChannelLockdown",
        CreateLockdownMode(Box::new(lockdowns::scl::CreateSingleChannelLockdown)),
    )?;
    module.set(
        "CreateSingleChannelLockdown",
        CreateLockdownMode(Box::new(lockdowns::role::CreateRoleLockdown)),
    )?;
    module.set(
        "QuickServerLockdown",
        lua.create_function(|_lua, _g: ()| {
            Ok(LockdownMode(Box::new(lockdowns::qsl::QuickServerLockdown)))
        })?,
    )?;
    module.set(
        "TraditionalServerLockdown",
        lua.create_function(|_lua, _g: ()| {
            Ok(LockdownMode(Box::new(
                lockdowns::tsl::TraditionalServerLockdown,
            )))
        })?,
    )?;
    module.set(
        "SingleChannelLockdown",
        lua.create_function(|_lua, channel_id: String| {
            let channel_id = channel_id
                .parse::<serenity::all::ChannelId>()
                .map_err(|_| LuaError::external("Failed to parse string to u64"))?;

            Ok(LockdownMode(Box::new(
                lockdowns::scl::SingleChannelLockdown(channel_id),
            )))
        })?,
    )?;
    module.set(
        "RoleLockdown",
        lua.create_function(|_lua, role_id: String| {
            let role_id = role_id
                .parse::<serenity::all::RoleId>()
                .map_err(|_| LuaError::external("Failed to parse string to u64"))?;

            Ok(LockdownMode(Box::new(lockdowns::role::RoleLockdown(
                role_id,
            ))))
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
