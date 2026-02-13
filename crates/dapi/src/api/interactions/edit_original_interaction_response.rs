use crate::{ApiReq, context::DiscordContext, controller::{DiscordProvider, SuperUserMessageTransform, SuperUserMessageTransformFlags}, types::EditWebhookMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditOriginalInteractionResponse {
    pub interaction_token: String,
    pub data: EditWebhookMessage,
}

impl ApiReq for EditOriginalInteractionResponse {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(mut self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        {
            // Apply superuser transformation to the message before sending, if applicable
            let transform = this
            .controller().
            superuser_transform_message_before_send(SuperUserMessageTransform {
                embeds: self.data.embeds.unwrap_or_default(),
                content: self.data.content
            }, SuperUserMessageTransformFlags::IS_EDIT_ORIGINAL_INTERACTION_RESPONSE)?;
            self.data.embeds = Some(transform.embeds);
            self.data.content = transform.content;
        }

        let msg = this.controller()
            .edit_original_interaction_response(&self.interaction_token, &self.data, files)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditOriginalInteractionResponse(self)
    }
}
