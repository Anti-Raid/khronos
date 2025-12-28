use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteUserReaction {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
    pub user_id: serenity::all::UserId,
}

impl ApiReq for DeleteUserReaction {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_user_reaction(self.channel_id, self.message_id, self.user_id, &self.reaction.into_serenity())
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
