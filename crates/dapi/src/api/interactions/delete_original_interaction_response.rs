use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteOriginalInteractionResponse {
    pub interaction_token: String,
}

impl ApiReq for DeleteOriginalInteractionResponse {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.controller()
            .delete_original_interaction_response(&self.interaction_token)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteOriginalInteractionResponse(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
