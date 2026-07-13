//use extract_map::ExtractMap;
use extract_map::ExtractMap;
use log::{error, warn};

use crate::{GuildId, Permissions, RoleId, UserId, types::{Channel, Member, PartialGuild, PermissionOverwriteType, Role}};

pub fn member_permissions(guild: &PartialGuild, member: &Member) -> Permissions {
    user_permissions(
        member.user.id,
        &member.roles,
        guild.id,
        &guild.roles,
        guild.owner_id,
    )
}

/// Helper function that can also be used from [`PartialGuild`].
/// Backported from https://github.com/serenity-rs/serenity/blob/efb5820cd5dd3325dff767bb2f96d380715dce30/src/model/guild/mod.rs
pub fn user_permissions(
    member_user_id: UserId,
    member_roles: &[RoleId],
    guild_id: GuildId,
    guild_roles: &ExtractMap<RoleId, Role>,
    guild_owner_id: UserId,
) -> Permissions {
    calculate_permissions(CalculatePermissions {
        is_guild_owner: member_user_id == guild_owner_id,
        everyone_permissions: if let Some(role) = guild_roles.get(&RoleId::new(guild_id.get())) {
            role.permissions
        } else {
            error!("@everyone role missing in {guild_id}");
            Permissions::empty()
        },
        user_roles_permissions: member_roles
            .iter()
            .map(|role_id| {
                if let Some(role) = guild_roles.get(role_id) {
                    role.permissions
                } else {
                    warn!("{member_user_id} on {guild_id} has non-existent role {role_id:?}",);
                    Permissions::empty()
                }
            })
            .collect(),
    })
}

pub fn user_permissions_in(
    channel: Option<&Channel>,
    member_user_id: UserId,
    member_roles: &[RoleId],
    guild_id: GuildId,
    guild_roles: &ExtractMap<RoleId, Role>,
    guild_owner_id: UserId,
) -> Permissions {
    let mut everyone_allow_overwrites = Permissions::empty();
    let mut everyone_deny_overwrites = Permissions::empty();
    let mut roles_allow_overwrites = Vec::new();
    let mut roles_deny_overwrites = Vec::new();
    let mut member_allow_overwrites = Permissions::empty();
    let mut member_deny_overwrites = Permissions::empty();

    if let Some(channel) = channel {
        for overwrite in channel.permission_overwrites.as_deref().unwrap_or_default().iter() {
            match overwrite.kind { 
                PermissionOverwriteType::Member => {
                    let user_id = UserId::new(overwrite.id.get());
                    if member_user_id == user_id {
                        member_allow_overwrites = overwrite.allow;
                        member_deny_overwrites = overwrite.deny;
                    }
                },
                PermissionOverwriteType::Role => {
                    let role_id = RoleId::new(overwrite.id.get());
                    if role_id.get() == guild_id.get() {
                        everyone_allow_overwrites = overwrite.allow;
                        everyone_deny_overwrites = overwrite.deny;
                    } else if member_roles.contains(&role_id) {
                        roles_allow_overwrites.push(overwrite.allow);
                        roles_deny_overwrites.push(overwrite.deny);
                    }
                },
                _ => continue
            }
        }
    }

    calculate_permissions_v2(CalculatePermissionsV2 {
        is_guild_owner: member_user_id == guild_owner_id,
        everyone_permissions: if let Some(role) = guild_roles.get(&RoleId::new(guild_id.get()))
        {
            role.permissions
        } else {
            error!("@everyone role missing in {}", guild_id);
            Permissions::empty()
        },
        user_roles_permissions: member_roles
            .iter()
            .map(|role_id| {
                if let Some(role) = guild_roles.get(role_id) {
                    role.permissions
                } else {
                    warn!(
                        "{} on {} has non-existent role {:?}",
                        member_user_id, guild_id, role_id
                    );
                    Permissions::empty()
                }
            })
            .collect(),
        everyone_allow_overwrites,
        everyone_deny_overwrites,
        roles_allow_overwrites,
        roles_deny_overwrites,
        member_allow_overwrites,
        member_deny_overwrites,
    })
}


struct CalculatePermissions {
    /// Whether the guild member is the guild owner
    pub is_guild_owner: bool,
    /// Base permissions given to @everyone (guild level)
    pub everyone_permissions: Permissions,
    /// Permissions allowed to a user by their roles (guild level)
    pub user_roles_permissions: Vec<Permissions>,
}

