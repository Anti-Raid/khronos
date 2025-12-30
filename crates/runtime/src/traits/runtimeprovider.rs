use std::collections::HashMap;

use crate::core::typesext::Vfs;

use super::ir::runtime as runtime_ir;

/// A runtime provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait RuntimeProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns a set of VFS's exposed by the runtime
    ///
    /// The HashMap maps VFS identifiers (keys) to their corresponding Vfs instances (values).
    /// These VFS instances can be used to create overlays or access runtime-provided virtual filesystems.
    fn get_exposed_vfs(&self) -> Result<HashMap<String, Vfs>, crate::Error>;

    /// Fetches the TenantState or returns a suitable default
    async fn get_tenant_state(&self) -> Result<runtime_ir::TenantState, crate::Error>;

    /// Sets the TenantState
    async fn set_tenant_state(&self, state: runtime_ir::TenantState) -> Result<(), crate::Error>;

    /// Returns the statistics of the bot.
    async fn stats(&self) -> Result<runtime_ir::RuntimeStats, crate::Error>;

    /// Returns various important links of the bot
    fn links(&self) -> Result<runtime_ir::RuntimeLinks, crate::Error>;

    /// Returns the list of events the bot can dispatch
    fn event_list(&self) -> Result<Vec<String>, crate::Error>;
}
