use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetReactions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
    pub burst: Option<bool>,
    pub after: Option<serenity::all::UserId>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

impl ApiReq for GetReactions {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, serenity::all::Permissions::empty())
            .await?;

        let users = this.controller()
            .get_reactions(
                self.channel_id, 
                self.message_id,
                &self.reaction.into_serenity(),
                self.burst,
                self.after,
                self.limit,
            )
            .await?;

        Ok(users)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetReactions(self)
    }
}
