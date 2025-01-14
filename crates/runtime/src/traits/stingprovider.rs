use antiraid_types::stings::{Sting, StingCreate};

/// A sting provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait StingProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns a list of all stings with a page number
    async fn list(&self, page: usize) -> Result<Vec<Sting>, crate::Error>;

    /// Returns a sting by ID, should return None if the sting does not exist
    async fn get(&self, id: uuid::Uuid) -> Result<Option<Sting>, crate::Error>;

    /// Creates a new sting returning the ID of the created sting.
    ///
    /// Should error if guild id mismatches providers' expected guild id
    async fn create(&self, sting: StingCreate) -> Result<uuid::Uuid, crate::Error>;

    /// Updates a sting to a new sting object
    ///
    /// Should error if guild id of the sting object mismatches providers' expected guild id
    /// or if the guild the sting is associated with mismatches the providers' expected guild id
    async fn update(&self, sting: Sting) -> Result<(), crate::Error>;

    /// Deletes a sting by ID
    ///
    /// Should error if the guild the sting is associated with mismatches the providers' expected guild id
    async fn delete(&self, id: uuid::Uuid) -> Result<(), crate::Error>;
}
