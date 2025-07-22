/// A HTTP server provider trait for making HTTP servers.
///
/// Note: This trait should not be implemented/returned by httpserver_provider if HTTP server support should not be available in the context.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait HTTPServerProvider: 'static + Clone {
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str, path: String) -> Result<(), crate::Error>;
}
