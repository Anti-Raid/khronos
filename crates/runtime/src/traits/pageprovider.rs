use super::ir::Page;

/// A page provider
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait PageProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Gets the current page for a template
    async fn get_page(&self) -> Option<Page>;

    /// Sets the current page for a template
    ///
    /// Note that this method must also set settingsoperation as desired. By default, a dummy
    /// implementation is provided to enable serde to work
    async fn set_page(&self, page: Page) -> Result<(), crate::Error>;

    /// Deletes the current page for a template
    async fn delete_page(&self) -> Result<(), crate::Error>;
}
