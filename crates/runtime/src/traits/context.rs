use mluau::{IntoLua, FromLua};

#[allow(async_fn_in_trait)]
pub trait KhronosContext: 'static + Clone + Sized {
    type SyscallArgs: FromLua;
    type SyscallRet: IntoLua;
    
    /// Perform a syscall to do something host-defined/controlled
    async fn syscall(&self, args: Self::SyscallArgs) -> Result<Self::SyscallRet, crate::Error>;
}
