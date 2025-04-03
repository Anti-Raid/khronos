#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::primitives::event::Event;
use crate::traits::context::KhronosContext as KhronosContextTrait;
use crate::utils::assets::AssetManager as AssetManagerTrait;
use crate::utils::pluginholder::PluginSet;
use crate::utils::prelude::setup_prelude;
use crate::utils::proxyglobal::proxy_global;
use crate::utils::require::{create_require_function, RequireController};
use crate::TemplateContext;

use super::runtime::KhronosRuntime;
use mlua::prelude::*;
use mlua_scheduler_ext::traits::IntoLuaThread;

/// A bytecode cacher for Luau scripts
///
/// Note that it is assumed for BytecodeCache to be uniquely made per runtime instance
/// and that the bytecode is not shared between runtimes
pub struct BytecodeCache(RefCell<std::collections::HashMap<String, Rc<Vec<u8>>>>);

impl Default for BytecodeCache {
    fn default() -> Self {
        Self::new()
    }
}

impl BytecodeCache {
    /// Create a new bytecode cache
    pub fn new() -> Self {
        BytecodeCache(RefCell::new(std::collections::HashMap::new()))
    }

    /// Returns the inner cache
    pub fn inner(&self) -> &RefCell<std::collections::HashMap<String, Rc<Vec<u8>>>> {
        &self.0
    }

    /// Clear the bytecode cache
    pub fn clear_bytecode_cache(&self) {
        self.inner().borrow_mut().clear();
    }

    /// Removes a script from the bytecode cache by name
    pub fn remove_bytecode_cache(&self, name: &str) {
        self.inner().borrow_mut().remove(name);
    }
}

/// A struct representing a Khronos isolate
///
/// An isolate allows for running scripts in a separate environment from the main runtime
/// (although not entirely seperated as they share the same lua vm)
///
/// There are two specializations/flavors of isolates:
/// - Isolates: These isolates are used when you only have a single common plugin set for
///   multiple scripts running on the isolate. The require function is read-only
/// - Subisolates: These isolates use a writable per-isolate require function and are useful
///   for running one-off scripts with different plugins available to them
///
/// Isolates are cheap to clone
/// 
/// Note: it is considered unsafe to store an Isolate in any Lua userdata
/// due to the potential possibility of mlua bugs occurring
#[derive(Clone)]
pub struct KhronosIsolate<AssetManager: AssetManagerTrait + Clone + 'static> {
    /// The inner khronos context for the isolate
    inner: KhronosRuntime,

    /// The plugin set for the isolate
    plugin_set: Rc<PluginSet>,

    /// The asset manager for the isolate
    asset_manager: Rc<AssetManager>,

    /// The internal bytecode cache for the isolate
    ///
    /// Users should AVOID using this directly. It is used internally by the isolate to cache
    /// repeatedly used scripts in bytecode form to avoid unneeded recompilation.
    bytecode_cache: Rc<BytecodeCache>,

    /// A handle to this runtime's global table
    global_table: LuaTable,

    /// A handle to this isolates require controller
    require: Option<Rc<IsolateRequireController<AssetManager>>>,
}

