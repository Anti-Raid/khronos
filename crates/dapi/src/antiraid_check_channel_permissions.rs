use crate::{context::DiscordContext, controller::DiscordProvider};

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

pub async fn antiraid_check_permissions<T: DiscordProvider>(this: &DiscordContext<T>, data: AntiRaidCheckChannelPermissionsOptions) -> Result<AntiRaidCheckChannelPermissionsResponse, crate::Error> {
    let (partial_guild, member, channel, permissions) = this.check_channel_permissions(data.user_id, data.channel_id, data.needed_permissions)
        .await?;

    Ok(AntiRaidCheckChannelPermissionsResponse {
            partial_guild,
            member,
            channel,
            permissions,
        }
    )
}
