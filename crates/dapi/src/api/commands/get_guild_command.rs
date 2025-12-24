use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildCommand {
    pub command_id: serenity::all::CommandId,
}

impl ApiReq for GetGuildCommand {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let resp = this.controller()
            .get_guild_command(self.command_id)
            .await?;

        Ok(resp)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildCommand(self)
    }
}
