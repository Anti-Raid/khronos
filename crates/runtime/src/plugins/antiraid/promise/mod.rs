use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use std::{future::Future, pin::Pin};

pub type LuaPromiseFut = Pin<Box<dyn Future<Output = LuaResult<LuaMultiValue>>>>;

/// Represents a promise that must be yielded to get the result.
///
/// LuaPromise's are not run at all until ``promise.yield`` is called
/// in Lua code
///
/// Note that a promise cannot be called multiple times. Attempting to do
/// will return an error
pub struct LuaPromise {
    pub inner: Box<dyn FnOnce(Lua) -> LuaPromiseFut>, // Box the stream to ensure its pinned,
}

impl LuaPromise {
    #[allow(dead_code)]
    pub fn new(fut: Box<dyn FnOnce(Lua) -> LuaPromiseFut>) -> Self {
        Self { inner: fut }
    }

    /// Creates a new function that returns a LuaPromise
    pub fn new_function<A, F, FR>(lua: &Lua, func: F) -> LuaResult<LuaFunction>
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
    {
        // We need userdata to be explicitly borrowed in the promise due to rust things but we
        // can otherwise just use raw mlua arg conversion code directly
        lua.create_function(
            move |_lua, args: LuaMultiValue| {
                let func = func.clone();

                Ok(LuaPromise {
                    inner: Box::new(move |lua| {
                        Box::pin(async move {
                            let args = A::from_lua_multi(args, &lua)?;
                            let ret = (func)(&lua, args).await?;
                            Ok(ret.into_lua_multi(&lua)?)
                        })
                    }),
                })
            },
        )
    }
}

/// Macro lua_promise!(arg1, arg2: Type2, |lua, {args}|, {
/// }) -> LuaPromise
/// Clones all arguments and the lua instance
#[macro_export]
#[deprecated(note = "Use either LuaPromise::new_function or add_promise_method/add_promise_method_mut instead")]
macro_rules! lua_promise {
    ($($arg:ident),* $(,)?, |$lua:ident, $($args:ident),*|, $code:block) => {
        {
            use $crate::plugins::antiraid::promise::LuaPromise;
            $(
                let $arg = $arg.clone();
            )*
            LuaPromise {
                inner: Box::new(
                    move |$lua| Box::pin(async move {
                        let resp = async {
                            $code
                        }.await;
                        match resp {
                            Ok(val) => val.into_lua_multi(&$lua),
                            Err(e) => Err(e),
                        }
                    })
                )
            }
        }
    };
}

/// A UserDataLuaPromise makes it easier to create promises
/// with minimal cloning
pub trait UserDataLuaPromise<T> {
    fn add_promise_function<A, F, FR>(&mut self, name: &str, func: F)
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
        T: LuaUserData + 'static;

    fn add_promise_method<A, F, FR>(&mut self, name: &str, func: F)
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, LuaUserDataRef<T>, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
        T: LuaUserData + 'static;

    fn add_promise_method_mut<A, F, FR>(&mut self, name: &str, func: F)
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, LuaUserDataRefMut<T>, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
        T: LuaUserData + 'static;
}

impl<I, T> UserDataLuaPromise<T> for I
where
    I: LuaUserDataMethods<T>,
{
    fn add_promise_function<A, F, FR>(&mut self, name: &str, func: F)
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
        T: LuaUserData + 'static,
    {
        // We need userdata to be explicitly borrowed in the promise due to rust things but we
        // can otherwise just use raw mlua arg conversion code directly
        self.add_function(
            name,
            move |_lua, args: LuaMultiValue| {
                let func = func.clone();

                Ok(LuaPromise {
                    inner: Box::new(move |lua| {
                        Box::pin(async move {
                            let args = A::from_lua_multi(args, &lua)?;
                            let ret = (func)(&lua, args).await?;
                            Ok(ret.into_lua_multi(&lua)?)
                        })
                    }),
                })
            },
        );
    }

    fn add_promise_method<A, F, FR>(&mut self, name: &str, func: F)
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, LuaUserDataRef<T>, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
        T: LuaUserData + 'static,
    {
        // We need userdata to be explicitly borrowed in the promise due to rust things but we
        // can otherwise just use raw mlua arg conversion code directly
        self.add_function(
            name,
            move |_lua, (this, args): (LuaAnyUserData, LuaMultiValue)| {
                let func = func.clone();

                Ok(LuaPromise {
                    inner: Box::new(move |lua| {
                        Box::pin(async move {
                            let this = this.borrow::<T>()?;
                            let args = A::from_lua_multi(args, &lua)?;
                            let ret = (func)(&lua, this, args).await?;
                            Ok(ret.into_lua_multi(&lua)?)
                        })
                    }),
                })
            },
        );
    }

    fn add_promise_method_mut<A, F, FR>(&mut self, name: &str, func: F)
    where
        A: FromLuaMulti + mlua::MaybeSend + 'static,
        F: AsyncFnOnce(&Lua, LuaUserDataRefMut<T>, A) -> LuaResult<FR>
            + mlua::MaybeSend
            + Clone
            + 'static,
        FR: mlua::IntoLuaMulti + mlua::MaybeSend + 'static,
        T: LuaUserData + 'static,
    {
        // We need userdata to be explicitly borrowed in the promise due to rust things but we
        // can otherwise just use raw mlua arg conversion code directly
        self.add_function(
            name,
            move |_lua, (this, args): (LuaAnyUserData, LuaMultiValue)| {
                let func = func.clone();

                Ok(LuaPromise {
                    inner: Box::new(move |lua| {
                        Box::pin(async move {
                            let this = this.borrow_mut::<T>()?;
                            let args = A::from_lua_multi(args, &lua)?;
                            let ret = (func)(&lua, this, args).await?;
                            Ok(ret.into_lua_multi(&lua)?)
                        })
                    }),
                })
            },
        );
    }
}

pub type LuaPromiseRef = LuaUserDataRefMut<LuaPromise>;

impl LuaUserData for LuaPromise {}

struct TestLuaPromise {
    n: i32,
}

impl LuaUserData for TestLuaPromise {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_promise_method_mut("incby", async move |_lua, mut this, n: i32| {
            this.n += n;
            Ok(this.n)
        });
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "yield",
        lua.create_scheduler_async_function(|lua, promise: LuaAnyUserData| async move {
            let ud_owned = promise.take::<LuaPromise>()?;
            let fut = (ud_owned.inner)(lua);
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
                    result: mlua::Result<mlua::MultiValue>,
                ) {
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                }
            }

            lua.set_app_data(thread_tracker.clone());

            let task_mgr = mlua_scheduler::taskmgr::TaskManager::new(
                &lua,
                std::rc::Rc::new(mlua_scheduler_ext::feedbacks::ChainFeedback::new(
                    thread_tracker,
                    TaskPrintError {},
                )),
                Duration::from_millis(1),
            );

            let scheduler = mlua_scheduler_ext::Scheduler::new(task_mgr.clone());

            scheduler.attach().expect("Failed to attach scheduler");

            let test_promise = TestLuaPromise { n: 0 };
            let args = (module, test_promise).into_lua_multi(&lua).unwrap();

            let f = lua
                .load(
                    r#"
                local promise, test_promise = ...
                print(test_promise:incby(1))
    
                local function test()
                    local res = promise.yield(test_promise:incby(2))
                    assert(res == 2)
                    return res
                end
    
                return test()
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
                    assert_eq!(*n, 2);
                }
                _ => {
                    panic!("Expected integer, got {:?}", res);
                }
            }
        });
    }
}
