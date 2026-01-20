use crate::traits::ir::globalkv::{CreateGlobalKv, GlobalKv, PartialGlobalKv};

/// A key-value provider.
///
/// General note: if scopes is empty, then the operation is global and not limited to any specific scope.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait GlobalKVProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Finds all key-value entries that currently exist in the given scope
    /// 
    /// E.g. %abc% will match any occurrence of abc
    async fn find(&self, scope: String, query: String) -> Result<Vec<PartialGlobalKv>, crate::Error>;

    /// Get a record from the key-value store in the given scope.
    async fn get(&self, key: String, version: i32, scope: String) -> Result<Option<GlobalKv>, crate::Error>;

    /// Create a new global kv entry
    async fn create(&self, entry: CreateGlobalKv) -> Result<(), crate::Error>;

    /// Delete a global kv entry
    async fn delete(&self, key: String, version: i32, scope: String) -> Result<(), crate::Error>;
}
