use crate::{ApiReq, context::DiscordContext, controller::{DiscordProvider, SuperUserMessageTransform, SuperUserMessageTransformFlags}, types::CreateInteractionResponse};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateInteractionResponseRequest {
    pub interaction_id: serenity::all::InteractionId,
    pub interaction_token: String,
    pub data: CreateInteractionResponse,
}

impl ApiReq for CreateInteractionResponseRequest {
    type Resp = ();

    async fn execute<T: DiscordProvider>(mut self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = self.data.take_files()?;

        match self.data {
            CreateInteractionResponse::Message(ref mut msg) 
            | CreateInteractionResponse::UpdateMessage(ref mut msg) 
            | CreateInteractionResponse::Defer(ref mut msg) => {
                // Apply superuser transformation to the message before sending, if applicable
                let embeds = std::mem::take(&mut msg.embeds).unwrap_or_default();
                let content = std::mem::take(&mut msg.content);
                let transform = this
                    .controller()
                    .superuser_transform_message_before_send(
                        SuperUserMessageTransform {
                            embeds,
                            content
                        },
                        SuperUserMessageTransformFlags::IS_CREATE_INTERACTION_RESPONSE,
                    )?;
                msg.embeds = Some(transform.embeds);
                msg.content = transform.content;
            },
            _ => {}
        }

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
