mod constants;
mod dispatch;
mod presets;
mod provider;
mod repl_completer;

use antiraid_types::ar_event::AntiraidEvent;
use clap::{Parser, ValueEnum};
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
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::env::consts::OS;
use std::env::var;
use std::rc::Rc;
use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use tokio::fs;

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum ReplTaskWaitMode {
    /// No waiting. If you do a task.delay, you may need to explicitly do a task.wait to allow for the task to execute after delay
    None,
    /// Wait for all tasks to finish after each execution (if required)
    WaitAfterExecution,
    /// Tokio yield before prompting for the next line
    YieldBeforePrompt,
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(name = "path")]
    /// The path to the script to run
    ///
    /// Environment variable: `SCRIPT`
    script: Option<Vec<PathBuf>>,

    /// What capbilities the script should have
    ///
    /// Can be useful for mocking etc.
    ///
    /// Environment variable: `ALLOWED_CAPS`
    #[clap(long, default_value = "[]")]
    allowed_caps: Vec<String>,

    /// Whether or not to be verbose
    ///
    /// Environment variable: `VERBOSE`
    #[clap(long, default_value = "false")]
    verbose: bool,

    /// Whether or not the default internal test functions
    /// should be attached or not
    ///
    /// Environment variable: `DISABLE_TEST_FUNCS`
    #[clap(long, default_value = "false")]
    disable_test_funcs: bool,

    /// Whether or not _G default proxying behavior should
    /// be disabled (hence making _G read only due to sandboxing)
    ///
    /// AntiRaid uses the proxy method to allow _G to be read-write
    /// while sandboxing lua globals, however for testing,
    /// you may want to check how dependent the script is on
    /// globals etc.
    ///
    /// Keep enabled if you are unsure
    ///
    /// Environment variable: `DISABLE_GLOBALS_PROXYING`
    #[clap(long, default_value = "false")]
    disable_globals_proxying: bool,

    /// Whether or not the internal "scheduler" library
    /// should be exposed to the script or not
    ///
    /// AntiRaid exposes this, however for testing, you may
    /// want to disable this to ensure your code is portable
    /// etc.
    ///
    /// Keep enabled if you are unsure
    ///
    /// Environment variable: `DISABLE_SCHEDULER_LIB`
    #[clap(long, default_value = "false")]
    disable_scheduler_lib: bool,

    /// Whether or not to expose the task library to the script
    ///
    /// AntiRaid exposes this and not exposing it will mean that
    /// basic functionality provided by the task library such as
    /// task.wait etc will not be available
    ///
    /// Keep enabled if you are unsure
    ///
    /// Environment variable: `DISABLE_TASK_LIB`
    #[clap(long, default_value = "false")]
    disable_task_lib: bool,

    /// Sets the repl wait mode.
    ///
    /// Environment variable: `REPL_WAIT_MODE`
    #[clap(long, default_value = "wait-after-execution")]
    repl_wait_mode: ReplTaskWaitMode,

    /// What preset to use for creating the event
    #[clap(long)]
    preset: Option<String>,

    /// The input data to use for creating the event
    /// using a preset
    ///
    /// Must be JSON encoded
    ///
    /// Environment variable: `PRESET_INPUT`
    #[clap(long)]
    preset_input: Option<String>,

    /// The raw event data to use for creating the event
    ///
    /// Overrides `preset`/`preset_input` if set
    ///
    /// Environment variable: `RAW_EVENT_DATA`
    #[clap(long)]
    raw_event_data: Option<String>,

    /// What guild_id to use for mocking
    ///
    /// Environment variable: `GUILD_ID`
    #[clap(long)]
    guild_id: Option<serenity::all::GuildId>,

    /// What owner_guild_id to use for mocking
    ///
    /// Environment variable: `OWNER_GUILD_ID`
    #[clap(long)]
    owner_guild_id: Option<serenity::all::GuildId>,

    #[clap(long)]
    /// The discord bot token to use for discord-related operations
    ///
    /// Optional, but required for discord-related operations
    ///
    /// Environment variable: `BOT_TOKEN``
    bot_token: Option<String>,

    /// The path to a config file containing e.g.
    /// the bot token etc
    ///
    /// Config file must be in env variable format. If the config file refers to another
    /// config file with `CONFIG_FILE`, it will be recursively loaded
    ///
    /// Environment variable: `CONFIG_FILE`
    #[clap(long)]
    config_file: Option<PathBuf>,

    /// The http client to use for discord operations
    #[clap(skip)]
    http: Option<Rc<serenity::all::Http>>,

    /// The cached khronos runtime arguments
    #[clap(skip)]
    cached_khronos_rt_args: Option<LuaMultiValue>,
}

/// Trait used in update_from_env_vars to get environment variables
pub trait EnvSource {
    fn var(&self, key: &str) -> Result<String, khronos_runtime::Error>;
    fn keep_config_file(&self) -> bool; // Whether to set config file to null if not found in env source
}

