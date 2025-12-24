use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannelMessages {
    pub channel_id: serenity::all::GenericChannelId,
    pub target: Option<serenity::all::MessagePagination>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

impl ApiReq for GetChannelMessages {
    type Resp = serde_json::Value;

    /// Fetches messages from a channel while enforcing required channel permissions.
    ///
    /// This request checks the current bot user, verifies the bot has `VIEW_CHANNEL` for the target
    /// channel, and if the channel is a voice channel also requires the `CONNECT` permission.
    /// On success returns the controller's raw JSON response containing the fetched messages.
    ///
    /// # Errors
    ///
    /// Returns an error if the current bot user is not available, the permission check fails,
    /// the bot lacks `CONNECT` for a voice channel, or the controller fails to fetch messages.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the retrieved messages as returned by the controller.
    ///
    /// # Examples
    ///
    /// ```
    /// // async fn example(ctx: &DiscordContext<_>) -> Result<(), crate::Error> {
    /// //     let req = GetChannelMessages {
    /// //         channel_id: /* GenericChannelId */ todo!(),
    /// //         target: None,
    /// //         limit: Some(50u8.into()),
    /// //     };
    /// //     let resp = req.execute(ctx).await?;
    /// //     // `resp` is a serde_json::Value with the messages
    /// //     Ok(())
    /// // }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        // Perform required checks
        let (_, _, guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::VIEW_CHANNEL).await?;

        if guild_channel.base.kind == serenity::all::ChannelType::Voice 
        && !perms.connect() {
            return Err("Bot does not have permission to connect to the given voice channel".into());
        }

        let msg = this.controller()
            .get_channel_messages(self.channel_id, self.target, self.limit)
            .await?;

        Ok(msg)
    }

    /// Convert this request into its API enum representation.
    ///
    /// # Returns
    ///
    /// The `API::GetChannelMessages` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::messages::get_channel_messages::GetChannelMessages;
    /// let req = GetChannelMessages { channel_id: 1.into(), target: None, limit: None };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::GetChannelMessages(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannelMessages(self)
    }
}