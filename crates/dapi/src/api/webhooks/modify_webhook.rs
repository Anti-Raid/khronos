use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditWebhook, get_format_from_image_data};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyWebhook {
    pub webhook_id: serenity::all::WebhookId,
    pub data: EditWebhook,
    pub reason: String,
}

impl ApiReq for ModifyWebhook {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        if let Some(ref avatar) = self.data.avatar.as_inner_ref() {
            let format = get_format_from_image_data(avatar)?;

            if format != "png" && format != "jpeg" && format != "gif" {
                return Err("Icon must be a PNG, JPEG, or GIF format".into());
            }
        }

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        if let Some(channel_id) = self.data.channel_id {
            this.check_channel_permissions(
                bot_user.id,   
                channel_id.widen(),
                Permissions::empty(),
            )
            .await?;
        }

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
        
        let webhook = this.controller()
            .modify_webhook(
                self.webhook_id,
                self.data,
                Some(self.reason.as_str())
            )
            .await?;

        Ok(webhook)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyWebhook(self)
    }
}
