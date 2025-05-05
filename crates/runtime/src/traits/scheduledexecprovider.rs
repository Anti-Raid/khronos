use crate::traits::ir::ScheduledExecution;

/// A scheduled execution provider.
///
/// A scheduled execution is a task that is executed once at (or after) a specific time or interval.
/// When a scheduled execution errors, the caller should *not* remove the scheduled execution
/// and should instead retry it at the next interval.
///
/// All scheduled executions have an ID and data associated with them for use by the caller.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait ScheduledExecProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Lists all scheduled executions
    async fn list(&self, id: Option<String>) -> Result<Vec<ScheduledExecution>, crate::Error>;

    /// Adds a new scheduled execution
    async fn add(&self, exec: ScheduledExecution) -> Result<(), crate::Error>;

    /// Removes a scheduled execution
    async fn remove(&self, id: String) -> Result<(), crate::Error>;
}
