use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ExecuteWebhook};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecuteWebhookRequest {
    pub webhook_id: serenity::all::WebhookId,
    pub webhook_token: String,
    pub thread_id: Option<serenity::all::ThreadId>,
    pub data: ExecuteWebhook,
}

impl ApiReq for ExecuteWebhookRequest {
    type Resp = serde_json::Value;

    /// Executes this webhook request and returns the webhook's JSON response.
    ///
    /// Validates the contained payload, verifies the webhook belongs to the current guild, includes any provided attachments as files, and forwards the execution to the controller.
    ///
    /// # Returns
    ///
    /// `Ok(serde_json::Value)` with the webhook execution result on success; `Err` if payload validation fails, the webhook is missing or not associated with the current guild, or the controller returns an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Given `req: ExecuteWebhookRequest` and `ctx: &DiscordContext<_>`
    /// let resp = tokio::runtime::Handle::current().block_on(async { req.execute(ctx).await });
    /// match resp {
    ///     Ok(json) => println!("Webhook executed: {}", json),
    ///     Err(e) => eprintln!("Execution failed: {}", e),
    /// }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        // Ensure webhook exists on the same server as the guild we're in
        let webhook = this.controller()
            .get_webhook(self.webhook_id)
            .await?;

        let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
            return Err("Webhook does not belong to a guild".into());
        };

        if guild_id != &this.guild_id().to_string() {
            return Err("Webhook does not belong to a guild".into());
        }

        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .execute_webhook(self.webhook_id, &self.webhook_token, self.thread_id, &self.data, files)
            .await?;

        Ok(msg)
    }

    /// Converts the request into the API enum variant used for API listing.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::ExecuteWebhook` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = crate::api::webhooks::execute_webhook::ExecuteWebhookRequest {
    ///     webhook_id: serenity::all::WebhookId::from(1),
    ///     webhook_token: String::from("token"),
    ///     thread_id: None,
    ///     data: crate::api::webhooks::execute_webhook::ExecuteWebhook::default(),
    /// };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::ExecuteWebhook(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ExecuteWebhook(self)
    }
}