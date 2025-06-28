pub enum LockdownError {}

/// A lockdown provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait LockdownProvider<T: lockdowns::LockdownDataStore>: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns a lockdown data store to be used with the lockdown library
    fn lockdown_data_store(&self) -> &T;

    /// Serenity HTTP client
    fn serenity_http(&self) -> &serenity::http::Http;
}
