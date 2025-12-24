use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannel {
    pub channel_id: serenity::all::GenericChannelId,
}

impl ApiReq for GetChannel {
    type Resp = serde_json::Value;

    /// Fetches a channel by its ID from the provided Discord context.
    ///
    /// On success returns the channel as a JSON value produced by the provider controller.
    ///
    /// # Errors
    ///
    /// Returns a `crate::Error` if the controller fails to retrieve the channel.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serenity::all::GenericChannelId;
    /// use your_crate::api::channels::GetChannel;
    ///
    /// let req = GetChannel { channel_id: GenericChannelId(123456789012345678) };
    /// // let resp = req.execute(&discord_context).await?;
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let channel = this
            .controller()
            .get_channel(self.channel_id)
            .await?;

        Ok(channel)
    }

    /// Wraps this `GetChannel` request in the corresponding `crate::apilist::API` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::channels::get_channel::GetChannel;
    /// use crate::apilist::API;
    ///
    /// let req = GetChannel {
    ///     channel_id: serenity::all::GenericChannelId::Channel(serenity::all::ChannelId(1)),
    /// };
    ///
    /// let api = req.to_apilist();
    /// assert!(matches!(api, API::GetChannel(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannel(self)
    }
}