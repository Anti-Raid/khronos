//! Single threaded khronos runtime struct/runner

#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Once;
use std::time::Instant;

use mlua_scheduler::taskmgr::{Hooks, SchedulerImpl};
use mluau::prelude::*;
use mluau_require::{AssetRequirer, FilesystemWrapper};

pub type S = mlua_scheduler::schedulers::rodan::CoreScheduler;

use crate::TemplateContext;
use crate::primitives::event::CreateEvent;
use crate::traits::context::KhronosContext as KhronosContextTrait;
use crate::utils::proxyglobal::proxy_global;

/// A function to be called when the Khronos runtime is marked as broken
pub type OnBrokenFunc = Box<dyn Fn()>;

/// Auxillary options for the creation of a Khronos runtime
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct RuntimeCreateOpts {
    pub disable_task_lib: bool,
    pub time_limit: Option<std::time::Duration>,
    pub give_time: std::time::Duration,
    //pub time_slice: Option<std::time::Duration>,
}


pub struct SchedulerHook {
    execution_stop_time: Rc<Cell<Option<std::time::Instant>>>,
    give_time: std::time::Duration,
}

impl Hooks for SchedulerHook {
    fn on_resume(&self, _thread: &mluau::Thread) {
        match self.execution_stop_time.get() {
            Some(curr_stop) => {
                // We need to give the thread some time to run

                // If current stopping time is less than now + give_time, meaning
                // the thread wouldn't be able to run for at least give_time,
                // extend the time a bit
                if curr_stop < Instant::now() + self.give_time {
                    // Extend the time a bit
                    self.execution_stop_time.set(Some(Instant::now() + self.give_time));
                }
            }
            None => {
                self.execution_stop_time.set(None);
            }
        }
    }
}

static FFLAG_SET_GLOBAL: Once = Once::new();

/// A struct representing the inner VMs and structures used by Khronos.
#[derive(Clone)]
pub struct KhronosRuntime {
    /// The lua vm itself
    ///
    /// Should not be publicly exposed. Instead, use create_custom_global or DataStore's
    pub(super) lua: Rc<RefCell<Option<Lua>>>,

    /// The lua compiler itself
    compiler: mluau::Compiler,

    /// The vm scheduler
    scheduler: S,

    /// Is the runtime instance 'broken' or not
    broken: Rc<Cell<bool>>,

    /// A function to be called if the runtime is marked as broken
    on_broken: Rc<RefCell<Option<OnBrokenFunc>>>,

    /// The last time the VM executed a script
    last_execution_time: Rc<Cell<Option<Instant>>>,

    /// The time limit for execution
    time_limit: Rc<Cell<Option<std::time::Duration>>>,

    /// The time the execution should stop at
    /// 
    /// Automatically calculated (usually) from time_limit and last_execution_time
    /// 
    /// Scheduler resumes may extend this time
    execution_stop_time: Rc<Cell<Option<Instant>>>,

    /// The time to allow a thread to run for before temporarily yielding it
    //time_slice: Rc<Cell<Option<std::time::Duration>>>,

    /// The shared store table for the runtime
    store_table: LuaTable,

    /// The base global table
    global_table: LuaTable,

    /// The proxy require function
    proxy_require: LuaFunction,

    /// runtime creation options
    opts: RuntimeCreateOpts,
}

