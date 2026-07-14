use std::collections::HashMap;

use crate::{AnyId, ApplicationId, ChannelId, EmojiId, ForumTagId, GuildId, Permissions, UserId, enum_number, multioption::MultiOption};
use serde::{Deserialize, Deserializer, Serialize, ser::SerializeMap};

enum_number! {
    /// A representation of a type of channel.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#channel-object-channel-types).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]

    pub enum ChannelType {
        /// An indicator that the channel is a text [`GuildChannel`].
        Text = 0,
        /// An indicator that the channel is a [`PrivateChannel`].
        Private = 1,
        /// An indicator that the channel is a voice [`GuildChannel`].
        Voice = 2,
        /// An indicator that the channel is a group DM.
        GroupDm = 3,
        /// An indicator that the channel is a channel category.
        Category = 4,
        /// An indicator that the channel is a `NewsChannel`.
        ///
        /// Note: `NewsChannel` is serialized into a [`GuildChannel`]
        News = 5,
        /// An indicator that the channel is a news thread [`GuildChannel`].
        NewsThread = 10,
        /// An indicator that the channel is a public thread [`GuildChannel`].
        PublicThread = 11,
        /// An indicator that the channel is a private thread [`GuildChannel`].
        PrivateThread = 12,
        /// An indicator that the channel is a stage [`GuildChannel`].
        Stage = 13,
        /// An indicator that the channel is a directory [`GuildChannel`] in a [hub].
        ///
        /// [hub]: https://support.discord.com/hc/en-us/articles/4406046651927-Discord-Student-Hubs-FAQ
        Directory = 14,
        /// An indicator that the channel is a forum [`GuildChannel`].
        Forum = 15,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// The video quality mode for a voice channel.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#channel-object-video-quality-modes).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]

    pub enum VideoQualityMode {
        /// An indicator that the video quality is chosen by Discord for optimal
        /// performance.
        Auto = 1,
        /// An indicator that the video quality is 720p.
        Full = 2,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// See [`StageInstance::privacy_level`].
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level).
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
    #[<default> = 2]
    pub enum StageInstancePrivacyLevel {
        /// The Stage instance is visible publicly. (deprecated)
        Public = 1,
        /// The Stage instance is visible to only guild members.
        GuildOnly = 2,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// See [`ThreadMetadata::auto_archive_duration`].
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#thread-metadata-object)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]

    pub enum AutoArchiveDuration {
        None = 0,
        OneHour = 60,
        OneDay = 1440,
        ThreeDays = 4320,
        OneWeek = 10080,
        _ => Unknown(u16),
    }
}

enum_number! {
    /// See [`GuildChannel::default_forum_layout`].
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#channel-object-forum-layout-types).
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]

    pub enum ForumLayoutType {
        /// No default has been set for forum channel.
        NotSet = 0,
        /// Display posts as a list.
        ListView = 1,
        /// Display posts as a collection of tiles.
        GalleryView = 2,
        _ => Unknown(u8),
    }
}

/// An object that specifies the emoji to use for Forum related emoji parameters.
///
/// See [Discord](https://discord.com/developers/docs/resources/channel#default-reaction-object)
/// [docs]()

#[derive(Debug, Clone)]

pub enum ForumEmoji {
    /// The id of a guild's custom emoji.
    Id(EmojiId),
    /// The unicode character of the emoji.
    Name(String),
}

#[derive(Deserialize)]
struct RawForumEmoji {
    emoji_id: Option<EmojiId>,
    emoji_name: Option<String>,
}

impl serde::Serialize for ForumEmoji {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            Self::Id(id) => {
                map.serialize_entry("emoji_id", id)?;
                map.serialize_entry("emoji_name", &None::<()>)?;
            },
            Self::Name(name) => {
                map.serialize_entry("emoji_id", &None::<()>)?;
                map.serialize_entry("emoji_name", name)?;
            },
        }

        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for ForumEmoji {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let helper = RawForumEmoji::deserialize(deserializer)?;
        match (helper.emoji_id, helper.emoji_name) {
            (Some(id), None) => Ok(ForumEmoji::Id(id)),
            (None, Some(name)) => Ok(ForumEmoji::Name(name)),
            (None, None) => {
                Err(serde::de::Error::custom("expected emoji_name or emoji_id, found neither"))
            },
            (Some(_), Some(_)) => {
                Err(serde::de::Error::custom("expected emoji_name or emoji_id, found both"))
            },
        }
    }
}

