use wasmtime::{Config, Engine, Store, Linker, Instance, ResourceLimiter};
use mluau::prelude::*;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use tokio::sync::broadcast::{channel as broadcast_channel, Sender as BroadcastSender, Receiver as BroadcastReceiver};
use tokio::sync::mpsc::{channel as mpsc_channel, Receiver as MpscReceiver};
use tokio::sync::Mutex as AsyncMutex;
use mlua_scheduler::{LuaSchedulerAsyncUserData, LuaSchedulerAsync};
use crate::primitives::blob::{Blob, BlobTaker};

#[derive(Clone)]
pub struct SharedWasmLimits {
    pub max_memory: usize,
    pub allocated_memory: Arc<AtomicUsize>,
}

impl ResourceLimiter for SharedWasmLimits {
    fn memory_growing(&mut self, current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        let addition = desired - current;
        
        let old = self.allocated_memory.fetch_add(addition, Ordering::SeqCst);
        if old + addition > self.max_memory {
            // Revert the allocation if it exceeded the limit
            self.allocated_memory.fetch_sub(addition, Ordering::SeqCst);
            return Ok(false);
        }
        
        Ok(true)
    }

    fn table_growing(&mut self, _current: u32, _desired: u32, _maximum: Option<u32>) -> wasmtime::Result<bool> {
        Ok(true)
    }
}

pub struct WasmContext {
    limits: SharedWasmLimits,
    next_msg: Option<bytes::Bytes>,
    luau_rx: Option<BroadcastReceiver<bytes::Bytes>>,
}

pub struct WasmState {
    runner: Option<(Instance, Store<WasmContext>, u64)>,
    join_handle: Mutex<Option<tokio::task::JoinHandle<wasmtime::Result<()>>>>,
    luau_tx: BroadcastSender<bytes::Bytes>, 
    wasm_rx: Arc<AsyncMutex<MpscReceiver<bytes::Bytes>>>,
}

impl LuaUserData for WasmState {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Send a message from Luau to WASM
        methods.add_method("send", |_lua, this, payload: BlobTaker| {
            let _ = this.luau_tx.send(payload.0);
            Ok(())
        });

        // Receive a message from WASM to Luau (async)
        methods.add_scheduler_async_method("recv", |_lua, this, ()| {
            let rx_arc = this.wasm_rx.clone();
            async move {
                let mut rx = rx_arc.lock().await;
                match rx.recv().await {
                    Some(msg) => Ok(Some(Blob { data: msg })),
                    None => Ok(None),
                }
            }
        });
        
        // Start execution (synchronous, spawns tokio task)
        methods.add_method_mut("start", |_lua, this, ()| {
            if let Some((instance, mut store, max_fuel_per_slice)) = this.runner.take() {
                let handle = tokio::spawn(async move {
                    let func = instance.get_typed_func::<(), ()>(&mut store, "main")?;
                    store.set_fuel(max_fuel_per_slice)?;
                    func.call_async(&mut store, ()).await
                });
                
                *this.join_handle.lock().unwrap() = Some(handle);
                Ok(())
            } else {
                Err(LuaError::external("WASM is already running or has finished"))
            }
        });
        
        // Wait for execution to finish
        methods.add_scheduler_async_method("wait", |_lua, this, ()| {
            let handle = this.join_handle.lock().unwrap().take();
            async move {
                if let Some(handle) = handle {
                    match handle.await {
                        Ok(Ok(_)) => Ok(()),
                        Ok(Err(e)) => Err(LuaError::external(e)),
                        Err(e) => Err(LuaError::external(e)),
                    }
                } else {
                    Err(LuaError::external("WASM is not running or has already been waited on"))
                }
            }
        });
    }
}

/// Tries to find memory export at either memory or mem. Bails out if we dont see it
fn get_memory(caller: &mut wasmtime::Caller<'_, WasmContext>) -> std::io::Result<wasmtime::Memory> {
    if let Some(mem) = caller.get_export("memory").and_then(|e| e.into_memory()) {
        return Ok(mem);
    }
    if let Some(mem) = caller.get_export("mem").and_then(|e| e.into_memory()) {
        return Ok(mem);
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "memory export not found (tried 'memory' and 'mem')"))
}

impl WasmState {
    pub async fn instantiate(
        engine: Engine,
        limits: SharedWasmLimits,
        wasm_bytes: &[u8],
        max_fuel_per_slice: u64,
    ) -> wasmtime::Result<Self> {
        let (luau_tx, luau_rx) = broadcast_channel::<bytes::Bytes>(1024);
        let (wasm_tx, wasm_rx) = mpsc_channel::<bytes::Bytes>(1024);
        
        let mut store = Store::new(&engine, WasmContext { 
            limits,
            next_msg: None,
            luau_rx: Some(luau_rx),
        });
        store.limiter(|ctx| &mut ctx.limits);
        
        let mut linker = Linker::new(&engine);
        
        // Import: WASM sends message to Luau
        linker.func_wrap2_async(
            "env", 
            "send", 
            move |mut caller: wasmtime::Caller<'_, WasmContext>, ptr: u32, len: u32| {
                let tx = wasm_tx.clone();
                Box::new(async move {
                    let memory = get_memory(&mut caller)?;
                        
                    let mut buffer = vec![0u8; len as usize];
                    memory.read(&caller, ptr as usize, &mut buffer)?;
                    
                    let _ = tx.send(buffer.into()).await;
                    caller.set_fuel(max_fuel_per_slice)?; // reset fuel
                    Ok(())
                })
            }
        )?;

