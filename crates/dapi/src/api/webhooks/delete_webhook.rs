use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteWebhook {
    pub webhook_id: serenity::all::WebhookId,
    pub reason: String,
}

impl ApiReq for DeleteWebhook {
    type Resp = ();

    /// Deletes the specified webhook from the current guild after validating the provided reason and verifying the bot has the `MANAGE_WEBHOOKS` permission.
    ///
    /// Returns `()` on success.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serenity::all::WebhookId;
    /// # use dapi::api::webhooks::DeleteWebhook;
    /// # async fn example(ctx: &dapi::DiscordContext<impl dapi::DiscordProvider>) -> Result<(), Box<dyn std::error::Error>> {
    /// let req = DeleteWebhook {
    ///     webhook_id: WebhookId(123456789012345678),
    ///     reason: "Removing unused webhook".into(),
    /// };
    ///
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(
            bot_user.id,   
            Permissions::MANAGE_WEBHOOKS,
        )
        .await?;


        let webhook = this.controller()
            .get_webhook(self.webhook_id)
            .await?;

        let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
            return Err("Webhook does not belong to a guild".into());
        };

        if guild_id != &this.guild_id().to_string() {
            return Err("Webhook does not belong to a guild".into());
        }
        
        this.controller()
            .delete_webhook(
                self.webhook_id,
                Some(self.reason.as_str())
            )
            .await?;

        Ok(())
    }

    /// Convert this `DeleteWebhook` request into the corresponding API enum variant.
    ///
    /// # Returns
    ///
    /// The `API::DeleteWebhook` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = DeleteWebhook { webhook_id: 123.into(), reason: "cleanup".into() };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::DeleteWebhook(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteWebhook(self)
    }
}