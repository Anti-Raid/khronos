#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledExecution {
    pub id: String,
    pub template_name: String,
    pub data: serde_json::Value,
    pub run_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateScheduledExecution {
    pub id: String,
    pub data: serde_json::Value,
    pub run_at: chrono::DateTime<chrono::Utc>,
}