impl<AssetManager: AssetManagerTrait + Clone + 'static> KhronosIsolate<AssetManager> {
    pub fn new_isolate(
        inner: KhronosRuntime,
        asset_manager: AssetManager,
        plugin_set: PluginSet,
    ) -> Result<Self, LuaError> {
        if inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must not sandboxed before creating an isolate".to_string(),
            ));
        }

        let (mut isolate, controller_ref) = Self::new(inner, asset_manager, plugin_set)?;
        isolate.lua().globals().set(
            "require",
            create_require_function(isolate.lua(), controller_ref)?,
        )?;

        setup_prelude(isolate.lua(), isolate.global_table.clone())?;

        isolate.inner_mut().sandbox()?; // Sandbox the runtime

        Ok(isolate)
    }

    pub fn new_subisolate(
        inner: KhronosRuntime,
        asset_manager: AssetManager,
        plugin_set: PluginSet,
    ) -> Result<Self, LuaError> {
        if !inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must be sandboxed before creating an subisolate".to_string(),
            ));
        }

        let (isolate, controller_ref) = Self::new(inner, asset_manager, plugin_set)?;

        isolate.global_table.set(
            "require",
            create_require_function(isolate.lua(), controller_ref)?,
        )?;

        setup_prelude(isolate.lua(), isolate.global_table.clone())?;

        Ok(isolate)
    }

    /// Helper method to make the core isolate without any specialization
    fn new(
        inner: KhronosRuntime,
        asset_manager: AssetManager,
        plugin_set: PluginSet,
    ) -> Result<(Self, Rc<IsolateRequireController<AssetManager>>), LuaError> {
        let global_table = proxy_global(inner.lua())?;

        let plugin_set = Rc::new(plugin_set);

        let mut isolate = Self {
            inner,
            plugin_set,
            asset_manager: Rc::new(asset_manager),
            global_table: global_table.clone(),
            bytecode_cache: Rc::new(BytecodeCache::new()),
            require: None,
        };

        let controller = Rc::new(IsolateRequireController::new(isolate.clone()));
        isolate.require = Some(controller.clone());

        // Convert plugin set to builtins table

        Ok((isolate, controller))
    }

    /// Returns the asset manager for the isolate
    #[inline]
    pub fn asset_manager(&self) -> &AssetManager {
        &self.asset_manager
    }

    /// Sets a new asset manager for the isolate
    #[inline]
    pub fn set_asset_manager(&mut self, asset_manager: AssetManager) {
        self.asset_manager = Rc::new(asset_manager);
    }

    /// Returns the lua vm for the isolate
    #[inline]
    pub fn lua(&self) -> &Lua {
        self.inner.lua()
    }

    /// Returns the inner khronos runtime for the isolate
    #[inline]
    pub fn inner(&self) -> &KhronosRuntime {
        &self.inner
    }

    /// Returns the inner khronos runtime for the isolate in mutable form
    #[inline]
    pub fn inner_mut(&mut self) -> &mut KhronosRuntime {
        &mut self.inner
    }

    /// Returns the plugin set for the isolate
    ///
    /// This is a reference to the Rc'd plugin set, not a clone
    #[inline]
    pub fn plugin_set(&self) -> &PluginSet {
        &self.plugin_set
    }

    /// Returns the global table for the isolate
    #[inline]
    pub fn global_table(&self) -> &LuaTable {
        &self.global_table
    }

    /// Returns the bytecode cache for the isolate
    ///
    /// Note that due to the Rc, it is not possible to access the BytecodeCache in mutable form
    /// and nor is this useful (as the cache has no mutable methods)
    pub fn bytecode_cache(&self) -> &BytecodeCache {
        &self.bytecode_cache
    }

    /// Sets a new bytecode cache for the isolate
    pub fn set_bytecode_cache(&mut self, cache: Rc<BytecodeCache>) {
        self.bytecode_cache = cache;
    }

    /// Returns the require controller for the isolate
    pub fn require(&self) -> Option<&IsolateRequireController<AssetManager>> {
        self.require.as_ref().map(|r| r.as_ref())
    }

    /// Creates a runtime shareable data object
    pub fn runtime_shareable_data(&self) -> RuntimeShareableData {
        RuntimeShareableData {
            global_table: self.global_table.clone(),
            store_table: self.inner.store_table().clone(),
        }
    }

    pub fn context_event_to_lua_multi<K: KhronosContextTrait>(
        &self,
        context: TemplateContext<K>,
        event: Event,
    ) -> Result<LuaMultiValue, LuaError> {
        match (event, context).into_lua_multi(self.inner.lua()) {
            Ok(f) => Ok(f),
            Err(e) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.inner.mark_broken(true)
                }

                Err(e)
            }
        }
    }

    /// Runs a script from the asset manager
    /// with the given KhronosContext and Event primitives
    pub async fn spawn_asset<K: KhronosContextTrait>(
        &self,
        cache_key: &str,
        path: &str,
        context: TemplateContext<K>,
        event: Event,
    ) -> Result<SpawnResult, LuaError> {
        let args = self.context_event_to_lua_multi(context, event)?;

        self.spawn_asset_with_args(cache_key, path, args).await
    }

    /// Runs a script from the asset manager
    pub async fn spawn_asset_with_args(
        &self,
        cache_key: &str,
        path: &str,
        args: LuaMultiValue,
    ) -> Result<SpawnResult, LuaError> {
        let code = self.asset_manager.get_file(path).map_err(|e| {
            LuaError::RuntimeError(format!("Failed to load asset '{}': {}", path, e))
        })?;
        self.spawn_script(cache_key, path, code.as_ref(), args)
            .await
    }

    /// Runs a script, returning the result as a LuaMultiValue
    ///
    /// Note that the bytecode is cached by-name. Use KhronosRuntimeInner::remove_bytecode_cache
    /// to remove a script from the cache.
    ///
    /// Note 2: You probably want spawn_asset or spawn_asset_with_args instead of this
    pub async fn spawn_script(
        &self,
        cache_key: &str,
        name: &str,
        code: &str,
        args: LuaMultiValue,
    ) -> LuaResult<SpawnResult> {
        let thread = {
            let mut cache = self.bytecode_cache.inner().borrow_mut();
            let bytecode = if let Some(bytecode) = cache.get(cache_key) {
                Cow::Borrowed(bytecode)
            } else {
                let compiler = self.inner.compiler();
                let bytecode = Rc::new(compiler.compile(code)?);
                cache.insert(cache_key.to_string(), bytecode.clone());
                Cow::Owned(bytecode)
            };

            //let bytecode = self.lua.load(script).set_name(name)?.dump()?;
            //cache.insert(name.to_string(), bytecode.clone());

            let lua = self.inner.lua();

            match lua
                .load(&**bytecode)
                .set_name(name)
                .set_mode(mlua::ChunkMode::Binary) // Ensure auto-detection never selects binary mode
                .set_environment(self.global_table.clone())
                .into_lua_thread(lua)
            {
                Ok(f) => f,
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.inner.mark_broken(true)
                    }

                    return Err(e);
                }
            }
        };

        // Update last_execution_time
        self.inner
            .update_last_execution_time(std::time::Instant::now());

        let res = self
            .inner
            .scheduler()
            .spawn_thread_and_wait("Exec", thread, args)
            .await?;

        // Now unwrap it
        let res = match res {
            Some(Ok(res)) => Some(res),
            Some(Err(e)) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.inner.mark_broken(true)
                }

                return Err(e);
            }
            None => None,
        };

        Ok(SpawnResult::new(res))
    }
}

