use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, serenity_backports::highest_role};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteGuildRole {
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

impl ApiReq for DeleteGuildRole {
    type Resp = ();

    /// Deletes a role from the current guild after validating reason, permissions, and role hierarchy.
    ///
    /// Performs validation of the provided reason, ensures the bot has the `MANAGE_ROLES` permission,
    /// confirms the target role exists in the guild, and verifies the bot's highest role is strictly
    /// higher than the role being deleted before invoking the controller to remove the role.
    ///
    /// # Errors
    ///
    /// Returns `Err` if any of the following occur:
    /// - the target `role_id` is the guild's default `@everyone` role;
    /// - the provided `reason` is rejected by validation;
    /// - the current bot user or guild context is missing;
    /// - the bot lacks the `MANAGE_ROLES` permission;
    /// - the target role cannot be found on the server;
    /// - the bot's highest role is not higher than the target role;
    /// - the controller fails to delete the role.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn example<T: dapi::DiscordProvider>(ctx: &dapi::DiscordContext<T>) -> Result<(), dapi::Error> {
    /// let req = dapi::api::guilds::delete_guild_role::DeleteGuildRole {
    ///     role_id: serenity::all::RoleId(123),
    ///     reason: "Clean up unused role".into(),
    /// };
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
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

        if bot_highest_role <= mod_role {
            return Err("The bot must have a role that is higher than the role it is trying to modify".into());
        }

        this.controller()
            .delete_guild_role(self.role_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Convert this `DeleteGuildRole` request into the corresponding `apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = DeleteGuildRole { role_id: serenity::all::RoleId::from(1), reason: "cleanup".into() };
    /// let api = req.to_apilist();
    /// // `api` is the `apilist::API::DeleteGuildRole` variant wrapping the original request.
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteGuildRole(self)
    }
}