/// Translated from the pseudo code at https://discord.com/developers/docs/topics/permissions#permission-overwrites
///
/// The comments within this file refer to the above link
fn calculate_permissions(data: CalculatePermissions) -> Permissions {
    if data.is_guild_owner {
        return Permissions::all();
    }

    // 1. Base permissions given to @everyone are applied at a guild level
    let mut permissions = data.everyone_permissions;
    // 2. Permissions allowed to a user by their roles are applied at a guild level
    for role_permission in data.user_roles_permissions {
        permissions |= role_permission;
    }

    if permissions.contains(Permissions::ADMINISTRATOR) {
        return Permissions::all();
    }

    permissions
}

/// Translated from the pseudo code at https://discord.com/developers/docs/topics/permissions#permission-overwrites
///
/// The comments within this file refer to the above link
fn calculate_permissions_v2(data: CalculatePermissionsV2) -> Permissions {
    if data.is_guild_owner {
        return Permissions::all();
    }

    // 1. Base permissions given to @everyone are applied at a guild level
    let mut permissions = data.everyone_permissions;
    // 2. Permissions allowed to a user by their roles are applied at a guild level
    for role_permission in data.user_roles_permissions {
        permissions |= role_permission;
    }

    if permissions.contains(Permissions::ADMINISTRATOR) {
        return Permissions::all();
    }

    // 3. Overwrites that deny permissions for @everyone are applied at a channel level
    permissions &= !data.everyone_deny_overwrites;
    // 4. Overwrites that allow permissions for @everyone are applied at a channel level
    permissions |= data.everyone_allow_overwrites;

    // 5. Overwrites that deny permissions for specific roles are applied at a channel level
    let mut role_deny_permissions = Permissions::empty();
    for p in data.roles_deny_overwrites {
        role_deny_permissions |= p;
    }
    permissions &= !role_deny_permissions;

    // 6. Overwrites that allow permissions for specific roles are applied at a channel level
    let mut role_allow_permissions = Permissions::empty();
    for p in data.roles_allow_overwrites {
        role_allow_permissions |= p;
    }
    permissions |= role_allow_permissions;

    // 7. Member-specific overwrites that deny permissions are applied at a channel level
    permissions &= !data.member_deny_overwrites;
    // 8. Member-specific overwrites that allow permissions are applied at a channel level
    permissions |= data.member_allow_overwrites;

    permissions
}

struct CalculatePermissionsV2 {
    /// Whether the guild member is the guild owner
    pub is_guild_owner: bool,
    /// Base permissions given to @everyone (guild level)
    pub everyone_permissions: Permissions,
    /// Permissions allowed to a user by their roles (guild level)
    pub user_roles_permissions: Vec<Permissions>,
    /// Overwrites that deny permissions for @everyone (channel level)
    pub everyone_allow_overwrites: Permissions,
    /// Overwrites that allow permissions for @everyone (channel level)
    pub everyone_deny_overwrites: Permissions,
    /// Overwrites that deny permissions for specific roles (channel level)
    pub roles_allow_overwrites: Vec<Permissions>,
    /// Overwrites that allow permissions for specific roles (channel level)
    pub roles_deny_overwrites: Vec<Permissions>,
    /// Member-specific overwrites that deny permissions (channel level)
    pub member_allow_overwrites: Permissions,
    /// Member-specific overwrites that allow permissions (channel level)
    pub member_deny_overwrites: Permissions,
}

impl Default for CalculatePermissionsV2 {
    fn default() -> Self {
        Self {
            is_guild_owner: false,
            everyone_permissions: Permissions::empty(),
            user_roles_permissions: Vec::new(),
            everyone_allow_overwrites: Permissions::empty(),
            everyone_deny_overwrites: Permissions::empty(),
            roles_allow_overwrites: Vec::new(),
            roles_deny_overwrites: Vec::new(),
            member_allow_overwrites: Permissions::empty(),
            member_deny_overwrites: Permissions::empty(),
        }
    }
}

pub fn highest_role(guild: &PartialGuild, member: &Member) -> Option<Role> {
    let mut highest: Option<&Role> = None;

    for role_id in &member.roles {
        if let Some(role) = guild.roles.get(role_id) {
            if let Some(highest_role) = highest {
                if role.position > highest_role.position || (role.position == highest_role.position && role.id < highest_role.id) {
                    highest = Some(role);
                }
            } else {
                highest = Some(role);
            }
        }
    }

    // Default to @everyone if no other roles match and it exists
    if highest.is_none() {
        highest = guild.roles.get(&RoleId::new(guild.id.get()));
    }

    highest.cloned()
}

