use crate::experiments::load_experiments;
use crate::filestorage::FileStorageProvider;
use crate::provider;
use crate::repl_completer;
use khronos_runtime::TemplateContext;
use khronos_runtime::mluau_require::AssetRequirer;
use khronos_runtime::mluau_require::FilesystemWrapper;
use khronos_runtime::mluau_require::vfs::PhysicalFS;
use khronos_runtime::primitives::event::CreateEvent;
use khronos_runtime::rt::mlua::prelude::*;
use khronos_runtime::rt::KhronosRuntime;
use khronos_runtime::rt::RuntimeCreateOpts;
use khronos_runtime::utils::proxyglobal::proxy_global;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use tokio::fs;

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
    LocalFs,
}

pub struct LuaSetupResult {
    pub rt: KhronosRuntime,
    pub global_table: LuaTable,
    pub benckark_instant: std::time::Instant,
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
    pub disable_scheduler_lib: bool,
    pub disable_task_lib: bool,
    pub use_custom_print: bool,
    pub safeenv: bool,
    pub experiments: Vec<String>,
    pub max_threads: Option<i64>,
    pub memory_limit: Option<usize>,
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

    /// What template name to use
    pub template_name: String,

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
    pub http: Option<Arc<serenity::all::Http>>,

    /// The cached khronos runtime arguments
    pub cached_context: Option<TemplateContext<provider::CliKhronosContext>>,

    /// Setup data
    pub setup_data: LuaSetupResult,

    /// CLI extension state
    pub ext_state: Rc<RefCell<CliExtensionState>>,

    /// Postgres Pool
    pub pool: Option<sqlx::PgPool>,
}

#[derive(serde::Deserialize, Debug)]
struct EventArgs {
    name: String,
    data: serde_json::Value,
}

impl Cli {
    fn parse_event_args(&self) -> EventArgs {
        if let Some(ref raw_event_data) = self.raw_event_data {
            serde_json::from_str(raw_event_data).expect("Failed to parse raw event data")
        } else {
            EventArgs {
                name: "NoEvent".to_string(),
                data: serde_json::json!({}),
            }
        }
    }

    /// Create a khronos context
    fn create_khronos_context(&self) -> provider::CliKhronosContext {
        provider::CliKhronosContext {
            allowed_caps: self.allowed_caps.clone(),
            guild_id: self.guild_id,
            http: self.http.clone(),
            file_storage_provider: self.file_storage_provider.clone(),
            pool: self.pool.clone(),
        }
    }

    pub async fn spawn_script(
        &mut self,
        name: &str,
        code: &str,
    ) -> LuaResult<LuaMultiValue> {
        let context = self.create_khronos_context();

        let event = self.parse_event_args();

        let create_event = CreateEvent::new(
            event.name.to_string(),
            None,
            event.data,
        );

        let ctx = self.setup_data.rt.create_context(context, create_event)?;

        let chunk_fn = self
            .setup_data
            .rt
            .eval_chunk(code, Some(name), Some(self.setup_data.global_table.clone()))?;

        self
        .setup_data
        .rt
        .call_in_scheduler(chunk_fn, ctx).await
    }

