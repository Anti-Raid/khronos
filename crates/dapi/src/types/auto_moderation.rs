use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, RoleId};
use serenity::model::guild::automod::*;

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
