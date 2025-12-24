use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::guilds::ModifyRolePosition, serenity_backports::highest_role};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildRolePositions {
    pub data: Vec<ModifyRolePosition>,
    pub reason: String,
}

impl ApiReq for ModifyGuildRolePositions {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };    

        let (guild, member, _) = this.check_permissions(bot_user.id, Permissions::MANAGE_ROLES)
            .await?;

        // Check roles
        let bot_highest_role = highest_role(&guild, &member)
            .ok_or_else(|| "Bot does not have a role")?;

        for modify_role_position in self.data.iter() {
            let Some(role) = guild.roles.get(&modify_role_position.id) else {
                return Err("Role not found in guild".into());
            };

            // Check current
            if role >= bot_highest_role || modify_role_position >= bot_highest_role {
                return Err(format!("Bot does not have permission to modify the requested role ({}, ``{}``)", role.id, role.name.replace("`", "\\`")).into());
            }
        }

        let roles = this.controller()
            .modify_guild_role_positions(self.data.iter(), Some(self.reason.as_str()))
            .await?;

        Ok(roles)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildRolePositions(self)
    }
}
