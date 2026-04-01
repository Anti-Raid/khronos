use dapi::EVENT_LIST;
use khronos_runtime::traits::runtimeprovider::RuntimeProvider;
use std::collections::HashMap;

use khronos_runtime::traits::context::KhronosContext;
use khronos_runtime::traits::ir::runtime as runtime_ir;

#[derive(Clone)]
pub struct CliKhronosContext {
}

impl KhronosContext for CliKhronosContext {
    type RuntimeProvider = CliRuntimeProvider;

    fn runtime_provider(&self) -> Option<Self::RuntimeProvider> {
        Some(CliRuntimeProvider {
        })
    }
}

#[derive(Clone)]
pub struct CliRuntimeProvider {
}

// TODO: Actually implement this correctly, for now everything is a stub
impl RuntimeProvider for CliRuntimeProvider {
    type SyscallArgs = bool; // dummy
    type SyscallRet = bool; // dummy

    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    fn get_exposed_vfs(&self) -> Result<HashMap<String, khronos_runtime::core::typesext::Vfs>, khronos_runtime::Error> {
        // CLI mode does not expose any VFS mappings
        Ok(std::collections::HashMap::new())
    }

    async fn syscall(&self, _ops: bool) -> Result<bool, khronos_runtime::Error> {
        Err("Not supported".into())
    }

    async fn stats(&self) -> Result<runtime_ir::RuntimeStats, khronos_runtime::Error> {
        // TODO: Support customizing this to smth sensible
        Ok(runtime_ir::RuntimeStats {
            total_cached_guilds: 0,
            total_guilds: 1,
            total_users: 1,
            last_started_at: chrono::Utc::now(),
        })
    }

    fn links(&self) -> Result<runtime_ir::RuntimeLinks, khronos_runtime::Error> {
        // TODO: Support customizing this to smth sensible
        Ok(runtime_ir::RuntimeLinks {
            support_server: "cli".to_string(),
            api_url: "cli".to_string(),
            frontend_url: "cli".to_string(),
            docs_url: "cli".to_string()
        })
    }

    fn event_list(&self) -> Result<Vec<String>, khronos_runtime::Error> {
        Ok(EVENT_LIST
            .iter()
            .copied()
            .map(|x| x.to_string())
            .collect::<Vec<String>>())
    }
}