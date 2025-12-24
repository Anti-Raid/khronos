use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildBan {
    pub user_id: serenity::all::UserId,
}

impl ApiReq for GetGuildBan {
    type Resp = serde_json::Value;

    /// Fetches the guild ban information for the specified user after ensuring the bot has ban permissions.
    ///
    /// On success returns the ban data as JSON.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serenity::all::UserId;
    /// # use crate::api::guilds::GetGuildBan;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let req = GetGuildBan { user_id: UserId(123456789) };
    /// let ctx = /* obtain DiscordContext<T> instance */ todo!();
    /// let ban = req.execute(&ctx).await?;
    /// // `ban` is a `serde_json::Value` with the ban details.
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };    

        this.check_permissions(bot_user.id, Permissions::BAN_MEMBERS)
        .await?;

        let ban = this.controller()
            .get_guild_ban(self.user_id)
            .await?;

        Ok(ban)
    }

    /// Convert this request into its API list representation.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = crate::api::guilds::get_guild_ban::GetGuildBan {
    ///     user_id: serenity::all::UserId(1),
    /// };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::GetGuildBan(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildBan(self)
    }
}