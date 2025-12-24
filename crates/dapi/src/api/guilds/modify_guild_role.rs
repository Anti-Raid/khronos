use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditRole, get_format_from_image_data, serenity_backports::{member_permissions, highest_role}};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildRole {
    pub role_id: serenity::all::RoleId,
    pub reason: String,
    pub data: EditRole,
}

impl ApiReq for ModifyGuildRole {
    type Resp = serde_json::Value;

    /// Executes the ModifyGuildRole request: validates inputs, checks permissions and role hierarchy, and updates the guild role.
    ///
    /// Performs validation of the provided reason and role name length, ensures the bot exists and has MANAGE_ROLES permission, verifies the bot's highest role is above the target role, checks guild feature support for role icons and icon format when provided, validates requested permissions against the bot's permissions, and then calls the controller to apply the role modifications.
    ///
    /// # Returns
    ///
    /// The updated role as a `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Context and request construction are environment-specific and omitted here.
    /// // This example demonstrates the intended use pattern.
    /// # async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    /// use crate::api::guilds::ModifyGuildRole;
    ///
    /// // let context: DiscordContext<_> = /* ... */;
    /// // let req = ModifyGuildRole { role_id, reason: "rename".into(), data: edit_role };
    /// // let updated = req.execute(&context).await?;
    ///
    /// # Ok(()) }
    /// ```
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

        if bot_highest_role <= mod_role {
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

    /// Convert this request into the central `API` enum used by the dispatcher.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a `ModifyGuildRole` request `req`, convert it into `API` for the dispatcher.
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::ModifyGuildRole(_) => (),
    ///     _ => unreachable!(),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildRole(self)
    }
}