#[derive(Debug, Clone)]
pub struct ScheduledExecution {
    pub id: String,
    pub template_name: String,
    pub data: serde_json::Value,
    pub run_at: chrono::DateTime<chrono::Utc>,
}
