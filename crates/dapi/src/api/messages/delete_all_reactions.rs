use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteAllReactions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for DeleteAllReactions {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

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
