use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SearchGuildMembers {
    pub query: String,
    pub limit: Option<serenity::nonmax::NonMaxU16>,
}

impl ApiReq for SearchGuildMembers {
    type Resp = serde_json::Value;

    /// Executes the guild member search using the provided Discord context.
    ///
    /// Validates that `limit`, if provided, does not exceed 1000 and returns an error with the message
    /// "The maximum `limit` for get_guild_members is 1000" when it does. On success, delegates the
    /// search to the context's controller and returns the resulting JSON value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crates_dapi::api::guilds::search_guild_members::SearchGuildMembers;
    ///
    /// # async fn example(ctx: &cratessome::DiscordContext<'_>) -> Result<(), Box<dyn std::error::Error>> {
    /// let req = SearchGuildMembers { query: "alice".into(), limit: Some(100) };
    /// let resp = req.execute(ctx).await?;
    /// // `resp` is a `serde_json::Value` containing the search results.
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        if let Some(limit) = self.limit {
            if limit.get() > 1000 {
                return Err("The maximum `limit` for get_guild_members is 1000".into());
            }
        }

        let data = this.controller()
            .search_guild_members(&self.query, self.limit)
            .await?;

        Ok(data)
    }

    /// Convert this request into the `API::SearchGuildMembers` enum variant used for routing.
    ///
    /// Returns the `API` enum variant that wraps this `SearchGuildMembers` request.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::guilds::search_guild_members::SearchGuildMembers;
    /// let req = SearchGuildMembers { query: "alice".into(), limit: None };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::SearchGuildMembers(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::SearchGuildMembers(self)
    }
}