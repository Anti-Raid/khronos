use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use std::{future::Future, pin::Pin};

pub type LuaPromiseFut = Pin<Box<dyn Future<Output = LuaResult<LuaMultiValue>>>>;

/// Represents a promise that must be yielded to get the result.
///
/// LuaPromise's are not run at all until ``promise.yield`` is called
/// in Lua code
pub struct LuaPromise {
    pub inner: Box<dyn Fn(Lua) -> LuaPromiseFut>, // Box the stream to ensure its pinned,
}

impl LuaPromise {
    #[allow(dead_code)]
    pub fn new(fut: Box<dyn Fn(Lua) -> LuaPromiseFut>) -> Self {
        Self { inner: fut }
    }

    pub fn new_generic<
        T: Future<Output = LuaResult<R>> + 'static,
        U: Fn(&Lua) -> T + Clone + 'static,
        R: IntoLuaMulti + 'static,
    >(
        func: U,
    ) -> Self {
        Self {
            inner: Box::new(move |lua| {
                let func_ref = func.clone();
                Box::pin(async move {
                    let fut = async move {
                        let fut = (func_ref)(&lua);
                        match fut.await {
                            Ok(val) => val.into_lua_multi(&lua),
                            Err(e) => Err(e),
                        }
                    };

                    fut.await
                })
            }),
        }
    }
}

/// Macro lua_promise!(arg1, arg2: Type2, |lua, {args}|, {
///     // Future code
/// })
///
/// Creates:
///
/// LuaPromise::new_generic(move |lua| {
///     let arg1 = arg1.clone();
///     let arg2 = arg2.clone();
///   
///     async move {
///        let c = |lua, arg1, arg2| {
///          // Future code
///        };
///
///        c(lua, args).await    
///    }
/// })
///
/// Clones all arguments and the lua instance
#[macro_export]
macro_rules! lua_promise {
    ($($arg:ident),* $(,)?, |$lua:ident, $($args:ident),*|, $code:block) => {
        {
            use $crate::plugins::antiraid::promise::LuaPromise;
            // let arg1 = arg1.clone();
            // let arg2 = arg2.clone();
            $(
                let $arg = $arg.clone();
            )*

            LuaPromise::new_generic(move |$lua| {
                // let arg1 = arg1.clone();
                // let arg2 = arg2.clone();
                // ...
                $(
                    let $arg = $arg.clone();
                )*
                let $lua = $lua.clone();

                async move {
                    $(
                        let $args = $args.clone();
                    )*

                    let $lua = $lua.clone();

                    $code
                }
            })
        }
    };
}

pub type LuaPromiseRef = LuaUserDataRefMut<LuaPromise>;

impl LuaUserData for LuaPromise {}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "yield",
        lua.create_scheduler_async_function(|lua, promise: LuaPromiseRef| async move {
            let fut = (promise.inner)(lua);
            fut.await
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_promises() {
        // Create tokio runtime and use spawn_local
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .worker_threads(10)
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            let lua = Lua::new();
            let module = init_plugin(&lua).unwrap();

            let thread_tracker = mlua_scheduler_ext::feedbacks::ThreadTracker::new();

            pub struct TaskPrintError {}

            impl mlua_scheduler::taskmgr::SchedulerFeedback for TaskPrintError {
                fn on_response(
                    &self,
                    _label: &str,
                    _tm: &mlua_scheduler::TaskManager,
                    _th: &mlua::Thread,
                    result: Option<mlua::Result<mlua::MultiValue>>,
                ) {
                    match result {
                        Some(Ok(_)) => {}
                        Some(Err(e)) => {
                            eprintln!("Error: {:?}", e);
                        }
                        None => {}
                    }
                }
            }

            lua.set_app_data(thread_tracker.clone());

            let task_mgr = mlua_scheduler::taskmgr::TaskManager::new(
                lua.clone(),
                std::rc::Rc::new(mlua_scheduler_ext::feedbacks::ChainFeedback::new(
                    thread_tracker,
                    TaskPrintError {},
                )),
                Duration::from_millis(1),
            );

            let scheduler = mlua_scheduler_ext::Scheduler::new(task_mgr.clone());

            scheduler.attach();

            let a = 3;
            let test_promise = lua_promise!(a, |_lua, a|, {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                Ok(-1 + a)
            });

            let args = (module, test_promise).into_lua_multi(&lua).unwrap();

            let f = lua
                .load(
                    r#"
                local promise, test_promise = ...
                print(test_promise)
    
                local function test()
                    local res = promise.yield(test_promise)
                    assert(res == 2)
                    return res
                end
    
                test()
                test() -- Test that it can be called multiple times

                return test() + 1
            "#,
                )
                .into_function()
                .unwrap();

            let th = lua.create_thread(f).unwrap();

            let result = scheduler
                .spawn_thread_and_wait("SpawnScript", th, args)
                .await
                .unwrap()
                .unwrap()
                .unwrap();

            assert!(result.len() == 1);

            let res = result.front().unwrap();

            match res {
                LuaValue::Integer(n) => {
                    assert_eq!(*n, 3);
                }
                _ => {
                    panic!("Expected integer, got {:?}", res);
                }
            }
        });
    }
}
