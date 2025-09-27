use serenity::all::*;
use serde::{Deserialize, Serialize};
use small_fixed_array::FixedString;

/// [Discord docs](https://discord.com/developers/docs/resources/channel#create-message)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
#[serde(tag = "type")]
pub enum ReactionType {
    Unicode {
        data: FixedString, // Unicode emoji name
    },
    Custom {
        animated: bool,
        id: EmojiId,
        name: Option<FixedString>,
    },
}

impl ReactionType {
    /// Converts to a serenity::all::ReactionType
    pub fn into_serenity(self) -> serenity::all::ReactionType {
        match self {
            ReactionType::Unicode { data } => serenity::all::ReactionType::Unicode(data),
            ReactionType::Custom { animated, id, name } => {
                serenity::all::ReactionType::Custom {
                    animated,
                    id,
                    name,
                }
            }
        }
    }
}