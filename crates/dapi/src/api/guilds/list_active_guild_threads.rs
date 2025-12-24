use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListActiveGuildThreads;

impl ApiReq for ListActiveGuildThreads {
    type Resp = serde_json::Value;

    /// Retrieve active guild threads from the configured Discord provider.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the active guild threads data.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn run() {
    /// let req = ListActiveGuildThreads;
    /// let ctx: DiscordContext<_> = /* obtain context */ todo!();
    /// let data = req.execute(&ctx).await.unwrap();
    /// // `data` is a `serde_json::Value` describing active guild threads
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let data = this.controller()
            .list_active_guild_threads()
            .await?;

        Ok(data)
    }

    /// Converts this request into the global API registry variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::guilds::list_active_guild_threads::ListActiveGuildThreads;
    /// let req = ListActiveGuildThreads;
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::ListActiveGuildThreads(_) => (),
    ///     _ => panic!("unexpected API variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ListActiveGuildThreads(self)
    }
}