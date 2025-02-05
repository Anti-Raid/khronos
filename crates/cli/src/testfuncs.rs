use mlua_scheduler::LuaSchedulerAsync;
use std::env::consts::OS;

pub fn attach_test_fns(lua: &mlua::Lua) {
    lua.globals()
        .set("_OS", OS.to_lowercase())
        .expect("Failed to set _OS global");

    lua.globals()
        .set(
            "_TEST_SYNC_WORK",
            lua.create_async_function(|lua, n: u64| async move {
                //let task_mgr = taskmgr::get(&lua);
                //println!("Async work: {}", n);
                tokio::time::sleep(std::time::Duration::from_secs(n)).await;
                //println!("Async work done: {}", n);

                let created_table = lua.create_table()?;
                created_table.set("test", "test")?;

                Ok(created_table)
            })
            .expect("Failed to create async function"),
        )
        .expect("Failed to set _OS global");

    lua.globals()
        .set(
            "_TEST_ASYNC_WORK",
            lua.create_scheduler_async_function(|lua, n: u64| async move {
                tokio::time::sleep(std::time::Duration::from_secs(n)).await;
                lua.create_table()
            })
            .expect("Failed to create async function"),
        )
        .expect("Failed to set _OS global");
}
