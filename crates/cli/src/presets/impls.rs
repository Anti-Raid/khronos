//! Provides the implementations for all Antiraid Event Preset Types

use super::defaults::{default_global_unknown_string, default_global_user_id};
use super::types::AntiraidEventPresetType;
use antiraid_types::ar_event::AntiraidEvent;
use antiraid_types::ar_event::{
    ExternalKeyUpdateEventData, ExternalKeyUpdateEventDataAction, GetSettingsEvent, KeyExpiryEvent,
    SettingExecuteEvent,
};
use serde_json::{from_value, Value};

pub trait CreateEventFromPresetType {
    fn to_event(&self, input_data: Value) -> Result<AntiraidEvent, khronos_runtime::Error>;
}

impl CreateEventFromPresetType for AntiraidEventPresetType {
    fn to_event(&self, input_data: Value) -> Result<AntiraidEvent, khronos_runtime::Error> {
        let input_data = match input_data {
            Value::Null => match self {
                Self::OnStartup => Value::Array(Vec::new()),
                _ => Value::Object(serde_json::Map::new()),
            },
            _ => input_data,
        };

        match self {
            Self::OnStartup => {
                let data: Option<Vec<String>> = from_value(input_data)?;
                Ok(AntiraidEvent::OnStartup(data.unwrap_or_default()))
            }
            Self::ExternalKeyUpdate => {
                let data: ExternalKeyUpdateEventDataPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::ExternalKeyUpdate(
                    data.into_external_key_update_event_data(),
                ))
            }
            Self::KeyExpiry => {
                let data: KeyExpiryEvent = from_value(input_data)?;
                Ok(AntiraidEvent::KeyExpiry(data))
            }
            Self::ExecuteSetting => {
                let data: SettingExecuteEvent = from_value(input_data)?;
                Ok(AntiraidEvent::ExecuteSetting(data))
            }
            Self::GetSettings => {
                let data: GetSettingsEvent = from_value(input_data)?;
                Ok(AntiraidEvent::GetSettings(data))
            }
        }
    }
}

fn default_external_key_update_action() -> ExternalKeyUpdateEventDataAction {
    ExternalKeyUpdateEventDataAction::Create
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExternalKeyUpdateEventDataPresetBaseInputData {
    #[serde(default = "default_global_unknown_string")]
    pub key_modified: String,
    #[serde(default = "default_global_user_id")]
    pub author: serenity::all::UserId,
    #[serde(default = "default_external_key_update_action")]
    pub action: ExternalKeyUpdateEventDataAction,
}

impl ExternalKeyUpdateEventDataPresetBaseInputData {
    fn into_external_key_update_event_data(self) -> ExternalKeyUpdateEventData {
        ExternalKeyUpdateEventData {
            key_modified: self.key_modified,
            author: self.author,
            action: self.action,
        }
    }
}
