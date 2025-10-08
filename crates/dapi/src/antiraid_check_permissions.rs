use crate::{ApiReq, controller::DiscordProvider};

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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissions {
    pub data: AntiRaidCheckPermissionsOptions
}

impl ApiReq for AntiRaidCheckPermissions {
    type Resp = AntiRaidCheckPermissionsResponse;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let (partial_guild, member, permissions) = this
        .check_permissions(self.data.user_id, self.data.needed_permissions)
        .await?;

        Ok(AntiRaidCheckPermissionsResponse {
            partial_guild,
            member,
            permissions,
        })

    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsAndHierarchy {
    pub data: AntiRaidCheckPermissionsAndHierarchyOptions
}

impl ApiReq for AntiRaidCheckPermissionsAndHierarchy {
    type Resp = AntiRaidCheckPermissionsResponse;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let (partial_guild, member, permissions) = this
        .check_permissions_and_hierarchy(
            self.data.user_id,
            self.data.target_id,
            self.data.needed_permissions,
        )
        .await?;

        Ok(AntiRaidCheckPermissionsResponse {
            partial_guild,
            member,
            permissions,
        })

    }
}
