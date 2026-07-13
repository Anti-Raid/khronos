use crate::{ApiReq, Permissions, ChannelId, MessageId, UserId, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteUserReaction {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub reaction: ReactionType,
    pub user_id: UserId,
}

impl ApiReq for DeleteUserReaction {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_user_reaction(self.channel_id, self.message_id, self.user_id, &self.reaction)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteUserReaction(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
