use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildBans {
    pub limit: Option<serenity::nonmax::NonMaxU16>,
    pub before: Option<serenity::all::UserId>,
    pub after: Option<serenity::all::UserId>,
}

impl ApiReq for GetGuildBans {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::BAN_MEMBERS)
        .await?;

        let mut target = None;
        if let Some(before) = self.before {
            target = Some(serenity::all::UserPagination::Before(before));
        } else if let Some(after) = self.after {
            target = Some(serenity::all::UserPagination::After(after));
        }

        if let Some(limit) = self.limit {
            if limit.get() > 1000 {
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
