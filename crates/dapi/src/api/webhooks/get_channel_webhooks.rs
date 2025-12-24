use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannelWebhooks {
    pub channel_id: serenity::all::GenericChannelId,
}

impl ApiReq for GetChannelWebhooks {
    type Resp = serde_json::Value;

    /// Retrieves the list of webhooks for the given channel after verifying the bot has `MANAGE_WEBHOOKS`.
    ///
    /// # Errors
    /// Returns an error if the current bot user cannot be obtained, the bot does not have `MANAGE_WEBHOOKS` for the channel, or if fetching webhooks fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::webhooks::GetChannelWebhooks;
    /// # use serenity::all::GenericChannelId;
    /// # async fn example<T: dapi::DiscordProvider>(ctx: &dapi::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = GetChannelWebhooks { channel_id: GenericChannelId(123) };
    /// let webhooks = req.execute(ctx).await?;
    /// // `webhooks` is a `serde_json::Value` containing the channel's webhooks.
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(
            bot_user.id,   
            self.channel_id,
            Permissions::MANAGE_WEBHOOKS,
        )
        .await?;

        let webhooks = this.controller()
            .get_channel_webhooks(self.channel_id)
            .await?;

        Ok(webhooks)
    }

    /// Convert the request into the corresponding `crate::apilist::API` enum variant.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::GetChannelWebhooks` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetChannelWebhooks { channel_id: serenity::all::GenericChannelId::from(1u64) };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::GetChannelWebhooks(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannelWebhooks(self)
    }
}