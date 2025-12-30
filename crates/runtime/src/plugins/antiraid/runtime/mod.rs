use std::rc::Rc;

use crate::traits::context::{KhronosContext, Limitations};
use crate::traits::runtimeprovider::RuntimeProvider;
use crate::TemplateContext;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;
use crate::traits::ir::runtime as runtime_ir;

 /// An runtime executor is used to perform basic 'runtime' operations from Lua
/// templates
pub struct RuntimeExecutor<T: KhronosContext> {
    limitations: Rc<Limitations>,
    runtime_provider: T::RuntimeProvider,
}

impl<T: KhronosContext> RuntimeExecutor<T> {
    pub fn check(
        &self,
        action: &str,
    ) -> Result<(), crate::Error> {
        if self.limitations.has_cap("runtime:*") // runtime:* means all runtime operations are allowed
        || self.limitations.has_cap(&format!("runtime:{action}")) // runtime:{action} means that a specific action is allowed
        {
            self.runtime_provider.attempt_action(&action)?; // Check rate limits
            return Ok(());
        }

        Err(format!(
            "runtime operation `{action}` not allowed in this template context",
        )
        .into())
    }
}

impl<T: KhronosContext> LuaUserData for RuntimeExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "RuntimeExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Type, |_, _this, _: ()| Ok("RuntimeExecutor"));
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, _: ()| Ok("RuntimeExecutor"));
        
        methods.add_method("getexposedvfs", |_lua, this, _: ()| {
            this.check("getexposedvfs").map_err(|x| LuaError::external(x.to_string()))?;
            let vfs_map = this.runtime_provider.get_exposed_vfs().map_err(|x| LuaError::external(x.to_string()))?;
            Ok(vfs_map)
        });

        methods.add_scheduler_async_method("gettenantstate", async |_lua, this, _: ()| {
            this.check("gettenantstate").map_err(|x| LuaError::external(x.to_string()))?;
            let state = this.runtime_provider.get_tenant_state().await.map_err(|x| LuaError::external(x.to_string()))?;
            Ok(state)
        });

        methods.add_scheduler_async_method("settenantstate", async |_lua, this, state: runtime_ir::TenantState| {
            this.check("settenantstate").map_err(|x| LuaError::external(x.to_string()))?;
            this.runtime_provider.set_tenant_state(state).await.map_err(|x| LuaError::external(x.to_string()))?;
            Ok(())
        });

        methods.add_scheduler_async_method("stats", async |_lua, this, _: ()| {
            this.check("stats").map_err(|x| LuaError::external(x.to_string()))?;
            let stats = this.runtime_provider.stats().await.map_err(|x| LuaError::external(x.to_string()))?;
            Ok(stats)
        });

        methods.add_method("links", |_lua, this, _: ()| {
            this.check("links").map_err(|x| LuaError::external(x.to_string()))?;
            let links = this.runtime_provider.links().map_err(|x| LuaError::external(x.to_string()))?;
            Ok(links)
        });

        methods.add_method("eventlist", |_lua, this, _: ()| {
            this.check("eventlist").map_err(|x| LuaError::external(x.to_string()))?;
            let events = this.runtime_provider.event_list().map_err(|x| LuaError::external(x.to_string()))?;
            Ok(events)
        });
    }
}

pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaValue> {
    let Some(runtime_provider) = token.context.runtime_provider() else {
        return Err(LuaError::external(
            "The runtime plugin is not supported in this context",
        ));
    };
    let executor = RuntimeExecutor::<T> {
        limitations: token.limitations.clone(),
        runtime_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}