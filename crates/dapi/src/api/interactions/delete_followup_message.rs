use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteFollowupMessage {
    pub interaction_token: String,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for DeleteFollowupMessage {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.controller()
            .delete_followup_message(&self.interaction_token, self.message_id)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteFollowupMessage(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
