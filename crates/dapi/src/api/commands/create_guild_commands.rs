use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateCommand};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildCommands {
    pub data: Vec<CreateCommand>,
}

impl ApiReq for CreateGuildCommands {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        for cmd in &self.data {
            cmd.validate()?;
        }

        let resp = this.controller()
            .create_guild_commands(&self.data)
            .await?;

        Ok(resp)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildCommands(self)
    }
}
