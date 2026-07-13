use std::collections::HashMap;

use crate::{ChannelId, GuildId, Permissions, RoleId, UserId, enum_number, internal_bitflags, multioption::MultiOption, serenity_backports::{greater_member_hierarchy_in, member_highest_role_in, member_permissions, user_permissions_in}, types::Channel};
use chrono::{DateTime, Utc};
use extract_map::{ExtractKey, ExtractMap};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MinPartialGuild {
    /// The unique Id identifying the guild.
    ///
    /// This is equivalent to the Id of the default role (`@everyone`).
    pub id: GuildId,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PartialGuild {
    /// The unique Id identifying the guild.
    ///
    /// This is equivalent to the Id of the default role (`@everyone`).
    pub id: GuildId,
    /// The name of the guild.
    pub name: String,
    /// The Id of the [`User`] who owns the guild.
    pub owner_id: UserId,
    /// A mapping of the guild's roles.
    pub roles: ExtractMap<RoleId, Role>,
    /// The guild NSFW state. See [`discord support article`].
    ///
    /// [`discord support article`]: https://support.discord.com/hc/en-us/articles/1500005389362-NSFW-Server-Designation
    pub nsfw_level: NsfwLevel,

    /// The guild features. More information available at [`discord documentation`].
    ///
    /// [`discord documentation`]: https://discord.com/developers/docs/resources/guild#guild-object-guild-features
    pub features: Vec<String>,
    /// Icon hash
    pub icon: Option<String>,

    #[serde(flatten)]
    pub extra_info: HashMap<String, serde_json::Value>,
}

impl PartialGuild {
    /// Returns the guild's icon URL.
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/icons/{}/{}.jpg?size=1024",
                self.id, hash
            )
        })
    }

    /// Calculate a [`Member`]'s permissions in the guild.
    #[must_use]
    pub fn member_permissions(&self, member: &Member) -> Permissions {
        member_permissions(self, member)
    }

    /// Gets the highest role a [`Member`] of this Guild has.
    ///
    /// Returns None if the member has no roles or the member from this guild.
    #[must_use]
    pub fn member_highest_role(&self, member: &Member) -> Option<&Role> {
        member_highest_role_in(&self.roles, member)
    }

    /// Returns which of two [`User`]s has a higher [`Member`] hierarchy.
    ///
    /// If both user IDs are the same, [`None`] is returned. If one of the users is the guild
    /// owner, their ID is returned.
    #[must_use]
    pub fn greater_member_hierarchy(&self, lhs: &Member, rhs: &Member) -> Option<UserId> {
        let lhs_highest_role = self.member_highest_role(lhs);
        let rhs_highest_role = self.member_highest_role(rhs);

        greater_member_hierarchy_in(
            lhs_highest_role,
            rhs_highest_role,
            self.owner_id,
            lhs,
            rhs,
        )
    }

    /// Calculate a [`Member`]'s permissions in a given channel in the guild.
    #[must_use]
    pub fn user_permissions_in(&self, channel: &Channel, member: &Member) -> Permissions {
        user_permissions_in(
            Some(channel),
            member.user.id,
            &member.roles,
            self.id,
            &self.roles,
            self.owner_id,
        )
    }
}

/// Information about a role within a guild.
///
/// [Discord docs](https://discord.com/developers/docs/topics/permissions#role-object).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Role {
    /// The Id of the role. Can be used to calculate the role's creation date.
    pub id: RoleId,
    /// The Id of the Guild the Role is in.
    #[serde(default)]
    pub guild_id: GuildId,
    /// The name of the role.
    pub name: String,
    /// A set of permissions that the role has been assigned.
    ///
    /// See the [`permissions`] module for more information.
    ///
    /// [`permissions`]: crate::model::permissions
    pub permissions: Permissions,
    /// The role's position in the position list. Roles are considered higher in hierarchy if their
    /// position is higher.
    ///
    /// The `@everyone` role is usually either `-1` or `0`.
    pub position: i16,

    #[serde(flatten)]
    pub extra_info: HashMap<String, serde_json::Value>,
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Role) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Role) -> Ordering {
        // Discord does position DESC, id ASC so:
        if self.position == other.position {
            other.id.cmp(&self.id)
        } else {
            self.position.cmp(&other.position)
        }
    }
}

impl ExtractKey<RoleId> for Role {
    fn extract_key(&self) -> &RoleId {
        &self.id
    }
}

bitflags::bitflags! {
    /// Flags for a guild member.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-flags).
    
    #[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct EditableGuildMemberFlags: u32 {
        /// Member is exempt from guild verification requirements
        const BYPASSES_VERIFICATION = 1 << 2;
    }
}

