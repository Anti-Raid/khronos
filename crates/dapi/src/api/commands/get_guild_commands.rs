use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildCommands;

impl ApiReq for GetGuildCommands {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let resp = this.controller()
            .get_guild_commands()
            .await?;

        Ok(resp)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildCommands(self)
    }
}
