//! Provides the implementations for all Antiraid Event Preset Types

use super::defaults::{
    default_global_guild_id, default_global_member, default_global_unknown_string,
    default_global_user, default_global_user_id, default_global_userinfo,
    default_moderation_end_correlation_id, default_moderation_start_correlation_id,
};
use super::types::AntiraidEventPresetType;
use antiraid_types::ar_event::{
    ExternalKeyUpdateEventData, ExternalKeyUpdateEventDataAction, ModerationEndEventData,
    TemplateSettingExecuteEventData, TemplateSettingExecuteEventDataAction,
};
use antiraid_types::{
    ar_event::{AntiraidEvent, ModerationAction},
    punishments::{Punishment, PunishmentState, PunishmentTarget},
    stings::{Sting, StingState, StingTarget},
    userinfo::UserInfo,
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
            Self::StingCreate => {
                let data: StingPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::StingCreate(data.into_sting()))
            }
            Self::StingUpdate => {
                let data: StingPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::StingUpdate(data.into_sting()))
            }
            Self::StingExpire => {
                let data: StingPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::StingExpire(data.into_sting()))
            }
            Self::StingDelete => {
                let data: StingPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::StingDelete(data.into_sting()))
            }
            Self::PunishmentCreate => {
                let data: PunishmentPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::PunishmentCreate(data.into_punishment()))
            }
            Self::PunishmentDelete => {
                let data: PunishmentPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::PunishmentDelete(data.into_punishment()))
            }
            Self::PunishmentExpire => {
                let data: PunishmentPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::PunishmentExpire(data.into_punishment()))
            }
            Self::OnStartup => {
                let data: Option<Vec<String>> = from_value(input_data)?;
                Ok(AntiraidEvent::OnStartup(data.unwrap_or_default()))
            }
            Self::BuiltinCommandExecute => {
                let data: BuiltinCommandExecuteDataPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::BuiltinCommandExecute(
                    data.into_builtin_command_execute_data(),
                ))
            }
            Self::PermissionCheckExecute => {
                let data: PermissionCheckData = from_value(input_data)?;
                Ok(AntiraidEvent::PermissionCheckExecute(
                    data.into_permission_check_data(),
                ))
            }
            Self::ModerationStart => {
                let data: ModerationStartEventDataPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::ModerationStart(
                    data.into_moderation_start_event_data(),
                ))
            }
            Self::ModerationEnd => {
                let data: ModerationEndEventDataPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::ModerationEnd(
                    data.into_moderation_end_event_data(),
                ))
            }
            Self::ExternalKeyUpdate => {
                let data: ExternalKeyUpdateEventDataPresetBaseInputData = from_value(input_data)?;
                Ok(AntiraidEvent::ExternalKeyUpdate(
                    data.into_external_key_update_event_data(),
                ))
            }
            Self::TemplateSettingExecute => {
                let data: TemplateSettingExecuteEventDataPresetBaseInputData =
                    from_value(input_data)?;
                Ok(AntiraidEvent::TemplateSettingExecute(
                    data.into_template_setting_execute_event_data(),
                ))
            }
        }
    }
}

fn default_sting_target() -> StingTarget {
    StingTarget::System
}

/// The base input data for a sting preset
#[derive(serde::Deserialize)]
struct StingPresetBaseInputData {
    /// The ID of the sting
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    /// Src of the sting, this can be useful to store the source of the sting
    pub src: Option<String>,
    /// The number of stings
    #[serde(default)]
    pub stings: i32,
    /// The reason for the stings (optional)
    pub reason: Option<String>,
    /// The reason the stings were voided
    pub void_reason: Option<String>,
    /// The guild ID the sting targets
    #[serde(default = "default_global_guild_id")]
    pub guild_id: serenity::all::GuildId,
    /// The creator of the sting
    #[serde(default = "default_sting_target")]
    pub creator: StingTarget,
    /// The target of the sting
    #[serde(default = "default_sting_target")]
    pub target: StingTarget,
    /// The state of the sting
    #[serde(default)]
    pub state: StingState,
    /// When the sting expires as a chrono duration
    pub duration: Option<std::time::Duration>,
    /// The data/metadata present within the sting, if any
    pub sting_data: Option<serde_json::Value>,
    /// When the sting was created
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    /// The handle log encountered while handling the sting
    pub handle_log: serde_json::Value,
}

impl StingPresetBaseInputData {
    fn into_sting(self) -> Sting {
        Sting {
            id: self.id,
            src: self.src,
            stings: self.stings,
            reason: self.reason,
            void_reason: self.void_reason,
            guild_id: self.guild_id,
            creator: self.creator,
            target: self.target,
            state: self.state,
            duration: self.duration,
            sting_data: self.sting_data,
            created_at: self.created_at,
            handle_log: self.handle_log,
        }
    }
}

fn default_punishment_target() -> PunishmentTarget {
    PunishmentTarget::System
}