enum_number! {
    /// Default message notification level for a guild.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-object-default-message-notification-level).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum DefaultMessageNotificationLevel {
        /// Receive notifications for everything.
        All = 0,
        /// Receive only mentions.
        Mentions = 1,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Setting used to filter explicit messages from members.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-object-explicit-content-filter-level).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum ExplicitContentFilter {
        /// Don't scan any messages.
        None = 0,
        /// Scan messages from members without a role.
        WithoutRole = 1,
        /// Scan messages sent by all members.
        All = 2,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Multi-Factor Authentication level for guild moderators.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-object-mfa-level).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum MfaLevel {
        /// MFA is disabled.
        None = 0,
        /// MFA is enabled.
        Elevated = 1,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// The level to set as criteria prior to a user being able to send
    /// messages in a [`Guild`].
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-object-verification-level).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum VerificationLevel {
        /// Does not require any verification.
        None = 0,
        /// Must have a verified email on the user's Discord account.
        Low = 1,
        /// Must also be a registered user on Discord for longer than 5 minutes.
        Medium = 2,
        /// Must also be a member of the guild for longer than 10 minutes.
        High = 3,
        /// Must have a verified phone on the user's Discord account.
        Higher = 4,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// The [`Guild`] nsfw level.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-object-guild-nsfw-level).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum NsfwLevel {
        /// The nsfw level is not specified.
        Default = 0,
        /// The guild is considered as explicit.
        Explicit = 1,
        /// The guild is considered as safe.
        Safe = 2,
        /// The guild is age restricted.
        AgeRestricted = 3,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// The [`Guild`] AFK timeout length.
    ///
    /// See [AfkMetadata::afk_timeout].
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum AfkTimeout {
        OneMinute = 60,
        FiveMinutes = 300,
        FifteenMinutes = 900,
        ThirtyMinutes = 1800,
        OneHour = 3600,
        _ => Unknown(u16),
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild#modify-guild).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct EditGuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // [Omitting region because Discord deprecated it]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub default_message_notifications: MultiOption<DefaultMessageNotificationLevel>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub explicit_content_filter: MultiOption<ExplicitContentFilter>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub afk_channel_id: MultiOption<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<AfkTimeout>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub icon: MultiOption<String>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub splash: MultiOption<String>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub discovery_splash: MultiOption<String>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub banner: MultiOption<String>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub system_channel_id: MultiOption<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub rules_channel_id: MultiOption<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: MultiOption<ChannelId>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub preferred_locale: MultiOption<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild#modify-guild-member)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct EditMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub channel_id: MultiOption<ChannelId>,

    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub communication_disabled_until: MultiOption<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<EditableGuildMemberFlags>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild#modify-guild-role)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct EditRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color")]
    pub colour: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub icon: MultiOption<String>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub unicode_emoji: MultiOption<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

internal_bitflags! {
    /// Describes a system channel flags.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/guild#guild-object-system-channel-flags).
    #[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
    pub struct SystemChannelFlags: u64 {
        /// Suppress member join notifications.
        const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
        /// Suppress server boost notifications.
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
        /// Suppress server setup tips.
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
        /// Hide member join sticker reply buttons.
        const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
        /// Suppress role subscription purchase and renewal notifications.
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;
        /// Hide role subscription sticker reply buttons.
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild#modify-guild-role-positions)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModifyRolePosition {
    pub id: RoleId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u16>,
}

impl PartialEq<Role> for ModifyRolePosition {
    fn eq(&self, other: &Role) -> bool {
        self.id == other.id && self.position == Some(other.position as u16)
    }
}

impl PartialOrd<Role> for ModifyRolePosition {
    fn partial_cmp(&self, other: &Role) -> Option<std::cmp::Ordering> {
        let self_pos = self.position.unwrap_or(other.position as u16);
        match self_pos.partial_cmp(&(other.position as u16)) {
            Some(std::cmp::Ordering::Equal) => other.id.partial_cmp(&self.id),
            res => res,
        }
    }
}


#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Member {
    /// Attached User struct.
    pub user: User,
    /// Vector of Ids of [`Role`]s given to the member.
    pub roles: Vec<RoleId>,
    /// The unique Id of the guild that the member is a part of.
    #[serde(default)]
    pub guild_id: GuildId,

    #[serde(flatten)]
    pub extra_info: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub system: bool,
    #[serde(flatten)]
    pub extra_info: HashMap<String, serde_json::Value>,
}
