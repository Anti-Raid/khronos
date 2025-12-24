use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateCommand};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildCommand {
    pub data: CreateCommand,
}

impl ApiReq for CreateGuildCommand {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        let resp = this.controller()
            .create_guild_command(&self.data)
            .await?;

        Ok(resp)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildCommand(self)
    }
}
