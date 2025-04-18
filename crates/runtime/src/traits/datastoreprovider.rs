pub use super::ir::DataStoreImpl;
use std::rc::Rc;

/// A data store provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait DataStoreProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns a builtin data store given its name
    fn get_builtin_data_store(&self, name: &str) -> Option<Rc<dyn DataStoreImpl>>;

    /// Returns all public builtin data stores
    fn public_builtin_data_stores(&self) -> Vec<String>;
}
