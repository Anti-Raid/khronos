use super::ir::KvRecord;
use crate::utils::khronos_value::KhronosValue;

/// A key-value provider.
///
/// General note: if scopes is empty, then the operation is global and not limited to any specific scope.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait KVProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, scopes: &[String], bucket: &str) -> Result<(), crate::Error>;

    /// List all scopes that currently exist
    async fn list_scopes(&self) -> Result<Vec<String>, crate::Error>;

    /// Finds all records with the specified query. % means wildcard before/after query. 
    /// 
    /// E.g. %abc% will match any occurrence of abc
    async fn find(&self, scopes: &[String], query: String) -> Result<Vec<KvRecord>, crate::Error>;

    /// Get a record from the key-value store.
    async fn get(&self, scopes: &[String], key: String) -> Result<Option<KvRecord>, crate::Error>;

    /// Set a record in the key-value store.
    async fn set(
        &self,
        scopes: &[String],
        key: String,
        value: KhronosValue,
    ) -> Result<(), crate::Error>;

    /// Delete a record from the key-value store.
    async fn delete(&self, scopes: &[String], key: String) -> Result<(), crate::Error>;
}
