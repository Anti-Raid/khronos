use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildRoles;

impl ApiReq for GetGuildRoles {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let roles = this.controller()
            .get_guild_roles()
            .await?;

        Ok(roles)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildRoles(self)
    }
}
