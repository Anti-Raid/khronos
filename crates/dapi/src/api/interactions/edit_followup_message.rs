use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditWebhookMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditFollowupMessage {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
    pub data: EditWebhookMessage,
}

impl ApiReq for EditFollowupMessage {
    type Resp = serde_json::Value;

    /// Execute the edit follow-up message API request.
    ///
    /// Sends the contained `EditWebhookMessage` (including any attachments) to edit the follow-up
    /// message identified by `interaction_token` and `message_id`, and returns the raw JSON response.
    ///
    /// # Returns
    ///
    /// `serde_json::Value` with the API response for the edited follow-up message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::executor::block_on;
    /// # // `ctx` and `req` would be created in real usage.
    /// # let ctx: crate::DiscordContext<_> = unimplemented!();
    /// # let req: crate::api::interactions::EditFollowupMessage = unimplemented!();
    /// let resp = block_on(req.execute(&ctx)).unwrap();
    /// // `resp` is a serde_json::Value containing the edited message.
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .edit_followup_message(&self.interaction_token, self.message_id, &self.data, files)
            .await?;

        Ok(msg)
    }

    /// Convert this request into its API list representation.
    ///
    /// # Returns
    ///
    /// An `crate::apilist::API` enum variant wrapping this `EditFollowupMessage`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let req = /* EditFollowupMessage { interaction_token: ..., message_id: ..., data: ... } */;
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::EditFollowupMessage(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditFollowupMessage(self)
    }
}