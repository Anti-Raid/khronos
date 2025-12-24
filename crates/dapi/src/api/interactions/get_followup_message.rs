use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetFollowupMessage {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for GetFollowupMessage {
    type Resp = serde_json::Value;

    /// Fetches the follow-up message corresponding to this request's interaction token and message ID.
    ///
    /// # Returns
    ///
    /// The retrieved message as a `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // `req` is a `GetFollowupMessage` and `ctx` is `&DiscordContext<impl DiscordProvider>`
    /// let msg_json = req.execute(&ctx).await.unwrap();
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let msg = this.controller()
            .get_followup_message(&self.interaction_token, self.message_id)
            .await?;

        Ok(msg)
    }

    /// Wraps this request in the `crate::apilist::API::GetFollowupMessage` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use serenity::all::MessageId;
    ///
    /// let req = GetFollowupMessage {
    ///     interaction_token: "token".into(),
    ///     message_id: MessageId::from(1),
    /// };
    ///
    /// if let crate::apilist::API::GetFollowupMessage(inner) = req.to_apilist() {
    ///     let _ = inner; // `inner` is the original `GetFollowupMessage`
    /// } else {
    ///     panic!("expected GetFollowupMessage variant");
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetFollowupMessage(self)
    }
}