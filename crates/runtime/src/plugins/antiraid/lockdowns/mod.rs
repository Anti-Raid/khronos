mod types;

use crate::lua_promise;
use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::traits::lockdownprovider::LockdownProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::TemplateContextRef;
use lockdowns::LockdownSet;
use mlua::prelude::*;
use types::{CreateLockdownMode, LockdownMode};

#[derive(Clone)]
/// An lockdown executor is used to manage AntiRaid lockdowns from Lua
/// templates
pub struct LockdownExecutor<T: KhronosContext> {
    pub context: T,
    pub lockdown_provider: T::LockdownProvider,
}

// @userdata LockdownExecutor
//
// Executes actions on discord
impl<T: KhronosContext> LockdownExecutor<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("lockdown:{}", action)) {
            return Err(LuaError::runtime(
                "Lockdown action is not allowed in this template context",
            ));
        }

        self.lockdown_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for LockdownExecutor<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "LockdownExecutor");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("fetch_lockdown_set", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_action("fetch_lockdown_set".to_string())?;

                // Get the current lockdown set
                let lockdown_set = LockdownSet::guild(
                    this.context.guild_id().ok_or_else(|| {
                        LuaError::external("This function can only be used in a guild context")
                    })?,
                    this.lockdown_provider.lockdown_data_store().clone(),
                )
                .await
                .map_err(|e| LuaError::external(format!("Error while fetching lockdown set: {}", e)))?;

                Ok(types::LockdownSet {
                    lockdown_set,
                    context: this.context.clone(),
                    lockdown_provider: this.lockdown_provider.clone(),
                })
            }))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownExecutor<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Methods
                    "fetch_lockdown_set",
                ],
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(
            |_, (token, scope): (TemplateContextRef<T>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let Some(lockdown_provider) = token.context.lockdown_provider(scope) else {
                    return Err(LuaError::external(
                        "The lockdown plugin is not supported in this context",
                    ));
                };

                let executor = LockdownExecutor {
                    context: token.context.clone(),
                    lockdown_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set(
        "CreateQuickServerLockdown",
        CreateLockdownMode(Box::new(lockdowns::qsl::CreateQuickServerLockdown)),
    )?;
    module.set(
        "CreateTraditionalServerLockdown",
        CreateLockdownMode(Box::new(lockdowns::tsl::CreateTraditionalServerLockdown)),
    )?;
    module.set(
        "CreateSingleChannelLockdown",
        CreateLockdownMode(Box::new(lockdowns::scl::CreateSingleChannelLockdown)),
    )?;
    module.set(
        "CreateSingleChannelLockdown",
        CreateLockdownMode(Box::new(lockdowns::role::CreateRoleLockdown)),
    )?;
    module.set(
        "QuickServerLockdown",
        lua.create_function(|_lua, _g: ()| {
            Ok(LockdownMode(Box::new(lockdowns::qsl::QuickServerLockdown)))
        })?,
    )?;
    module.set(
        "TraditionalServerLockdown",
        lua.create_function(|_lua, _g: ()| {
            Ok(LockdownMode(Box::new(
                lockdowns::tsl::TraditionalServerLockdown,
            )))
        })?,
    )?;
    module.set(
        "SingleChannelLockdown",
        lua.create_function(|_lua, channel_id: String| {
            let channel_id = channel_id
                .parse::<serenity::all::ChannelId>()
                .map_err(|_| LuaError::external("Failed to parse string to u64"))?;

            Ok(LockdownMode(Box::new(
                lockdowns::scl::SingleChannelLockdown(channel_id),
            )))
        })?,
    )?;
    module.set(
        "RoleLockdown",
        lua.create_function(|_lua, role_id: String| {
            let role_id = role_id
                .parse::<serenity::all::RoleId>()
                .map_err(|_| LuaError::external("Failed to parse string to u64"))?;

            Ok(LockdownMode(Box::new(lockdowns::role::RoleLockdown(
                role_id,
            ))))
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

#[cfg(test)]
mod lockdown_test {
    use std::time::Duration;

    use mlua::prelude::*;

    use crate::lua_promise;

    /// Critical mlua behavior that should be tested to ensure the plugin works as expected
    #[test]
    fn test_mlua_luaasyncdata() {
        // Create tokio runtime and use spawn_local
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .worker_threads(10)
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            let lua = Lua::new();

            let promise_tab = crate::plugins::antiraid::promise::init_plugin(&lua).unwrap();
            lua.globals().set("promise", promise_tab).unwrap();

            struct MyUserdata {
                values: Vec<u64>,
                sort_times: u64,
            }

            impl LuaUserData for MyUserdata {
                fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
                    fields.add_meta_field(LuaMetaMethod::Type, "MyUserdata");
                }

                fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
                    methods.add_method_mut("sort", |_, this, _g: ()| {
                        this.values.sort(); // Sort the values in ascending order
                        this.sort_times += 1; // Increment the sort times
                        Ok(this.sort_times)
                    });

                    methods.add_function(
                        "promise_waitadd",
                        |_, (this, k): (LuaAnyUserData, String)| {
                            Ok(lua_promise!(this, k, |_lua, this, k|, {
                                let k = k.parse::<u64>().map_err(|_| {
                                    LuaError::external("Failed to parse string to u64")
                                })?;

                                let mut this = this
                                    .borrow_mut::<MyUserdata>()
                                    .map_err(|_| LuaError::external("Failed to borrow userdata"))?;

                                // Wait k seconds using tokio to simulate some async operation
                                tokio::time::sleep(std::time::Duration::from_secs(k)).await;

                                // Add the parsed value to the vector
                                this.values.push(k);
                                this.sort_times = 0;

                                Ok(())
                            }))
                        },
                    );
                }
            }

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

            let args = MyUserdata {
                values: vec![],
                sort_times: 0,
            }
            .into_lua_multi(&lua)
            .expect("Args into_lua_multi failed"); // Convert to LuaMultiValue for passing to spawn_thread_and_wait

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

            let scheduler_lib = mlua_scheduler::userdata::scheduler_lib(&lua)
                .expect("Failed to create scheduler lib");

            lua.globals()
                .set(
                    "task",
                    mlua_scheduler::userdata::task_lib(&lua, scheduler_lib)
                        .expect("Failed to create task lib"),
                )
                .expect("Failed to set task lib");

            lua.sandbox(true)
                .expect("Failed to enable sandboxing for Lua instance");

            let f = lua
                .load(
                    r#"
                local mud = ...
                task.delay(2, function()
                    -- This function will be called after 2 seconds and should fail due to borrow mut
                    print("Sorting now...")
                    local t1 = os.time()
                    local ok, err = pcall(mud.sort, mud)
                    assert(not ok, "This should error: " .. tostring(err))
                    print("Sort error'd successfully! sort_times: " .. tostring(os.time() - t1) .. " seconds after sort")
                end)

                promise.yield(mud:promise_waitadd("5")) -- This will add 5 to the vector after 3 seconds
                
                "#,
                )
                .into_function()
                .expect("Failed to load empty function");

            let th = lua.create_thread(f).unwrap();

            let result = scheduler
                .spawn_thread_and_wait("SpawnScript", th, args)
                .await
                .expect("Failed to load empty function")
                .expect("Failed to load empty function")
                .expect("Failed to load empty function");

            println!("Result: {:?}", result);
        });
    }
}
