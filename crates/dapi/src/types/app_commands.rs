use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{Permissions, enum_number, types::ChannelType};

enum_number! {
    /// The type of an application command.
    ///
    /// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum CommandType {
        ChatInput = 1,
        User = 2,
        Message = 3,
        PrimaryEntryPoint = 4,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Signifies how the invocation of a command of type [`PrimaryEntryPoint`] should be handled.
    ///
    /// [`PrimaryEntryPoint`]: CommandType::PrimaryEntryPoint
    /// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-entry-point-command-handler-types)
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum EntryPointHandlerType {
        AppHandler = 1,
        DiscordLaunchActivity = 2,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// The type of an [`CommandOption`].
    ///
    /// [Discord docs](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum CommandOptionType {
        SubCommand = 1,
        SubCommandGroup = 2,
        String = 3,
        Integer = 4,
        Boolean = 5,
        User = 6,
        Channel = 7,
        Role = 8,
        Mentionable = 9,
        Number = 10,
        Attachment = 11,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// An enum representing the [installation contexts].
    ///
    /// [interaction contexts](https://discord.com/developers/docs/resources/application#application-object-application-integration-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum InstallationContext {
        Guild = 0,
        User = 1,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// An enum representing the different [interaction contexts].
    ///
    /// [interaction contexts](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-context-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum InteractionContext {
        /// Interaction can be used within servers
        Guild = 0,
        /// Interaction can be used within DMs with the app's bot user
        BotDm = 1,
        /// Interaction can be used within Group DMs and DMs other than the app's bot user
        PrivateChannel = 2,
        _ => Unknown(u8),
    }
}

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

impl CreateCommand {
    pub fn validate(&self) -> Result<(), crate::Error> {
        crate::validator::validate_command(self)
    }
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
