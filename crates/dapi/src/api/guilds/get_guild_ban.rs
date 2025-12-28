use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetGuildBan {
    pub user_id: serenity::all::UserId,
}

impl ApiReq for GetGuildBan {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };    

        this.check_permissions(bot_user.id, Permissions::BAN_MEMBERS)
        .await?;

        let ban = this.controller()
            .get_guild_ban(self.user_id)
            .await?;

        Ok(ban)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildBan(self)
    }
}
