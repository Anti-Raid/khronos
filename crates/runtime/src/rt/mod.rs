//! Single threaded khronos runtime struct/runner

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::primitives::event::Event;
use crate::traits::context::KhronosContext as KhronosContextTrait;
use crate::utils::proxyglobal::proxy_global;
use crate::utils::{assets::AssetManager as AssetManagerTrait, pluginholder::PluginSet};
use crate::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::TaskManager;
use mlua_scheduler_ext::traits::IntoLuaThread;
use mlua_scheduler_ext::Scheduler;

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

/// Auxillary options for the creation of a Khronos runtime
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct RuntimeCreateOpts {
    pub disable_scheduler_lib: bool,
    pub disable_task_lib: bool,
}

/// A struct representing the options for creating a Khronos runtime
pub struct KhronosRuntimeInterruptData {
    /// When the runtime last executed a script
    pub last_execution_time: Option<Instant>,
}

#[derive(Clone)]
/// A struct representing the inner VMs and structures used by Khronos.
pub struct KhronosRuntime {
    /// The lua vm itself
    lua: Lua,

    /// The lua compiler itself
    compiler: mlua::Compiler,

    /// The vm scheduler
    scheduler: Scheduler,

    /// Has the runtime been sandboxed
    sandboxed: bool,

    /// Is the runtime instance 'broken' or not
    broken: Rc<Cell<bool>>,

    /// The last time the VM executed a script
    last_execution_time: Rc<Cell<Option<Instant>>>,

    /// runtime creation options
    opts: RuntimeCreateOpts,
}

impl KhronosRuntime {
    /// Creates a new Khronos runtime from scratch
    ///
    /// Note that the resulting lua vm is *not* sandboxed until KhronosRuntime::sandbox() is called
    pub fn new<
        SF: mlua_scheduler::taskmgr::SchedulerFeedback + 'static,
        OnInterruptFunc: Fn(&Lua, KhronosRuntimeInterruptData) -> LuaResult<LuaVmState> + mlua::MaybeSend + 'static,
    >(
        sched_feedback: SF,
        opts: RuntimeCreateOpts,
        on_interrupt: Option<OnInterruptFunc>,
    ) -> Result<Self, LuaError> {
        let lua = Lua::new_with(
            LuaStdLib::ALL_SAFE,
            LuaOptions::new().catch_rust_panics(true),
        )?;

        let compiler = mlua::Compiler::new()
            .set_optimization_level(2)
            .set_type_info_level(1);

        lua.set_compiler(compiler.clone());

        let scheduler = Scheduler::new(TaskManager::new(
            lua.clone(),
            Rc::new(sched_feedback),
            Duration::from_millis(1),
        ));

        scheduler.attach();

        let scheduler_lib = mlua_scheduler::userdata::scheduler_lib(&lua)?;

        // Add in basic globals
        if !opts.disable_scheduler_lib {
            lua.globals().set("scheduler", scheduler_lib.clone())?;
        }

        if !opts.disable_task_lib {
            lua.globals().set(
                "task",
                mlua_scheduler::userdata::task_lib(&lua, scheduler_lib)?,
            )?;
        }

        mlua_scheduler::userdata::patch_coroutine_lib(&lua)?;

        let broken = Rc::new(Cell::new(false));
        let broken_ref = broken.clone();
        let last_execution_time = Rc::new(Cell::new(None));
        let last_execution_time_ref = last_execution_time.clone();

        if let Some(on_interrupt) = on_interrupt {
            lua.set_interrupt(move |lua| {
                let broken = broken_ref.get();
                if broken {
                    return Ok(LuaVmState::Yield);
                }

                on_interrupt(
                    lua,
                    KhronosRuntimeInterruptData {
                        last_execution_time: last_execution_time_ref.get(),
                    },
                )
            });
        } else {
            lua.set_interrupt(move |_lua| {
                let broken = broken_ref.get();
                if broken {
                    return Ok(LuaVmState::Yield);
                }

                Ok(LuaVmState::Continue)
            });
        }

        Ok(Self {
            lua,
            compiler,
            scheduler,
            sandboxed: false,
            broken,
            last_execution_time,
            opts,
        })
    }

    /// Returns the lua vm
    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    /// Returns the lua compiler being used
    pub fn compiler(&self) -> &mlua::Compiler {
        &self.compiler
    }

    /// Sets the lua compiler being used on both the lua vm and the runtime
    pub fn set_compiler(&mut self, compiler: mlua::Compiler) {
        self.lua.set_compiler(compiler.clone());
        self.compiler = compiler;
    }

    /// Returns the scheduler
    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    /// Returns the last execution time
    ///
    /// This may be None if the VM has not executed a script yet
    pub fn last_execution_time(&self) -> Option<Instant> {
        self.last_execution_time.get()
    }

