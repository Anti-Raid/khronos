use serde::{Deserialize, Serialize};
use serde_json::Value;
use serenity::all::*;
use std::collections::HashMap;

/// A builder for creating a new [`Command`].
///
/// [`Command`]: crate::model::application::Command
///
/// Discord docs:
/// - [global command](https://discord.com/developers/docs/interactions/application-commands#create-global-application-command)
/// - [guild command](https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub kind: Option<CommandType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<EntryPointHandlerType>,

    #[serde(flatten)]
    pub fields: EditCommand,
}

impl Default for CreateCommand {
    fn default() -> Self {
        Self {
            kind: Some(CommandType::ChatInput),
            handler: Some(EntryPointHandlerType::AppHandler),
            fields: Default::default(),
        }
    }
}

/// A builder for editing an existing [`Command`].
///
/// [`Command`]: crate::model::application::Command
///
/// Discord docs:
/// - [global command](https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command)
/// - [guild command](https://discord.com/developers/docs/interactions/application-commands#edit-guild-application-command)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct EditCommand {
    pub name: Option<String>,
    pub name_localizations: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub description_localizations: HashMap<String, String>,
    pub options: Vec<CreateCommandOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<InstallationContext>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContext>>,
    pub nsfw: bool,
}

impl Default for EditCommand {
    fn default() -> Self {
        Self {
            name: Some("my-command".into()),
            name_localizations: HashMap::new(),
            description: Some("My command description".into()),
            description_localizations: HashMap::new(),
            options: Vec::default(),
            default_member_permissions: None,
            dm_permission: None,
            integration_types: None,
            contexts: None,
            nsfw: false,
        }
    }
}

/// A builder for creating a new [`CommandOption`].
///
/// [`Self::kind`], [`Self::name`], and [`Self::description`] are required fields.
///
/// [`CommandOption`]: crate::model::application::CommandOption
///
/// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateCommandOption {
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub choices: Vec<CreateCommandOptionChoice>,
    #[serde(default)]
    pub options: Vec<CreateCommandOption>,
    #[serde(default)]
    pub channel_types: Vec<ChannelType>,
    #[serde(default)]
    pub min_value: Option<serde_json::Number>,
    #[serde(default)]
    pub max_value: Option<serde_json::Number>,
    #[serde(default)]
    pub min_length: Option<u16>,
    #[serde(default)]
    pub max_length: Option<u16>,
    #[serde(default)]
    pub autocomplete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateCommandOptionChoice {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: Value,
}
