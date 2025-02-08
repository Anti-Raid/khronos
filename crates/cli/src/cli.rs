use crate::dispatch::parse_event;
use crate::presets::impls::CreateEventFromPresetType;
use crate::presets::types::AntiraidEventPresetType;
use crate::provider;
use crate::repl_completer;
use antiraid_types::ar_event::AntiraidEvent;
use khronos_runtime::primitives::event::Event;
use khronos_runtime::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use mlua_scheduler::XRc;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::env::consts::OS;
use std::rc::Rc;
use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use tokio::fs;

#[derive(Default, Debug, Clone, Copy)]
pub enum ReplTaskWaitMode {
    /// No waiting. If you do a task.delay, you may need to explicitly do a task.wait to allow for the task to execute after delay
    None,
    #[default]
    /// Wait for all tasks to finish after each execution (if required)
    WaitAfterExecution,
    /// Tokio yield before prompting for the next line
    YieldBeforePrompt,
}

pub struct LuaSetupResult {
    pub lua: Lua,
    pub global_tab: LuaTable,
    pub task_mgr: mlua_scheduler::taskmgr::TaskManager,
}

impl std::fmt::Debug for LuaSetupResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LuaSetupResult")
            .field("lua", &"Lua")
            .field("global_tab", &"LuaTable")
            .field("task_mgr", &"TaskManager")
            .finish()
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
/// Auxillary options for the CLI
pub struct CliAuxOpts {
    pub disable_test_funcs: bool,
    pub disable_globals_proxying: bool,
    pub disable_scheduler_lib: bool,
    pub disable_task_lib: bool,
}

#[derive(Debug)]
pub struct Cli {
    /// The path to the script to run
    pub script: Option<Vec<PathBuf>>,

    /// What capbilities the script should have (comma separated)
    ///
    /// Can be useful for mocking etc.
    pub allowed_caps: Vec<String>,

    /// Whether or not to be verbose
    pub verbose: bool,

    /// The auxiliary options for the CLI
    pub aux_opts: CliAuxOpts,

    /// Sets the repl wait mode.
    pub repl_wait_mode: ReplTaskWaitMode,

    /// What preset to use for creating the event
    pub preset: Option<String>,

    /// The input data to use for creating the event
    /// using a preset
    ///
    /// Must be JSON encoded
    pub preset_input: Option<String>,

    /// The raw event data to use for creating the event
    ///
    /// Overrides `preset`/`preset_input` if set
    pub raw_event_data: Option<String>,

    /// What internal context data to use for mocking
    pub context_data: Option<String>,

    /// What guild_id to use for mocking
    pub guild_id: Option<serenity::all::GuildId>,

    /// What owner_guild_id to use for mocking
    pub owner_guild_id: Option<serenity::all::GuildId>,

    #[allow(dead_code)]
    /// The discord bot token to use for discord-related operations
    ///
    /// Optional, but required for discord-related operations
    pub bot_token: Option<String>,

    #[allow(dead_code)]
    /// The path to a config file containing e.g.
    /// the bot token etc
    pub config_file: Option<PathBuf>,

    /// The http client to use for discord operations
    pub http: Option<Rc<serenity::all::Http>>,

    /// The cached khronos runtime arguments
    pub cached_khronos_rt_args: Option<LuaMultiValue>,

    /// Setup data
    pub setup_data: LuaSetupResult,
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

    /// Create a khronos context
    fn create_khronos_context(&self, global_table: LuaTable) -> provider::CliKhronosContext {
        let context_data = if let Some(ref context_data) = self.context_data {
            serde_json::from_str(context_data).expect("Failed to parse context data")
        } else {
            serde_json::Value::Null
        };

        provider::CliKhronosContext {
            data: context_data,
            aux_opts: self.aux_opts,
            allowed_caps: self.allowed_caps.clone(),
            guild_id: self.guild_id,
            owner_guild_id: self.owner_guild_id,
            global_table,
            http: self.http.clone(),
            cache: None, // Not yet implemented
        }
    }

    /// Create an event from the parsed event args
    fn create_event(&self) -> Event {
        let event = self.parse_event_args();
        let create_event = parse_event(&event).expect("Failed to parse event");
        Event::from_create_event(&create_event)
    }

    pub async fn spawn_script(&mut self, name: &str, code: &str) -> LuaResult<LuaMultiValue> {
        let args = if let Some(args) = self.cached_khronos_rt_args.clone() {
            args
        } else {
            let cli_context = self.create_khronos_context(self.setup_data.global_tab.clone());
            let template_context: TemplateContext<provider::CliKhronosContext> =
                TemplateContext::new(cli_context);
            let event = self.create_event();
            let args = (event, template_context).into_lua_multi(&self.setup_data.lua)?;
            self.cached_khronos_rt_args = Some(args.clone()); // Ensure we cache it
            args
        };

        let f = self
            .setup_data
            .lua
            .load(code)
            .set_name(name)
            .set_environment(self.setup_data.global_tab.clone())
            .into_function()?;

        let th = self.setup_data.lua.create_thread(f)?;
        //println!("Spawning thread: {:?}", th.to_pointer());

        let scheduler = mlua_scheduler_ext::Scheduler::get(&self.setup_data.lua);
        let output = scheduler
            .spawn_thread_and_wait("SpawnScript", th, args)
            .await?;

        match output {
            Some(result) => result,
            None => Ok(LuaMultiValue::new()),
        }
    }

