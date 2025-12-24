use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuild;

impl ApiReq for GetGuild {
    type Resp = serde_json::Value;

    /// Fetches the current guild from the configured Discord provider and returns it as JSON.
    ///
    /// Calls the provider's controller to obtain the guild data and returns it as a `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume `ctx` is a prepared `DiscordContext` with a provider.
    /// // let ctx = /* DiscordContext::new(provider) */ ;
    /// // let guild_json = GetGuild.execute(&ctx).await.unwrap();
    /// // assert!(guild_json.is_object());
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let guild = this
            .controller()
            .get_guild()
            .await?;

        Ok(guild)
    }

    /// Convert the request into the API registry enum variant.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::GetGuild` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = crate::api::guilds::get_guild::GetGuild;
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::GetGuild(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuild(self)
    }
}