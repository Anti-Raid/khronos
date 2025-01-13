use serde::{Deserialize, Serialize};

/// Represents a full record complete with metadata
#[derive(Serialize, Deserialize)]
pub struct KvRecord {
    pub key: String,
    pub value: serde_json::Value,
    pub exists: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A key-value provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait KVProvider: 'static + Clone {
    /// Finds all records with the specified query. % means wildcard before/after query. E.g. %abc% will match any occurrence of abc
    async fn find(&self, query: String) -> Result<Vec<KvRecord>, crate::Error>;

    /// Returns if a specific key exists in the key-value store.
    async fn exists(&self, key: String) -> Result<bool, crate::Error>;

    /// Returns all keys in the key-value store.
    async fn keys(&self) -> Result<Vec<String>, crate::Error>;

    /// Get a record from the key-value store.
    async fn get(&self, key: String) -> Result<Option<KvRecord>, crate::Error>;

    /// Set a record in the key-value store.
    async fn set(&self, key: String, value: serde_json::Value) -> Result<(), crate::Error>;

    /// Delete a record from the key-value store.
    async fn delete(&self, key: String) -> Result<(), crate::Error>;
}
