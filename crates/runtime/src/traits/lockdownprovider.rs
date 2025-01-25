use super::ir::Lockdown;

/// A lockdown provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait LockdownProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns a list of all lockdowns.
    async fn list(&self) -> Result<Vec<Lockdown>, crate::Error>;

    /// Creates a quick server lockdown
    ///
    /// Should return the ID of the created lockdown
    async fn qsl(&self, reason: String) -> Result<uuid::Uuid, crate::Error>;

    /// Creates a traditional server lockdown
    ///
    /// Should return the ID of the created lockdown
    async fn tsl(&self, reason: String) -> Result<uuid::Uuid, crate::Error>;

    /// Creates a single channel lockdown given channel ID
    ///
    /// Should return the ID of the created lockdown
    async fn scl(
        &self,
        channel_id: serenity::all::ChannelId,
        reason: String,
    ) -> Result<uuid::Uuid, crate::Error>;

    /// Creates a role lockdown given role ID
    ///
    /// Should return the ID of the created lockdown
    async fn role(
        &self,
        role_id: serenity::all::RoleId,
        reason: String,
    ) -> Result<uuid::Uuid, crate::Error>;

    /// Removes a lockdown given ID
    async fn remove(&self, id: uuid::Uuid) -> Result<(), crate::Error>;
}