    pub async fn setup_lua_vm(aux_opts: CliAuxOpts) -> LuaSetupResult {
        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::default())
            .expect("Failed to create Lua");

        let compiler = mlua::Compiler::new().set_optimization_level(2);

        lua.set_compiler(compiler);

        let thread_tracker = mlua_scheduler_ext::feedbacks::ThreadTracker::new();

        lua.set_app_data(thread_tracker.clone());

        let task_mgr = mlua_scheduler::taskmgr::TaskManager::new(
            lua.clone(),
            XRc::new(thread_tracker),
            Duration::from_millis(1),
        );

        let scheduler = mlua_scheduler_ext::Scheduler::new(task_mgr.clone());

        scheduler.attach();

        // Test related functions, not available outside of script runner
        if !aux_opts.disable_test_funcs {
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

        if !aux_opts.disable_scheduler_lib {
            lua.globals()
                .set("scheduler", scheduler_lib.clone())
                .expect("Failed to set scheduler global");
        }

        if !aux_opts.disable_task_lib {
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
        let global_tab = if !aux_opts.disable_globals_proxying {
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

        LuaSetupResult {
            lua,
            global_tab,
            task_mgr,
        }
    }

    pub async fn entrypoint(&mut self) {
        if let Some(ref script) = self.script.clone() {
            if self.verbose {
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

                let output = self
                    .spawn_script(&name, &contents)
                    .await
                    .expect("Failed to spawn script");

                println!("Output: {:?}", output);

                self.setup_data
                    .task_mgr
                    .wait_till_done(Duration::from_millis(1000))
                    .await;
            }
        } else {
            if self.verbose {
                println!("Spawning REPL");
            }

            // Inspired from https://github.com/mlua-rs/mlua/blob/main/examples/repl.rs
            let mut editor: Editor<repl_completer::LuaStatementCompleter, DefaultHistory> =
                Editor::new().expect("Failed to create editor");

            editor.set_helper(Some(repl_completer::LuaStatementCompleter {
                lua: self.setup_data.lua.clone(),
                global_tab: self.setup_data.global_tab.clone(),
            }));

            loop {
                let mut prompt = "> ";
                let mut line = String::new();

                loop {
                    match self.repl_wait_mode {
                        ReplTaskWaitMode::None | ReplTaskWaitMode::WaitAfterExecution => {}
                        ReplTaskWaitMode::YieldBeforePrompt => {
                            tokio::task::yield_now().await;
                        }
                    };

                    match editor.readline(prompt) {
                        Ok(input) => line.push_str(&input),
                        Err(_) => return,
                    }

                    match self.try_spawn_as("repl", &line).await {
                        Ok(values) => {
                            editor.add_history_entry(line).unwrap();

                            if !values.is_empty() {
                                println!(
                                    "{}",
                                    values
                                        .iter()
                                        .map(|value| format!("{:#?}", value))
                                        .collect::<Vec<_>>()
                                        .join("\t")
                                );
                            }

                            match self.repl_wait_mode {
                                ReplTaskWaitMode::None | ReplTaskWaitMode::YieldBeforePrompt => {}
                                ReplTaskWaitMode::WaitAfterExecution => {
                                    if !self.setup_data.task_mgr.is_empty() {
                                        println!("[waiting for all pending tasks to finish]");
                                    }

                                    self.setup_data
                                        .task_mgr
                                        .wait_till_done(Duration::from_millis(1000))
                                        .await;
                                }
                            }
                            break;
                        }
                        Err(LuaError::SyntaxError {
                            incomplete_input: true,
                            ..
                        }) => {
                            // continue reading input and append it to `line`
                            #[allow(clippy::single_char_add_str)]
                            line.push_str("\n"); // separate input lines
                            prompt = ">> ";
                        }
                        Err(e) => {
                            editor.add_history_entry(line).unwrap();
                            eprintln!("error: {}", e);
                            break;
                        }
                    }
                }
            }
        }

        if self.verbose {
            println!("Stopping task manager");
        }

        self.setup_data.task_mgr.stop();
    }

    /// Try calling a line, first as an expression with an added "return " before it
    /// and then as a statement.
    ///
    /// Used internally for the REPL
    async fn try_spawn_as(&mut self, name: &str, code: &str) -> LuaResult<LuaMultiValue> {
        match self.spawn_script(name, &format!("return {}", code)).await {
            Ok(result) => return Ok(result),
            Err(LuaError::SyntaxError { .. }) => {}
            Err(e) => return Err(e),
        }
        self.spawn_script(name, code).await
    }
}