        // Import: WASM asks for the length of the next message (blocks/awaits)
        let luau_tx_len = luau_tx.clone();
        linker.func_wrap0_async(
            "env", 
            "recv_await", 
            move |mut caller: wasmtime::Caller<'_, WasmContext>| {
                let tx = luau_tx_len.clone();
                Box::new(async move {
                    if caller.data().next_msg.is_none() {
                        let mut rx = caller.data_mut().luau_rx.take().unwrap_or_else(|| tx.subscribe());
                        loop {
                            match rx.recv().await {
                                Ok(msg) => {
                                    caller.data_mut().next_msg = Some(msg);
                                    break;
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                    caller.data_mut().next_msg = None;
                                    break;
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            }
                        }
                        caller.data_mut().luau_rx = Some(rx);
                    }
                    
                    let len = caller.data().next_msg.as_ref().map(|m| m.len()).unwrap_or(0);
                    caller.set_fuel(max_fuel_per_slice)?; // reset fuel
                    Ok(len as u32)
                })
            }
        )?;

        // Import: WASM synchronously copies the awaited message into the allocated pointer
        linker.func_wrap(
            "env", 
            "recv_into", 
            move |mut caller: wasmtime::Caller<'_, WasmContext>, ptr: u32| -> wasmtime::Result<u32> {
                let msg = caller.data_mut().next_msg.take().unwrap_or_default();
                
                let memory = get_memory(&mut caller)?;
                    
                memory.write(&mut caller, ptr as usize, &msg)?;
                
                Ok(msg.len() as u32)
            }
        )?;

        let module = wasmtime::Module::new(&engine, wasm_bytes)?;
            
        let instance = linker.instantiate_async(&mut store, &module).await?; 

        Ok(WasmState { 
            runner: Some((instance, store, max_fuel_per_slice)), 
            join_handle: Mutex::new(None),
            luau_tx, 
            wasm_rx: Arc::new(AsyncMutex::new(wasm_rx)), 
        })
    }
}

pub fn init_plugin(lua: &Lua, max_memory: usize, max_fuel_per_slice: u64) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    let mut config = Config::new();
    config.async_support(true);
    config.consume_fuel(true);
    
    let engine = Engine::new(&config).map_err(|e| LuaError::external(e))?;
    
    let shared_limits = SharedWasmLimits {
        max_memory,
        allocated_memory: Arc::new(AtomicUsize::new(0)),
    };
    
    let newwasm = lua.create_scheduler_async_function(move |_lua, wasm_bytes: BlobTaker| {
        let engine = engine.clone(); 
        let limits = shared_limits.clone();
        
        async move {
            WasmState::instantiate(engine, limits, &wasm_bytes.0, max_fuel_per_slice)
                .await
                .map_err(|e| LuaError::external(e))
        }
    })?;


    module.set("newwasm", newwasm)?;
    module.set_readonly(true);
    Ok(module)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use mluau::prelude::*;
    use mluau_require::create_memory_vfs_from_map;
    use tokio::runtime::LocalOptions;

    use crate::rt::{KhronosRuntime, RuntimeCreateOpts};

    #[test]
    fn test_wasm_execution() -> LuaResult<()> {
        let rt = tokio::runtime::Builder::new_current_thread().build_local(LocalOptions::default()).unwrap();
        rt.block_on(async move {
            // The test Luau script
            let script = r#"
                return function(wasm_bytes) 
                local wasm_pkg = require"@antiraid/wasm"
                -- Boot it up
                local wasm = wasm_pkg.newwasm(wasm_bytes)
                wasm:start()
                
                -- Read the initial hello message
                local initial_msg = wasm:recv()
                local initial_str = tostring(initial_msg)
                if initial_str ~= "Hello from WASM worker!" then
                    error("WASM said: " .. initial_str)
                end
                
                -- Send a message
                wasm:send("Testing 123")
                
                -- Read the echo reply
                local reply = wasm:recv()
                local reply_str = tostring(reply)
                if reply_str ~= "[WASM Echo] You sent: Testing 123" then
                    error("WASM replied: " .. reply_str)
                end
            end
            "#;
            let mut vfs_map = HashMap::new();
            vfs_map.insert("init.luau".to_string(), script.to_string()); 

            let rt = KhronosRuntime::new(
                RuntimeCreateOpts {
                    disable_task_lib: false,
                    ..Default::default()
                },
                None::<(fn(&Lua, LuaThread) -> Result<(), LuaError>, fn(LuaLightUserData) -> ())>,
                create_memory_vfs_from_map(vfs_map).into(),
                "antiraid"
            )?;
            
            let wasm_bytes = std::fs::read("../../target/wasm32-unknown-unknown/release/test_plugin.wasm").expect("Failed to read test_plugin.wasm");
            let wasm_bytes_buf = rt.with_lua(move |l| l.create_string(&wasm_bytes))?;

            let f = rt.eval_script::<LuaFunction>("./init")?;
            rt.call_in_scheduler::<_, ()>(f, wasm_bytes_buf).await?;

            Ok(())
        })
    }
}