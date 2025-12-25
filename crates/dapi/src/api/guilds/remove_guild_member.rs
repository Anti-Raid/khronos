use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildMember {
    pub user_id: serenity::all::UserId,
    pub reason: String,
}

impl ApiReq for RemoveGuildMember {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions_and_hierarchy(
            bot_user.id,
            self.user_id,
            Permissions::KICK_MEMBERS,
        )
        .await?;

        this.controller()
            .remove_guild_member(self.user_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::RemoveGuildMember(self)
    }
}
