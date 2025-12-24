use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildChannels;

impl ApiReq for GetGuildChannels {
    type Resp = serde_json::Value;

    /// Fetches the current guild's channels.
    ///
    /// # Returns
    ///
    /// `Ok` with a `serde_json::Value` containing the guild channels on success, `Err` with a `crate::Error` if retrieving channels fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dapi::api::guilds::GetGuildChannels;
    /// // `ctx` is a `&DiscordContext<_>` obtained from your application
    /// # async fn example(ctx: &dapi::DiscordContext<impl dapi::DiscordProvider>) -> Result<(), dapi::Error> {
    /// let resp = GetGuildChannels.execute(ctx).await?;
    /// println!("{}", resp);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let chans = this
            .controller()
            .get_guild_channels()
            .await?;

        Ok(chans)
    }

    /// Convert this request into its corresponding variant of the API registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use dapi::api::guilds::get_guild_channels::GetGuildChannels;
    /// let req = GetGuildChannels;
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetGuildChannels(_) => {}
    ///     _ => panic!("unexpected API variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildChannels(self)
    }
}