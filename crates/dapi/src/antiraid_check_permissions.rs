use crate::controller::DiscordProvider;

use super::context::DiscordContext;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsOptions {
    pub user_id: serenity::all::UserId,
    pub needed_permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsAndHierarchyOptions {
    pub user_id: serenity::all::UserId,
    pub target_id: serenity::all::UserId,
    pub needed_permissions: serenity::all::Permissions,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsResponse {
    pub partial_guild: serenity::all::PartialGuild,
    pub member: serenity::all::Member,
    pub permissions: serenity::all::Permissions,
}

pub async fn antiraid_check_permissions<T: DiscordProvider>(this: &DiscordContext<T>, data: AntiRaidCheckPermissionsOptions) -> Result<AntiRaidCheckPermissionsResponse, crate::Error> {
    let (partial_guild, member, permissions) = this
        .check_permissions(data.user_id, data.needed_permissions)
        .await?;

    Ok(AntiRaidCheckPermissionsResponse {
        partial_guild,
        member,
        permissions,
    })
}

pub async fn check_permissions_and_hierarchy<T: DiscordProvider>(this: &DiscordContext<T>, data: AntiRaidCheckPermissionsAndHierarchyOptions) -> Result<AntiRaidCheckPermissionsResponse, crate::Error> {
    let (partial_guild, member, permissions) = this
        .check_permissions_and_hierarchy(
            data.user_id,
            data.target_id,
            data.needed_permissions,
        )
        .await?;

    Ok(AntiRaidCheckPermissionsResponse {
        partial_guild,
        member,
        permissions,
    })
}