use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, serenity_backports::highest_role};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteGuildRole {
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

impl ApiReq for DeleteGuildRole {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        if self.role_id.to_string() == this.guild_id().to_string() {
            return Err("Cannot remove the default @everyone role".into());
        }

        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        let (guild, member, _) = this.check_permissions(
            bot_user.id,
            Permissions::MANAGE_ROLES,
        )
        .await?; 

        let bot_highest_role = highest_role(&guild, &member)
            .ok_or_else(|| "The bot must have roles in order to modify a guild role")?;

        let mod_role = guild.roles.get(&self.role_id)
            .ok_or_else(|| "The role being modified could not be found on the server")?;

        if bot_highest_role <= *mod_role {
            return Err("The bot must have a role that is higher than the role it is trying to modify".into());
        }

        this.controller()
            .delete_guild_role(self.role_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteGuildRole(self)
    }
}
