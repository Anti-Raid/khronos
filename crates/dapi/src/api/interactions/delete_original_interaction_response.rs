use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteOriginalInteractionResponse {
    pub interaction_token: String,
}

impl ApiReq for DeleteOriginalInteractionResponse {
    type Resp = ();

    /// Deletes the original interaction response identified by the stored interaction token.
    ///
    /// This sends a request to remove the original reply that was created for the interaction represented by `interaction_token`.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err(crate::Error)` if the delete operation fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example(ctx: &DiscordContext<impl DiscordProvider>) -> Result<(), crate::Error> {
    /// let req = DeleteOriginalInteractionResponse { interaction_token: "abc123".into() };
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.controller()
            .delete_original_interaction_response(&self.interaction_token)
            .await?;

        Ok(())
    }

    /// Converts this request into the crate's API enum variant used for API listing/registration.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apilist::API;
    /// let req = crate::requests::DeleteOriginalInteractionResponse { interaction_token: "tkn".into() };
    /// let api = req.to_apilist();
    /// match api {
    ///     API::DeleteOriginalInteractionResponse(inner) => {
    ///         assert_eq!(inner.interaction_token, "tkn");
    ///     }
    ///     _ => panic!("expected DeleteOriginalInteractionResponse variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteOriginalInteractionResponse(self)
    }
}