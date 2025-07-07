//! Single threaded khronos runtime struct/runner

#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

use crate::utils::prelude::disable_harmful;
use mluau::prelude::*;
use mlua_scheduler::{ReturnTracker, TaskManager};

/// A wrapper around the Lua vm that cannot be cloned
pub struct KhronosLuaRef<'a>(&'a Lua);

impl std::ops::Deref for KhronosLuaRef<'_> {
    type Target = Lua;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub struct RuntimeGlobalTable(pub LuaTable);

/// A function to be called when the Khronos runtime is marked as broken
pub type OnBrokenFunc = Box<dyn Fn()>;

/// Auxillary options for the creation of a Khronos runtime
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct RuntimeCreateOpts {
    pub disable_task_lib: bool,
}

/// A struct representing the options for creating a Khronos runtime
pub struct KhronosRuntimeInterruptData {
    /// When the runtime last executed a script
    pub last_execution_time: Option<Instant>,
}

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
    scheduler: TaskManager,

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
        OnInterruptFunc: Fn(&Lua, &KhronosRuntimeInterruptData) -> LuaResult<LuaVmState> + 'static,
        ThreadCreationCallbackFunc: Fn(&Lua, LuaThread) -> Result<(), mluau::Error> + 'static,
        ThreadDestructionCallbackFunc: Fn() -> () + 'static,
    >(
        opts: RuntimeCreateOpts,
        on_interrupt: Option<OnInterruptFunc>,
        on_thread_event_callback: Option<(
            ThreadCreationCallbackFunc,
            ThreadDestructionCallbackFunc,
        )>,
    ) -> Result<Self, LuaError> {
        log::debug!("Creating new Khronos runtime");
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

        let scheduler = TaskManager::new(&lua, ReturnTracker::new());

        scheduler.attach()?;

        scheduler.run_in_task();

        if !opts.disable_task_lib {
            lua.globals()
                .set("task", mlua_scheduler::userdata::task_lib(&lua)?)?;
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
                    return Err(mluau::Error::RuntimeError(format!(
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
                    return Err(mluau::Error::RuntimeError(format!(
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

        let store_table = lua.create_table()?;
        lua.set_app_data::<RuntimeGlobalTable>(RuntimeGlobalTable(store_table.clone()));

        lua.globals()
            .set("__kanalytics_memusagebeforeregister", lua.used_memory())?;

        // Load core modules
        lua.register_module(
            "@antiraid/datetime",
            crate::core::datetime::init_plugin(&lua)?,
        )?;
        lua.register_module(
            "@antiraid/interop",
            crate::core::interop::init_plugin(&lua)?,
        )?;
        lua.register_module("@antiraid/lazy", crate::core::lazy::init_plugin(&lua)?)?;
        lua.register_module("@antiraid/luau", crate::core::luau::init_plugin(&lua)?)?;
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
            lua: Rc::new(RefCell::new(Some(lua))),
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
    ///
    /// The use of this function is *highly* discouraged. Do *not* hold the returned value across await points
    /// as it is internally a RefCell
    #[deprecated(since = "0.0.1", note = "Avoid directly using the lua vm.")]
    pub fn lua(&'_ self) -> std::cell::Ref<'_, Option<Lua>> {
        log::debug!("Getting lua vm");
        self.lua.borrow()
    }

    /// Returns the lua compiler being used
    pub fn compiler(&self) -> &mluau::Compiler {
        log::debug!("Getting lua compiler");
        &self.compiler
    }

    /// Sets the lua compiler being used on both the lua vm and the runtime
    pub fn set_compiler(&mut self, compiler: mluau::Compiler) {
        log::debug!("Setting lua compiler");
        let Some(ref lua) = *self.lua.borrow() else {
            return;
        };
        lua.set_compiler(compiler.clone());
        self.compiler = compiler;
    }

    /// Returns the scheduler
    pub fn scheduler(&self) -> &TaskManager {
        log::debug!("Getting scheduler");
        &self.scheduler
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
    }

    /// Returns whether the runtime is broken or not
    pub fn is_broken(&self) -> bool {
        log::debug!("Getting if runtime is broken");
        self.broken.get()
    }

    /// Returns if the runtime is sandboxed or not
    pub fn is_sandboxed(&self) -> bool {
        log::debug!("Getting if runtime is sandboxed");
        self.sandboxed
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

    /// Sandboxes the VM after all extra needed setup has been performed.
    ///
    /// Note that Isolates cannot be created if the runtime is sandboxed
    /// and that Subisolates cannot be created if the runtime is not sandboxed
    pub fn sandbox(&mut self) -> Result<(), LuaError> {
        log::debug!("Sandboxing runtime");
        if self.sandboxed {
            return Ok(());
        }

        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        lua.globals()
            .set("__kanalytics_memusagebeforesandbox", lua.used_memory())?;

        lua.sandbox(true)?;
        lua.globals().set_readonly(true);
        self.sandboxed = true;
        Ok(())
    }

    /// Un-sandboxes the VM
    ///
    /// DANGER: This should not be run after isolates have been created
    pub fn unsandbox(&mut self) -> Result<(), LuaError> {
        log::debug!("Unsandboxing runtime");
        if !self.sandboxed {
            return Ok(());
        }

        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        lua.sandbox(false)?;
        lua.globals().set_readonly(false);
        self.sandboxed = false;
        Ok(())
    }

    /// Returns the maximum number of threads allowed in the runtime
    pub fn max_threads(&self) -> i64 {
        log::debug!("Getting max threads");
        self.max_threads.get()
    }

    /// Sets the maximum number of threads allowed in the runtime
    pub fn set_max_threads(&self, max_threads: i64) {
        log::debug!("Setting max threads to {}", max_threads);
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
        log::debug!("Getting current threads");
        self.current_threads.get()
    }

    /// Returns the current memory usage of the runtime
    ///
    /// Returns `0` if the lua vm is not valid
    pub fn memory_usage(&self) -> usize {
        log::debug!("Getting memory usage");
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
        log::debug!("Setting memory limit to {}", limit);
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        lua.set_memory_limit(limit)
    }

    /// Returns the store table for the runtime
    pub fn store_table(&self) -> &LuaTable {
        log::debug!("Getting store table");
        &self.store_table
    }

    /// Sets the print function to use stdout
    pub fn use_stdout_print(&self) -> LuaResult<()> {
        // Ensure print is global as everything basically relies on print
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        log::debug!("Setting print global");
        lua.globals().set(
            "print",
            lua.create_function(|_lua, values: LuaMultiValue| {
                if !values.is_empty() {
                    println!(
                        "{}",
                        values
                            .iter()
                            .map(|value| {
                                match value {
                                    LuaValue::String(s) => format!("{}", s.display()),
                                    _ => format!("{:#?}", value),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\t")
                    );
                } else {
                    println!("nil");
                }

                Ok(())
            })?,
        )?;
        Ok(())
    }

    /// Executes a function in the lua vm.
    ///
    /// The given lua vm is wrapped in a KhronosLuaRef to try and block cloning
    pub fn exec_lua<F: FnOnce(KhronosLuaRef) -> LuaResult<()> + mluau::MaybeSend + 'static>(
        &self,
        func: F,
    ) -> LuaResult<()> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        (func)(KhronosLuaRef(lua))
    }

    /// Sets a custom global
    pub fn set_custom_global<
        A: IntoLua + mluau::MaybeSend + 'static,
        V: IntoLua + mluau::MaybeSend + 'static,
    >(
        &self,
        name: A,
        value: V,
    ) -> LuaResult<()> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        lua.globals().set(name, value)?;
        Ok(())
    }

    /// Sets a custom global function
    pub fn set_custom_global_function<
        F: Fn(KhronosLuaRef, A) -> LuaResult<R> + mluau::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    >(
        &self,
        name: &str,
        func: F,
    ) -> LuaResult<()> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        lua.globals().set(
            name,
            lua.create_function(move |lua, args: A| (func)(KhronosLuaRef(lua), args))?,
        )?;
        Ok(())
    }

    /// Closes the lua vm and marks the runtime as broken
    ///
    /// This is similar to ``mark_broken`` but will not call any callbacks
    pub fn close(&self) -> Result<(), crate::Error> {
        self.broken.set(true); // Mark the runtime as broken if it is closed

        #[cfg(feature = "strong_count_supported")]
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
