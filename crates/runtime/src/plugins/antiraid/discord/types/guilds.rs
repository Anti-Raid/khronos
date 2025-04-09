use serenity::all::*;
use serde::{Deserialize, Serialize};
use crate::plugins::antiraid::typesext::MultiOption;

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
    // [Omitting owner_id as we can't use it]
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
    pub communication_disabled_until: MultiOption<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<GuildMemberFlags>,
}
