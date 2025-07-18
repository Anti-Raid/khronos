#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::cell::RefCell;
use std::rc::Rc;

use crate::primitives::event::Event;
use crate::require::{AssetRequirer, FilesystemWrapper};
use crate::traits::context::KhronosContext as KhronosContextTrait;
use crate::utils::khronos_value::KhronosValue;
use crate::utils::prelude::setup_prelude;
use crate::utils::proxyglobal::proxy_global;
use crate::TemplateContext;

use super::runtime::KhronosRuntime;
use mluau::prelude::*;
use rand::distr::{Alphanumeric, SampleString};

/// A bytecode cacher for Luau scripts
///
/// Note that it is assumed for BytecodeCache to be uniquely made per runtime instance
/// and that the bytecode is not shared between runtimes
struct FunctionCache(RefCell<std::collections::HashMap<String, LuaFunction>>);

impl Default for FunctionCache {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionCache {
    /// Create a new function cache
    pub fn new() -> Self {
        Self(RefCell::new(std::collections::HashMap::new()))
    }

    /// Returns the inner cache
    pub fn inner(&self) -> &RefCell<std::collections::HashMap<String, LuaFunction>> {
        &self.0
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

    /// The internal function_cache for the isolate
    function_cache: Rc<FunctionCache>,

    /// A handle to this runtime's global table
    global_table: LuaTable,
}

impl KhronosIsolate {
    pub fn new_isolate(
        inner: KhronosRuntime,
        asset_manager: FilesystemWrapper,
    ) -> Result<Self, LuaError> {
        if inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must not sandboxed before creating an isolate".to_string(),
            ));
        }

        let mut isolate = Self::new(inner, asset_manager, false)?;

        isolate.inner_mut().sandbox()?; // Sandbox the runtime

        Ok(isolate)
    }

    pub fn new_subisolate(
        inner: KhronosRuntime,
        asset_manager: FilesystemWrapper,
    ) -> Result<Self, LuaError> {
        if !inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must be sandboxed before creating an subisolate".to_string(),
            ));
        }

        Self::new(inner, asset_manager, true)
    }

    /// Helper method to make the core isolate
    fn new(
        inner: KhronosRuntime,
        asset_manager: FilesystemWrapper,
        is_subisolate: bool,
    ) -> Result<Self, LuaError> {
        log::debug!("Creating new isolate");
        let id = Alphanumeric.sample_string(&mut rand::rng(), 16);

        let global_table = {
            let Some(ref lua) = *inner.lua.borrow_mut() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            let global_table = proxy_global(lua)?;

            global_table.set("__kanalytics_memusageafterisolateproxy", lua.used_memory())?;

            let controller =
                AssetRequirer::new(asset_manager.clone(), id.clone(), global_table.clone());

            if is_subisolate {
                global_table.set("require", lua.create_require_function(controller)?)?;
            } else {
                lua.globals()
                    .set("require", lua.create_require_function(controller)?)?;
            }

            global_table.set(
                "__kanalytics_memusageafterrequirecreated",
                lua.used_memory(),
            )?;

            setup_prelude(lua, global_table.clone())?;

            global_table.set("__kanalytics_memusageafterprelude", lua.used_memory())?;

            global_table
        };

        Ok(Self {
            id,
            asset_manager,
            inner,
            global_table,
            function_cache: Rc::new(FunctionCache::new()),
        })
    }

    /// Returns the asset manager for the isolate. Note that the asset manager cannot be changed
    /// after the isolate is created
    #[inline]
    pub fn asset_manager(&self) -> &FilesystemWrapper {
        &self.asset_manager
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
    ///
    /// Returns `None` if the runtime is closed
    #[inline]
    pub fn global_table(&self) -> Option<&LuaTable> {
        if self.inner.is_closed() {
            return None;
        }
        Some(&self.global_table)
    }

    /// Returns the id of the isolate
    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Creates a new TemplateContext with the given KhronosContext
    pub fn create_context<K: KhronosContextTrait>(
        &self,
        context: K,
    ) -> Result<CreatedKhronosContext, LuaError> {
        let Some(ref lua) = *self.inner.lua.borrow_mut() else {
            return Err(LuaError::RuntimeError(
                "Lua instance is no longer valid".to_string(),
            ));
        };

        let context = TemplateContext::new(lua, context)?;

        match context.into_lua(lua) {
            Ok(f) => Ok(CreatedKhronosContext(f)),
            Err(e) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.inner
                        .mark_broken(true)
                        .map_err(|e| LuaError::external(e.to_string()))?;
                }

                Err(e)
            }
        }
        // Lua should be dropped here
    }

    /// Runs a script from the asset manager
    ///
    /// with the given KhronosContext and Event primitives
    pub async fn spawn_asset(
        &self,
        cache_key: &str,
        path: &str,
        context: CreatedKhronosContext,
        event: Event,
    ) -> Result<SpawnResult, LuaError> {
        let args = {
            let Some(ref lua) = *self.inner.lua.borrow_mut() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            match (event, context.0).into_lua_multi(lua) {
                Ok(f) => f,
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.inner
                            .mark_broken(true)
                            .map_err(|e| LuaError::external(e.to_string()))?;
                    }

                    return Err(e);
                }
            }
            // Lua should be dropped here
        };

        let code = self
            .asset_manager
            .get_file(path.to_string())
            .map_err(|e| LuaError::RuntimeError(format!("Failed to load asset '{path}': {e}")))?;
        let code = String::from_utf8(code)
            .map_err(|e| LuaError::RuntimeError(format!("Failed to decode asset '{path}': {e}")))?;
        self.spawn_script(cache_key, path, &code, args).await
    }

    /// Runs a script from the asset manager
    ///
    /// with the given KhronosContext and Event primitives
    pub async fn spawn_known_script(
        &self,
        cache_key: &str,
        name: &str,
        code: &str,
        context: CreatedKhronosContext,
        event: Event,
    ) -> Result<SpawnResult, LuaError> {
        let args = {
            let Some(ref lua) = *self.inner.lua.borrow_mut() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            match (event, context.0).into_lua_multi(lua) {
                Ok(f) => f,
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.inner
                            .mark_broken(true)
                            .map_err(|e| LuaError::external(e.to_string()))?;
                    }

                    return Err(e);
                }
            }
            // Lua should be dropped here
        };

        self.spawn_script(cache_key, name, code, args).await
    }

    // Internal method to actually spawn the script
    async fn spawn_script(
        &self,
        cache_key: &str,
        name: &str,
        code: &str,
        args: LuaMultiValue,
    ) -> LuaResult<SpawnResult> {
        {
            if !self.inner.scheduler().is_running() {
                log::info!("Scheduler not running, running it now");
                self.inner.scheduler().run_in_task();
                tokio::task::yield_now().await;
            }
        }

        let thread = {
            let Some(ref lua) = *self.inner.lua.borrow() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            let mut cache = self.function_cache.inner().borrow_mut();
            let f = if let Some(f) = cache.get(cache_key) {
                f.clone() // f is cheap to clone
            } else {
                let compiler = self.inner.compiler();
                let bytecode = compiler.compile(code)?;

                self.global_table
                    .set("__kanalytics_memusagebeforefn", lua.used_memory())?;

                let function = match lua
                    .load(&bytecode)
                    .set_name(name.to_string())
                    .set_mode(mluau::ChunkMode::Binary) // Ensure auto-detection never selects binary mode
                    .set_environment(self.global_table.clone())
                    .into_function()
                {
                    Ok(f) => f,
                    Err(e) => {
                        // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                        if let LuaError::MemoryError(_) = e {
                            // Mark VM as broken
                            self.inner
                                .mark_broken(true)
                                .map_err(|e| LuaError::external(e.to_string()))?;
                        }
                        return Err(e);
                    }
                };

                self.global_table
                    .set("__kanalytics_memusageafterfn", lua.used_memory())?;

                cache.insert(cache_key.to_string(), function.clone());
                function
            };

            match lua.create_thread(f) {
                Ok(f) => f,
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.inner
                            .mark_broken(true)
                            .map_err(|e| LuaError::external(e.to_string()))?;
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
            .spawn_thread_and_wait(thread, args)
            .await?;

        // Do a GC
        {
            let Some(ref lua) = *self.inner.lua.borrow() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            lua.gc_collect()?;
            lua.gc_collect()?; // Twice to ensure we get all the garbage
        }

        // Now unwrap it
        let res = match res {
            Some(Ok(res)) => Some(res),
            Some(Err(e)) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.inner
                        .mark_broken(true)
                        .map_err(|e| LuaError::external(e.to_string()))?;
                }

                return Err(e);
            }
            None => None,
        };

        Ok(SpawnResult::new(res))
    }

    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

