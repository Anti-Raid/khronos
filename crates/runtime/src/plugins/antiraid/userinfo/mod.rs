use mlua::prelude::*;

use crate::lua_promise;
use crate::traits::context::KhronosContext;
use crate::traits::userinfoprovider::UserInfoProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::TemplateContextRef;

#[derive(Clone)]
/// An user info executor is used to fetch UserInfo's about users
pub struct UserInfoExecutor<T: KhronosContext> {
    context: T,
    userinfo_provider: T::UserInfoProvider,
}

// @userdata LockdownExecutor
//
// Executes actions on discord
impl<T: KhronosContext> UserInfoExecutor<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("userinfo:{}", action)) {
            return Err(LuaError::runtime(
                "User info action is not allowed in this template context",
            ));
        }

        self.userinfo_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for UserInfoExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "UserInfoExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |_, this, (user,): (String,)| {
            let user: serenity::all::UserId = user
                .parse()
                .map_err(|e| LuaError::external(format!("Error while parsing user id: {}", e)))?;

            Ok(lua_promise!(this, user, |lua, this, user|, {
                this.check_action("get".to_string())?;

                let userinfo = this.userinfo_provider.get(user).await
                .map_err(|e| LuaError::external(e.to_string()))?;

                let value = lua.to_value(&userinfo)?;

                Ok(value)
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
                let Some(userinfo_provider) = token.context.userinfo_provider(scope) else {
                    return Err(LuaError::external(
                        "The userinfo plugin is not supported in this context",
                    ));
                };

                let executor = UserInfoExecutor {
                    context: token.context.clone(),
                    userinfo_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true);
    Ok(module)
}
