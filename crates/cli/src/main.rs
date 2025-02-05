mod constants;
mod dispatch;
mod presets;
mod provider;

use antiraid_types::ar_event::AntiraidEvent;
use clap::Parser;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use mlua_scheduler::XRc;
use presets::impls::CreateEventFromPresetType;
use presets::types::AntiraidEventPresetType;
use std::str::FromStr;
use std::{env::consts::OS, path::PathBuf, time::Duration};
use tokio::fs;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(name = "path")]
    /// The path to the script to run
    script: Option<Vec<PathBuf>>,

    /// What capbilities the script should have
    ///
    /// Can be useful for mocking etc.
    #[clap(short, long)]
    allowed_caps: Vec<String>,

    /// Whether or not to be verbose
    #[clap(short, long)]
    verbose: bool,

    /// What preset to use for creating the event
    #[clap(short, long)]
    preset: Option<String>,

    /// The input data to use for creating the event
    /// using a preset
    ///
    /// Must be JSON encoded
    #[clap(short, long)]
    preset_input: Option<String>,

    /// The raw event data to use for creating the event
    ///
    /// Overrides `preset`/`preset_input` if set
    #[clap(short, long)]
    raw_event_data: Option<String>,

    /// What guild_id to use for mocking
    #[clap(short, long)]
    guild_id: Option<serenity::all::GuildId>,

    /// What owner_guild_id to use for mocking
    #[clap(short, long)]
    owner_guild_id: Option<serenity::all::GuildId>,
}

impl Cli {
    fn parse_event_args(&self) -> AntiraidEvent {
        if let Some(ref raw_event_data) = self.raw_event_data {
            serde_json::from_str(raw_event_data).expect("Failed to parse raw event data")
        } else if let Some(ref preset) = self.preset {
            let preset =
                AntiraidEventPresetType::from_str(preset).expect("Failed to parse preset type");

            let input = if let Some(input) = &self.preset_input {
                let input: serde_json::Value =
                    serde_json::from_str(input).expect("Failed to parse preset input data");
                input
            } else {
                serde_json::Value::Null
            };

            preset
                .to_event(input)
                .expect("Failed to create event from preset")
        } else {
            panic!("No event data provided")
        }
    }

    fn create_khronos_context(&self, global_table: LuaTable) -> provider::CliKhronosContext {
        provider::CliKhronosContext {
            allowed_caps: self.allowed_caps.clone(),
            guild_id: self.guild_id,
            owner_guild_id: self.owner_guild_id,
            global_table,
        }
    }

    async fn spawn_script(
        &self,
        lua: mlua::Lua,
        name: impl Into<String>,
        code: impl Into<String> + mlua::AsChunk<'_>,
        global: LuaTable,
    ) -> mlua::Result<()> {
        let f = lua
            .load(code)
            .set_name(name)
            .set_environment(global)
            .into_function()?;

        let th = lua.create_thread(f)?;
        //println!("Spawning thread: {:?}", th.to_pointer());

        let scheduler = mlua_scheduler_ext::Scheduler::get(&lua);
        let output = scheduler
            .spawn_thread_and_wait("SpawnScript", th, mlua::MultiValue::new())
            .await;

        println!("Output: {:?}", output);

        //println!("Spawned thread: {:?}", th.to_pointer());
        Ok(())
    }
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let event = cli.parse_event_args();

    // Create tokio runtime and use spawn_local
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .worker_threads(10)
        .build()
        .unwrap();

    let local = tokio::task::LocalSet::new();

    local.block_on(&rt, async {
        let lua = mlua::Lua::new_with(mlua::StdLib::ALL_SAFE, mlua::LuaOptions::default())
            .expect("Failed to create Lua");

        let compiler = mlua::Compiler::new().set_optimization_level(2);

        lua.set_compiler(compiler);

        let thread_tracker = mlua_scheduler_ext::feedbacks::ThreadTracker::new();

        lua.set_app_data(thread_tracker.clone());

        let task_mgr = mlua_scheduler::taskmgr::TaskManager::new(
            lua.clone(),
            XRc::new(mlua_scheduler_ext::feedbacks::ChainFeedback::new(
                thread_tracker,
                TaskPrintError {},
            )),
            Duration::from_millis(1),
        );

        let scheduler = mlua_scheduler_ext::Scheduler::new(task_mgr.clone());

        scheduler.attach();

        // Test related functions, not available outside of script runner
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

        let scheduler_lib =
            mlua_scheduler::userdata::scheduler_lib(&lua).expect("Failed to create scheduler lib");

        lua.globals()
            .set("scheduler", scheduler_lib.clone())
            .expect("Failed to set scheduler global");

        lua.globals()
            .set(
                "task",
                mlua_scheduler::userdata::task_lib(&lua, scheduler_lib)
                    .expect("Failed to create table"),
            )
            .expect("Failed to set task global");

        mlua_scheduler::userdata::patch_coroutine_lib(&lua).expect("Failed to patch coroutine lib");

        lua.sandbox(true).expect("Sandboxed VM"); // Sandbox VM

        // Setup the global table using a metatable
        //
        // SAFETY: This works because the global table will not change in the VM
        let global_mt = lua.create_table().expect("Failed to create table");
        let global_tab = lua.create_table().expect("Failed to create table");

        // Proxy reads to globals if key is in globals, otherwise to the table
        global_mt
            .set("__index", lua.globals())
            .expect("Failed to set __index");
        global_tab
            .set("_G", global_tab.clone())
            .expect("Failed to set _G");

        // Provies writes
        // Forward to _G if key is in globals, otherwise to the table
        let globals_ref = lua.globals();
        global_mt
            .set(
                "__newindex",
                lua.create_function(
                    move |_lua, (tab, key, value): (LuaTable, LuaValue, LuaValue)| {
                        let v = globals_ref.get::<LuaValue>(key.clone())?;

                        if !v.is_nil() {
                            globals_ref.set(key, value)
                        } else {
                            tab.raw_set(key, value)
                        }
                    },
                )
                .expect("Failed to create function"),
            )
            .expect("Failed to set __newindex");

        // Set __index on global_tab to point to _G
        global_tab.set_metatable(Some(global_mt));

        if let Some(ref script) = cli.script {
            if cli.verbose {
                println!("Running script: {:?}", script);
            }

            for path in script {
                let name = match fs::canonicalize(path).await {
                    Ok(p) => p.to_string_lossy().to_string(),
                    Err(_) => path.to_string_lossy().to_string(),
                };
                let contents = match fs::read_to_string(&path).await {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to read script: {:?}", e);
                        continue;
                    }
                };

                cli.spawn_script(lua.clone(), name, contents, global_tab.clone())
                    .await
                    .expect("Failed to spawn script");

                task_mgr.wait_till_done(Duration::from_millis(1000)).await;
            }
        }

        if cli.verbose {
            println!("Stopping task manager");
        }

        task_mgr.stop();
        //std::process::exit(0);
    });
}

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
