use crate::dispatch::parse_event;
use crate::experiments::load_experiments;
use crate::filestorage::FileStorageProvider;
use crate::presets::impls::CreateEventFromPresetType;
use crate::presets::types::AntiraidEventPresetType;
use crate::provider;
use crate::provider::CliKhronosContext;
use crate::repl_completer;
use antiraid_types::ar_event::AntiraidEvent;
use khronos_runtime::primitives::event::Event;
use khronos_runtime::utils::pluginholder::PluginSet;
use khronos_runtime::utils::prelude::setup_prelude;
use khronos_runtime::utils::proxyglobal::proxy_global;
use khronos_runtime::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use mlua_scheduler::XRc;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::cell::RefCell;
use std::env::consts::OS;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{path::PathBuf, time::Duration};
use tokio::fs;

pub static PLUGIN_SET: LazyLock<PluginSet> = LazyLock::new(|| {
    let mut plugins = PluginSet::new();
    plugins.add_default_plugins::<CliKhronosContext>();
    plugins
});

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CliEntrypointAction {
    Repl {
        task_wait_mode: ReplTaskWaitMode,
    },
    RunScripts {
        scripts: Vec<PathBuf>,
    },
    InlineScript {
        script: String,
        task_wait_mode: ReplTaskWaitMode,
    },
}

#[derive(Default, Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum ReplTaskWaitMode {
    /// No waiting. If you do a task.delay, you may need to explicitly do a task.wait to allow for the task to execute after delay
    None,
    #[default]
    /// Wait for all tasks to finish after each execution (if required)
    WaitAfterExecution,
    /// Tokio yield before prompting for the next line
    ///
    /// Does not apply to InlineScript
    YieldBeforePrompt,
}

#[derive(Debug, Clone, Copy)]
pub enum FileStorageBackend {
    #[cfg(feature = "sqlite")]
    SqliteInMemory,
    #[cfg(feature = "sqlite")]
    SqliteFile,
    #[cfg(feature = "sqlite")]
    SqliteFileNoSynchronize,
    LocalFs,
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
/// Auxillary options for the CLI
pub struct CliAuxOpts {
    pub disable_test_funcs: bool,
    pub disable_globals_proxying: bool,
    pub disable_scheduler_lib: bool,
    pub disable_task_lib: bool,
    pub experiments: Vec<String>,
}

#[derive(Debug, Clone, Default)]
/// Mutable cli extension state
pub struct CliExtensionState {
    pub requested_entrypoint: Option<CliEntrypointAction>,
}

impl CliExtensionState {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cli {
    /// What capbilities the script should have (comma separated)
    ///
    /// Can be useful for mocking etc.
    pub allowed_caps: Vec<String>,

    /// Whether or not to be verbose
    pub verbose: bool,

    /// The auxiliary options for the CLI
    pub aux_opts: CliAuxOpts,

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

    /// The file storage backend to use
    pub file_storage_backend: FileStorageBackend,

    /// The file storage to use
    ///
    /// If unset, the following will be used:
    ///
    /// If $XDG_DATA_HOME is set, $XDG_DATA_HOME/khronos-cli will be used
    /// Otherwise, $HOME/.local/share/khronos-cli will be used on Linux/MacOS
    /// and %APPDATA%/khronos-cli will be used on Windows
    pub file_storage_provider: Rc<dyn FileStorageProvider>,

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

    /// CLI extension state
    pub ext_state: Rc<RefCell<CliExtensionState>>,
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
            aux_opts: self.aux_opts.clone(),
            allowed_caps: self.allowed_caps.clone(),
            guild_id: self.guild_id,
            owner_guild_id: self.owner_guild_id,
            global_table,
            http: self.http.clone(),
            cache: None, // Not yet implemented
            file_storage_provider: self.file_storage_provider.clone(),
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

