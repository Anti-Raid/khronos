use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildRole {
    pub role_id: serenity::all::RoleId,
}

impl ApiReq for GetGuildRole {
    type Resp = serde_json::Value;

    /// Fetches the guild role identified by `role_id` using the provided Discord context.
    ///
    /// On success returns a JSON value representing the guild role as returned by the provider.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serenity::all::RoleId;
    /// # use crate::api::guilds::get_guild_role::GetGuildRole;
    /// # async fn example(ctx: &crate::context::DiscordContext<impl crate::controller::DiscordProvider>) {
    /// let req = GetGuildRole { role_id: RoleId(123) };
    /// let role_json = req.execute(ctx).await.unwrap();
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let role = this.controller()
            .get_guild_role(self.role_id)
            .await?;

        Ok(role)
    }

    /// Convert the request into the corresponding `apilist::API` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use serenity::all::RoleId;
    /// let req = GetGuildRole { role_id: RoleId::from(123u64) };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::GetGuildRole(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildRole(self)
    }
}