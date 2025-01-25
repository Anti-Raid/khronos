/// Lockdown IR struct
pub struct Lockdown {
    pub id: uuid::Uuid,
    pub reason: String,
    pub r#type: String,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
