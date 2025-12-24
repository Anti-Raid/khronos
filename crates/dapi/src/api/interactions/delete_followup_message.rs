use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteFollowupMessage {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for DeleteFollowupMessage {
    type Resp = ();

    /// Deletes the follow-up message associated with this interaction token and message ID.
    ///
    /// On success, returns `()`.
    ///
    /// # Errors
    ///
    /// Returns `crate::Error` if the controller fails to delete the follow-up message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crates::dapi::api::interactions::DeleteFollowupMessage;
    /// # async fn example(req: DeleteFollowupMessage, ctx: &crate::DiscordContext<impl crate::DiscordProvider>) -> Result<(), crate::Error> {
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.controller()
            .delete_followup_message(&self.interaction_token, self.message_id)
            .await?;

        Ok(())
    }

    /// Converts this request into the corresponding `crate::apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = DeleteFollowupMessage {
    ///     interaction_token: "token".into(),
    ///     message_id: serenity::all::MessageId::from(1),
    /// };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::DeleteFollowupMessage(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteFollowupMessage(self)
    }
}