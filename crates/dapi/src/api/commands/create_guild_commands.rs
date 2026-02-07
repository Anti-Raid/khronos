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

            {
                let Some(ref name) = cmd.fields.name else {
                    return Err("Command name is required".into());
                };

                if !this.controller().can_manage_guild_command(name) {
                    return Err("Cannot create this guild command: not authorized".into());
                }
            }
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
