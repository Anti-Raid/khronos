mod cli;
mod constants;
mod dispatch;
mod experiments;
mod filestorage;
mod presets;
mod provider;
mod repl_completer;

use clap::{Parser, ValueEnum};
use cli::{Cli, CliAuxOpts};
use std::env::var;
use std::path::PathBuf;
use std::rc::Rc;
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

impl From<ReplTaskWaitMode> for cli::ReplTaskWaitMode {
    fn from(mode: ReplTaskWaitMode) -> Self {
        match mode {
            ReplTaskWaitMode::None => cli::ReplTaskWaitMode::None,
            ReplTaskWaitMode::WaitAfterExecution => cli::ReplTaskWaitMode::WaitAfterExecution,
            ReplTaskWaitMode::YieldBeforePrompt => cli::ReplTaskWaitMode::YieldBeforePrompt,
        }
    }
}

#[derive(Debug, Parser)]
struct CliArgs {
    #[arg(name = "path")]
    /// The path to the script to run
    ///
    /// Environment variable: `SCRIPT`
    script: Option<Vec<PathBuf>>,

    /// What capbilities the script should have (comma separated)
    ///
    /// Can be useful for mocking etc.
    ///
    /// Environment variable: `ALLOWED_CAPS`
    #[clap(long)]
    allowed_caps: Option<String>,

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

    /// What internal context data to use for mocking
    ///
    /// Environment variable: `CONTEXT_DATA`
    #[clap(long)]
    context_data: Option<String>,

    /// What guild_id to use for mocking
    ///
    /// If unset, all APIs will default to the default_global_guild_id
    /// hardcoded in Khronos to be AntiRaids support server.
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

    /// What experiments to load into the CLI, comma separated
    ///
    /// These experiments are for internal use only and may need additional
    /// dependencies to be installed/available
    ///
    /// Environment variable: `EXPERIMENTS`
    #[clap(long)]
    experiments: Option<String>,

    /// Whether or not file storage should be disabled entirely
    ///
    /// Environment variable: `DISABLE_FILE_STORAGE`
    #[clap(long, default_value = "false")]
    disable_file_storage: bool,

    /// The base path to use for file storage
    ///
    /// If unset, See the rules from https://docs.rs/dirs/latest/dirs/fn.data_dir.html
    ///
    /// Environment variable: `FILE_STORAGE_BASE_PATH`
    #[clap(long)]
    file_storage_base_path: Option<PathBuf>,

    /// The path to a config file containing e.g.
    /// the bot token etc
    ///
    /// Config file must be in env variable format. If the config file refers to another
    /// config file with `CONFIG_FILE`, it will be recursively loaded
    ///
    /// Environment variable: `CONFIG_FILE`
    #[clap(long)]
    config_file: Option<PathBuf>,
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

impl CliArgs {
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

        if let Ok(context_data) = src.var("CONTEXT_DATA") {
            self.context_data = Some(context_data);
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

        if let Ok(experiments) = src.var("EXPERIMENTS") {
            self.experiments = Some(experiments);
        }

        if let Ok(disable_file_storage) = src.var("DISABLE_FILE_STORAGE") {
            self.disable_file_storage = disable_file_storage
                .parse()
                .expect("Failed to parse disable file storage");
        }

        if let Ok(file_storage_base_path) = src.var("FILE_STORAGE_BASE_PATH") {
            self.file_storage_base_path = Some(PathBuf::from(file_storage_base_path));
        }

        if let Ok(config_file) = src.var("CONFIG_FILE") {
            self.config_file = Some(PathBuf::from(config_file));
        } else if !src.keep_config_file() {
            self.config_file = None;
        }
    }

    /// Parses/updates the config from environment variables as well as config file
    pub async fn finalize(mut self) -> Cli {
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

        if self.verbose {
            println!("Config: {:#?}", self);
        }

        let aux_opts = CliAuxOpts {
            disable_test_funcs: self.disable_test_funcs,
            disable_globals_proxying: self.disable_globals_proxying,
            disable_scheduler_lib: self.disable_scheduler_lib,
            disable_task_lib: self.disable_task_lib,
            experiments: {
                if let Some(experiments) = self.experiments {
                    experiments
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    vec![]
                }
            },
        };

        Cli {
            script: self.script,
            allowed_caps: {
                if let Some(allowed_caps) = self.allowed_caps {
                    allowed_caps
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    vec![]
                }
            },
            verbose: self.verbose,
            aux_opts: aux_opts.clone(),
            repl_wait_mode: self.repl_wait_mode.into(),
            preset: self.preset,
            preset_input: self.preset_input,
            raw_event_data: self.raw_event_data,
            context_data: self.context_data,
            guild_id: self.guild_id,
            owner_guild_id: self.owner_guild_id,
            bot_token: self.bot_token.clone(),
            config_file: self.config_file,
            http: self
                .bot_token
                .as_ref()
                .map(|token| Rc::new(serenity::all::Http::new(token))),
            cached_khronos_rt_args: None,
            setup_data: Cli::setup_lua_vm(aux_opts).await,
            file_storage_provider: {
                if self.disable_file_storage {
                    None
                } else {
                    let base_path = self.file_storage_base_path.clone().unwrap_or_else(|| {
                        let base_path = var("XDG_DATA_HOME")
                            .map(|s| PathBuf::from(s).join("khronos-cli"))
                            .unwrap_or_else(|_| {
                                dirs::data_dir()
                                    .expect("Failed to get data dir")
                                    .join("khronos-cli")
                            });

                        if !base_path.exists() {
                            std::fs::create_dir_all(&base_path)
                                .expect("Failed to create base path");
                        }

                        base_path
                    });

                    Some(Rc::new(
                        filestorage::LocalFileStorageProvider::new(base_path)
                            .await
                            .expect("Failed to create file storage provider"),
                    ))
                }
            },
        }
    }
}

fn main() {
    env_logger::init();

    let cli_args = CliArgs::parse();

    // Create tokio runtime and use spawn_local
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .worker_threads(10)
        .build()
        .unwrap();

    let local = tokio::task::LocalSet::new();

    local.block_on(&rt, async {
        let mut cli = cli_args.finalize().await;

        cli.entrypoint().await;
    });
}
