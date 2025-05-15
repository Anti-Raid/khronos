use super::ir::ListObjectsResponse;

/// A object storage provider.
///
/// Unlike a key-value provider, an object storage provider allows for storing larger data blobs more efficiently at several costs/drawbacks:
/// - Slower access times (as it may go over the network/make multiple HTTP requests versus Postgres/MySQL connection + binary protocol)
/// - Only bytes may be stored (unlike kv API which can store most antiraid/luau types)
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait ObjectStorageProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns the bucket name for the object storage provider
    fn bucket_name(&self) -> String;

    /// List all files in the servers bucket with the specified (optional) prefix.
    async fn list_files(&self, prefix: Option<String>) -> Result<Vec<ListObjectsResponse>, crate::Error>;

    /// Returns if a specific key exists in the key-value store.
    async fn file_exists(&self, key: String) -> Result<bool, crate::Error>;

    /// Downloads a file from the key-value store.
    async fn download_file(&self, key: String) -> Result<Vec<u8>, crate::Error>;

    /// Returns the URL to a file in the key-value store.
    async fn get_file_url(&self, key: String) -> Result<String, crate::Error>;

    /// Upload a file to the key-value store.
    async fn upload_file(&self, key: String, data: Vec<u8>) -> Result<(), crate::Error>;

    /// Delete a file from the key-value store.
    async fn delete_file(&self, key: String) -> Result<(), crate::Error>;
}
