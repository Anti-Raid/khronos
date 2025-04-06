use super::context::KhronosContext;

/// A scheduled execution provider.
/// 
/// A scheduled execution is a task that is executed once at (or after) a specific time or interval.
/// When a scheduled execution errors, the caller should *not* remove the scheduled execution
/// and should instead retry it at the next interval.
/// 
/// All scheduled executions have an ID and data associated with them for use by the caller.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait ScheduledExecProvider<T: KhronosContext>: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Lists all scheduled executions of a specific template given the template context
    async fn list(
        &self,
        context: &T,
    ) -> Result<Vec<(String, serde_json::Value)>, crate::Error>;

    /// Adds a new scheduled execution
    async fn add(
        &self,
        id: String,
        data: serde_json::Value,
        run_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), crate::Error>;

    /// Removes a scheduled execution
    async fn remove(&self, id: String) -> Result<(), crate::Error>;
}