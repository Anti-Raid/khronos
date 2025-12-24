use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuild;

impl ApiReq for GetGuild {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let guild = this
            .controller()
            .get_guild()
            .await?;

        Ok(guild)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuild(self)
    }
}
