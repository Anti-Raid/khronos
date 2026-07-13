use crate::{ApiReq, ChannelId, Permissions, context::DiscordContext, controller::DiscordProvider, types::ChannelType};

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteChannel {
    pub channel_id: ChannelId,
    pub reason: String,
}

impl ApiReq for DeleteChannel {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let bot_user = this.current_user();

        let (_partial_guild, _bot_member, guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::empty())
        .await?;

        match guild_channel.kind {
            ChannelType::PublicThread | ChannelType::PrivateThread => {
                // Check if the bot has permissions to manage threads
                if !perms
                    .contains(Permissions::MANAGE_THREADS)
                {
                    return Err("Bot does not have permission to manage this thread".into());
                }
            },
            _ => {
                // Check if the bot has permissions to manage channels
                if !perms
                    .contains(Permissions::MANAGE_CHANNELS)
                {
                    return Err("Bot does not have permission to manage this channel".into());
                }
            }
        }

        let channel = this
            .controller()
            .delete_channel(self.channel_id, Some(self.reason.as_str()))
            .await?;

        Ok(channel)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteChannel(self)
    }
}
