use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildPreview;

impl ApiReq for GetGuildPreview {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let guild_preview = this
            .controller()
            .get_guild_preview()
            .await?;

        Ok(guild_preview)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildPreview(self)
    }
}
