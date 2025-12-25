use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetFollowupMessage {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for GetFollowupMessage {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let msg = this.controller()
            .get_followup_message(&self.interaction_token, self.message_id)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetFollowupMessage(self)
    }
}