/// An object that represents a tag able to be applied to a thread in a `GUILD_FORUM` channel.
///
/// See [Discord docs](https://discord.com/developers/docs/resources/channel#forum-tag-object)

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ForumTag {
    /// The id of the tag.
    pub id: ForumTagId,
    /// The name of the tag (0-20 characters).
    pub name: String,
    /// Whether this tag can only be added to or removed from threads by a member with the
    /// MANAGE_THREADS permission.
    pub moderated: bool,
    /// The emoji to display next to the tag.
    #[serde(flatten)]
    pub emoji: Option<ForumEmoji>,
}

enum_number! {
    /// The sort order for threads in a forum.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#channel-object-sort-order-types).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum SortOrder {
        /// Sort forum posts by activity.
        LatestActivity = 0,
        /// Sort forum posts by creation time (from most recent to oldest).
        CreationDate = 1,
        _ => Unknown(u8),
    }
}

bitflags::bitflags! {
    /// Describes extra features of the channel.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#channel-object-channel-flags).
    #[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct ChannelFlags: u16 {
        /// This thread is pinned to the top of its parent GUILD_FORUM channel
        const PINNED = 1 << 1;
        /// Whether a tag is required to be specified when creating a
        /// thread in a GUILD_FORUM channel. Tags are specified in the applied_tags field.
        const REQUIRE_TAG = 1 << 4;
    }
}

enum_number! {
    #[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PermissionOverwriteType {
        /// Permission overwrite targets an individual role.
        Role = 0,
        /// Permission overwrite targets an individual member.
        Member = 1,
        _ => Unknown(u8),
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#overwrite-object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    pub id: AnyId,
    #[serde(rename = "type")]
    pub kind: PermissionOverwriteType,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MinPartialChannel {
    /// ID of the channel.
    pub id: ChannelId,
}

/// For Discord's documentation on channels, refer to [Discord Docs/Channel].
///
/// [Discord Docs/Channel]: https://discord.com/developers/docs/resources/channel
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    /// ID of the guild the channel is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// ID of the channel.
    pub id: ChannelId,
    /// Whether users can be invited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    /// Type of the channel.
    ///
    /// This can be used to determine what fields *might* be available.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// Name of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the channel has been configured to be NSFW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// ID of the creator of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,
    /// ID of the parent channel.
    ///
    /// For guild channels this is the ID of the parent category channel.
    ///
    /// For threads this is the ID of the channel the thread was created in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    /// Explicit permission overwrites for members and roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    /// Sorting position of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    #[serde(flatten)]
    pub extra_info: HashMap<String, serde_json::Value>,
}

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
    pub user_limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
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
    pub default_thread_rate_limit_per_user: Option<u16>,
}

impl Default for CreateChannel {
    fn default() -> Self {
        Self {
            name: "my-channel".into(),
            kind: Some(ChannelType::Text),
            topic: Some("My channel topic".into()),
            bitrate: None,
            user_limit: None,
            rate_limit_per_user: Some(5),
            position: Some(7),
            permission_overwrites: Some(vec![]),
            parent_id: None,
            nsfw: Some(true),
            rtc_region: Some("us-west".into()),
            video_quality_mode: Some(VideoQualityMode::Auto),
            default_auto_archive_duration: Some(AutoArchiveDuration::OneDay),
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
    pub rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u16>,
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
    pub default_thread_rate_limit_per_user: Option<u16>,
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
            kind: Some(ChannelType::Text),
            position: Some(7),
            topic: Some("My channel topic".into()),
            nsfw: Some(true),
            rate_limit_per_user: Some(5),
            bitrate: None,
            permission_overwrites: None,
            parent_id: MultiOption::new(Some(ChannelId::default())),
            rtc_region: MultiOption::new(Some("us-west".into())),
            video_quality_mode: Some(VideoQualityMode::Auto),
            default_auto_archive_duration: Some(AutoArchiveDuration::OneDay),
            flags: Some(ChannelFlags::all()),
            available_tags: None,
            default_reaction_emoji: MultiOption::new(Some(ForumEmoji::Id(
                EmojiId::default(),
            ))),
            default_thread_rate_limit_per_user: None,
            default_sort_order: None,
            default_forum_layout: None,
            status: Some("online".into()),
            user_limit: Some(10),
            archived: Some(false),
            auto_archive_duration: Some(AutoArchiveDuration::OneDay),
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
    pub webhook_channel_id: ChannelId,
}

enum_number! {
    /// Type of target for a voice channel invite.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/invite#invite-object-invite-target-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum InviteTargetType {
        Stream = 1,
        EmbeddedApplication = 2,
        _ => Unknown(u8),
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild#modify-guild-channel-positions)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModifyChannelPosition {
    pub id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
}