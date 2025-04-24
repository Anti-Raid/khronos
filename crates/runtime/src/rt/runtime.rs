//! Single threaded khronos runtime struct/runner

#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::utils::prelude::disable_harmful;
use mlua::prelude::*;
use mlua_scheduler::TaskManager;
use mlua_scheduler_ext::feedbacks::{ChainFeedback, ThreadTracker};
use mlua_scheduler_ext::Scheduler;

/// A function to be called when the Khronos runtime is marked as broken
pub type OnBrokenFunc = Box<dyn Fn(&Lua)>;

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

    /// The maximum number of threads the runtime can spawn. This is internally verified using a thread callback
    ///
    /// If unset, this will be set to i64::MAX by default, meaning there is no limit on the number of threads that can be spawned
    max_threads: Rc<Cell<i64>>,

    /// Stores the current number of threads
    current_threads: Rc<Cell<i64>>,

    /// A function to be called if the runtime is marked as broken
    on_broken: Rc<RefCell<Option<OnBrokenFunc>>>,

    /// The last time the VM executed a script
    last_execution_time: Rc<Cell<Option<Instant>>>,

    /// The shared store table for the runtime
    store_table: LuaTable,

    /// runtime creation options
    opts: RuntimeCreateOpts,
}

impl KhronosRuntime {
    /// Creates a new Khronos runtime from scratch
    ///
    /// Note that the resulting lua vm is *not* sandboxed until KhronosRuntime::sandbox() is called
    pub fn new<
        SF: mlua_scheduler::taskmgr::SchedulerFeedback + 'static,
        OnInterruptFunc: Fn(&Lua, &KhronosRuntimeInterruptData) -> LuaResult<LuaVmState> + 'static,
        ThreadCreationCallbackFunc: Fn(&Lua, LuaThread) -> Result<(), mlua::Error> + 'static,
        ThreadDestructionCallbackFunc: Fn() -> () + 'static, 
    >(
        sched_feedback: SF,
        opts: RuntimeCreateOpts,
        on_interrupt: Option<OnInterruptFunc>,
        on_thread_event_callback: Option<(ThreadCreationCallbackFunc, ThreadDestructionCallbackFunc)>,
    ) -> Result<Self, LuaError> {
        let lua = Lua::new_with(
            LuaStdLib::ALL_SAFE,
            LuaOptions::new().catch_rust_panics(true),
        )?;

        let compiler = mlua::Compiler::new()
            .set_optimization_level(2)
            .set_type_info_level(1);

        lua.set_compiler(compiler.clone());

        let tt = ThreadTracker::new(); // Set up the critical thread tracker feedback
        lua.set_app_data(tt.clone());

        let sched_feedback = ChainFeedback::new(tt, sched_feedback);

        let scheduler = Scheduler::new(TaskManager::new(
            lua.clone(),
            Rc::new(sched_feedback),
            Duration::from_micros(500),
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

        let broken = Rc::new(Cell::new(false));
        let broken_ref = broken.clone();
        let last_execution_time = Rc::new(Cell::new(None));
        let last_execution_time_ref = last_execution_time.clone();

        if let Some(on_interrupt) = on_interrupt {
            lua.set_interrupt(move |lua| {
                // If the runtime is broken, yield the lua vm immediately
                let broken = broken_ref.get();
                if broken {
                    return Ok(LuaVmState::Yield);
                }

                on_interrupt(
                    lua,
                    &KhronosRuntimeInterruptData {
                        last_execution_time: last_execution_time_ref.get(),
                    },
                )
            });
        } else {
            lua.set_interrupt(move |_lua| {
                // If the runtime is broken, yield the lua vm immediately
                let broken = broken_ref.get();
                if broken {
                    return Ok(LuaVmState::Yield);
                }

                Ok(LuaVmState::Continue)
            });
        }

        disable_harmful(&lua)?;

        let current_threads = Rc::new(Cell::new(0));
        let max_threads = Rc::new(Cell::new(i64::MAX)); // Default to i64::MAX if not set

        let current_threads_ref = current_threads.clone();
        let max_threads_ref = max_threads.clone();

        if let Some(on_thread_event_callback) = on_thread_event_callback {
            lua.set_thread_creation_callback(move |lua, thread| {
                let new = current_threads_ref.get() + 1;
                current_threads_ref.set(new);

                log::debug!("Thread count now: {}, max: {}", new, max_threads_ref.get());
                if new > max_threads_ref.get() {
                    // Prevent runaway threads
                    return Err(mlua::Error::RuntimeError(format!(
                        "Maximum number of threads exceeded: {} (current: {}, max: {})",
                        new,
                        current_threads_ref.get(),
                        max_threads_ref.get()
                    )));
                }

                // Call the user provided callback
                on_thread_event_callback.0(lua, thread)?;

                Ok(())
            });

            let current_threads_ref = current_threads.clone();

            lua.set_thread_collection_callback(move |_| {
                let mut new = current_threads_ref.get() - 1;
                // Ensure we don't go negative
                if new < 0 {
                    new = 0;
                }
                current_threads_ref.set(new);

                // Call the user provided callback
                on_thread_event_callback.1();
            });
        } else {
            lua.set_thread_creation_callback(move |_lua, _thread| {
                let new = current_threads_ref.get() + 1;
                current_threads_ref.set(new);

                log::debug!("Thread count now: {}, max: {}", new, max_threads_ref.get());
                if new > max_threads_ref.get() {
                    // Prevent runaway threads
                    return Err(mlua::Error::RuntimeError(format!(
                        "Maximum number of threads exceeded: {} (current: {}, max: {})",
                        new,
                        current_threads_ref.get(),
                        max_threads_ref.get()
                    )));
                }

                Ok(())
            });

            let current_threads_ref = current_threads.clone();

            lua.set_thread_collection_callback(move |_| {
                let mut new = current_threads_ref.get() - 1;
                // Ensure we don't go negative
                if new < 0 {
                    new = 0;
                }
                current_threads_ref.set(new);
            });
        }

        Ok(Self {
            store_table: lua.create_table()?,
            lua,
            compiler,
            scheduler,
            sandboxed: false,
            broken,
            max_threads,
            current_threads,
            on_broken: Rc::new(RefCell::new(None)),
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

        // Call the on_broken callback if the runtime is marked as broken
        // to ensure anything that needs to be cleaned up is cleaned up
        if broken {
            if let Some(ref on_broken) = *self.on_broken.borrow() {
                on_broken(&self.lua);
            }
        }
    }

    /// Returns if a on_broken callback is set
    pub fn has_on_broken(&self) -> bool {
        self.on_broken.borrow().is_some()
    }

    /// Registers a callback to be called when the runtime is marked as broken
    pub fn set_on_broken(&self, callback: OnBrokenFunc) {
        self.on_broken.borrow_mut().replace(callback);
    }

    /// Sandboxes the VM after all extra needed setup has been performed.
    ///
    /// Note that Isolates cannot be created if the runtime is sandboxed
    /// and that Subisolates cannot be created if the runtime is not sandboxed
    pub fn sandbox(&mut self) -> Result<(), LuaError> {
        if self.sandboxed {
            return Ok(());
        }

        self.lua.sandbox(true)?;
        self.lua.globals().set_readonly(true);
        self.sandboxed = true;
        Ok(())
    }

    /// Un-sandboxes the VM
    ///
    /// DANGER: This should not be run after isolates have been created
    pub fn unsandbox(&mut self) -> Result<(), LuaError> {
        if !self.sandboxed {
            return Ok(());
        }

        self.lua.sandbox(false)?;
        self.lua.globals().set_readonly(false);
        self.sandboxed = false;
        Ok(())
    }

    /// Returns the maximum number of threads allowed in the runtime
    pub fn max_threads(&self) -> i64 {
        self.max_threads.get()
    }

    /// Sets the maximum number of threads allowed in the runtime
    pub fn set_max_threads(&self, max_threads: i64) {
        // Ensure we don't set a negative value
        if max_threads < 0 {
            self.max_threads.set(0);
        } else {
            self.max_threads.set(max_threads);
        }
    }

    /// Returns the current number of threads in the runtime
    ///
    /// The current number of threads is immutable and cannot be directly modified by the user.
    pub fn current_threads(&self) -> i64 {
        self.current_threads.get()
    }

    /// Returns the store table for the runtime
    pub fn store_table(&self) -> &LuaTable {
        &self.store_table
    }
}
