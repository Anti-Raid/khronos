use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateCommand};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildCommand {
    pub data: CreateCommand,
}

impl ApiReq for CreateGuildCommand {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        {
            let Some(ref name) = self.data.fields.name else {
                return Err("Command name is required".into());
            };

            if !this.controller().can_manage_guild_command(name) {
                return Err("Cannot create this guild command: not authorized".into());
            }
        }

        let resp = this.controller()
            .create_guild_command(&self.data)
            .await?;

        Ok(resp)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildCommand(self)
    }
}
