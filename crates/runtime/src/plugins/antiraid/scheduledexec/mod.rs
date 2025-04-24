mod types;

use mlua::prelude::*;
use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use crate::lua_promise;
use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::traits::scheduledexecprovider::ScheduledExecProvider;
use crate::TemplateContextRef;

#[derive(Clone)]
/// An scheduled execution executor is used to manage scheduled executions
pub struct ScheduledExecExecutor<T: KhronosContext> {
    context: T,
    scheduled_exec_provider: T::ScheduledExecProvider,
}

impl<T: KhronosContext> ScheduledExecExecutor<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("scheduledexec:{}", action)) {
            return Err(LuaError::runtime(
                "User info action is not allowed in this template context",
            ));
        }

        self.scheduled_exec_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for ScheduledExecExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "ScheduledExecExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("list", |_, this, (id,): (Option<String>,)| {
            Ok(lua_promise!(this, id, |lua, this, id|, {
                this.check_action("list".to_string())?;

                let execs = this.scheduled_exec_provider.list(id).await
                .map_err(|e| LuaError::external(e.to_string()))?
                .into_iter()
                .map(|x| types::ScheduledExecution {
                    id: x.id,
                    template_name: x.template_name,
                    data: x.data,
                    run_at: x.run_at
                })
                .collect::<Vec<_>>();

                lua.to_value_with(&execs, LUA_SERIALIZE_OPTIONS)
            }))
        });

        methods.add_method("add", |_, this, (data,): (LuaValue,)| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let cse = lua.from_value::<types::CreateScheduledExecution>(data)?;

                this.check_action("add".to_string())?;

                this.scheduled_exec_provider.add(
                    crate::traits::ir::ScheduledExecution {
                        template_name: this.context.template_name(),
                        id: cse.id,
                        data: cse.data,
                        run_at: cse.run_at,
                    }
                ).await
                .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        methods.add_method("remove", |_, this, id: String| {
            Ok(lua_promise!(this, id, |_lua, this, id|, {
                this.check_action("remove".to_string())?;

                this.scheduled_exec_provider.remove(id).await
                .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<ScheduledExecExecutor<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Methods
                    "list",
                    "add",
                    "remove"
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
            |_, token: TemplateContextRef<T>| {
                let Some(scheduled_exec_provider) = token.context.scheduled_exec_provider() else {
                    return Err(LuaError::external(
                        "The scheduledexec plugin is not supported in this context",
                    ));
                };

                let executor = ScheduledExecExecutor {
                    context: token.context.clone(),
                    scheduled_exec_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true);
    Ok(module)
}