pub struct EnvVarEnvSource {}

impl EnvSource for EnvVarEnvSource {
    fn var(&self, key: &str) -> Result<String, khronos_runtime::Error> {
        var(key).map_err(|e| e.into())
    }

    fn keep_config_file(&self) -> bool {
        true
    }
}

pub struct DotEnvyEnvSource {
    map: dotenvy::EnvMap,
}

impl EnvSource for DotEnvyEnvSource {
    fn var(&self, key: &str) -> Result<String, khronos_runtime::Error> {
        self.map.var(key).map_err(|e| e.into())
    }

    fn keep_config_file(&self) -> bool {
        false // Ensure its set to null if not found in env
    }
}

impl Cli {
    /// Update from env var source
    fn update_from_env_vars(&mut self, src: impl EnvSource) {
        // First update from environment variables
        if let Ok(script) = src.var("SCRIPT") {
            self.script = serde_json::from_str(&script).expect("Failed to parse script");
        }

        if let Ok(allowed_caps) = src.var("ALLOWED_CAPS") {
            self.allowed_caps =
                serde_json::from_str(&allowed_caps).expect("Failed to parse allowed caps");
        }

        if let Ok(verbose) = src.var("VERBOSE") {
            self.verbose = verbose.parse().expect("Failed to parse verbose");
        }

        if let Ok(disable_test_funcs) = src.var("DISABLE_TEST_FUNCS") {
            self.disable_test_funcs = disable_test_funcs
                .parse()
                .expect("Failed to parse disable test funcs");
        }

        if let Ok(disable_globals_proxying) = src.var("DISABLE_GLOBALS_PROXYING") {
            self.disable_globals_proxying = disable_globals_proxying
                .parse()
                .expect("Failed to parse DISABLE_GLOBALS_PROXYING");
        }

        if let Ok(disable_scheduler_lib) = src.var("DISABLE_SCHEDULER_LIB") {
            self.disable_scheduler_lib = disable_scheduler_lib
                .parse()
                .expect("Failed to parse disable scheduler lib");
        }

        if let Ok(disable_task_lib) = src.var("DISABLE_TASK_LIB") {
            self.disable_task_lib = disable_task_lib
                .parse()
                .expect("Failed to parse disable task lib");
        }

        if let Ok(repl_wait_mode) = src.var("REPL_WAIT_MODE") {
            self.repl_wait_mode = <ReplTaskWaitMode as ValueEnum>::from_str(&repl_wait_mode, true)
                .expect("Failed to parse repl wait mode");
        }

        if let Ok(preset) = src.var("PRESET") {
            self.preset = Some(preset);
        }

        if let Ok(preset_input) = src.var("PRESET_INPUT") {
            self.preset_input = Some(preset_input);
        }

        if let Ok(raw_event_data) = src.var("RAW_EVENT_DATA") {
            self.raw_event_data = Some(raw_event_data);
        }

        if let Ok(guild_id) = src.var("GUILD_ID") {
            self.guild_id = Some(serenity::all::GuildId::new(
                guild_id.parse().expect("Failed to parse guild id"),
            ));
        }

        if let Ok(owner_guild_id) = src.var("OWNER_GUILD_ID") {
            self.owner_guild_id = Some(serenity::all::GuildId::new(
                owner_guild_id
                    .parse()
                    .expect("Failed to parse owner guild id"),
            ));
        }

        if let Ok(bot_token) = src.var("BOT_TOKEN") {
            self.bot_token = Some(bot_token);
        }

        if let Ok(config_file) = src.var("CONFIG_FILE") {
            self.config_file = Some(PathBuf::from(config_file));
        } else if !src.keep_config_file() {
            self.config_file = None;
        }
    }

    /// Parses/updates the config from environment variables as well as config file
    async fn finalize(&mut self) {
        self.update_from_env_vars(EnvVarEnvSource {});

        while let Some(ref config_file) = self.config_file {
            let contents = fs::read_to_string(config_file)
                .await
                .expect("Failed to read config");

            let map = dotenvy::EnvLoader::with_reader(contents.as_bytes())
                .load()
                .expect("Failed to load config");

            let src = DotEnvyEnvSource { map };

            self.update_from_env_vars(src);
        }

        // If bot token is specified, make a serenity http client
        self.http = self
            .bot_token
            .as_ref()
            .map(|token| Rc::new(serenity::all::Http::new(token)));

        if self.verbose {
            println!("Config: {:#?}", self);
        }
    }

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
            http: self.http.clone(),
        }
    }

    async fn spawn_script(
        &mut self,
        event: &CreateEvent,
        lua: &Lua,
        name: &str,
        code: &str,
        global: LuaTable,
    ) -> LuaResult<LuaMultiValue> {
        let args = if let Some(args) = self.cached_khronos_rt_args.clone() {
            args
        } else {
            let cli_context = self.create_khronos_context(global.clone());
            let template_context = TemplateContext::new(cli_context);
            let event = Event::from_create_event(event);
            let args = (event, template_context).into_lua_multi(lua)?;
            self.cached_khronos_rt_args = Some(args.clone()); // Ensure we cache it
            args
        };

        let f = lua
            .load(code)
            .set_name(name)
            .set_environment(global)
            .into_function()?;

        let th = lua.create_thread(f)?;
        //println!("Spawning thread: {:?}", th.to_pointer());

        let scheduler = mlua_scheduler_ext::Scheduler::get(lua);
        let output = scheduler
            .spawn_thread_and_wait("SpawnScript", th, args)
            .await?;

        match output {
            Some(result) => result,
            None => Ok(LuaMultiValue::new()),
        }
    }
}

