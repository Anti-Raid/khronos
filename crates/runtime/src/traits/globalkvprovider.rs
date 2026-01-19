use crate::traits::ir::globalkv::{AttachResult, CreateGlobalKv, GlobalKv};

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
    /// 
    /// Note that `data` will/should not be populated in the returned GlobalKv entries
    async fn find(&self, scope: String, query: String) -> Result<Vec<GlobalKv>, crate::Error>;

    /// Lists all attached key-value entries in the given scope
    ///
    /// Note that `data` will/should not be populated in the returned GlobalKv entries
    async fn list_attached(&self, scopes: &[String], query: String) -> Result<Vec<GlobalKv>, crate::Error>;


    /// Get a record from the key-value store in the given scope.
    /// 
    /// Note that `data` *may* be populated in the returned GlobalKv entry
    async fn get(&self, key: String, version: i32, scope: String) -> Result<Option<GlobalKv>, crate::Error>;

    /// Attach to a global kv entry in the given scope
    /// 
    /// If purchases are needed, this may fail with an error containing the URL to purchase the item etc.
    async fn attach(&self, key: String, version: i32, scope: String) -> Result<AttachResult, crate::Error>;

    /// Create a new global kv entry
    async fn create(&self, entry: CreateGlobalKv) -> Result<(), crate::Error>;

    /// Delete a global kv entry
    async fn delete(&self, key: String, version: i32, scope: String) -> Result<(), crate::Error>;
}
