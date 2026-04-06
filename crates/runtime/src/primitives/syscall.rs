use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

/// Trait that defines a syscall
#[allow(async_fn_in_trait)]
pub trait Syscall: 'static {
    type SyscallArgs: FromLua;
    type SyscallRet: IntoLua;

    async fn syscall(&self, args: Self::SyscallArgs) -> Result<Self::SyscallRet, crate::Error>;
}

pub struct RawSyscall<T: Syscall>(T);

impl<T: Syscall> RawSyscall<T> {
    pub fn new(handler: T) -> Self { Self(handler) }
}

impl<T: Syscall> std::fmt::Debug for RawSyscall<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawSyscall").finish()
    }
}

impl<T: Syscall + 'static> LuaUserData for RawSyscall<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method("async", async |_lua, this, ops| {
            let state = this.0.syscall(ops).await.map_err(|x| LuaError::external(x.to_string()))?;
            Ok(state)
        });
    }
}