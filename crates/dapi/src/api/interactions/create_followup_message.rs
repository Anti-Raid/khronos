use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ExecuteWebhook};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateFollowupMessage {
    pub interaction_token: String,
    pub data: ExecuteWebhook,
}

impl ApiReq for CreateFollowupMessage {
    type Resp = serde_json::Value;

    /// Sends the create-followup-message request and returns the created message payload as JSON.
    ///
    /// Prepares any attached files from `self.data.attachments` before invoking the provider controller.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the created follow-up message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crates::dapi::api::interactions::CreateFollowupMessage;
    /// # async fn example(ctx: &crates::dapi::DiscordContext<impl crates::dapi::DiscordProvider>) -> Result<(), Box<dyn std::error::Error>> {
    /// let req = CreateFollowupMessage {
    ///     interaction_token: "token".into(),
    ///     data: Default::default(), // build ExecuteWebhook as needed
    /// };
    /// let msg_json = req.execute(ctx).await?;
    /// println!("{}", msg_json);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .create_followup_message(&self.interaction_token, &self.data, files)
            .await?;

        Ok(msg)
    }

    /// Convert this request into the `crate::apilist::API` variant for creating a follow-up message.
    ///
    /// # Returns
    ///
    /// An `crate::apilist::API::CreateFollowupMessage` variant containing this `CreateFollowupMessage` request.
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateFollowupMessage(self)
    }
}