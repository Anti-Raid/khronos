use crate::traits::context::KhronosContext;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

#[derive(Clone)]
pub struct TemplateContext<T: KhronosContext> {
    pub context: T,

    /// Store table
    store_table: LuaTable,
}

impl<T: KhronosContext> std::fmt::Debug for TemplateContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateContext")
            .field("store_table", &self.store_table)
            .finish()
    }
}

impl<T: KhronosContext> TemplateContext<T> {
    pub(crate) fn new(store_table: LuaTable, context: T) -> LuaResult<Self> {
        Ok(Self {
            context,
            store_table,
        })
    }
}

impl<T: KhronosContext> LuaUserData for TemplateContext<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // Fields
        fields.add_field_method_get("store", |_, this| Ok(this.store_table.clone()));

        fields.add_meta_field(LuaMetaMethod::Type, "TemplateContext".to_string());
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method("syscall", async |_lua, this, ops| {
            let state = this.context.syscall(ops).await.map_err(|x| LuaError::external(x.to_string()))?;
            Ok(state)
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, _, _: ()| Ok("TemplateContext"));
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}
