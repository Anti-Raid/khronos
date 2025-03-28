use crate::plugins::antiraid::typesext::MultiOption;
use nonmax::NonMaxU16;
use serde::{Deserialize, Serialize};
use serenity::all::*;

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct GetChannelOptions {
    pub channel_id: serenity::all::ChannelId,
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#modify-channel-json-params-guild-channel).
///
/// Unlike Serenity, AntiRaid combines EditChannel and EditThread to allow using standard Discord typings
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct EditChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<ChannelType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<NonMaxU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<NonMaxU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub parent_id: MultiOption<ChannelId>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub rtc_region: MultiOption<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<CreateForumTag>>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub default_reaction_emoji: MultiOption<ForumEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<NonMaxU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    applied_tags: Option<Vec<ForumTagId>>,
}

impl Default for EditChannel {
    fn default() -> Self {
        Self {
            name: Some("my-channel".into()),
            kind: Some(serenity::all::ChannelType::Text),
            position: Some(7),
            topic: Some("My channel topic".into()),
            nsfw: Some(true),
            rate_limit_per_user: Some(serenity::nonmax::NonMaxU16::new(5).unwrap()),
            bitrate: None,
            permission_overwrites: None,
            parent_id: MultiOption::new(Some(serenity::all::ChannelId::default())),
            rtc_region: MultiOption::new(Some("us-west".into())),
            video_quality_mode: Some(serenity::all::VideoQualityMode::Auto),
            default_auto_archive_duration: Some(serenity::all::AutoArchiveDuration::OneDay),
            flags: Some(serenity::all::ChannelFlags::all()),
            available_tags: None,
            default_reaction_emoji: MultiOption::new(Some(serenity::all::ForumEmoji::Id(
                serenity::all::EmojiId::default(),
            ))),
            default_thread_rate_limit_per_user: None,
            default_sort_order: None,
            default_forum_layout: None,
            status: Some("online".into()),
            user_limit: Some(serenity::nonmax::NonMaxU16::new(10).unwrap()),
            archived: Some(false),
            auto_archive_duration: Some(serenity::all::AutoArchiveDuration::OneDay),
            locked: Some(false),
            invitable: Some(true),
            applied_tags: None,
        }
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#forum-tag-object-forum-tag-structure)
///
/// Contrary to the [`ForumTag`] struct, only the name field is required.
#[must_use]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateForumTag {
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<EmojiId>,
    pub emoji_name: Option<String>,
}
