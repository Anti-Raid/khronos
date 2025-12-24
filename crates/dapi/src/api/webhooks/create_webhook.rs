use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateWebhook};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateWebhook {
    pub channel_id: serenity::all::GenericChannelId,
    pub reason: String,
    pub data: CreateWebhook,
}

impl ApiReq for CreateWebhook {
    type Resp = serde_json::Value;

    /// Creates a webhook in the given channel after validating the provided reason and checking that the bot has MANAGE_WEBHOOKS permission.
    ///
    /// The request is validated (reason), the current bot user is required, channel permissions are enforced, and the controller is used to create the webhook with the provided data and reason.
    ///
    /// # Returns
    ///
    /// `serde_json::Value` containing the created webhook object.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serenity::all::GenericChannelId;
    /// use dapi::api::webhooks::create_webhook::CreateWebhook as CreateWebhookReq;
    /// // `CreateWebhook` (the request) contains `channel_id`, `reason`, and `data`.
    /// # async fn example<T: dapi::DiscordProvider>(ctx: &dapi::DiscordContext<T>) -> Result<(), Box<dyn std::error::Error>> {
    /// let req = CreateWebhookReq {
    ///     channel_id: GenericChannelId(123456789012345678),
    ///     reason: "Initial setup".into(),
    ///     data: Default::default(), // fill with appropriate webhook creation data
    /// };
    /// let webhook_json = req.execute(ctx).await?;
    /// println!("Created webhook: {}", webhook_json);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_WEBHOOKS)
        .await?;

        let webhook = this
            .controller()
            .create_webhook(self.channel_id, &self.data, Some(self.reason.as_str()))
            .await?;

        Ok(webhook)
    }

    /// Convert this `CreateWebhook` request into the API registry enum variant.
    ///
    /// # Returns
    ///
    /// `crate::apilist::API::CreateWebhook(self)` - the API enum wrapping this request.
    ///
    /// # Examples
    ///
    /// ```
    /// // Note: constructing a full `CreateWebhook` may require fields not shown here;
    /// // this example demonstrates the conversion only.
    /// let req: crate::api::webhooks::create_webhook::CreateWebhook = unsafe {
    ///     std::mem::MaybeUninit::zeroed().assume_init()
    /// };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::CreateWebhook(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateWebhook(self)
    }
}