use crate::{ApiReq, Permissions,UserId, context::DiscordContext, controller::DiscordProvider, dhttp::UserPagination};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetGuildBans {
    pub limit: Option<u16>,
    pub before: Option<UserId>,
    pub after: Option<UserId>,
}

impl ApiReq for GetGuildBans {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::BAN_MEMBERS)
        .await?;

        let mut target = None;
        if let Some(before) = self.before {
            target = Some(UserPagination::Before(before));
        } else if let Some(after) = self.after {
            target = Some(UserPagination::After(after));
        }

        if let Some(limit) = self.limit {
            if limit > 1000 {
                return Err("Limit must be less than 1000".into());
            }
        }

        let bans = this.controller()
            .get_guild_bans(target, self.limit)
            .await?;

        Ok(bans)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildBans(self)
    }
}
