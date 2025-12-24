use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListGuildMembers {
    pub limit: Option<serenity::nonmax::NonMaxU16>,
    pub after: Option<serenity::all::UserId>,
}

impl ApiReq for ListGuildMembers {
    type Resp = serde_json::Value;

    /// Execute the request to list guild members using the provided Discord context.
    ///
    /// # Errors
    /// Returns an error if `limit` is greater than 1000 or if the underlying provider/controller call fails.
    /// The specific validation error message is: "The maximum `limit` for get_guild_members is 1000".
    ///
    /// # Returns
    /// A `serde_json::Value` containing the API response data for the requested guild members.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::api::guilds::list_guild_members::ListGuildMembers;
    ///
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = ListGuildMembers { limit: Some(100), after: None };
    /// let resp = req.execute(ctx).await?;
    /// println!("{}", resp);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        if let Some(limit) = self.limit {
            if limit.get() > 1000 {
                return Err("The maximum `limit` for get_guild_members is 1000".into());
            }
        }

        let data = this.controller()
            .list_guild_members(self.limit, self.after)
            .await?;

        Ok(data)
    }

    /// Convert this request into the `crate::apilist::API::ListGuildMembers` enum variant.
    ///
    /// # Returns
    ///
    /// The `API::ListGuildMembers` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use crates_dapi::api::guilds::list_guild_members::ListGuildMembers;
    /// let req = ListGuildMembers { limit: None, after: None };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::ListGuildMembers(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ListGuildMembers(self)
    }
}