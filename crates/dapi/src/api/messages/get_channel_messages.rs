use crate::{ApiReq, ChannelId, Permissions, context::DiscordContext, controller::DiscordProvider, types::ChannelType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetChannelMessages {
    pub channel_id: ChannelId,
    pub target: Option<crate::types::MessagePagination>,
    pub limit: Option<u8>,
}

impl ApiReq for GetChannelMessages {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        // Perform required checks
        let (_, _, guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::VIEW_CHANNEL).await?;

        if guild_channel.kind == ChannelType::Voice 
        && !perms.contains(Permissions::CONNECT) {
            return Err("Bot does not have permission to connect to the given voice channel".into());
        }

        let msg = this.controller()
            .get_channel_messages(self.channel_id, self.target.map(|t| t.into()), self.limit)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannelMessages(self)
    }
}
