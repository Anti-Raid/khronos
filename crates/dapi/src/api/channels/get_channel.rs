use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannel {
    pub channel_id: serenity::all::GenericChannelId,
}

impl ApiReq for GetChannel {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let channel = this
            .controller()
            .get_channel(self.channel_id)
            .await?;

        Ok(channel)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannel(self)
    }
}
