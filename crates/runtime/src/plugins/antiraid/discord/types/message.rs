use super::allowed_mentions::CreateAllowedMentions;
use super::attachment::CreateMessageAttachment;
use super::embed::CreateEmbed;
use super::poll::CreatePoll;
use serde::{Deserialize, Serialize};
use serenity::all::*;

/// [Discord docs](https://discord.com/developers/docs/resources/channel#create-message)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,
    #[serde(default)]
    pub tts: bool,
    #[serde(default)]
    pub embeds: Vec<CreateEmbed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    #[serde(default)]
    pub sticker_ids: Vec<StickerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default)]
    pub enforce_nonce: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#edit-message)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct EditMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
}
