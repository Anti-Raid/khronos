use crate::traits::context::{KhronosContext};
use crate::traits::globalkvprovider::GlobalKVProvider;
use crate::TemplateContext;
use crate::traits::ir::globalkv::CreateGlobalKv;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// An kv executor is used to execute key-value ops from Lua
/// templates
pub struct GlobalKvExecutor<T: KhronosContext> {
    global_kv_provider: T::GlobalKVProvider,
}

impl<T: KhronosContext> GlobalKvExecutor<T> {
    pub fn check(
        &self,
        scope: &str
    ) -> Result<(), crate::Error> {
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

        methods.add_scheduler_async_method("find", async move |_, this, (query, scope): (String, String)| {
            this.check("find")
                .map_err(|e| LuaError::runtime(e.to_string()))?;

            let keys = this
                .global_kv_provider
                .find(scope, query)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(keys)
        });

        methods.add_scheduler_async_method(
            "get",
            async move |_, this, (key, version, scope): (String, i32, String)| {
                this.check("get")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                let record = this
                    .global_kv_provider
                    .get(key, version, scope)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(record)
            },
        );

        methods.add_scheduler_async_method(
            "create",
            async move |_lua, this, entry: CreateGlobalKv| {
                this.check("create")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.global_kv_provider
                    .create(entry)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            },
        );

        methods.add_scheduler_async_method(
            "delete",
            async move |_, this, (key, version, scope): (String, i32, String)| {
                this.check("delete")
                    .map_err(|e| LuaError::runtime(e.to_string()))?;

                this.global_kv_provider
                    .delete(key, version, scope)
                    .await
                    .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
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
        global_kv_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}
