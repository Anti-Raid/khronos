use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::guilds::ModifyRolePosition, serenity_backports::highest_role};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildRolePositions {
    pub data: Vec<ModifyRolePosition>,
    pub reason: String,
}

impl ApiReq for ModifyGuildRolePositions {
    type Resp = serde_json::Value;

    /// Executes the ModifyGuildRolePositions request: validates the reason and bot permissions, ensures each requested role exists and is modifiable by the bot, then applies the position changes via the controller.
    ///
    /// Validation performed:
    /// - The provided reason is checked.
    /// - The current bot user must be present.
    /// - The bot must have the `MANAGE_ROLES` permission in the target guild.
    /// - The bot must have a highest role; each requested role must exist in the guild and be below the bot's highest role.
    ///
    /// # Returns
    ///
    /// The JSON response returned by the controller representing the updated roles on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the reason is invalid, the current user is missing, the bot lacks `MANAGE_ROLES`,
    /// the bot has no role, a requested role is not found in the guild, or the bot does not have permission to modify a requested role.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(ctx: &DiscordContext<impl DiscordProvider>) -> Result<(), crate::Error> {
    /// use crate::api::guilds::ModifyGuildRolePositions;
    /// use crate::models::ModifyRolePosition;
    ///
    /// let req = ModifyGuildRolePositions {
    ///     data: vec![ ModifyRolePosition { id: 123.into(), position: 2 } ],
    ///     reason: "Reorder roles".into(),
    /// };
    ///
    /// let response = req.execute(ctx).await?;
    /// // `response` is a serde_json::Value describing the updated roles.
    /// # Ok(())
    /// # }
    /// ```
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

    /// Wraps this request into the API enum so it can be dispatched by the API registry.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::ModifyGuildRolePositions` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = ModifyGuildRolePositions { data: vec![], reason: String::from("reorder") };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::ModifyGuildRolePositions(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildRolePositions(self)
    }
}