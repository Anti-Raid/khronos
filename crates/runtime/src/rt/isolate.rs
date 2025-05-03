#![allow(clippy::disallowed_methods)] // Allow RefCell borrow here

use crate::utils::require_v2::AssetRequirer;
use rand::distributions::DistString;

use crate::traits::context::KhronosContext as KhronosContextTrait;
use crate::utils::prelude::setup_prelude;
use crate::utils::proxyglobal::proxy_global;
use crate::TemplateContext;
use crate::utils::require_v2::FilesystemWrapper;

use super::runtime::KhronosRuntime;
use mlua::prelude::*;
use mlua_scheduler_ext::traits::IntoLuaThread;
use rand::distributions::Alphanumeric;

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
        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let (controller, global_table) = {
            let Some(ref lua) = *inner.lua.borrow_mut() else {
                return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
            };  
            
            let global_table = proxy_global(lua)?;

            let controller = AssetRequirer::new(
                asset_manager.clone(),
                id.clone(),
                global_table.clone()
            );
    
            if is_subisolate {
                global_table.set(
                    "require",
                    lua.create_require_function(controller.clone())?,
                )?;
            } else {
                lua.globals().set(
                    "require",
                    lua.create_require_function(controller.clone())?,
                )?;
            }
    
            setup_prelude(lua, global_table.clone())?;
            
            (controller, global_table)
        };

        Ok(Self {
            id,
            asset_manager,
            asset_requirer: controller,
            inner,
            global_table,
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

    /// Runs a script. If code is `None`, it will load the script from the asset manager
    /// using the path provided
    pub async fn spawn<K: KhronosContextTrait>(
        &self,
        path: &str,
        code: Option<String>,
        context: TemplateContext<K>,
    ) -> Result<SpawnResult, LuaError> {
        let code = match code {
            Some(code) => code.into_bytes(),
            None => self.asset_manager.get_file(path).map_err(|e| {
                LuaError::RuntimeError(format!("Failed to load asset '{}': {}", path, e))
            })?,
        };

        let compiler = self.inner.compiler();
        let bytecode = compiler.compile(code)?;

        let args = {
            let Some(ref lua) = *self.inner.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
            };

            context.into_lua_multi(lua)?
        };

        self.spawn_script(path, &bytecode, args)
            .await
    }

    /// Runs a script from the asset manager in a loop, restarting the script after 
    /// spawn_thread_and_wait exits prematurely
    ///
    /// The spawn_loop will exit when runtime is closed automatically
    pub async fn spawn_loop<
        K: KhronosContextTrait,
        OnError: (Fn(&KhronosIsolate, LuaError) -> OnErrorRet) + 'static,
        OnErrorRet: std::future::Future<Output = Result<(), crate::Error>>,
    >(
        &self,
        path: String,
        code: Option<String>,
        context: TemplateContext<K>,
        on_error: OnError,
    ) -> Result<tokio::task::JoinHandle<Result<(), crate::Error>>, LuaError> {
        let code = match code {
            Some(code) => code.into_bytes(),
            None => self.asset_manager.get_file(&path).map_err(|e| {
                LuaError::RuntimeError(format!("Failed to load asset '{}': {}", path, e))
            })?,
        };

        let compiler = self.inner.compiler();
        let bytecode = compiler.compile(code)?;
        
        let args = {
            let Some(ref lua) = *self.inner.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
            };

            context.into_lua_multi(lua)?
        };

        let self_ref = self.clone();
        Ok(tokio::task::spawn_local(
            async move {
                loop {
                    // Ensure Lua is not closed
                    if self_ref.inner.is_closed() {
                        return Ok(());
                    }

                    let res = self_ref.spawn_script(&path, &bytecode, args.clone())
                    .await;

                    match res {
                        Ok(_) => {},
                        Err(e) => {
                            // spawn_script already handles memory errors, so lets just 
                            // call the error handler
                            (on_error)(&self_ref, e).await?;
                        }
                    };
                }        
            }
        ))
    }

    /// Runs a script, returning the result as a SpawnResult
    ///
    /// Note 2: You probably want spawn_asset or spawn_asset_with_args instead of this
    async fn spawn_script(
        &self,
        name: &str,
        bytecode: &[u8],
        args: LuaMultiValue,
    ) -> LuaResult<SpawnResult> {
        let thread = {
            let Some(ref lua) = *self.inner.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
            };    

            //println!("Is VM Owned: {}", lua.is_owned());
            //println!("VM Strong Count: {}", lua.strong_count());    

            match lua
                .load(bytecode)
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
                        self.inner.mark_broken(true).map_err(|e| LuaError::external(e.to_string()))?;
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
        
        // Do a GC
        {
            let Some(ref lua) = *self.inner.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
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
                    self.inner.mark_broken(true).map_err(|e| LuaError::external(e.to_string()))?;
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

    /// Note: It is a logic error to call this if the runtime is closed
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

                let result_value = {
                    let Some(ref lua) = *isolate.inner().lua.borrow() else {
                        return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
                    };
                    
                    lua.from_value::<serde_json::Value>(value) 
                }; // Lua should be dropped here

                match result_value {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                        if let LuaError::MemoryError(_) = e {
                            // Mark VM as broken
                            isolate.inner().mark_broken(true).map_err(|e| LuaError::external(e.to_string()))?;
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
                            return Err(LuaError::RuntimeError("Lua instance is no longer valid".to_string()));
                        };

                        lua.from_value::<serde_json::Value>(v)
                    }; // Lua should be dropped here

                    match result_value {
                        Ok(v) => arr.push(v),
                        Err(e) => {
                            // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                            if let LuaError::MemoryError(_) = e {
                                // Mark VM as broken
                                isolate.inner().mark_broken(true).map_err(|e| LuaError::external(e.to_string()))?;
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
