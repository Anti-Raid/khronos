use crate::utils::khronos_value::KhronosValue;

/// Represents a full record complete with metadata
pub struct KvRecord {
    pub id: String,
    pub key: String,
    pub value: KhronosValue,
    pub scopes: Vec<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Returns when the key will expire, if set.
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}
