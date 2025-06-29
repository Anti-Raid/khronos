//! Contains the available preset types in Khronos CLI

use strum::{IntoStaticStr, VariantNames};

#[derive(Debug, serde::Serialize, serde::Deserialize, IntoStaticStr, VariantNames, Clone, Copy)]
#[must_use]
pub enum AntiraidEventPresetType {
    /// An on startup event is fired when a set of templates are modified
    ///
    /// The inner Vec<String> is the list of templates modified/reloaded
    OnStartup,

    /// A key external modify event. Fired when a key is modified externally
    ExternalKeyUpdate,

    /// A key expiry event
    KeyExpiry,

    /// A get settings event
    GetSettings,

    /// A execute setting event
    ExecuteSetting,
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
            "onstartup" | "on_startup" | "startup" | "start" | "init" => Ok(Self::OnStartup),
            "externalkeyupdate" => Ok(Self::ExternalKeyUpdate),
            "keyexpiry" => Ok(Self::KeyExpiry),
            "getsettings" | "get_settings" => Ok(Self::GetSettings),
            "executesetting" | "execute_setting" => Ok(Self::ExecuteSetting),
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
