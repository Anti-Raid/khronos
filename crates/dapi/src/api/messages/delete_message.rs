use crate::{ApiReq, Permissions, ChannelId, MessageId, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteMessage {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub reason: String,
}

impl ApiReq for DeleteMessage {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_message(self.channel_id, self.message_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteMessage(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