#[derive(Clone, Debug)]
pub struct CreatedKhronosContext(LuaValue);

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
    pub(crate) fn new(result: Option<LuaMultiValue>) -> Self {
        Self { result }
    }

    pub fn into_multi_value(self) -> LuaMultiValue {
        match self.result {
            Some(res) => res,
            None => LuaMultiValue::with_capacity(0),
        }
    }

    /// Converts the result into a KhronosValue
    pub fn into_khronos_value(self, isolate: &KhronosIsolate) -> LuaResult<KhronosValue> {
        let Some(values) = self.result else {
            return Ok(KhronosValue::Null);
        };

        match values.len() {
            0 => Ok(KhronosValue::Null),
            1 => {
                let value = values.into_iter().next().unwrap();

                let result_value = {
                    let Some(ref lua) = *isolate.inner().lua.borrow() else {
                        return Err(LuaError::RuntimeError(
                            "Lua instance is no longer valid".to_string(),
                        ));
                    };

                    KhronosValue::from_lua(value, lua)
                }; // Lua should be dropped here

                match result_value {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                        if let LuaError::MemoryError(_) = e {
                            // Mark VM as broken
                            isolate
                                .inner()
                                .mark_broken(true)
                                .map_err(|e| LuaError::external(e.to_string()))?;
                        }

                        Err(e)
                    }
                }
            }
            _ => {
                let mut arr = Vec::with_capacity(values.len());

                for v in values {
                    let result_value = {
                        let Some(ref lua) = *isolate.inner().lua.borrow() else {
                            return Err(LuaError::RuntimeError(
                                "Lua instance is no longer valid".to_string(),
                            ));
                        };

                        KhronosValue::from_lua(v, lua)
                    }; // Lua should be dropped here

                    match result_value {
                        Ok(v) => arr.push(v),
                        Err(e) => {
                            // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                            if let LuaError::MemoryError(_) = e {
                                // Mark VM as broken
                                isolate
                                    .inner()
                                    .mark_broken(true)
                                    .map_err(|e| LuaError::external(e.to_string()))?;
                            }

                            return Err(e);
                        }
                    }
                }

                Ok(KhronosValue::List(arr))
            }
        }
    }
}
