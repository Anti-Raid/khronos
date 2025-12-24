use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteWebhook {
    pub webhook_id: serenity::all::WebhookId,
    pub reason: String,
}

impl ApiReq for DeleteWebhook {
    type Resp = ();

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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteWebhook(self)
    }
}
