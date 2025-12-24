use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteChannel {
    pub channel_id: serenity::all::GenericChannelId,
    pub reason: String,
}

impl ApiReq for DeleteChannel {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        let (_partial_guild, _bot_member, guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::empty())
        .await?;

        match guild_channel.base.kind {
            serenity::all::ChannelType::PublicThread | serenity::all::ChannelType::PrivateThread => {
                // Check if the bot has permissions to manage threads
                if !perms
                    .manage_threads()
                {
                    return Err("Bot does not have permission to manage this thread".into());
                }
            },
            _ => {
                // Check if the bot has permissions to manage channels
                if !perms
                    .manage_channels()
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