    pub async fn setup_lua_vm(
        aux_opts: CliAuxOpts,
        ext_state: Rc<RefCell<CliExtensionState>>,
    ) -> LuaSetupResult {
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

        // Override require function for plugin support and increased security
        lua.globals()
            .set(
                "require",
                lua.create_function(|this, module: String| PLUGIN_SET.require(this, module))
                    .expect("Failed to create require function"),
            )
            .expect("Failed to set require function");

        // Proxy globals if enabled
        let global_tab = if !aux_opts.disable_globals_proxying {
            proxy_global(&lua).expect("Failed to create proxy global table")
        } else {
            lua.globals()
        };

        lua.globals()
            .set(
                "cli",
                Self::setup_cli_specific_table(ext_state.clone(), &lua, &aux_opts),
            )
            .expect("Failed to set cli global");

        setup_prelude(&lua, global_tab.clone()).expect("Failed to setup prelude");

        lua.sandbox(true).expect("Sandboxed VM"); // Sandbox VM

        LuaSetupResult {
            lua,
            global_tab,
            task_mgr,
        }
    }

    fn setup_cli_specific_table(
        ext_state: Rc<RefCell<CliExtensionState>>,
        lua: &Lua,
        aux_opts: &CliAuxOpts,
    ) -> LuaTable {
        let cli_table = lua.create_table().expect("Failed to create cli table");

        // Load experiments
        let experiments_table =
            load_experiments(lua, &aux_opts.experiments).expect("Failed to load experiments");

        cli_table
            .set("exp", experiments_table)
            .expect("Failed to set experiments global");

        crate::cli_extensions::load_extensions(ext_state, lua, &cli_table)
            .expect("Failed to load cli extensions");

        cli_table
    }

    pub async fn entrypoint(&mut self, action: CliEntrypointAction) {
        match action {
            CliEntrypointAction::RunScripts { scripts } => {
                if self.verbose {
                    println!("Running script: {:?}", scripts);
                }

                for path in &scripts {
                    let path = if path.is_dir() {
                        // First, check for a bundled.luau file inside the directory
                        let bundled_path = path.join("bundled.luau");
                        if bundled_path
                            .try_exists()
                            .expect("Failed to look for bundled.luau")
                        {
                            bundled_path
                        } else {
                            // Look for an init.luau to bundle using darklua
                            let init_path = path.join("init.luau");

                            if !init_path
                                .try_exists()
                                .expect("Failed to look for init.luau")
                            {
                                eprintln!("Failed to find init.luau in directory: {:?}", path);
                                continue;
                            }

                            // Bundle the directory
                            println!("Bundling directory: {:?}", path);

                            let resources = darklua_core::Resources::from_file_system();
                            darklua_core::process(
                                &resources,
                                darklua_core::Options::new(init_path.clone())
                                    .with_output(&bundled_path)
                                    .with_configuration(
                                        darklua_core::Configuration::default()
                                        .with_bundle_configuration(darklua_core::BundleConfiguration::new(
                                            darklua_core::rules::bundle::BundleRequireMode::Path(
                                                serde_json::from_value(
                                                    serde_json::json!({})
                                                ).expect("Failed to parse bundle require mode")
                                            )
                                        ))
                                        .with_generator(darklua_core::GeneratorParameters::default_readable())
                                        .with_location(path.clone())
                                        .with_rule(
                                            {
                                                let rule: Box<dyn darklua_core::rules::Rule> = Box::new(darklua_core::rules::RemoveTypes::default());
                                                rule
                                            }
                                        ),
                                    ),
                            )
                            .expect("Failed to bundle directory");

                            bundled_path.to_path_buf()
                        }
                    } else {
                        path.to_path_buf()
                    };

                    let path = &path;

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

                    let values = match self.spawn_script(&name, &contents).await {
                        Ok(values) => values,
                        Err(e) => {
                            eprintln!("error: {}", e);
                            continue;
                        }
                    };

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

                    self.setup_data
                        .task_mgr
                        .wait_till_done(Duration::from_millis(1000))
                        .await;
                }
            }
            CliEntrypointAction::Repl { task_wait_mode } => {
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
                        match task_wait_mode {
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

                                match task_wait_mode {
                                    ReplTaskWaitMode::None
                                    | ReplTaskWaitMode::YieldBeforePrompt => {}
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
            CliEntrypointAction::InlineScript {
                script,
                task_wait_mode,
            } => match self.try_spawn_as("repl", &script).await {
                Ok(values) => {
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

                    match task_wait_mode {
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
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                }
            },
        }
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
