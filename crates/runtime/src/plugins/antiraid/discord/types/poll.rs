use serde::{Deserialize, Serialize};
use serenity::all::*;

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
