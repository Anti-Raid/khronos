use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ExecuteWebhook};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateFollowupMessage {
    pub interaction_token: String,
    pub data: ExecuteWebhook,
}

impl ApiReq for CreateFollowupMessage {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .create_followup_message(&self.interaction_token, &self.data, files)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateFollowupMessage(self)
    }
}
