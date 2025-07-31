use serde::{Deserialize, Serialize};
use serenity::all::*;

use crate::internal_enum_number;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePoll {
    pub question: CreatePollMedia,
    pub answers: Vec<CreatePollAnswer>,
    pub duration: u8,
    pub allow_multiselect: bool,
    pub layout_type: Option<PollLayoutType>,
}

/// "Only text is supported."
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePollMedia {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreatePollAnswerMedia {
    pub text: Option<String>,
    pub emoji: Option<PollMediaEmoji>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreatePollAnswer {
    pub poll_media: CreatePollAnswerMedia,
}

internal_enum_number! {
    /// Represents the different layouts that a [`Poll`] may have.
    ///
    /// Currently, there is only the one option.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/poll#layout-type)
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[non_exhaustive]
    #[<default> = 1]
    pub enum PollLayoutType {
        Default = 1,
        _ => Unknown(u8),
    }
}

/// The "Partial Emoji" attached to a [`PollMedia`] model.
///
/// [Discord docs](https://discord.com/developers/docs/resources/poll#poll-media-object)
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PollMediaEmoji {
    name: Option<String>,
    id: Option<EmojiId>,
}
