use super::allowed_mentions::CreateAllowedMentions;
use super::attachment::CreateMessageAttachment;
use super::embed::CreateEmbed;
use super::poll::CreatePoll;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serenity::all::*;
use std::collections::HashMap;
use super::serenity_component::Component as SerenityComponent;

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object).
#[derive(Clone, Debug)]
pub enum CreateInteractionResponse {
    /// Acknowledges a Ping (only required when your bot uses an HTTP endpoint URL).
    ///
    /// Corresponds to Discord's `PONG`.
    Pong,
    /// Responds to an interaction with a message.
    ///
    /// Corresponds to Discord's `CHANNEL_MESSAGE_WITH_SOURCE`.
    Message(CreateInteractionResponseMessage),
    /// Acknowledges the interaction in order to edit a response later. The user sees a loading
    /// state.
    ///
    /// Corresponds to Discord's `DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE`.
    Defer(CreateInteractionResponseMessage),
    /// Only valid for component-based interactions (seems to work for modal submit interactions
    /// too even though it's not documented).
    ///
    /// Acknowledges the interaction. You can optionally edit the original message later. The user
    /// does not see a loading state.
    ///
    /// Corresponds to Discord's `DEFERRED_UPDATE_MESSAGE`.
    Acknowledge,
    /// Only valid for component-based interactions.
    ///
    /// Edits the message the component was attached to.
    ///
    /// Corresponds to Discord's `UPDATE_MESSAGE`.
    UpdateMessage(CreateInteractionResponseMessage),
    /// Only valid for autocomplete interactions.
    ///
    /// Responds to the autocomplete interaction with suggested choices.
    ///
    /// Corresponds to Discord's `APPLICATION_COMMAND_AUTOCOMPLETE_RESULT`.
    Autocomplete(CreateAutocompleteResponse),
    /// Not valid for Modal and Ping interactions
    ///
    /// Responds to the interaction with a popup modal.
    ///
    /// Corresponds to Discord's `MODAL`.
    Modal(CreateModal),
    /// Not valid for autocomplete and Ping interactions. Only available for applications with
    /// Activities enabled.
    ///
    /// Responds to the interaction by launching the Activity associated with the app.
    ///
    /// Corresponds to Discord's `LAUNCH_ACTIVITY`.
    LaunchActivity,
}

impl Default for CreateInteractionResponse {
    fn default() -> Self {
        Self::Message(Default::default())
    }
}

impl CreateInteractionResponse {
    pub fn take_files<'a>(&self) -> Result<Vec<serenity::all::CreateAttachment<'a>>, crate::Error> {
        match self {
            Self::Message(x) => {
                if let Some(ref x) = x.attachments {
                    x.take_files()
                } else {
                    Ok(Vec::new())
                }
            }
            Self::Defer(x) => {
                if let Some(ref x) = x.attachments {
                    x.take_files()
                } else {
                    Ok(Vec::new())
                }
            }
            Self::UpdateMessage(x) => {
                if let Some(ref x) = x.attachments {
                    x.take_files()
                } else {
                    Ok(Vec::new())
                }
            }
            _ => Ok(Vec::new()),
        }
    }
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-messages).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateInteractionResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SerenityComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
}

impl serde::Serialize for CreateInteractionResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(
            "type",
            &match self {
                Self::Pong => 1,
                Self::Message(_) => 4,
                Self::Defer(_) => 5,
                Self::Acknowledge => 6,
                Self::UpdateMessage(_) => 7,
                Self::Autocomplete(_) => 8,
                Self::Modal(_) => 9,
                Self::LaunchActivity => 12,
            },
        )?;

        match self {
            Self::Pong => map.serialize_entry("data", &None::<()>)?,
            Self::Message(x) => map.serialize_entry("data", &x)?,
            Self::Defer(x) => map.serialize_entry("data", &x)?,
            Self::Acknowledge => map.serialize_entry("data", &None::<()>)?,
            Self::UpdateMessage(x) => map.serialize_entry("data", &x)?,
            Self::Autocomplete(x) => map.serialize_entry("data", &x)?,
            Self::Modal(x) => map.serialize_entry("data", &x)?,
            Self::LaunchActivity => map.serialize_entry("data", &None::<()>)?,
        }

        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for CreateInteractionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let map = serde_json::Map::deserialize(deserializer)?;

        let raw_kind = map
            .get("type")
            .ok_or_else(|| D::Error::missing_field("type"))?
            .clone();

        let ty = raw_kind
            .as_u64()
            .ok_or_else(|| D::Error::custom("type must be a number"))?;

        match ty {
            1 => Ok(Self::Pong),
            4 => {
                let data = map
                    .get("data")
                    .ok_or_else(|| D::Error::missing_field("data"))?
                    .clone();

                serde_json::from_value(data).map(Self::Message)
            }
            5 => {
                let data = map
                    .get("data")
                    .ok_or_else(|| D::Error::missing_field("data"))?
                    .clone();

                serde_json::from_value(data).map(Self::Defer)
            }
            6 => Ok(Self::Acknowledge),
            7 => {
                let data = map
                    .get("data")
                    .ok_or_else(|| D::Error::missing_field("data"))?
                    .clone();

                serde_json::from_value(data).map(Self::UpdateMessage)
            }
            8 => {
                let data = map
                    .get("data")
                    .ok_or_else(|| D::Error::missing_field("data"))?
                    .clone();

                serde_json::from_value(data).map(Self::Autocomplete)
            }
            9 => {
                let data = map
                    .get("data")
                    .ok_or_else(|| D::Error::missing_field("data"))?
                    .clone();

                serde_json::from_value(data).map(Self::Modal)
            }
            12 => Ok(Self::LaunchActivity),
            _ => {
                return Err(D::Error::custom(format!(
                    "Unknown interaction response type: {ty}",
                )));
            }
        }
        .map_err(D::Error::custom)
    }
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateAutocompleteResponse {
    choices: Vec<AutocompleteChoice>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AutocompleteValue {
    String(String),
    Integer(u64),
    Float(f64),
}

// Same as CommandOptionChoice according to Discord, see
// [Autocomplete docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete).
#[must_use]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutocompleteChoice {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: AutocompleteValue,
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-modal).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateModal {
    components: Vec<ActionRow>,
    custom_id: String,
    title: String,
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#create-followup-message)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateInteractionResponseFollowup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    // [Omitting username: not supported in interaction followups]
    // [Omitting avatar_url: not supported in interaction followups]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    pub embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SerenityComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
    pub attachments: Option<CreateMessageAttachment>,
}

/// A builder to specify the fields to edit in an existing [`Webhook`]'s message.
///
/// [Discord docs](https://discord.com/developers/docs/resources/webhook#edit-webhook-message)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EditWebhookMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SerenityComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
}

#[cfg(test)]
mod test {
    use mluau::LuaSerdeExt;
    use crate::plugins::antiraid::discord::types::serenity_component;
    #[test]
    fn test_comp_serde() {
        let src = serde_json::json!({
            "type": 1,
            "components": [
                {
                    "type": 2,
                    "style": 1,
                    "label": "Support Server",
                    "url": "https://discord.gg/9BJWSrEBBJ"
                }
            ]
        });
        println!("{:?}", src);
        let _src_comp: serenity_component::ActionRow = serde_json::from_value(src.clone()).unwrap();
        let lua = mluau::Lua::new();
        let data = lua.to_value(&src).unwrap();
        let _comp: serenity_component::Component = lua.from_value(data).unwrap();
    }
}