pub fn greater_member_hierarchy_in(
    lhs_highest_role: Option<&Role>,
    rhs_highest_role: Option<&Role>,
    owner_id: UserId,
    lhs: &Member,
    rhs: &Member,
) -> Option<UserId> {
    // Check that the IDs are the same. If they are, neither is greater.
    if lhs.user.id == rhs.user.id {
        return None;
    }

    // Check if either user is the guild owner.
    if lhs.user.id == owner_id {
        return Some(lhs.user.id);
    } else if rhs.user.id == owner_id {
        return Some(rhs.user.id);
    }

    let lhs_role = lhs_highest_role.map_or((RoleId::new(1), 0), |r| (r.id, r.position));

    let rhs_role = rhs_highest_role.map_or((RoleId::new(1), 0), |r| (r.id, r.position));

    // If LHS and RHS both have no top position or have the same role ID, then no one wins.
    if (lhs_role.1 == 0 && rhs_role.1 == 0) || (lhs_role.0 == rhs_role.0) {
        return None;
    }

    // If LHS's top position is higher than RHS, then LHS wins.
    if lhs_role.1 > rhs_role.1 {
        return Some(lhs.user.id);
    }

    // If RHS's top position is higher than LHS, then RHS wins.
    if rhs_role.1 > lhs_role.1 {
        return Some(rhs.user.id);
    }

    // If LHS and RHS both have the same position, but LHS has the lower role ID, then LHS
    // wins.
    //
    // If RHS has the higher role ID, then RHS wins.
    if lhs_role.1 == rhs_role.1 && lhs_role.0 < rhs_role.0 {
        Some(lhs.user.id)
    } else {
        Some(rhs.user.id)
    }
}

pub fn member_highest_role_in<'a>(
    roles: &'a ExtractMap<RoleId, Role>,
    member: &Member,
) -> Option<&'a Role> {
    let mut highest: Option<&Role> = None;

    for role_id in &member.roles {
        if let Some(role) = roles.get(role_id) {
            // Skip this role if this role in iteration has:
            // - a position less than the recorded highest
            // - a position equal to the recorded, but a higher ID
            if let Some(highest) = highest
                && (role.position < highest.position
                    || (role.position == highest.position && role.id > highest.id))
            {
                continue;
            }

            highest = Some(role);
        }
    }

    highest
}

/// The `enum_number!` macro generates `From` implementations to convert between values and the
/// enum which can then be utilized by `serde` with `#[serde(from = "u8", into = "u8")]`.
///
/// When defining the enum like this:
/// ```ignore
/// enum_number! {
///     /// The `Foo` enum
///     #[derive(Clone, Copy, Deserialize, Serialize)]
///     #[serde(from = "u8", into = "u8")]
///     pub enum Foo {
///         /// First
///         Aah = 1,
///         /// Second
///         Bar = 2,
///         _ => Unknown(u8),
///     }
/// }
/// ```
///
/// Code like this will be generated:
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// #
/// /// The `Foo` enum
/// #[derive(Clone, Copy, Deserialize, Serialize)]
/// #[serde(from = "u8", into = "u8")]
/// pub enum Foo {
///     /// First
///     Aah,
///     /// Second,
///     Bar,
///     /// Variant value is unknown.
///     Unknown(u8),
/// }
///
/// impl From<u8> for Foo {
///     fn from(value: u8) -> Self {
///         match value {
///             1 => Self::Aah,
///             2 => Self::Bar,
///             unknown => Self::Unknown(unknown),
///         }
///     }
/// }
///
/// impl From<Foo> for u8 {
///     fn from(value: Foo) -> Self {
///         match value {
///             Foo::Aah => 1,
///             Foo::Bar => 2,
///             Foo::Unknown(unknown) => unknown,
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! enum_number {
    (
        $(#[$outer:meta])*
        $(#[<default> = $default:literal])?
        $vis:vis enum $Enum:ident {
            $(
                $(#[doc = $doc:literal])*
                $(#[cfg $($cfg:tt)*])?
                $Variant:ident = $value:literal,
            )*
            _ => Unknown($T:ty),
        }
    ) => {
        $(#[$outer])*
        $vis struct $Enum (pub $T);

        $(
            impl Default for $Enum {
                fn default() -> Self {
                    Self($default)
                }
            }
        )?

        #[allow(non_snake_case, non_upper_case_globals)]
        #[allow(clippy::allow_attributes, reason = "Does not always trigger due to macro")]
        impl $Enum {
            $(
                $(#[doc = $doc])*
                $(#[cfg $($cfg)*])?
                $vis const $Variant: Self = Self($value);
            )*

            /// Variant value is unknown.
            #[must_use]
            $vis const fn Unknown(val: $T) -> Self {
                Self(val)
            }
        }
    };
}
