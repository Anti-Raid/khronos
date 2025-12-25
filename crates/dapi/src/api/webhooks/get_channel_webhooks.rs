use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannelWebhooks {
    pub channel_id: serenity::all::GenericChannelId,
}

impl ApiReq for GetChannelWebhooks {
    type Resp = serde_json::Value;

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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannelWebhooks(self)
    }
}
