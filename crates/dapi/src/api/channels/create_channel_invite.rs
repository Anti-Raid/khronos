use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateInvite};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateChannelInvite {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: CreateInvite,
    pub reason: String,
}

impl ApiReq for CreateChannelInvite {
    type Resp = serde_json::Value;

    /// Creates an invite for the specified channel after validating the provided reason and required permissions.
    ///
    /// The request validates the reason string, ensures the current bot user exists, checks that the bot has
    /// the `CREATE_INSTANT_INVITE` permission on the target channel, and then delegates to the controller to
    /// create the invite.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the created invite object.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the reason is invalid, the current user is not available, the bot lacks the
    /// `CREATE_INSTANT_INVITE` permission for the channel, or if the controller call fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # async fn run_example() -> Result<(), crate::Error> {
    /// use your_crate::api::channels::CreateChannelInvite;
    /// use your_crate::{DiscordContext, DiscordProvider, CreateInvite};
    ///
    /// // Construct request (fields shown conceptually)
    /// let req = CreateChannelInvite {
    ///     channel_id: /* GenericChannelId */,
    ///     data: CreateInvite { /* ... */ },
    ///     reason: "Creating an invite".to_string(),
    /// };
    ///
    /// // Execute using a DiscordContext implementation
    /// let ctx: DiscordContext<impl DiscordProvider> = /* obtain context */;
    /// let invite_json = req.execute(&ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::CREATE_INSTANT_INVITE)
        .await?;

        let invite = this
            .controller()
            .create_channel_invite(self.channel_id, &self.data, Some(self.reason.as_str()))
            .await?;

        Ok(invite)
    }

    /// Wraps this request in the API enum as the `CreateChannelInvite` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// // given `req: CreateChannelInvite`
    /// let api = req.to_apilist();
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateChannelInvite(self)
    }
}