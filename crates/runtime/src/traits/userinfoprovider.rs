use antiraid_types::userinfo::UserInfo;

/// A user info provider.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait UserInfoProvider: 'static + Clone {
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Returns the user info for a user ID
    async fn get(&self, user_id: serenity::all::UserId) -> Result<UserInfo, crate::Error>;
}
