use crate::{ApiReq, ChannelId, MessageId, Permissions, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BulkDeleteMessages {
    pub channel_id: ChannelId,
    pub messages: Vec<MessageId>,
    pub reason: String,
}

impl ApiReq for BulkDeleteMessages {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .bulk_delete_messages(self.channel_id, serde_json::json!({"messages": self.messages}), Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::BulkDeleteMessages(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