    /// Updates the last execution time
    pub fn update_last_execution_time(&self, time: Instant) {
        self.last_execution_time.set(Some(time));
    }

    /// Returns whether the runtime is broken or not
    pub fn is_broken(&self) -> bool {
        self.broken.get()
    }

    /// Returns if the runtime is sandboxed or not
    pub fn is_sandboxed(&self) -> bool {
        self.sandboxed
    }

    /// Returns the runtime creation options
    pub fn opts(&self) -> &RuntimeCreateOpts {
        &self.opts
    }

    /// Sets the runtime to be broken
    pub fn mark_broken(&self, broken: bool) {
        self.broken.set(broken);
    }

    /// Sandboxes the VM after all extra needed setup has been performed. Note that KhronosIsolates
    /// cannot be created until this is called.
    pub fn sandbox(&mut self) -> Result<(), LuaError> {
        if self.sandboxed {
            return Ok(());
        }

        self.lua.sandbox(true)?;
        self.lua.globals().set_readonly(true);
        self.sandboxed = true;
        Ok(())
    }
}

/// A struct representing a Khronos context
pub struct KhronosIsolate<AssetManager: AssetManagerTrait + Clone> {
    /// The inner khronos context for the isolate
    inner: KhronosRuntime,

    /// The plugin set for the isolate
    plugin_set: PluginSet,

    /// The asset manager for the isolate
    asset_manager: AssetManager,

    /// The internal bytecode cache for the isolate
    ///
    /// Users should AVOID using this directly. It is used internally by the isolate to cache
    /// repeatedly used scripts in bytecode form to avoid unneeded recompilation.
    bytecode_cache: Rc<BytecodeCache>,

    /// A handle to this runtime's global table
    global_table: LuaTable,
}

impl<AssetManager: AssetManagerTrait + Clone> KhronosIsolate<AssetManager> {
    pub fn new(inner: KhronosRuntime, asset_manager: AssetManager) -> Result<Self, LuaError> {
        if !inner.is_sandboxed() {
            return Err(LuaError::RuntimeError(
                "Khronos runtime must be sandboxed before creating an isolate".to_string(),
            ));
        }

        let lua = inner.lua();
        let global_table = proxy_global(lua)?;

        Ok(Self {
            inner,
            plugin_set: PluginSet::new(),
            asset_manager,
            global_table,
            bytecode_cache: Rc::new(BytecodeCache::new()),
        })
    }

    /// Returns the asset manager for the isolate
    #[inline]
    pub fn asset_manager(&self) -> &AssetManager {
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

    /// Returns the plugin set for the isolate
    #[inline]
    pub fn plugin_set(&self) -> &PluginSet {
        &self.plugin_set
    }

    /// Returns the plugin set for the isolate in mutable form
    ///
    /// This is useful to load the default plugins into the plugin set if you want to
    #[inline]
    pub fn plugin_set_mut(&mut self) -> &mut PluginSet {
        &mut self.plugin_set
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

    /// Runs a script from the asset manager
    /// with the given KhronosContext and Event primitives
    #[inline]
    pub async fn spawn_asset<K: KhronosContextTrait>(
        &self,
        path: &str,
        context: TemplateContext<K>,
        event: Event,
    ) -> Result<SpawnResult, LuaError> {
        let args = match (event, context).into_lua_multi(&self.inner.lua) {
            Ok(f) => f,
            Err(e) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.inner.mark_broken(true)
                }

                return Err(e);
            }
        };

        self.spawn_asset_with_args(path, args).await
    }

    /// Runs a script from the asset manager
    #[inline]
    pub async fn spawn_asset_with_args(
        &self,
        path: &str,
        args: LuaMultiValue,
    ) -> Result<SpawnResult, LuaError> {
        let code = self
            .asset_manager
            .get_file(path)
            .map_err(|_| LuaError::RuntimeError(format!("Failed to load asset: {}", path)))?;
        self.spawn_script(path, &code, args).await
    }

    /// Runs a script, returning the result as a LuaMultiValue
    ///
    /// Note that the bytecode is cached by-name. Use KhronosRuntimeInner::remove_bytecode_cache
    /// to remove a script from the cache.
    ///
    /// Note 2: You probably want spawn_asset or spawn_asset_with_args instead of this
    pub async fn spawn_script(
        &self,
        name: &str,
        code: &str,
        args: LuaMultiValue,
    ) -> LuaResult<SpawnResult> {
        let thread = {
            let mut cache = self.bytecode_cache.inner().borrow_mut();
            let bytecode = if let Some(bytecode) = cache.get(name) {
                Cow::Borrowed(bytecode)
            } else {
                let compiler = self.inner.compiler();
                let bytecode = Rc::new(compiler.compile(code)?);
                cache.insert(name.to_string(), bytecode.clone());
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
            .last_execution_time
            .set(Some(std::time::Instant::now()));

        let res = self
            .inner
            .scheduler
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
