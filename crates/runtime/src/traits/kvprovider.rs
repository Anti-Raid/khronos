use super::ir::KvRecord;
use crate::utils::khronos_value::KhronosValue;

/// A key-value provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait KVProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, scopes: &[String], bucket: &str) -> Result<(), crate::Error>;

    /// List all scopes that currently exist
    async fn list_scopes(&self) -> Result<Vec<String>, crate::Error>;

    /// Finds all records with the specified query. % means wildcard before/after query. E.g. %abc% will match any occurrence of abc
    async fn find(&self, scopes: &[String], query: String) -> Result<Vec<KvRecord>, crate::Error>;

    /// Returns if a specific key exists in the key-value store.
    async fn exists(&self, scopes: &[String], key: String) -> Result<bool, crate::Error>;

    /// Returns all keys in the key-value store.
    async fn keys(&self, scopes: &[String]) -> Result<Vec<String>, crate::Error>;

    /// Get a record from the key-value store.
    async fn get(&self, scopes: &[String], key: String) -> Result<Option<KvRecord>, crate::Error>;

    /// Get a record from the key-value store by ID.
    async fn get_by_id(&self, id: String) -> Result<Option<KvRecord>, crate::Error>;

    /// Set a record in the key-value store.
    ///
    /// Returns a KvRecord with exists set to true if the key already exists, or false if it was created.
    async fn set(
        &self,
        scopes: &[String],
        key: String,
        value: KhronosValue,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(bool, String), crate::Error>;

    /// Set a record in the key-value store by ID which already exists in the database
    ///
    /// Returns a KvRecord. ``exists`` is guaranteed to be true
    async fn set_by_id(
        &self,
        id: String,
        value: KhronosValue,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error>;

    /// Sets the expiry of a key in the key-value store
    async fn set_expiry(
        &self,
        scopes: &[String],
        key: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error>;

    /// Sets the expiry of a key in the key-value store by ID
    async fn set_expiry_by_id(
        &self,
        id: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error>;

    /// Delete a record from the key-value store.
    async fn delete(&self, scopes: &[String], key: String) -> Result<(), crate::Error>;

    /// Delete a record from the key-value store by ID.
    async fn delete_by_id(&self, id: String) -> Result<(), crate::Error>;
}