fn main() {
    env_logger::init();

    let mut cli = Cli::parse();
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
        cli.finalize().await;

        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::default())
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
        if !cli.disable_test_funcs {
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

        if !cli.disable_scheduler_lib {
            lua.globals()
                .set("scheduler", scheduler_lib.clone())
                .expect("Failed to set scheduler global");
        }

        if !cli.disable_task_lib {
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
        let global_tab = if !cli.disable_globals_proxying {
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

        entrypoint(&mut cli, &create_event, lua, global_tab, &task_mgr).await;

        if cli.verbose {
            println!("Stopping task manager");
        }

        task_mgr.stop();
        //std::process::exit(0);
    });
}

async fn entrypoint(
    cli: &mut Cli,
    create_event: &CreateEvent,
    lua: Lua,
    global_tab: LuaTable,
    task_mgr: &mlua_scheduler::taskmgr::TaskManager,
) {
    if let Some(ref script) = cli.script.clone() {
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

            let output = cli
                .spawn_script(create_event, &lua, &name, &contents, global_tab.clone())
                .await
                .expect("Failed to spawn script");

            println!("Output: {:?}", output);

            task_mgr.wait_till_done(Duration::from_millis(1000)).await;
        }
    } else {
        if cli.verbose {
            println!("Spawning REPL in separate thread");
        }

        // Inspired from https://github.com/mlua-rs/mlua/blob/main/examples/repl.rs
        let mut editor: Editor<repl_completer::LuaStatementCompleter, DefaultHistory> =
            Editor::new().expect("Failed to create editor");

        editor.set_helper(Some(repl_completer::LuaStatementCompleter {
            lua: lua.clone(),
            global_tab: global_tab.clone(),
        }));

        loop {
            let mut prompt = "> ";
            let mut line = String::new();

            loop {
                match cli.repl_wait_mode {
                    ReplTaskWaitMode::None | ReplTaskWaitMode::WaitAfterExecution => {}
                    ReplTaskWaitMode::YieldBeforePrompt => {
                        tokio::task::yield_now().await;
                    }
                };

                match editor.readline(prompt) {
                    Ok(input) => line.push_str(&input),
                    Err(_) => return,
                }

                match try_spawn_as(create_event, cli, &lua, "repl", &line, global_tab.clone()).await
                {
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

                        match cli.repl_wait_mode {
                            ReplTaskWaitMode::None | ReplTaskWaitMode::YieldBeforePrompt => {}
                            ReplTaskWaitMode::WaitAfterExecution => {
                                if !task_mgr.is_empty() {
                                    println!("[waiting for all pending tasks to finish]");
                                }

                                task_mgr.wait_till_done(Duration::from_millis(1000)).await;
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
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        }
    }
}

/// Try calling a line, first as an expression with an added "return " before it
/// and then as a statement.
async fn try_spawn_as(
    create_event: &CreateEvent,
    cli: &mut Cli,
    lua: &Lua,
    name: &str,
    code: &str,
    global_tab: LuaTable,
) -> LuaResult<LuaMultiValue> {
    if let Ok(result) = cli
        .spawn_script(
            create_event,
            lua,
            name,
            &format!("return {}", code),
            global_tab.clone(),
        )
        .await
    {
        return Ok(result);
    }

    cli.spawn_script(create_event, lua, name, code, global_tab)
        .await
}

pub struct TaskPrintError {
    pub thread_limit: usize,
    pub threads: XRc<XRefCell<usize>>,
}

impl mlua_scheduler::taskmgr::SchedulerFeedback for TaskPrintError {
    fn on_thread_add(
        &self,
        _label: &str,
        _creator: &LuaThread,
        _thread: &LuaThread,
    ) -> LuaResult<()> {
        let mut threads = self.threads.borrow_mut();
        if *threads >= self.thread_limit {
            return Err(LuaError::external("Thread limit reached"));
        }

        *threads += 1;

        Ok(())
    }

    fn on_response(
        &self,
        _label: &str,
        _tm: &mlua_scheduler::TaskManager,
        _th: &LuaThread,
        result: LuaResult<LuaMultiValue>,
    ) {
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
