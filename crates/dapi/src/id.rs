use std::{fmt, marker::PhantomData};

use serde::{Deserializer, de::Error};

/// The inner storage of an ID.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(Rust, packed)]
pub struct Id<W> {
    inner: u64, // TODO: Get something better
    _marker: PhantomData<W>
} 

impl<W> Id<W> {
    pub const fn new(id: u64) -> Self {
        Self { inner: id, _marker: PhantomData }
    }

    pub const fn get(self) -> u64 {
        self.inner
    }
}

impl<W> fmt::Debug for Id<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner;
        inner.fmt(f)
    }
}

impl<W> From<u64> for Id<W> {
    fn from(id: u64) -> Self {
        Id::new(id)
    }
}

impl<W: Copy> PartialEq<u64> for Id<W> {
    fn eq(&self, u: &u64) -> bool {
        self.get() == *u
    }
}

impl<W> From<Id<W>> for u64 {
    fn from(id: Id<W>) -> u64 {
        id.get()
    }
}

struct SnowflakeVisitor<W> {
    _marker: PhantomData<W>
}

impl<W> serde::de::Visitor<'_> for SnowflakeVisitor<W> {
    type Value = Id<W>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or integer snowflake that is not u64::MAX")
    }

    fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
        self.visit_u64(u64::try_from(value).map_err(Error::custom)?)
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Id::new(value))
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        value.parse().map(Id::new).map_err(Error::custom)
    }
}

impl<'de, W> serde::Deserialize<'de> for Id<W> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(SnowflakeVisitor { _marker: PhantomData })
    }
}

impl<W> serde::Serialize for Id<W> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&{ self.inner })
    }
}

macro_rules! id {
    ($($name:ident, $wrapper:ident, $doc:literal;)*) => {
        $(
            #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
            struct $wrapper;

            #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, Debug)]
            #[repr(transparent)]
            #[doc = $doc]
            pub struct $name(Id<$wrapper>);

            impl $name {
                #[doc = concat!("Creates a new ", stringify!($name), " from a u64.")]
                #[must_use]
                pub const fn new(id: u64) -> Self {
                    Self(Id::new(id))
                }

                /// Retrieves the inner `id` as a [`u64`].
                #[must_use]
                pub const fn get(self) -> u64 {
                    self.0.get()
                }
            }
        )*
    }
}

id! {
    UserId, UserMarker, "User ID";
    RoleId, RoleMarker, "Role ID";
    ChannelId, ChannelMarker, "Channel ID";
    MessageId, MessageMarker, "Message ID";
    RuleId, RuleMarker, "Rule ID";
    CommandId, CommandMarker, "Command ID";
    GuildId, GuildWrapper, "GuildId";
    AttachmentId, AttachmentWrapper, "Attachment ID";
    ApplicationId, ApplicationWrapper, "ApplicationId";
    EmojiId, EmojiWrapper, "EmojiId";
    ForumTagId, ForumTagWrapper, "ForumTagId";
    StickerId, StickerWrapper, "StickerId";
    AnyId, AnyWrapper, "AnyId";
    SkuId, SkuWrapper, "SkuId";
}