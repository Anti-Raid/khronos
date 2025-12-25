use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditRole, get_format_from_image_data, serenity_backports::highest_role};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildRole {
    pub role_id: serenity::all::RoleId,
    pub reason: String,
    pub data: EditRole,
}

impl ApiReq for ModifyGuildRole {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        if let Some(ref name) = self.data.name {
            if name.len() > 100 || name.is_empty() {
                return Err("Role name must be a maximum of 100 characters and not empty".into());
            }
        }

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        let (guild, member, bot_perms) = this.check_permissions(
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

        let mut guild_has_role_icons = false;
        for feature in guild.features.iter() {
            if feature.as_str() == "ROLE_ICONS" { 
                guild_has_role_icons = true 
            }
        }
        
        if let Some(permissions) = self.data.permissions {
            if !bot_perms.contains(permissions) {
                return Err(format!("Bot does not have permissions: {:?}", permissions.difference(bot_perms)).into());
            }
        }

        if let Some(icon) = self.data.icon.as_inner_ref() {
            if !guild_has_role_icons {
                return Err("Guild does not have the Role Icons feature and as such cannot create a role with a role_icon field".into());
            }

            let format = get_format_from_image_data(icon)?;

            if format != "png" && format != "jpeg" && format != "gif" {
                return Err("Icon must be a PNG, JPEG, or GIF format".into());
            }
        }

        let role = this.controller()
            .modify_guild_role(self.role_id, self.data, Some(self.reason.as_str()))
            .await?;

        Ok(role)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildRole(self)
    }
}
