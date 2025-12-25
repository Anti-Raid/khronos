use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildRole {
    pub role_id: serenity::all::RoleId,
}

impl ApiReq for GetGuildRole {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let role = this.controller()
            .get_guild_role(self.role_id)
            .await?;

        Ok(role)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildRole(self)
    }
}
