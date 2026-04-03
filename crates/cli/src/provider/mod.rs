use khronos_runtime::traits::context::KhronosContext;

#[derive(Clone)]
pub struct CliKhronosContext {}

impl KhronosContext for CliKhronosContext {
    type SyscallArgs = bool; // dummy
    type SyscallRet = bool; // dummy

    async fn syscall(&self, _ops: bool) -> Result<bool, khronos_runtime::Error> {
        Err("Not supported".into())
    }
}
