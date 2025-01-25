#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Lockdown {
    pub id: uuid::Uuid,
    pub reason: String,
    pub r#type: String,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::traits::ir::Lockdown> for Lockdown {
    fn from(value: crate::traits::ir::Lockdown) -> Self {
        Self {
            id: value.id,
            reason: value.reason,
            r#type: value.r#type,
            data: value.data,
            created_at: value.created_at,
        }
    }
}
