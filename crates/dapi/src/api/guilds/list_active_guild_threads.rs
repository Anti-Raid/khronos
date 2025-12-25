use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListActiveGuildThreads;

impl ApiReq for ListActiveGuildThreads {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let data = this.controller()
            .list_active_guild_threads()
            .await?;

        Ok(data)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ListActiveGuildThreads(self)
    }
}
