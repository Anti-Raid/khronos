use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetWebhook {
    pub webhook_id: serenity::all::WebhookId,
}

impl ApiReq for GetWebhook {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let webhook = this.controller()
            .get_webhook(self.webhook_id)
            .await?;

        let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
            return Err("Webhook does not belong to a guild".into());
        };

        if guild_id != &this.controller().guild_context()?.to_string() {
            return Err("Webhook does not belong to a guild".into());
        }

        Ok(webhook)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetWebhook(self)
    }
}