/// The result from spawning a script from `KhronosIsolate::spawn_script` and other
/// spawning functions
///
/// This is a wrapper around the LuaMultiValue returned from the script to allow for
/// convenient conversion to serde_json::Value and LuaMultiValue without dealing with
/// scheduler implementation details
pub struct SpawnResult {
    result: Option<LuaMultiValue>,
}

impl SpawnResult {
    pub fn new(result: Option<LuaMultiValue>) -> Self {
        Self { result }
    }

    pub fn into_multi_value(self) -> LuaMultiValue {
        match self.result {
            Some(res) => res,
            None => LuaMultiValue::with_capacity(0),
        }
    }

    pub fn into_serde_json_value<AssetManager: AssetManagerTrait + Clone>(
        self,
        isolate: &KhronosIsolate<AssetManager>,
    ) -> LuaResult<serde_json::Value> {
        let Some(values) = self.result else {
            return Ok(serde_json::Value::Null);
        };

        match values.len() {
            0 => Ok(serde_json::Value::Null),
            1 => {
                let value = values.into_iter().next().unwrap();

                match isolate.lua().from_value::<serde_json::Value>(value) {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                        if let LuaError::MemoryError(_) = e {
                            // Mark VM as broken
                            isolate.inner().mark_broken(true)
                        }

                        Err(e)
                    }
                }
            }
            _ => {
                let mut arr = Vec::with_capacity(values.len());

                for v in values {
                    match isolate.lua().from_value::<serde_json::Value>(v) {
                        Ok(v) => arr.push(v),
                        Err(e) => {
                            // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                            if let LuaError::MemoryError(_) = e {
                                // Mark VM as broken
                                isolate.inner().mark_broken(true)
                            }

                            return Err(e);
                        }
                    }
                }

                Ok(serde_json::Value::Array(arr))
            }
        }
    }
}

#[derive(Clone)]
pub struct IsolateRequireController<T: AssetManagerTrait + Clone + 'static> {
    isolate: KhronosIsolate<T>,
    requires_cache: RefCell<HashMap<String, LuaMultiValue>>,
}

impl<T: AssetManagerTrait + Clone + 'static> IsolateRequireController<T> {
    pub fn new(isolate: KhronosIsolate<T>) -> Self {
        Self {
            isolate,
            requires_cache: RefCell::new(HashMap::new()),
        }
    }

    /// Returns the inner isolate
    pub fn inner(&self) -> &KhronosIsolate<T> {
        &self.isolate
    }

    /// Returns the require cache
    pub fn requires_cache(&self) -> &RefCell<std::collections::HashMap<String, LuaMultiValue>> {
        &self.requires_cache
    }

    /// Clears the require cache
    pub fn clear_require_cache(&self) {
        self.requires_cache.borrow_mut().clear();
    }
}

impl<T: AssetManagerTrait + Clone> RequireController for IsolateRequireController<T> {
    fn get_builtins(&self) -> Option<LuaResult<LuaTable>> {
        let tab = match self.isolate.lua().create_table() {
            Ok(tab) => tab,
            Err(e) => return Some(Err(e)),
        };

        for (name, plugin) in self.isolate.plugin_set.iter() {
            match tab.set(name.clone(), *plugin) {
                Ok(_) => {}
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.isolate.inner().mark_broken(true)
                    }

                    return Some(Err(e));
                }
            }
        }

        Some(Ok(tab))
    }

    fn get_builtin(&self, builtin: &str) -> Option<LuaResult<LuaMultiValue>> {
        if let Ok(table) = self.isolate.lua().globals().get::<LuaTable>(builtin) {
            return Some(table.into_lua_multi(self.isolate.lua()));
        }

        let tab = self
            .isolate
            .plugin_set
            .load_plugin(self.isolate.lua(), builtin)?;

        match tab {
            Ok(table) => Some(table.into_lua_multi(self.isolate.lua())),
            Err(e) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.isolate.inner().mark_broken(true)
                }

                None
            }
        }
    }

    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
        self.isolate.asset_manager.get_file(path)
    }

    fn get_cached(&self, path: &str) -> Option<LuaMultiValue> {
        self.requires_cache.borrow().get(path).cloned()
    }

    fn cache(&self, path: String, contents: LuaMultiValue) {
        self.requires_cache.borrow_mut().insert(path, contents);
    }

    fn global_table(&self) -> LuaTable {
        self.isolate.global_table().clone()
    }
}

/// Workaround to a mlua bug where storing a mlua::Lua in a userdata
/// leads to a segfault when the userdata is dropped
#[derive(Clone)]
pub struct RuntimeShareableData {
    /// Global table
    pub global_table: LuaTable,
    /// Store table
    pub store_table: LuaTable,
}