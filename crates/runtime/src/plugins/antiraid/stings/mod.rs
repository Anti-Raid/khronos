mod types;
use crate::{
    traits::{context::KhronosContext, stingprovider::StingProvider},
    utils::executorscope::ExecutorScope,
    TemplateContextRef,
};
use mlua::prelude::*;
use types::{Sting, StingAggregateSet, StingCreate};

use crate::lua_promise;

/// An sting executor is used to execute actions related to stings from Lua
/// templates
#[derive(Clone)]
pub struct StingExecutor<T: KhronosContext> {
    context: T,
    sting_provider: T::StingProvider,
}

impl<T: KhronosContext> StingExecutor<T> {
    pub fn check_action(&self, action: String) -> Result<(), crate::Error> {
        if !self.context.has_cap(&format!("sting:{}", action)) {
            return Err("Sting operation not allowed in this template context".into());
        }

        self.sting_provider.attempt_action(&action)?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for StingExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "StingExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("list", |_, this, page: usize| {
            Ok(lua_promise!(this, page, |lua, this, page|, {
                this.check_action("list".to_string())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let stings = this.sting_provider.list(page).await.map_err(|e|
                    mlua::Error::external(format!("Failed to list stings: {}", e))
                )?
                .into_iter()
                .map(types::Sting::from)
                .collect::<Vec<_>>();

                let v = lua.to_value(&stings)?;

                Ok(v)
            }))
        });

        methods.add_method("get", |_, this, id: String| {
            let id = uuid::Uuid::parse_str(&id).map_err(|e| LuaError::FromLuaConversionError {
                from: "string",
                to: "uuid".to_string(),
                message: Some(e.to_string()),
            })?;

            Ok(lua_promise!(this, id, |lua, this, id|, {
                this.check_action("get".to_string())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let sting = this.sting_provider.get(id).await.map_err(|e|
                    mlua::Error::external(format!("Failed to get sting: {}", e))
                )?
                .map(types::Sting::from);

                let v = lua.to_value(&sting)?;

                Ok(v)
            }))
        });

        methods.add_method("create", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let sting = lua.from_value::<StingCreate>(data)?;

                this.check_action("create".to_string())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let sting_id = this.sting_provider.create(sting.into()).await.map_err(|e|
                    mlua::Error::external(format!("Failed to create sting: {}", e))
                )?;

                Ok(sting_id.to_string())
            }))
        });

        methods.add_method("update", |_, this, data: LuaValue| {
            Ok(lua_promise!(this, data, |lua, this, data|, {
                let sting = lua.from_value::<Sting>(data)?;

                this.check_action("update".to_string())
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.sting_provider.update(sting.into()).await.map_err(|e|
                    mlua::Error::external(format!("Failed to update sting: {}", e))
                )?;

                Ok(())
            }))
        });

        methods.add_method("delete", |lua, this, id: LuaValue| {
            let id =
                lua.from_value::<uuid::Uuid>(id)
                    .map_err(|e| LuaError::FromLuaConversionError {
                        from: "string",
                        to: "uuid".to_string(),
                        message: Some(e.to_string()),
                    })?;

            Ok(lua_promise!(this, id, |_lua, this, id|, {
                this.check_action("delete".to_string())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.sting_provider.delete(id).await.map_err(|e|
                    mlua::Error::external(format!("Failed to delete sting: {}", e))
                )?;

                Ok(())
            }))
        });

        methods.add_method("guild_aggregate", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_action("guild_aggregate".to_string())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let stings = this.sting_provider.guild_aggregate().await.map_err(|e|
                    mlua::Error::external(format!("Failed to get guild aggregate for stings: {}", e))
                )?;

                Ok(StingAggregateSet {
                    aggregates: stings.into_iter().map(types::StingAggregate::from).collect(),
                })
            }))
        });

        methods.add_method("guild_user_aggregate", |_, this, user_id: String| {
            let user_id: serenity::all::UserId = user_id.parse().map_err(|e| {
                LuaError::external(format!("Invalid user id: {}", e))
            })?;
            Ok(lua_promise!(this, user_id, |_lua, this, user_id|, {
                this.check_action("guild_aggregate".to_string())
                .map_err(|e| LuaError::runtime(e.to_string()))?;

                let stings = this.sting_provider.guild_user_aggregate(user_id).await.map_err(|e|
                    mlua::Error::external(format!("Failed to get guild user aggregate for stings: {}", e))
                )?;

                Ok(StingAggregateSet {
                    aggregates: stings.into_iter().map(types::StingAggregate::from).collect(),
                })
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
                let Some(sting_provider) = token.context.sting_provider(scope) else {
                    return Err(LuaError::external(
                        "The stings plugin is not supported in this context",
                    ));
                };

                let executor = StingExecutor {
                    context: token.context.clone(),
                    sting_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
