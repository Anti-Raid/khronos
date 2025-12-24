use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditWebhookMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditOriginalInteractionResponse {
    pub interaction_token: String,
    pub data: EditWebhookMessage,
}

impl ApiReq for EditOriginalInteractionResponse {
    type Resp = serde_json::Value;

    /// Sends the edit request for the original interaction response and returns the resulting JSON message.
    ///
    /// The request will include any files extracted from `data.attachments` before sending.
    ///
    /// # Returns
    ///
    /// The edited message as a `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Given an `EditOriginalInteractionResponse` instance `req` and a `DiscordContext` `ctx`:
    /// // let resp = tokio::runtime::Runtime::new().unwrap().block_on(req.execute(&ctx)).unwrap();
    /// // assert!(resp.is_object());
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .edit_original_interaction_response(&self.interaction_token, &self.data, files)
            .await?;

        Ok(msg)
    }

    /// Convert this `EditOriginalInteractionResponse` into the `crate::apilist::API` enum variant.
    ///
    /// This wraps the request in `crate::apilist::API::EditOriginalInteractionResponse` so it can be
    /// dispatched through the centralized API list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::api::interactions::edit_original_interaction_response::EditOriginalInteractionResponse;
    /// # use crate::apilist::API;
    /// let req: EditOriginalInteractionResponse = unimplemented!();
    /// let api_variant: API = req.to_apilist();
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditOriginalInteractionResponse(self)
    }
}