use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, serenity_backports::{member_permissions, highest_role}};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildMemberRole {
    pub user_id: serenity::all::UserId,
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

impl ApiReq for RemoveGuildMemberRole {
    type Resp = ();

    /// Removes a role from a guild member after validating the provided reason, the bot's permissions, and role hierarchy.
    ///
    /// This request validates the removal reason, ensures the bot is a guild member with the `Manage Roles` permission,
    /// verifies the bot has a highest role and that the target role exists and is lower than the bot's highest role,
    /// then instructs the controller to remove the role from the specified member.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the provided reason is invalid,
    /// - the current bot user cannot be determined,
    /// - the bot is not a member of the guild,
    /// - the bot lacks the `Manage Roles` permission,
    /// - the bot has no roles in the guild,
    /// - the target role does not exist in the guild,
    /// - the target role is higher than or equal to the bot's highest role,
    /// - or if the controller call or serialization/deserialization operations fail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serenity::all::{UserId, RoleId};
    /// # use crate::api::guilds::RemoveGuildMemberRole;
    /// async fn example(ctx: &crate::DiscordContext<impl crate::DiscordProvider>) -> Result<(), crate::Error> {
    ///     let req = RemoveGuildMemberRole {
    ///         user_id: UserId(123),
    ///         role_id: RoleId(456),
    ///         reason: "remove stale role".to_string(),
    ///     };
    ///     req.execute(ctx).await
    /// }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        let bot_member_json = this.controller().get_guild_member(bot_user.id).await?;

        if bot_member_json.is_null() {
            return Err("Bot user not found in guild".into());
        }

        let bot_member = serde_json::from_value::<serenity::all::Member>(bot_member_json)?;

        let guild_json = this.controller().get_guild().await?;
        let guild = serde_json::from_value::<serenity::all::PartialGuild>(guild_json)?;

        let resolved = member_permissions(&guild, &bot_member);

        if !resolved.manage_roles() {
            return Err("Bot does not have permission to manage roles".into());
        }

        let Some(bot_highest_role) = highest_role(&guild, &bot_member) else {
            return Err("Bot does not have a role".into());
        };

        let Some(role_to_remove) = guild.roles.get(&self.role_id) else {
            return Err("Role to remove from member not found in guild".into());
        };

        if role_to_remove >= bot_highest_role {
            return Err(format!("Bot does not have permission to remove the requested role ({}, ``{}``) from the member", role_to_remove.id, role_to_remove.name.replace("`", "\\`")).into());
        }

        this.controller()
            .remove_guild_member_role(self.user_id, self.role_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Convert the request into the corresponding `apilist::API` variant.
    
    ///
    
    /// # Returns
    
    ///
    
    /// An `apilist::API` value that wraps this `RemoveGuildMemberRole` request.
    
    ///
    
    /// # Examples
    
    ///
    
    /// ```
    
    /// use crates::dapi::api::guilds::remove_guild_member_role::RemoveGuildMemberRole;
    
    /// use serenity::all::{UserId, RoleId};
    
    ///
    
    /// let req = RemoveGuildMemberRole {
    
    ///     user_id: UserId(1),
    
    ///     role_id: RoleId(2),
    
    ///     reason: "cleanup".into(),
    
    /// };
    
    /// let api = req.to_apilist();
    
    /// // `api` is `apilist::API::RemoveGuildMemberRole(req)`
    
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::RemoveGuildMemberRole(self)
    }
}