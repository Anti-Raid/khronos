use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildMember {
    pub user_id: serenity::all::UserId,
}

impl ApiReq for GetGuildMember {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let data = this.controller()
            .get_guild_member(self.user_id)
            .await?;

        Ok(data)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildMember(self)
    }
}
