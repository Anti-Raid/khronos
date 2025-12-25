use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SearchGuildMembers {
    pub query: String,
    pub limit: Option<serenity::nonmax::NonMaxU16>,
}

impl ApiReq for SearchGuildMembers {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        if let Some(limit) = self.limit {
            if limit.get() > 1000 {
                return Err("The maximum `limit` for get_guild_members is 1000".into());
            }
        }

        let data = this.controller()
            .search_guild_members(&self.query, self.limit)
            .await?;

        Ok(data)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::SearchGuildMembers(self)
    }
}
