use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildChannels;

impl ApiReq for GetGuildChannels {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let chans = this
            .controller()
            .get_guild_channels()
            .await?;

        Ok(chans)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildChannels(self)
    }
}
