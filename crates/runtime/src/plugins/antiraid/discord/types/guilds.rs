use serenity::all::*;
use serde::{Deserialize, Serialize};
use crate::plugins::antiraid::typesext::MultiOption;

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
