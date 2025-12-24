use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetWebhook {
    pub webhook_id: serenity::all::WebhookId,
}

impl ApiReq for GetWebhook {
    type Resp = serde_json::Value;

    /// Fetches a webhook and verifies it belongs to the current guild in the given context.
    ///
    /// Returns the webhook JSON value when the webhook's `guild_id` is present and matches
    /// the context's guild ID; otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns an error with the message `"Webhook does not belong to a guild"` when the
    /// webhook has no `guild_id` or when that `guild_id` does not match the context's guild.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serenity::all::WebhookId;
    /// # use serde_json::Value;
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>, id: WebhookId) -> Result<Value, crate::Error> {
    /// let req = crate::api::webhooks::get_webhook::GetWebhook { webhook_id: id };
    /// let webhook_json = req.execute(ctx).await?;
    /// # Ok(webhook_json)
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let webhook = this.controller()
            .get_webhook(self.webhook_id)
            .await?;

        let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
            return Err("Webhook does not belong to a guild".into());
        };

        if guild_id != &this.guild_id().to_string() {
            return Err("Webhook does not belong to a guild".into());
        }

        Ok(webhook)
    }

    /// Convert this `GetWebhook` request into the corresponding `API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetWebhook { webhook_id: serenity::all::WebhookId::from(123u64) };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::GetWebhook(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetWebhook(self)
    }
}