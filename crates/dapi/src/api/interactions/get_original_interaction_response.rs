use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetOriginalInteractionResponse {
    pub interaction_token: String,
}

impl ApiReq for GetOriginalInteractionResponse {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let resp = this.controller()
            .get_original_interaction_response(&self.interaction_token)
            .await?;

        Ok(resp)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetOriginalInteractionResponse(self)
    }
}
