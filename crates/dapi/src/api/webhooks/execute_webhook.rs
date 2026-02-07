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

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        // Ensure webhook exists on the same server as the guild we're in
        let webhook = this.controller()
            .get_webhook(self.webhook_id)
            .await?;

        let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
            return Err("Webhook does not belong to a guild".into());
        };

        if guild_id != &this.controller().guild_context()?.to_string() {
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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ExecuteWebhook(self)
    }
}
