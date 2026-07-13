use crate::{ApiReq, UserId, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListGuildMembers {
    pub limit: Option<u16>,
    pub after: Option<UserId>,
}

impl ApiReq for ListGuildMembers {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        if let Some(limit) = self.limit {
            if limit > 1000 {
                return Err("The maximum `limit` for get_guild_members is 1000".into());
            }
        }

        let data = this.controller()
            .list_guild_members(self.limit, self.after)
            .await?;

        Ok(data)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ListGuildMembers(self)
    }
}