/// The base input data for a punishment preset
#[derive(serde::Deserialize)]
pub struct PunishmentPresetBaseInputData {
    /// The ID of the applied punishment
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    /// Src of the sting, this can be useful if a module wants to store the source of the sting
    pub src: Option<String>,
    /// The guild id of the applied punishment
    #[serde(default = "default_global_guild_id")]
    pub guild_id: serenity::all::GuildId,
    /// The punishment string
    #[serde(default = "default_global_unknown_string")]
    pub punishment: String,
    /// Creator of the punishment
    #[serde(default = "default_punishment_target")]
    pub creator: PunishmentTarget,
    /// The target of the punishment
    #[serde(default = "default_punishment_target")]
    pub target: PunishmentTarget,
    /// The handle log encountered while handling the punishment
    #[serde(default)]
    pub handle_log: serde_json::Value,
    /// When the punishment was created
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Duration of the punishment
    pub duration: Option<std::time::Duration>,
    /// The reason for the punishment
    #[serde(default)]
    pub reason: String,
    /// The state of the sting
    #[serde(default)]
    pub state: PunishmentState,
    /// Extra misc data
    pub data: Option<serde_json::Value>,
}

impl PunishmentPresetBaseInputData {
    fn into_punishment(self) -> Punishment {
        Punishment {
            id: self.id,
            src: self.src,
            guild_id: self.guild_id,
            punishment: self.punishment,
            creator: self.creator,
            target: self.target,
            handle_log: self.handle_log,
            created_at: self.created_at,
            duration: self.duration,
            reason: self.reason,
            state: self.state,
            data: self.data,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct BuiltinCommandExecuteDataPresetBaseInputData {
    #[serde(default = "default_global_unknown_string")]
    pub command: String,
    #[serde(default = "default_global_user_id")]
    pub user_id: serenity::all::UserId,
    #[serde(default = "default_global_userinfo")]
    pub user_info: UserInfo,
}

impl BuiltinCommandExecuteDataPresetBaseInputData {
    fn into_builtin_command_execute_data(
        self,
    ) -> antiraid_types::ar_event::BuiltinCommandExecuteData {
        antiraid_types::ar_event::BuiltinCommandExecuteData {
            command: self.command,
            user_id: self.user_id,
            user_info: self.user_info,
        }
    }
}

fn default_permission_check_data_perm() -> kittycat::perms::Permission {
    kittycat::perms::Permission::from_string("~foo.bar")
}

#[derive(serde::Deserialize)]
pub struct PermissionCheckData {
    #[serde(default = "default_permission_check_data_perm")]
    pub perm: kittycat::perms::Permission,
    #[serde(default = "default_global_user_id")]
    pub user_id: serenity::all::UserId,
    #[serde(default = "default_global_userinfo")]
    pub user_info: UserInfo,
}

impl PermissionCheckData {
    fn into_permission_check_data(self) -> antiraid_types::ar_event::PermissionCheckData {
        antiraid_types::ar_event::PermissionCheckData {
            perm: self.perm,
            user_id: self.user_id,
            user_info: self.user_info,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(tag = "action")]
pub enum ModerationActionPresetBaseInputData {
    Kick {
        #[serde(default = "default_global_member")]
        member: serenity::all::Member, // The target to kick
    },
    TempBan {
        #[serde(default = "default_global_user")]
        user: serenity::all::User, // The target to ban
        #[serde(default)]
        duration: u64, // Duration, in seconds
        #[serde(default)]
        prune_dmd: u8,
    },
    Ban {
        #[serde(default = "default_global_user")]
        user: serenity::all::User, // The target to ban
        #[serde(default)]
        prune_dmd: u8,
    },
    Unban {
        #[serde(default = "default_global_user")]
        user: serenity::all::User, // The target to unban
    },
    Timeout {
        #[serde(default = "default_global_member")]
        member: serenity::all::Member, // The target to timeout
        #[serde(default)]
        duration: u64, // Duration, in seconds
    },
    Prune {
        user: Option<serenity::all::User>,
        #[serde(default)]
        prune_opts: serde_json::Value,
        #[serde(default)]
        channels: Vec<serenity::all::ChannelId>,
    },
}

impl ModerationActionPresetBaseInputData {
    fn into_moderation_action(self) -> ModerationAction {
        match self {
            Self::Kick { member } => ModerationAction::Kick { member },
            Self::TempBan {
                user,
                duration,
                prune_dmd,
            } => ModerationAction::TempBan {
                user,
                duration,
                prune_dmd,
            },
            Self::Ban { user, prune_dmd } => ModerationAction::Ban { user, prune_dmd },
            Self::Unban { user } => ModerationAction::Unban { user },
            Self::Timeout { member, duration } => ModerationAction::Timeout { member, duration },
            Self::Prune {
                user,
                prune_opts,
                channels,
            } => ModerationAction::Prune {
                user,
                prune_opts,
                channels,
            },
        }
    }
}

fn default_moderation_action() -> ModerationActionPresetBaseInputData {
    ModerationActionPresetBaseInputData::Kick {
        member: default_global_member(),
    }
}

#[derive(serde::Deserialize)]
pub struct ModerationStartEventDataPresetBaseInputData {
    #[serde(default = "default_moderation_start_correlation_id")]
    pub correlation_id: uuid::Uuid, // This will also be sent on ModerationEndEventData to correlate the events while avoiding duplication of data
    #[serde(default = "default_moderation_action")]
    pub action: ModerationActionPresetBaseInputData,
    #[serde(default = "default_global_member")]
    pub author: serenity::all::Member,
    #[serde(default)]
    pub num_stings: i32,
    #[serde(default)]
    pub reason: Option<String>,
}

impl ModerationStartEventDataPresetBaseInputData {
    fn into_moderation_start_event_data(
        self,
    ) -> antiraid_types::ar_event::ModerationStartEventData {
        antiraid_types::ar_event::ModerationStartEventData {
            correlation_id: self.correlation_id,
            action: self.action.into_moderation_action(),
            author: self.author,
            num_stings: self.num_stings,
            reason: self.reason,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ModerationEndEventDataPresetBaseInputData {
    #[serde(default = "default_moderation_end_correlation_id")]
    pub correlation_id: uuid::Uuid, // Will correlate with a ModerationStart's event data
}

impl ModerationEndEventDataPresetBaseInputData {
    fn into_moderation_end_event_data(self) -> ModerationEndEventData {
        ModerationEndEventData {
            correlation_id: self.correlation_id,
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "op")]
pub enum TemplateSettingExecuteEventDataActionPresetBaseInputData {
    View {
        #[serde(default)]
        filters: indexmap::IndexMap<String, Value>,
    },
    Create {
        #[serde(default)]
        fields: indexmap::IndexMap<String, Value>,
    },
    Update {
        #[serde(default)]
        fields: indexmap::IndexMap<String, Value>,
    },
    Delete {
        #[serde(default)]
        primary_key: Value,
    },
}

impl TemplateSettingExecuteEventDataActionPresetBaseInputData {
    fn into_template_setting_execute_event_data_action(
        self,
    ) -> TemplateSettingExecuteEventDataAction {
        match self {
            Self::View { filters } => TemplateSettingExecuteEventDataAction::View { filters },
            Self::Create { fields } => TemplateSettingExecuteEventDataAction::Create { fields },
            Self::Update { fields } => TemplateSettingExecuteEventDataAction::Update { fields },
            Self::Delete { primary_key } => {
                TemplateSettingExecuteEventDataAction::Delete { primary_key }
            }
        }
    }
}

fn default_template_setting_execute_event_data_action(
) -> TemplateSettingExecuteEventDataActionPresetBaseInputData {
    TemplateSettingExecuteEventDataActionPresetBaseInputData::View {
        filters: indexmap::IndexMap::new(),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateSettingExecuteEventDataPresetBaseInputData {
    #[serde(default = "default_global_unknown_string")]
    pub template_id: String,
    #[serde(default = "default_global_unknown_string")]
    pub setting_id: String,
    #[serde(default = "uuid::Uuid::new_v4")]
    // We don't need any consistent correlation ID here like we do with moderation actions
    pub correlation_id: uuid::Uuid, // A response from this must include a "correlation_id" field with this value so
    #[serde(default = "default_template_setting_execute_event_data_action")]
    pub action: TemplateSettingExecuteEventDataActionPresetBaseInputData,
    #[serde(default = "default_global_user_id")]
    pub author: serenity::all::UserId,
}

impl TemplateSettingExecuteEventDataPresetBaseInputData {
    fn into_template_setting_execute_event_data(self) -> TemplateSettingExecuteEventData {
        TemplateSettingExecuteEventData {
            template_id: self.template_id,
            setting_id: self.setting_id,
            correlation_id: self.correlation_id,
            action: self
                .action
                .into_template_setting_execute_event_data_action(),
            author: self.author,
        }
    }
}

#[cfg(test)]
mod preset_impls_test {
    use super::CreateEventFromPresetType;
    use crate::presets::types::AntiraidEventPresetType;
    use std::str::FromStr;
    use strum::VariantNames;

    #[test]
    fn ensure_null_inputs() {
        for variant in AntiraidEventPresetType::VARIANTS {
            let variant = AntiraidEventPresetType::from_str(variant).unwrap();

            let evt = variant
                .to_event(serde_json::Value::Null)
                .unwrap_or_else(|_| panic!("Failed to create event from preset: {:?}", variant));

            let evt_fmt = format!("{:?}", evt);

            println!("Event: {:?}", evt);

            assert!(
                !evt_fmt.contains("GuildId(InnerId(0))"),
                "GuildId of 0 found in debug of {:?} with fmt: {}",
                variant,
                evt_fmt
            );

            assert!(
                !evt_fmt.contains("UserId(InnerId(0))"),
                "UserId of 0 found in debug of {:?} with fmt: {}",
                variant,
                evt_fmt
            )
        }
    }
}
