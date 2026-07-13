use crate::{ApiReq, Permissions, UserId, controller::DiscordProvider, types::{Member, PartialGuild}};

use super::context::DiscordContext;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsOptions {
    pub user_id: UserId,
    pub needed_permissions: Permissions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsAndHierarchyOptions {
    pub user_id: UserId,
    pub target_id: UserId,
    pub needed_permissions: Permissions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AntiRaidCheckPermissionsResponse {
    pub partial_guild: PartialGuild,
    pub member: Member,
    pub permissions: Permissions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::AntiRaidCheckPermissions(self)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::AntiRaidCheckPermissionsAndHierarchy(self)
    }
}
