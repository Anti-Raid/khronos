use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateChannel};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateGuildChannel {
    pub reason: String,
    pub data: CreateChannel,
}

impl ApiReq for CreateGuildChannel {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };
        
        let (_, _, bot_perms) = this.check_permissions(bot_user.id, Permissions::MANAGE_CHANNELS)
        .await?;

        if let Some(ref topic) = self.data.topic {
            if topic.len() > 1024 {
                return Err("Topic must be less than 1024 characters".into());
            }
        }

        if let Some(ref rate_limit_per_user) = self.data.rate_limit_per_user {
            if rate_limit_per_user.get() > 21600 {
                return Err("Rate limit per user must be less than 21600 seconds".into());
            }
        }

        if let Some(ref permission_overwrites) = self.data.permission_overwrites {
            // Check for ManageRoles permission
            if !bot_perms
                .manage_roles()
            {
                return Err("Bot does not have permission to manage roles".into());
            }

            for overwrite in permission_overwrites.iter() {
                if !bot_perms.contains(overwrite.allow) {
                    return Err(format!("Bot does not have permission to allow: {:?}", overwrite.allow).into());
                }
                
                if !bot_perms.contains(overwrite.deny) {
                    return Err(format!("Bot does not have permission to deny: {:?}", overwrite.deny).into());
                }
            }
        }

        if let Some(ref available_tags) = self.data.available_tags {
            for tag in available_tags.iter() {
                if tag.name.len() > 20 {
                    return Err("Tag name must be less than 20 characters".into());
                }
            }
        }

        if let Some(ref default_thread_rate_limit_per_user) =
            self.data.default_thread_rate_limit_per_user
        {
           if default_thread_rate_limit_per_user.get() > 21600 {
                return Err("Default thread rate limit per user must be less than 21600 seconds".into());
            }
        }

        let channel = this
            .controller()
            .create_guild_channel(&self.data, Some(self.reason.as_str()))
            .await?;

        Ok(channel)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildChannel(self)
    }
}
