use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateInteractionResponse};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateInteractionResponseRequest {
    pub interaction_id: serenity::all::InteractionId,
    pub interaction_token: String,
    pub data: CreateInteractionResponse,
}

impl ApiReq for CreateInteractionResponseRequest {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = self.data.take_files()?;

        this.controller()
            .create_interaction_response(self.interaction_id, &self.interaction_token, &self.data, files)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateInteractionResponse(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
