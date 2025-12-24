use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditWebhookMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditOriginalInteractionResponse {
    pub interaction_token: String,
    pub data: EditWebhookMessage,
}

impl ApiReq for EditOriginalInteractionResponse {
    type Resp = serde_json::Value;

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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditOriginalInteractionResponse(self)
    }
}
