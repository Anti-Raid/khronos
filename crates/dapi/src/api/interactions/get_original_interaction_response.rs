use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetOriginalInteractionResponse {
    pub interaction_token: String,
}

impl ApiReq for GetOriginalInteractionResponse {
    type Resp = serde_json::Value;

    /// Fetches the original response for an interaction identified by the provided token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::api::interactions::GetOriginalInteractionResponse;
    /// # async fn run<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = GetOriginalInteractionResponse { interaction_token: "token".into() };
    /// let resp = req.execute(ctx).await?;
    /// println!("{}", resp);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let resp = this.controller()
            .get_original_interaction_response(&self.interaction_token)
            .await?;

        Ok(resp)
    }

    /// Convert this `GetOriginalInteractionResponse` request into the API enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetOriginalInteractionResponse { interaction_token: "token".into() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetOriginalInteractionResponse(inner) => {
    ///         assert_eq!(inner.interaction_token, "token");
    ///     }
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetOriginalInteractionResponse(self)
    }
}