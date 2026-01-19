use std::rc::Rc;

use crate::traits::context::{KhronosContext, Limitations};
use crate::traits::globalkvprovider::GlobalKVProvider;
use crate::TemplateContext;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// An kv executor is used to execute key-value ops from Lua
/// templates
pub struct GlobalKvExecutor<T: KhronosContext> {
    limitations: Rc<Limitations>,
    global_kv_provider: T::GlobalKVProvider,
}

impl<T: KhronosContext> GlobalKvExecutor<T> {
    pub fn check(
        &self,
        scope: &str
    ) -> Result<(), crate::Error> {
        if !self.limitations.has_cap(format!("globalkv:access:{scope}").as_str()) {
            return Err(format!(
                "Global KV operation not allowed in this template context for scope '{scope}'",
            )
            .into());
        }
        self.global_kv_provider.attempt_action(scope)?; // Check rate limits
        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for GlobalKvExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "GlobalKvExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, _: ()| Ok("GlobalKvExecutor"));

        methods.add_scheduler_async_method("list", async move |_, this, _g: ()| {
            this.check("list")
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let keys = this
                .global_kv_provider
                .list()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(keys)
        });

        methods.add_scheduler_async_method(
            "get",
            async move |_, this, (key, version): (String, i32)| {
                this.check("get")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let record = this
                    .global_kv_provider
                    .get(key, version)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(record)
            },
        );
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaValue> {
    let Some(global_kv_provider) = token.context.global_kv_provider() else {
        return Err(LuaError::external(
            "The global key-value plugin is not supported in this context",
        ));
    };
    let executor = GlobalKvExecutor::<T> {
        limitations: token.limitations.clone(),
        global_kv_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}
