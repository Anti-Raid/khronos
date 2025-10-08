use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissionsOptions {
    pub user_id: serenity::all::UserId,
    pub channel_id: serenity::all::GenericChannelId,
    pub needed_permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissionsResponse {
    pub partial_guild: serenity::all::PartialGuild,
    pub channel: serenity::all::GuildChannel,
    pub member: serenity::all::Member,
    pub permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissions {
    pub data: AntiRaidCheckChannelPermissionsOptions
}

impl ApiReq for AntiRaidCheckChannelPermissions {
    type Resp = AntiRaidCheckChannelPermissionsResponse;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let (partial_guild, member, channel, permissions) = this.check_channel_permissions(self.data.user_id, self.data.channel_id, self.data.needed_permissions)
            .await?;

        Ok(AntiRaidCheckChannelPermissionsResponse {
            partial_guild,
            member,
            channel,
            permissions,
        })

    }
}
