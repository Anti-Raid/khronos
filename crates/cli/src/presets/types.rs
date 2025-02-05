//! Contains the available preset types in Khronos CLI

use strum::{IntoStaticStr, VariantNames};

#[derive(Debug, serde::Serialize, serde::Deserialize, IntoStaticStr, VariantNames, Clone, Copy)]
#[must_use]
pub enum AntiraidEventPresetType {
    /// A sting create event. Dispatched when a sting is created
    StingCreate,

    /// A sting update event. Dispatched when a sting is updated
    StingUpdate,

    /// A sting expiry event. Dispatched when a sting expires
    StingExpire,

    /// A sting delete event. Dispatched when a sting is manually deleted
    StingDelete,

    /// A punishment create event. Dispatched when a punishment is created
    PunishmentCreate,

    /// A punishment expiration event. Dispatched when a punishment expires
    PunishmentExpire,

    /// A punishment delete event. Dispatched when a punishment is manually deleted
    PunishmentDelete,

    /// An on startup event is fired when a set of templates are modified
    ///
    /// The inner Vec<String> is the list of templates modified/reloaded
    OnStartup,

    /// A builtin command execute event is fired when a core/builtin command is executed
    ///
    /// This contains three fields, the command name, the user id and the UserInfo
    BuiltinCommandExecute,

    /// A permission check event is fired when a permission check is done
    PermissionCheckExecute,

    /// A moderation start event is fired prior to the execution of a moderation action
    ModerationStart,

    /// A moderation end event is fired after the execution of a moderation action
    ///
    /// Note that this event is not guaranteed to be fired (e.g. the action fails, jobserver timeout etc.)
    ModerationEnd,

    /// A key external modify event. Fired when a key is modified externally
    ExternalKeyUpdate,

    /// A template setting execute event. Fired when a template setting is executed
    TemplateSettingExecute,
}

impl AntiraidEventPresetType {
    /// Get the name of the preset
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.into()
    }
}

impl std::str::FromStr for AntiraidEventPresetType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stingcreate" | "sting.create" => Ok(Self::StingCreate),
            "stingupdate" | "sting.update" => Ok(Self::StingUpdate),
            "stingexpire" | "sting.expire" => Ok(Self::StingExpire),
            "stingdelete" | "sting.delete" => Ok(Self::StingDelete),
            "punishmentcreate" | "punishment.create" => Ok(Self::PunishmentCreate),
            "punishmentexpire" | "punishment.expire" => Ok(Self::PunishmentExpire),
            "punishmentdelete" | "punishment.delete" => Ok(Self::PunishmentDelete),
            "onstartup" | "on_startup" | "startup" | "start" | "init" => Ok(Self::OnStartup),
            "builtincommandexecute" => Ok(Self::BuiltinCommandExecute),
            "permissioncheckexecute" => Ok(Self::PermissionCheckExecute),
            "moderationstart" => Ok(Self::ModerationStart),
            "moderationend" => Ok(Self::ModerationEnd),
            "externalkeyupdate" => Ok(Self::ExternalKeyUpdate),
            "templatesettingexecute" => Ok(Self::TemplateSettingExecute),
            _ => Err(format!(
                "Unknown preset type: {}, expected one of {:?}",
                s,
                Self::VARIANTS
            )),
        }
    }
}

#[cfg(test)]
mod test_presets_enum {
    use super::AntiraidEventPresetType;
    use antiraid_types::ar_event::AntiraidEvent;
    use strum::VariantNames;

    /// Ensure all preset types are valid event variants
    #[test]
    fn test_all_presets_exist() {
        for variant in AntiraidEvent::VARIANTS {
            assert!(AntiraidEventPresetType::VARIANTS.contains(variant));
        }

        for variant in AntiraidEventPresetType::VARIANTS {
            assert!(AntiraidEvent::VARIANTS.contains(variant));
        }
    }
}
