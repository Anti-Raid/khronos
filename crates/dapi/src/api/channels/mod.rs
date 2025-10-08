use serenity::all::Permissions;

use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditChannel as EditChannelData};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditChannel {
    pub channel_id: serenity::all::GenericChannelId,
    pub reason: String,
    pub data: EditChannelData,
}

impl ApiReq for EditChannel {
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

        if let Some(ref topic) = self.data.topic {
            if topic.len() > 1024 {
                return Err("Topic must be less than 1024 characters".into());
            }
        }

        if let Some(ref rate_limit_per_user) = self.data.rate_limit_per_user {
            if rate_limit_per_user.get() > 21600 {
                return Err(
                    "Rate limit per user must be less than 21600 seconds".into(),
                );
            }
        }

        if let Some(ref permission_overwrites) = self.data.permission_overwrites {
            // Check for ManageRoles permission
            if !perms
                .manage_roles()
            {
                return Err(
                    "Bot does not have permission to manage roles".into(),
                );
            }

            for overwrite in permission_overwrites.iter() {
                if !perms.contains(overwrite.allow) {
                    return Err(
                        format!("Bot does not have permission to allow: {:?}", overwrite.allow).into(),
                    );
                } else if !perms.contains(overwrite.deny) {
                    return Err(
                        format!("Bot does not have permission to deny: {:?}", overwrite.deny).into(),
                    );
                }
            }
        }

        if let Some(ref available_tags) = self.data.available_tags {
            for tag in available_tags.iter() {
                if tag.name.len() > 20 {
                    return Err(
                        "Tag name must be less than 20 characters".into()
                    );
                }
            }
        }

        if let Some(ref default_thread_rate_limit_per_user) =
            self.data.default_thread_rate_limit_per_user
        {
            if default_thread_rate_limit_per_user.get() > 21600 {
                return Err(
                    "Default thread rate limit per user must be less than 21600 seconds".into()
                );
            }
        }

        let channel = this
            .controller()
            .edit_channel(self.channel_id, &self.data, Some(self.reason.as_str()))
            .await?;

        Ok(channel)
    }
}