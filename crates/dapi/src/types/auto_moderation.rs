use serde::{Deserialize, Serialize};
use crate::{ChannelId, RoleId, enum_number};

enum_number! {
    /// Indicates in what event context a rule should be checked.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum EventType {
        MessageSend = 1,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Indicates in what event context a rule should be checked.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    
    #[non_exhaustive]
    pub enum TriggerType {
        Keyword = 1,
        Spam = 3,
        KeywordPreset = 4,
        MentionSpam = 5,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Internally pre-defined wordsets which will be searched for in content.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-preset-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]

    #[non_exhaustive]
    pub enum KeywordPresetType {
        Profanity = 1,
        SexualContent = 2,
        Slurs = 3,
        _ => Unknown(u8),
    }
}

/// Characterizes the type of content which can trigger the rule.
///
/// Discord docs:
/// [type](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types),
/// [metadata](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Trigger {
    #[serde(rename = "trigger_type")]
    kind: TriggerType,
    #[serde(rename = "trigger_metadata")]
    metadata: TriggerMetadata,
}

/// An action which will execute whenever a rule is triggered.
///
/// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object).
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Action {
    #[serde(rename = "type")]
    pub kind: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ActionMetadata>,
}

/// Metadata for an action.
#[derive(Default, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ActionMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_message: Option<String>,
}

enum_number! {
    /// See [`Action`]
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[non_exhaustive]
    pub enum ActionType {
        BlockMessage = 1,
        Alert = 2,
        Timeout = 3,
        _ => Unknown(u8),
    }
}

/// Individual change for trigger metadata within an audit log entry.
///
/// Different fields are relevant based on the value of trigger_type. See
/// [`Change::TriggerMetadata`].
///
/// [`Change::TriggerMetadata`]: crate::model::guild::audit_log::Change::TriggerMetadata
///
/// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata).

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TriggerMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<KeywordPresetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_total_limit: Option<u8>,
}


/// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#create-auto-moderation-rule)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct CreateAutoModRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    event_type: EventType,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempt_roles: Option<Vec<RoleId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempt_channels: Option<Vec<ChannelId>>,
}

impl CreateAutoModRule {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref exempt_roles) = self.exempt_roles {
            if exempt_roles.len() > 20 {
                return Err("A maximum of 20 exempt_roles can be provided".into());
            }
        }

        if let Some(ref exempt_channels) = self.exempt_channels {
            if exempt_channels.len() > 20 {
                return Err("A maximum of 20 exempt_channels can be provided".into());
            }
        }

        Ok(())
    }
}

impl Default for CreateAutoModRule {
    fn default() -> Self {
        Self {
            name: None,
            trigger: None,
            actions: None,
            enabled: None,
            exempt_roles: None,
            exempt_channels: None,
            event_type: EventType::MessageSend,
        }
    }
}

/// A builder for editing guild AutoMod rules.
///
/// # Examples
///
/// See [`GuildId::edit_automod_rule`] for details.
///
/// [Discord docs](https://discord.com/developers/docs/resources/auto-moderation#modify-auto-moderation-rule)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[must_use]
pub struct EditAutoModRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    event_type: EventType,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    trigger_metadata: Option<TriggerMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempt_roles: Option<Vec<RoleId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempt_channels: Option<Vec<ChannelId>>,
}

impl EditAutoModRule {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref exempt_roles) = self.exempt_roles {
            if exempt_roles.len() > 20 {
                return Err("A maximum of 20 exempt_roles can be provided".into());
            }
        }

        if let Some(ref exempt_channels) = self.exempt_channels {
            if exempt_channels.len() > 20 {
                return Err("A maximum of 20 exempt_channels can be provided".into());
            }
        }

        Ok(())
    }
}

impl Default for EditAutoModRule {
    fn default() -> Self {
        Self {
            name: None,
            trigger_metadata: None,
            actions: None,
            enabled: None,
            exempt_roles: None,
            exempt_channels: None,
            event_type: EventType::MessageSend,
        }
    }
}
