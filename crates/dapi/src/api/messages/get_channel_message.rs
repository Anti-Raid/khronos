use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannelMessage {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for GetChannelMessage {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        // Perform required checks
        let (_, _, guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::VIEW_CHANNEL).await?;

        if guild_channel.base.kind == serenity::all::ChannelType::Voice 
        && !perms.connect() {
            return Err("Bot does not have permission to connect to the given voice channel".into());
        }

        let msg = this.controller()
            .get_channel_message(self.channel_id, self.message_id)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannelMessage(self)
    }
}
