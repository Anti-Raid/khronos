use crate::traits::ir::globalkv::GlobalKv;

/// A key-value provider.
///
/// General note: if scopes is empty, then the operation is global and not limited to any specific scope.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait GlobalKVProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// List all key-value entries that currently exist
    async fn list(&self) -> Result<Vec<GlobalKv>, crate::Error>;

    /// Get a record from the key-value store.
    async fn get(&self, key: String, version: i32) -> Result<Option<GlobalKv>, crate::Error>;
}
