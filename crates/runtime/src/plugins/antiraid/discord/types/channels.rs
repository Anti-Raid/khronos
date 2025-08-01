use crate::{core::typesext::MultiOption, internal_enum_number};
use nonmax::NonMaxU16;
use serde::{Deserialize, Serialize};
use serenity::all::*;

/// A builder for creating a new [`GuildChannel`] in a [`Guild`].
///
/// Except [`Self::name`], all fields are optional.
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild#create-guild-channel).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateChannel {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChannelType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<NonMaxU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<NonMaxU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<GenericChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<ForumEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<ForumTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<NonMaxU16>,
}

impl Default for CreateChannel {
    fn default() -> Self {
        Self {
            name: "my-channel".into(),
            kind: Some(ChannelType::Text),
            topic: Some("My channel topic".into()),
            bitrate: None,
            user_limit: None,
            rate_limit_per_user: Some(serenity::nonmax::NonMaxU16::new(5).unwrap()),
            position: Some(7),
            permission_overwrites: Some(vec![]),
            parent_id: None,
            nsfw: Some(true),
            rtc_region: Some("us-west".into()),
            video_quality_mode: Some(serenity::all::VideoQualityMode::Auto),
            default_auto_archive_duration: Some(serenity::all::AutoArchiveDuration::OneDay),
            default_reaction_emoji: None,
            available_tags: Some(vec![]),
            default_sort_order: None,
            default_forum_layout: None,
            default_thread_rate_limit_per_user: None,
        }
    }
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
    pub parent_id: MultiOption<GenericChannelId>,
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
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<ForumTagId>>,
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
            parent_id: MultiOption::new(Some(GenericChannelId::default())),
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
#[must_use]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumTag {
    pub id: ForumTagId,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<EmojiId>,
    pub emoji_name: Option<String>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#forum-tag-object-forum-tag-structure)
///
/// Contrary to the [`ForumTag`] struct, only the name field is required.
#[must_use]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateForumTag {
    pub id: Option<ForumTagId>,
    pub name: String,
    pub moderated: Option<bool>,
    pub emoji_id: Option<EmojiId>,
    pub emoji_name: Option<String>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#create-channel-invite)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateInvite {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_uses: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_type: Option<InviteTargetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_application_id: Option<ApplicationId>,
}

/// Discord docs: https://discord.com/developers/docs/resources/channel#follow-announcement-channel
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct FollowAnnouncementChannelData {
    pub webhook_channel_id: GenericChannelId,
}

internal_enum_number! {
    /// Type of target for a voice channel invite.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[non_exhaustive]
    pub enum InviteTargetType {
        Stream = 1,
        EmbeddedApplication = 2,
        _ => Unknown(u8),
    }
}