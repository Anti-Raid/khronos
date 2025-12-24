use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditRole, get_format_from_image_data};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildRole {
    pub reason: String,
    pub data: EditRole,
}

impl ApiReq for CreateGuildRole {
    type Resp = serde_json::Value;

    /// Create a new guild role after validating the request data, the bot's permissions, and an optional role icon.
    ///
    /// This performs the following checks before creating the role:
    /// - Validates the provided reason.
    /// - If a role name is provided, ensures its length is between 1 and 100 characters.
    /// - Ensures the current bot user is available.
    /// - Verifies the bot has MANAGE_ROLES and, if specific permissions are requested for the new role, that the bot possesses those permissions.
    /// - If an icon is provided, ensures the guild supports role icons and that the image format is PNG, JPEG, or GIF.
    ///
    /// # Returns
    ///
    /// The created role as a JSON value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::{CreateGuildRole, EditRole, DiscordContext, DiscordProvider};
    /// # async fn example<T: DiscordProvider>(ctx: &DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = CreateGuildRole {
    ///     reason: "initial setup".into(),
    ///     data: EditRole::default(), // fill fields as needed
    /// };
    /// let role_json = req.execute(ctx).await?;
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

    /// Wraps this `CreateGuildRole` request in the API enum for dispatch.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = CreateGuildRole { reason: String::from("reason"), data: EditRole::default() };
    /// let api = req.to_apilist();
    /// if let crate::apilist::API::CreateGuildRole(_) = api {
    ///     // dispatched as CreateGuildRole
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildRole(self)
    }
}