impl KhronosRuntime {
    /// Creates a new Khronos runtime from scratch
    ///
    /// Note that the resulting lua vm is *not* sandboxed until KhronosRuntime::sandbox() is called
    pub fn new<
        ThreadCreationCallbackFunc: Fn(&Lua, LuaThread) -> Result<(), mluau::Error> + 'static,
        ThreadDestructionCallbackFunc: Fn(LuaLightUserData) + 'static,
        FS: mluau_require::vfs::FileSystem + 'static,
    >(
        opts: RuntimeCreateOpts,
        on_thread_event_callback: Option<(
            ThreadCreationCallbackFunc,
            ThreadDestructionCallbackFunc,
        )>,
        vfs: FS,
    ) -> Result<Self, LuaError> {
        log::debug!("Creating new Khronos runtime");
        
        // Allow <<>> syntax
        FFLAG_SET_GLOBAL.call_once(|| {
            // Allow <<>> syntax
            if let Err(e) = Lua::set_fflag("LuauExplicitTypeExpressionInstantiation", true) {
                log::warn!("Failed to enable LuauExplicitTypeExpressionInstantiation fflag: {:?}", e);
            }
        });

        let lua = Lua::new_with(
            LuaStdLib::ALL_SAFE,
            LuaOptions::new()
                .catch_rust_panics(true)
                .disable_error_userdata(true),
        )?;

        let compiler = mluau::Compiler::new()
            .set_optimization_level(2)
            .set_type_info_level(1);

        lua.set_compiler(compiler.clone());

        let time_limit = Rc::new(Cell::new(opts.time_limit));
        let execution_stop_time = match opts.time_limit {
            Some(limit) => Rc::new(Cell::new(Some(Instant::now() + limit))),
            None => Rc::new(Cell::new(None)),
        };
        let scheduler = S::setup(&lua, Rc::new(SchedulerHook {
            execution_stop_time: execution_stop_time.clone(),
            give_time: opts.give_time
        })).map_err(|e| LuaError::external(format!("Failed to create scheduler: {}", e)))?;

        if !opts.disable_task_lib {
            lua.globals()
                .set("task", mlua_scheduler::userdata::task_lib::<S>(&lua)?)?;
        }

        log::debug!("Khronos runtime created successfully");

        let broken = Rc::new(Cell::new(false));
        let broken_ref = broken.clone();
        let last_execution_time: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));
        //let time_slice = Rc::new(Cell::new(opts.time_slice));

        let execution_stop_time_ref = execution_stop_time.clone();
        //let time_slice_ref = time_slice.clone();
        lua.set_interrupt(move |_lua| {
            // If the runtime is broken, yield the lua vm immediately
            let broken = broken_ref.get();
            if broken {
                return Ok(LuaVmState::Yield);
            }
            
            if let Some(limit) = execution_stop_time_ref.get() {
                if Instant::now() > limit {
                    return Err(LuaError::RuntimeError(
                        "Script execution time limit exceeded".to_string(),
                    ));
                }
            }

            Ok(LuaVmState::Continue)
        });

        // Drop getfenv/setfenv (makes life more annoying and confusing with them present)
        lua.globals().set("getfenv", LuaValue::Nil)?;
        lua.globals().set("setfenv", LuaValue::Nil)?;

        // Ensure _G.print and _G.eprint are nil
        lua.globals().set("print", LuaValue::Nil)?;
        lua.globals().set("eprint", LuaValue::Nil)?;
        lua.globals().set("require", LuaValue::Nil)?;

        // Setup require function
        let global_table = proxy_global(&lua)?;
        let controller = AssetRequirer::new(FilesystemWrapper::new(vfs), "main".to_string(), global_table.clone());
        let require = lua.create_require_function(controller)?;
        global_table
            .set("require", require)?;

        let proxy_require = lua.load("return require(...)")
            .set_environment(global_table.clone())
            .set_name("/init.luau")
            .set_mode(mluau::ChunkMode::Text)
            .try_cache()
            .into_function()?;

        if let Some(on_thread_event_callback) = on_thread_event_callback {
            lua.set_thread_creation_callback(on_thread_event_callback.0);
            lua.set_thread_collection_callback(on_thread_event_callback.1);
        }

        // Now, sandbox the lua vm
        lua.sandbox(true)?;
        lua.globals().set_readonly(true);
        lua.globals().set_safeenv(true);

        // Create a store table
        let store_table = lua.create_table()?;

        // Load core modules
        lua.register_module(
            "@antiraid/channel",
            crate::core::channel::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@antiraid/datetime",
            crate::core::datetime::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@antiraid/interop",
            crate::core::interop::init_plugin(&lua)?,
        )?;
        lua.register_module("@antiraid/luau", crate::core::luau::init_plugin(&lua)?)?;
        lua.register_module(
            "@antiraid/datamgmt",
            crate::core::datamgmt::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@antiraid/typesext",
            crate::core::typesext::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@lune/datetime",
            crate::plugins::lune::datetime::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@lune/regex",
            crate::plugins::lune::regex::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@lune/serde",
            crate::plugins::lune::serde::init_plugin(&lua)?,
        )?;

        Ok(Self {
            store_table,
            global_table,
            lua: Rc::new(RefCell::new(Some(lua))),
            compiler,
            scheduler,
            broken,
            on_broken: Rc::new(RefCell::new(None)),
            last_execution_time,
            time_limit,
            execution_stop_time,
            //time_slice,
            opts,
            proxy_require
        })
    }

    /// Returns the scheduler
    pub fn scheduler(&self) -> &S {
        log::debug!("Getting scheduler");
        &self.scheduler
    }

    /// Returns the global table
    pub fn global_table(&self) -> &LuaTable {
        &self.global_table
    }

    /// Returns the last execution time
    ///
    /// This may be None if the VM has not executed a script yet
    pub fn last_execution_time(&self) -> Option<Instant> {
        log::debug!("Getting last execution time");
        self.last_execution_time.get()
    }

    /// Updates the last execution time
    pub fn update_last_execution_time(&self, time: Instant) {
        log::debug!("Updating last execution time");
        self.last_execution_time.set(Some(time));

        // Update the execution stop time as well
        self.execution_stop_time.set(self.time_limit.get().map(|limit| time + limit));
    }

    /// Returns the time limit for execution
    pub fn time_limit(&self) -> Option<std::time::Duration> {
        self.time_limit.get()
    }

    /// Sets the time limit for execution
    pub fn set_time_limit(&self, limit: Option<std::time::Duration>) {
        self.time_limit.set(limit);
    }

    /// Returns whether the runtime is broken or not
    pub fn is_broken(&self) -> bool {
        log::debug!("Getting if runtime is broken");
        self.broken.get()
    }

    /// Returns the runtime creation options
    pub fn opts(&self) -> &RuntimeCreateOpts {
        log::debug!("Getting runtime creation options");
        &self.opts
    }

    /// Sets the runtime to be broken. This will also attempt to close the lua vm but
    /// will still call the on_broken callback if it is set regardless of return of close
    ///
    /// It is a logic error to call this function while holding a reference to the lua vm
    pub fn mark_broken(&self, broken: bool) -> Result<(), crate::Error> {
        log::debug!("Marking runtime as broken");
        let mut stat = Ok(());
        match self.close() {
            Ok(_) => {}
            Err(e) => {
                self.broken.set(true); // Ensure runtime is still at least marked as broken
                stat = Err(e); // Set return value to the error
            }
        };

        // Call the on_broken callback if the runtime is marked as broken
        //
        // This must be called regardless of if close failed or not to ensure at least
        // other handles are closed
        if broken {
            if let Some(ref on_broken) = *self.on_broken.borrow() {
                on_broken();
            }
        }

        stat
    }

    /// Returns if a on_broken callback is set
    pub fn has_on_broken(&self) -> bool {
        log::debug!("Getting if on_broken callback is set");
        self.on_broken.borrow().is_some()
    }

    /// Registers a callback to be called when the runtime is marked as broken
    pub fn set_on_broken(&self, callback: OnBrokenFunc) {
        log::debug!("Setting on_broken callback");
        self.on_broken.borrow_mut().replace(callback);
    }

    /// Returns the current memory usage of the runtime
    ///
    /// Returns `0` if the lua vm is not valid
    pub fn memory_usage(&self) -> usize {
        let Some(ref lua) = *self.lua.borrow() else {
            return 0;
        };
        lua.used_memory()
    }

    /// Sets a memory limit for the runtime
    ///
    /// The memory limit is set in bytes and will be enforced by the lua vm itself
    /// (e.g. using mlua)
    pub fn set_memory_limit(&self, limit: usize) -> Result<usize, LuaError> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        lua.set_memory_limit(limit)
    }

    /// Returns the store table for the runtime
    pub fn store_table(&self) -> &LuaTable {
        &self.store_table
    }

    /// Execute a closure with the lua vm if it is valid
    pub fn with_lua<F, R>(&self, func: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua) -> LuaResult<R>,
    {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        self.handle_error(func(lua))
    }

    /// Creates a new TemplateContext with the given KhronosContext
    pub fn create_context<K: KhronosContextTrait>(
        &self,
        context: K,
        event: CreateEvent,
    ) -> Result<TemplateContext<K>, LuaError> {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        let context = TemplateContext::new(self.store_table.clone(), context, event)?;
        Ok(context)
    }

    /// Helper methods to handle errors correctly, dispatching mark_broken calls if theres
    /// a memory error etc.
    pub fn handle_error<T>(&self, resp: LuaResult<T>) -> LuaResult<T> {
        match resp {
            Ok(f) => Ok(f),
            Err(e) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.mark_broken(true)
                    .map_err(|e| LuaError::external(e.to_string()))?;
                }

                return Err(e);
            }
        }
    }

    /// Loads/evaluates a script
    pub fn eval_script<R>(
        &self,
        path: &str,
    ) -> LuaResult<R> 
    where
        R: FromLuaMulti,
    {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        self.handle_error(self.proxy_require.call(path))
    }

    /// Loads/evaluates a chunk of code into a function
    pub fn eval_chunk(
        &self,
        code: &str,
        name: Option<&str>,
        env: Option<LuaTable>,
    ) -> LuaResult<LuaFunction> {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        let chunk = match name {
            Some(n) => lua.load(code).set_name(n),
            None => lua.load(code),
        };
        let chunk = match env {
            Some(e) => chunk.set_environment(e),
            None => chunk.set_environment(self.global_table.clone()),
        };
        let chunk = chunk
            .set_compiler(self.compiler.clone())
            .set_mode(mluau::ChunkMode::Text)
            .try_cache();

        self.handle_error(chunk.into_function())
    }

    /// Helper method to call a function inside of the scheduler as a thread
    pub async fn call_in_scheduler<A, R>(
        &self,
        func: LuaFunction,
        args: A,
    ) -> LuaResult<R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        let (th, args) = {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
            };
            (self.handle_error(lua.create_thread(func))?, self.handle_error(args.into_lua_multi(lua))?)
        };

        // Update last_execution_time
        self.update_last_execution_time(std::time::Instant::now());

        let res = self.handle_error(self
            .scheduler
            .run_in_scheduler(th, args)
            .await)?;

        {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
            };

            self.handle_error(R::from_lua_multi(res, lua))
        }
    }

    pub fn from_value<T: for<'de> serde::Deserialize<'de>>(&self, value: LuaValue) -> LuaResult<T> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        self.handle_error(lua.from_value(value))
    }

    /// Closes the lua vm and marks the runtime as broken
    ///
    /// This is similar to ``mark_broken`` but will not call any callbacks
    pub fn close(&self) -> Result<(), crate::Error> {
        self.broken.set(true); // Mark the runtime as broken if it is closed

        {
            if let Some(ref lua) = *self.lua.borrow_mut() {
                {
                    // Ensure strong_count == 1
                    if lua.strong_count() > 1 {
                        log::warn!("Lua VM is still in use and may not be closed immediately");
                    }
                }
            } else {
                return Ok(()); // Lua VM is already closed
            }
        }

        *self.lua.borrow_mut() = None; // Drop the Lua VM
        self.broken.set(true); // Mark the runtime as broken if it is closed

        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.lua.borrow().is_none()
    }
}
