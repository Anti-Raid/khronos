use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateInteractionResponse};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateInteractionResponseRequest {
    pub interaction_id: serenity::all::InteractionId,
    pub interaction_token: String,
    pub data: CreateInteractionResponse,
}

impl ApiReq for CreateInteractionResponseRequest {
    type Resp = ();

    /// Send the interaction response to Discord via the provider's controller.
    ///
    /// On success, the request completes with `()`.
    ///
    /// # Errors
    ///
    /// Returns an error if extracting files from the response data fails or if the provider controller call fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::api::interactions::create_interaction_response::CreateInteractionResponseRequest;
    /// # async fn doc_example<T: crate::DiscordProvider>(this: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = CreateInteractionResponseRequest {
    ///     interaction_id: serenity::all::InteractionId(1),
    ///     interaction_token: "token".into(),
    ///     data: crate::CreateInteractionResponse::default(),
    /// };
    /// req.execute(this).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = self.data.take_files()?;

        this.controller()
            .create_interaction_response(self.interaction_id, &self.interaction_token, &self.data, files)
            .await?;

        Ok(())
    }

    /// Convert this request into the API dispatch enum variant.
    ///
    /// This wraps the request in `crate::apilist::API::CreateInteractionResponse` so it can be routed
    /// through the crate's API dispatch.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let req = CreateInteractionResponseRequest {
    ///     interaction_id: serenity::all::InteractionId::from(1),
    ///     interaction_token: String::from("token"),
    ///     data: CreateInteractionResponse::default(), // construct appropriate payload
    /// };
    /// let api = req.to_apilist();
    /// if let crate::apilist::API::CreateInteractionResponse(inner) = api {
    ///     // `inner` is the original request
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateInteractionResponse(self)
    }
}