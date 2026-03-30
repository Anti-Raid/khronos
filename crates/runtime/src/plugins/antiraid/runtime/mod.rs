use crate::traits::context::{KhronosContext};
use crate::traits::runtimeprovider::RuntimeProvider;
use crate::TemplateContext;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// An runtime executor is used to perform basic 'runtime' operations from Lua
/// templates
pub struct RuntimeExecutor<T: KhronosContext> {
    runtime_provider: T::RuntimeProvider,
}

impl<T: KhronosContext> RuntimeExecutor<T> {
    pub fn check(
        &self,
        action: &str,
    ) -> Result<(), crate::Error> {
        self.runtime_provider.attempt_action(action)?;
        Ok(())
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

        methods.add_scheduler_async_method("syscall", async |_lua, this, ops| {
            this.check("syscall").map_err(|x| LuaError::external(x.to_string()))?;
            let state = this.runtime_provider.syscall(ops).await.map_err(|x| LuaError::external(x.to_string()))?;
            Ok(state)
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
        runtime_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}