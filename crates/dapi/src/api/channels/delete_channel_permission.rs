use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteChannelPermission {
    pub channel_id: serenity::all::GenericChannelId,
    pub overwrite_id: serenity::all::TargetId,
    pub reason: String,
}

impl ApiReq for DeleteChannelPermission {
    type Resp = ();

    /// Deletes a permission overwrite for a channel using the provided overwrite target and reason.
    ///
    /// Validates the provided reason, ensures the current bot user exists and has the `MANAGE_ROLES` permission,
    /// and then instructs the controller to remove the permission overwrite for the given channel.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if validation, permission checks, the current user lookup, or controller call fail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crates::api::channels::DeleteChannelPermission;
    /// # async fn example(ctx: &DiscordContext<impl DiscordProvider>) -> Result<(), crate::Error> {
    /// let req = DeleteChannelPermission {
    ///     channel_id,
    ///     overwrite_id,
    ///     reason: "remove obsolete overwrite".into(),
    /// };
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::MANAGE_ROLES)
        .await?;

        this
            .controller()
            .delete_channel_permission(self.channel_id, self.overwrite_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Convert this request into its `apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::api::channels::delete_channel_permission::DeleteChannelPermission;
    /// let req = DeleteChannelPermission {
    ///     channel_id: 0.into(),
    ///     overwrite_id: 0.into(),
    ///     reason: "cleanup".into(),
    /// };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::DeleteChannelPermission(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteChannelPermission(self)
    }
}