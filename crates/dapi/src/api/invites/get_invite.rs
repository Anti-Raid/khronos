use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetInvite {
    pub code: String,
    pub with_counts: bool,
    pub with_expiration: bool,
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,
}

impl ApiReq for GetInvite {
    type Resp = serde_json::Value;

    /// Fetches a Discord invite using the request parameters.
    ///
    /// # Returns
    ///
    /// `serde_json::Value` representing the invite information on success.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct a GetInvite request and call `execute` with a DiscordContext instance.
    /// // `ctx` in this example represents a `DiscordContext<T>` already configured for the provider.
    /// let req = GetInvite {
    ///     code: "abc123".to_string(),
    ///     with_counts: true,
    ///     with_expiration: false,
    ///     guild_scheduled_event_id: None,
    /// };
    /// // let resp = tokio::runtime::Runtime::new().unwrap().block_on(req.execute(&ctx)).unwrap();
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let invite = this.controller()
            .get_invite(&self.code, self.with_counts, self.with_expiration, self.guild_scheduled_event_id)
            .await?;

        Ok(invite)
    }

    /// Convert this request into the API list enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetInvite {
    ///     code: "abc".into(),
    ///     with_counts: false,
    ///     with_expiration: false,
    ///     guild_scheduled_event_id: None,
    /// };
    /// match req.to_apilist() {
    ///     crate::apilist::API::GetInvite(inner) => {
    ///         assert_eq!(inner.code, "abc");
    ///     }
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetInvite(self)
    }
}