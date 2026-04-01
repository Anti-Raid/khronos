use crate::traits::context::KhronosContext;
use mluau::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::plugins::antiraid;

#[derive(Clone)]
pub struct TemplateContext<T: KhronosContext> {
    pub context: T,

    /// Store table
    store_table: LuaTable,

    /// Cached plugin data
    pub(crate) cached_plugin_data: Rc<RefCell<HashMap<String, LuaValue>>>,
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
            cached_plugin_data: Rc::default(), // Safety note: the cached plugin data must be reset for subcontexts to avoid privilege escalation across subcontexts
        })
    }

    /// Gets a plugin from cache or runs 'f' to get it
    fn get_plugin<F, V>(&self, lua: &Lua, plugin_name: &str, f: F) -> LuaResult<LuaValue>
    where
        F: FnOnce(&Lua, &Self) -> LuaResult<V>,
        V: IntoLua,
    {
        let mut cached_plugin_data = self
            .cached_plugin_data
            .try_borrow_mut()
            .map_err(|e| LuaError::external(e.to_string()))?;

        if let Some(v) = cached_plugin_data.get(plugin_name) {
            return Ok(v.clone());
        }

        let v = f(lua, self)?.into_lua(lua)?;

        cached_plugin_data.insert(plugin_name.to_string(), v.clone());

        Ok(v)
    }
}

impl<T: KhronosContext> LuaUserData for TemplateContext<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // Plugins
        fields.add_field_method_get("Runtime", |lua, this| {
            this.get_plugin(lua, "Runtime", antiraid::runtime::init_plugin)
        });

        // Fields
        fields.add_field_method_get("store", |_, this| Ok(this.store_table.clone()));

        fields.add_field_method_get("memory_limit", |_lua, this| Ok(this.context.memory_limit()));

        fields.add_meta_field(LuaMetaMethod::Type, "TemplateContext".to_string());
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
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
