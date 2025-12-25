use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateWebhook};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateWebhookRequest {
    pub channel_id: serenity::all::GenericChannelId,
    pub reason: String,
    pub data: CreateWebhook,
}

impl ApiReq for CreateWebhookRequest {
    type Resp = serde_json::Value;

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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateWebhook(self)
    }
}
