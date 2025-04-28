#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::utils::require_v2::AssetRequirer;
use rand::distributions::DistString;

use crate::primitives::event::Event;
use crate::traits::context::KhronosContext as KhronosContextTrait;
use crate::utils::pluginholder::PluginSet;
use crate::utils::prelude::setup_prelude;
use crate::utils::proxyglobal::proxy_global;
use crate::TemplateContext;
use crate::utils::require_v2::FilesystemWrapper;

use super::runtime::KhronosRuntime;
use mlua::prelude::*;
use mlua_scheduler_ext::traits::IntoLuaThread;
use rand::distributions::Alphanumeric;

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
pub struct KhronosIsolate {
    /// The inner khronos context for the isolate
    inner: KhronosRuntime,

    /// Isolate id
    id: String,

    /// The asset manager for the isolate
    asset_manager: FilesystemWrapper,

    /// The asset requirer for the isolate
    asset_requirer: AssetRequirer,

    /// The internal bytecode cache for the isolate
    ///
    /// Users should AVOID using this directly. It is used internally by the isolate to cache
    /// repeatedly used scripts in bytecode form to avoid unneeded recompilation.
    bytecode_cache: Rc<BytecodeCache>,

    /// A handle to this runtime's global table
    global_table: LuaTable,
}

impl KhronosIsolate {
    pub fn new_isolate<T: vfs::FileSystem + 'static>(
        inner: KhronosRuntime,
        asset_manager: T,
    ) -> Result<Self, LuaError> {
        if inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must not sandboxed before creating an isolate".to_string(),
            ));
        }

        let am = FilesystemWrapper::new(asset_manager);
        let mut isolate = Self::new(inner, am, false)?;

        isolate.inner_mut().sandbox()?; // Sandbox the runtime

        Ok(isolate)
    }

    pub fn new_subisolate<T: vfs::FileSystem + 'static>(
        inner: KhronosRuntime,
        asset_manager: T,
    ) -> Result<Self, LuaError> {
        if !inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must be sandboxed before creating an subisolate".to_string(),
            ));
        }

        let am = FilesystemWrapper::new(asset_manager);
        Self::new(inner, am, true)
    }

    /// Helper method to make the core isolate
    fn new(
        inner: KhronosRuntime,
        asset_manager: FilesystemWrapper,
        is_subisolate: bool,
    ) -> Result<Self, LuaError> {
        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let global_table = proxy_global(inner.lua())?;

        let controller = AssetRequirer::new(
            asset_manager.clone(),
            id.clone(),
            global_table.clone()
        );

        if is_subisolate {
            global_table.set(
                "require",
                inner.lua().create_require_function(controller.clone())?,
            )?;
        } else {
            inner.lua().globals().set(
                "require",
                inner.lua().create_require_function(controller.clone())?,
            )?;
        }

        setup_prelude(inner.lua(), global_table.clone())?;

        Ok(Self {
            id,
            asset_manager,
            asset_requirer: controller,
            inner,
            global_table,
            bytecode_cache: Rc::new(BytecodeCache::new()),
        })
    }

    /// Returns the asset manager for the isolate. Note that the asset manager cannot be changed
    /// after the isolate is created
    #[inline]
    pub fn asset_manager(&self) -> &FilesystemWrapper {
        &self.asset_manager
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

    /// Returns the asset requirer for the isolate
    #[inline]
    pub fn asset_requirer(&self) -> &AssetRequirer {
        &self.asset_requirer
    }

    /// Returns the id of the isolate
    #[inline]
    pub fn id(&self) -> &str {
        &self.id
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
        let code = String::from_utf8(code)
            .map_err(|e| LuaError::RuntimeError(format!("Failed to decode asset '{}': {}", path, e)))?;
        self.spawn_script(cache_key, path, &code, args)
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

    pub fn into_serde_json_value(
        self,
        isolate: &KhronosIsolate,
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

/// Workaround to a mlua bug where storing a mlua::Lua in a userdata
/// leads to a segfault when the userdata is dropped
#[derive(Clone)]
pub struct RuntimeShareableData {
    /// Global table
    pub global_table: LuaTable,
    /// Store table
    pub store_table: LuaTable,
}