    pub async fn setup_lua_vm(
        aux_opts: CliAuxOpts,
        ext_state: Rc<RefCell<CliExtensionState>>,
    ) -> LuaSetupResult {
        let current_dir = std::env::current_dir().expect("Failed to get current dir");

        log::trace!("Current dir: {current_dir:?}");

        let time_now = std::time::Instant::now();
        let runtime = KhronosRuntime::new(
            RuntimeCreateOpts {
                disable_task_lib: aux_opts.disable_task_lib,
                time_limit: Some(std::time::Duration::from_secs(5)),
                give_time: std::time::Duration::from_millis(500),
            },
            None::<(fn(&Lua, LuaThread) -> Result<(), LuaError>, fn(LuaLightUserData) -> ())>,
            PhysicalFS::new(current_dir.clone())
        )
        .expect("Failed to create runtime");
        log::debug!("Lua VM created in {:?}", time_now.elapsed());

        if let Some(memory_limit) = aux_opts.memory_limit {
            runtime
                .set_memory_limit(memory_limit)
                .expect("Failed to set memory limit");
        }

        let ext_state_ref = ext_state.clone();
        let global_table = runtime
            .with_lua(move |lua| {
                let tab = proxy_global(lua)?;
                // Expose require function that uses our custom global table as its environment
                tab.set("require", lua.create_require_function(AssetRequirer::new(FilesystemWrapper::new(PhysicalFS::new(current_dir)), "tgt".to_string(), tab.clone()))?)?;
                tab.set("cli", Self::setup_cli_specific_table(ext_state_ref, lua, &aux_opts))?;
                tab.set("print", lua.create_function(|_lua, values: LuaMultiValue| {
                        if !values.is_empty() {
                            println!(
                                "{}",
                                values
                                    .iter()
                                    .map(|value| {
                                        match value {
                                            LuaValue::String(s) => format!("{}", s.display()),
                                            _ => format!("{value:#?}"),
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .join("\t")
                            );
                        } else {
                            println!("nil");
                        }

                        Ok(())
                    })?,
                )?;

                Ok(tab)
            })
            .expect("Failed to get global table");

        LuaSetupResult {
            rt: runtime,
            global_table,
            benckark_instant: time_now,
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
            .expect("Failed to load cli extensions to table");

        cli_table
    }

    pub async fn entrypoint(&mut self, action: CliEntrypointAction) {
        log::debug!(
            "Entrypoint: At time instant: {:?}",
            self.setup_data.benckark_instant.elapsed()
        );
        match action {
            CliEntrypointAction::RunScripts { scripts } => {
                if self.verbose {
                    println!("Running script: {scripts:?}");
                }

                for path in &scripts {
                    let path = if path.is_dir() {
                        let init_path = path.join("init.luau");

                        if !init_path
                            .try_exists()
                            .expect("Failed to look for init.luau")
                        {
                            eprintln!("Failed to find init.luau in directory: {path:?}");
                            continue;
                        }

                        init_path
                    } else {
                        path.to_path_buf()
                    };

                    let path = &path;

                    let contents = match fs::read_to_string(&path).await {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to read script: {e:?}");
                            continue;
                        }
                    };

                    let values = match self
                        .spawn_script(&format!("{}", path.display()), &contents)
                        .await
                    {
                        Ok(values) => values,
                        Err(e) => {
                            eprintln!("error: {e}");
                            continue;
                        }
                    };

                    if !values.is_empty() {
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|value| {
                                    match value {
                                        LuaValue::String(s) => format!("\"{}\"", s.display()),
                                        _ => format!("{value:#?}"),
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                    }

                    self.setup_data
                        .rt
                        .scheduler()
                        .wait_till_done()
                        .await
                        .expect("Failed to wait till done");
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
                    runtime: self.setup_data.rt.clone(),
                    global_tab: self
                        .setup_data
                        .rt
                        .with_lua(|lua| Ok(lua.globals()))
                        .expect("Failed to get global table"),
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

                        match self.try_spawn_as("=repl", &line).await {
                            Ok(values) => {
                                editor.add_history_entry(line).unwrap();

                                if !values.is_empty() {
                                    println!(
                                        "{}",
                                        values
                                            .iter()
                                            .map(|value| {
                                                match value {
                                                    LuaValue::String(s) => {
                                                        format!("\"{}\"", s.display())
                                                    }
                                                    _ => format!("{value:#?}"),
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                            .join("\t")
                                    );
                                }

                                match task_wait_mode {
                                    ReplTaskWaitMode::None
                                    | ReplTaskWaitMode::YieldBeforePrompt => {}
                                    ReplTaskWaitMode::WaitAfterExecution => {
                                        self.setup_data
                                            .rt
                                            .scheduler()
                                            .wait_till_done()
                                            .await
                                            .expect("Failed to wait till done");
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
                                eprintln!("error: {e}");
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
                                .map(|value| {
                                    match value {
                                        LuaValue::String(s) => format!("\"{}\"", s.display()),
                                        _ => format!("{value:#?}"),
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                    }

                    match task_wait_mode {
                        ReplTaskWaitMode::None | ReplTaskWaitMode::YieldBeforePrompt => {}
                        ReplTaskWaitMode::WaitAfterExecution => {
                            self.setup_data
                                .rt
                                .scheduler()
                                .wait_till_done()
                                .await
                                .expect("Failed to wait till done");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                }
            },
        }
    }

    /// Try calling a line, first as an expression with an added "return " before it
    /// and then as a statement.
    ///
    /// Used internally for the REPL
    async fn try_spawn_as(&mut self, name: &str, code: &str) -> LuaResult<LuaMultiValue> {
        let ret_code = format!("return {code}");
        match self.spawn_script(name, &ret_code).await {
            Ok(result) => return Ok(result),
            Err(LuaError::SyntaxError { .. }) => {}
            Err(e) => return Err(e),
        }
        self.spawn_script(name, code).await
    }
}
