use crate::{ApiReq, ChannelId, MessageId, Permissions, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateReaction {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub reaction: ReactionType,
}

impl ApiReq for CreateReaction {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::READ_MESSAGE_HISTORY | Permissions::ADD_REACTIONS)
            .await?;

        this.controller()
            .create_reaction(self.channel_id, self.message_id, &self.reaction)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateReaction(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
