use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildRoles;

impl ApiReq for GetGuildRoles {
    type Resp = serde_json::Value;

    /// Request the guild's roles from the configured Discord provider.
    ///
    /// # Returns
    ///
    /// The JSON value containing the guild roles as returned by the provider.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::api::guilds::get_guild_roles::GetGuildRoles;
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let roles = GetGuildRoles.execute(ctx).await?;
    /// println!("roles: {}", roles);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let roles = this.controller()
            .get_guild_roles()
            .await?;

        Ok(roles)
    }

    /// Convert the request into the `API::GetGuildRoles` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::guilds::get_guild_roles::GetGuildRoles;
    /// use crate::apilist::API;
    ///
    /// let req = GetGuildRoles;
    /// let api = req.to_apilist();
    /// if let API::GetGuildRoles(inner) = api {
    ///     let _ = inner; // `inner` is the original `GetGuildRoles` value
    /// } else {
    ///     panic!("expected API::GetGuildRoles variant");
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildRoles(self)
    }
}