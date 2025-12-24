use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditRole, get_format_from_image_data};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildRole {
    pub reason: String,
    pub data: EditRole,
}

impl ApiReq for CreateGuildRole {
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

        let (guild, _, bot_perms) = this.check_permissions(
            bot_user.id,
            Permissions::MANAGE_ROLES,
        )
        .await?; 

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
            .create_guild_role(self.data, Some(self.reason.as_str()))
            .await?;

        Ok(role)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildRole(self)
    }
}
