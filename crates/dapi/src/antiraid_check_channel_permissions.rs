use crate::{ApiReq, ChannelId, Permissions, UserId, context::DiscordContext, controller::DiscordProvider, types::{Channel, Member, PartialGuild}};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissionsOptions {
    pub user_id: UserId,
    pub channel_id: ChannelId,
    pub needed_permissions: Permissions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckChannelPermissionsResponse {
    pub partial_guild: PartialGuild,
    pub channel: Channel,
    pub member: Member,
    pub permissions: Permissions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::AntiRaidCheckChannelPermissions(self)
    }
}
