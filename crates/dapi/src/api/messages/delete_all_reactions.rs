use crate::{ApiReq, Permissions, ChannelId, MessageId, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteAllReactions {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
}

impl ApiReq for DeleteAllReactions {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_all_reactions(self.channel_id, self.message_id)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAllReactions(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
