use crate::lua_promise;
use crate::traits::context::KhronosContext;
use crate::traits::lockdownprovider::LockdownProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::TemplateContextRef;
use mlua::prelude::*;

#[derive(Clone)]
/// An lockdown executor is used to manage AntiRaid lockdowns from Lua
/// templates
pub struct LockdownExecutor<T: KhronosContext> {
    context: T,
    lockdown_provider: T::LockdownProvider,
}

// @userdata LockdownExecutor
//
// Executes actions on discord
impl<T: KhronosContext> LockdownExecutor<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("lockdown:{}", action)) {
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
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Type, |_, _, (): ()| Ok("LockdownExecutor"));

        methods.add_method("list", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |lua, this, _g|, {
                this.check_action("list".to_string())?;

                // Get the current lockdown set
                let lockdowns = this.lockdown_provider.list().await.map_err(|e| {
                    LuaError::external(format!("Error while fetching lockdowns: {}", e))
                })?;

                let value = lua.to_value(&lockdowns)?;

                Ok(value)
            }))
        });

        methods.add_method("qsl", |_, this, reason: String| {
            Ok(lua_promise!(this, reason, |_lua, this, reason|, {
                this.check_action("qsl".to_string())?;

                let lockdown_id = this.lockdown_provider.qsl(reason).await.map_err(|e| {
                    LuaError::external(format!("Error while creating lockdown: {}", e))
                })?;

                Ok(lockdown_id.to_string())
            }))
        });

        methods.add_method("tsl", |_, this, reason: String| {
            Ok(lua_promise!(this, reason, |_lua, this, reason|, {
                this.check_action("tsl".to_string())?;

                let lockdown_id = this.lockdown_provider.tsl(reason).await.map_err(|e| {
                    LuaError::external(format!("Error while creating lockdown: {}", e))
                })?;

                Ok(lockdown_id.to_string())
            }))
        });

        methods.add_method("scl", |_, this, (channel, reason): (String, String)| {
            let channel: serenity::all::ChannelId = channel.parse().map_err(|e| {
                LuaError::external(format!("Error while parsing channel id: {}", e))
            })?;

            Ok(
                lua_promise!(this, channel, reason, |_lua, this, channel, reason|, {
                    this.check_action("scl".to_string())?;

                    let lockdown_id = this.lockdown_provider.scl(channel, reason).await.map_err(|e| {
                        LuaError::external(format!("Error while creating lockdown: {}", e))
                    })?;
    
                    Ok(lockdown_id.to_string())
                }),
            )
        });

        methods.add_method("role", |_, this, (role, reason): (String, String)| {
            let role: serenity::all::RoleId = role
                .parse()
                .map_err(|e| LuaError::external(format!("Error while parsing role id: {}", e)))?;

            Ok(
                lua_promise!(this, role, reason, |_lua, this, role, reason|, {
                    this.check_action("role".to_string())?;

                    let lockdown_id = this.lockdown_provider.role(role, reason).await.map_err(|e| {
                        LuaError::external(format!("Error while creating lockdown: {}", e))
                    })?;
    
                    Ok(lockdown_id.to_string())
                }),
            )
        });

        methods.add_method("remove", |_, this, id: String| {
            let id: uuid::Uuid = id.parse().map_err(|e| {
                LuaError::external(format!("Error while parsing lockdown id: {}", e))
            })?;

            Ok(lua_promise!(this, id, |_lua, this, id|, {
                this.check_action("remove".to_string())?;

                this.lockdown_provider.remove(id).await.map_err(|e| {
                    LuaError::external(format!("Error while removing lockdown: {}", e))
                })?;
                
                Ok(())
            }))
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(|_, (token, scope): (TemplateContextRef<T>, Option<String>)| {
            let scope = ExecutorScope::scope_str(scope)?;
            let Some(lockdown_provider) = token.context.lockdown_provider(scope) else {
                return Err(LuaError::external(
                    "The lockdown plugin is not supported in this context",
                ));
            };
            
            let executor = LockdownExecutor {
                context: token.context.clone(),
                lockdown_provider,
            };

            Ok(executor)
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
