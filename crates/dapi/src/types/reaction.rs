use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError, ser::SerializeMap};

use crate::EmojiId;

/// The type of a [`Reaction`] sent.

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ReactionType {
    /// A reaction with a [`Guild`]s custom [`Emoji`], which is unique to the guild.
    Custom {
        /// Whether the emoji is animated.
        animated: bool,
        /// The Id of the custom [`Emoji`].
        id: EmojiId,
        /// The name of the custom emoji. This is primarily used for decoration and distinguishing
        /// the emoji client-side.
        name: Option<String>,
    },
    /// A reaction with a twemoji.
    Unicode(String),
}

impl ReactionType {
    pub fn as_data(&self) -> String {
        match self {
            ReactionType::Custom {
                id,
                name,
                ..
            } => {
                format!("{}:{id}", name.as_deref().unwrap_or_default())
            },
            ReactionType::Unicode(unicode) => {
                utf8_percent_encode(unicode, NON_ALPHANUMERIC).to_string()
            },
        }
    }
}

// Manual impl needed to decide enum variant by presence of `id`
impl<'de> Deserialize<'de> for ReactionType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct PartialEmoji {
            #[serde(default)]
            animated: bool,
            id: Option<EmojiId>,
            name: Option<String>,
        }
        let emoji = PartialEmoji::deserialize(deserializer)?;
        Ok(match (emoji.id, emoji.name) {
            (Some(id), name) => ReactionType::Custom {
                animated: emoji.animated,
                id,
                name,
            },
            (None, Some(name)) => ReactionType::Unicode(name),
            (None, None) => return Err(DeError::custom("invalid reaction type data")),
        })
    }
}

impl Serialize for ReactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ReactionType::Custom {
                animated,
                id,
                name,
            } => {
                let mut map = serializer.serialize_map(Some(3))?;

                map.serialize_entry("animated", animated)?;
                map.serialize_entry("id", id)?;
                map.serialize_entry("name", name)?;

                map.end()
            },
            ReactionType::Unicode(name) => {
                let mut map = serializer.serialize_map(Some(1))?;

                map.serialize_entry("name", name)?;

                map.end()
            },
        }
    }
}
