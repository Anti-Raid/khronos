use serenity::all::*;
use serde::{Deserialize, Serialize};
use crate::multioption::MultiOption;
use super::MessageFlags;
use super::allowed_mentions::CreateAllowedMentions;
use super::attachment::CreateMessageAttachment;
use super::embed::CreateEmbed;
use super::poll::CreatePoll;
use super::serenity_component::Component as SerenityComponent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWebhook {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "MultiOption::should_not_serialize")]
    pub avatar: MultiOption<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/webhook#execute-webhook)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct ExecuteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SerenityComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<GenericId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
}