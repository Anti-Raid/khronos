use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A builder to create an embed in a message
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<CreateEmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CreateEmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<CreateEmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<CreateEmbedAuthor>,
    /// No point using a Cow slice, as there is no set_fields method
    /// and CreateEmbedField is not public.
    #[serde(default)]
    pub fields: Vec<CreateEmbedField>,
}

/// A builder to create the footer data for an embed.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateEmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEmbedImage {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

/// A builder to create the author data of an embed. See [`CreateEmbed::author`]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateEmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}
