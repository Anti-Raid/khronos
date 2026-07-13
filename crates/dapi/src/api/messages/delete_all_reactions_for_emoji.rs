use crate::{ApiReq, ChannelId, MessageId, Permissions, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteAllReactionsForEmoji {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub reaction: ReactionType,
}

impl ApiReq for DeleteAllReactionsForEmoji {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_all_reactions_for_emoji(self.channel_id, self.message_id, &self.reaction)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAllReactionsForEmoji(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
