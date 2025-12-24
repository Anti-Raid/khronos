use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildCommands;

impl ApiReq for GetGuildCommands {
    type Resp = serde_json::Value;

    /// Fetches the guild's application commands as JSON from the Discord controller.
    ///
    /// Returns the JSON value representing the guild's application commands as provided by the controller.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crates::dapi::api::commands::get_guild_commands::GetGuildCommands;
    /// # use crates::dapi::DiscordContext;
    /// # async fn example<T: crate::DiscordProvider>(context: &DiscordContext<T>) {
    /// let req = GetGuildCommands;
    /// let resp = req.execute(context).await.unwrap();
    /// // `resp` is a `serde_json::Value` containing the guild commands
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let resp = this.controller()
            .get_guild_commands()
            .await?;

        Ok(resp)
    }

    /// Convert this request into the API enum variant for listing API requests.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::GetGuildCommands` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = crate::api::commands::get_guild_commands::GetGuildCommands;
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::GetGuildCommands(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildCommands(self)
    }
}