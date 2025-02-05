mod constants;
mod dispatch;
mod presets;
mod provider;

use antiraid_types::ar_event::AntiraidEvent;
use clap::Parser;
use dispatch::parse_event;
use khronos_runtime::primitives::event::CreateEvent;
use khronos_runtime::primitives::event::Event;
use khronos_runtime::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use mlua_scheduler::XRc;
use mlua_scheduler::XRefCell;
use presets::impls::CreateEventFromPresetType;
use presets::types::AntiraidEventPresetType;
use std::env::consts::OS;
use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use tokio::fs;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(name = "path")]
    /// The path to the script to run
    script: Option<Vec<PathBuf>>,

    /// What capbilities the script should have
    ///
    /// Can be useful for mocking etc.
    #[clap(short, long, default_value = "[]")]
    allowed_caps: Vec<String>,

    /// Whether or not to be verbose
    #[clap(short, long, default_value = "false")]
    verbose: bool,

    /// Whether or not internal test functions should be attached
    /// or not
    #[clap(short, long, default_value = "true")]
    test_funcs: bool,

    /// Whether or not _G should proxy or remain readonly
    ///
    /// AntiRaid uses the proxy method, however for testing,
    /// you may want to check how dependent the script is on
    /// globals etc.
    ///
    /// Keep enabled if you are unsure
    #[clap(short, long, default_value = "true")]
    proxy_globals: bool,

    /// Whether or not the internal "scheduler" library
    /// should be exposed to the script
    ///
    /// AntiRaid exposes this, however for testing, you may
    /// want to disable this to ensure your code is portable
    /// etc.
    ///
    /// Keep enabled if you are unsure
    #[clap(short, long, default_value = "true")]
    expose_scheduler_lib: bool,

    /// Whether or not to expose the task library to the script
    ///
    /// AntiRaid exposes this and not exposing it will mean that
    /// basic functionality provided by the task library such as
    /// task.wait etc will not be available
    ///
    /// Keep enabled if you are unsure
    #[clap(short, long, default_value = "true")]
    expose_task_lib: bool,

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
            let input = if let Some(input) = &self.preset_input {
                let input: serde_json::Value =
                    serde_json::from_str(input).expect("Failed to parse preset input data");
                input
            } else {
                serde_json::Value::Null
            };

            AntiraidEventPresetType::OnStartup
                .to_event(input)
                .expect("Failed to create event from preset")
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
        event: &CreateEvent,
        lua: mlua::Lua,
        name: impl Into<String>,
        code: impl Into<String> + mlua::AsChunk<'_>,
        global: LuaTable,
    ) -> mlua::Result<()> {
        let cli_context = self.create_khronos_context(global.clone());
        let template_context = TemplateContext::new(cli_context);

        let event = Event::from_create_event(event);
        let args = (event, template_context)
            .into_lua_multi(&lua)
            .expect("Failed to convert");

        let f = lua
            .load(code)
            .set_name(name)
            .set_environment(global)
            .into_function()?;

        let th = lua.create_thread(f)?;
        //println!("Spawning thread: {:?}", th.to_pointer());

        let scheduler = mlua_scheduler_ext::Scheduler::get(&lua);
        let output = scheduler
            .spawn_thread_and_wait("SpawnScript", th, args)
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
    let create_event = parse_event(&event).expect("Failed to parse event");

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
                TaskPrintError {
                    thread_limit: 100000000,
                    threads: XRc::new(XRefCell::new(0)),
                },
            )),
            Duration::from_millis(1),
        );

        let scheduler = mlua_scheduler_ext::Scheduler::new(task_mgr.clone());

        scheduler.attach();

        // Test related functions, not available outside of script runner
        if cli.test_funcs {
            lua.globals()
                .set("_OS", OS.to_lowercase())
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

        let scheduler_lib =
            mlua_scheduler::userdata::scheduler_lib(&lua).expect("Failed to create scheduler lib");

        if cli.expose_scheduler_lib {
            lua.globals()
                .set("scheduler", scheduler_lib.clone())
                .expect("Failed to set scheduler global");
        }

        if cli.expose_task_lib {
            lua.globals()
                .set(
                    "task",
                    mlua_scheduler::userdata::task_lib(&lua, scheduler_lib)
                        .expect("Failed to create table"),
                )
                .expect("Failed to set task global");
        }

        mlua_scheduler::userdata::patch_coroutine_lib(&lua).expect("Failed to patch coroutine lib");

        lua.sandbox(true).expect("Sandboxed VM"); // Sandbox VM

        // Proxy globals if enabled
        let global_tab = if cli.proxy_globals {
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

            global_tab
        } else {
            lua.globals()
        };

        entrypoint(cli, lua, create_event, global_tab, &task_mgr).await;

        task_mgr.stop();
        //std::process::exit(0);
    });
}

/// The core entrypoint for the CLI (after the init code has ran)
async fn entrypoint(
    cli: Cli,
    lua: mlua::Lua,
    create_event: CreateEvent,
    global_tab: LuaTable,
    task_mgr: &mlua_scheduler::taskmgr::TaskManager,
) {
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

            cli.spawn_script(
                &create_event,
                lua.clone(),
                name,
                contents,
                global_tab.clone(),
            )
            .await
            .expect("Failed to spawn script");

            task_mgr.wait_till_done(Duration::from_millis(1000)).await;
        }
    }

    if cli.verbose {
        println!("Stopping task manager");
    }
}

pub struct TaskPrintError {
    pub thread_limit: usize,
    pub threads: XRc<XRefCell<usize>>,
}

impl mlua_scheduler::taskmgr::SchedulerFeedback for TaskPrintError {
    fn on_thread_add(
        &self,
        _label: &str,
        _creator: &mlua::Thread,
        _thread: &mlua::Thread,
    ) -> mlua::Result<()> {
        let mut threads = self.threads.borrow_mut();
        if *threads >= self.thread_limit {
            return Err(mlua::Error::external("Thread limit reached"));
        }

        *threads += 1;

        Ok(())
    }

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
                eprintln!("Error: {}", e);
            }
        }
    }
}
