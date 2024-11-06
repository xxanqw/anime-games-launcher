use serde_json::{json, Value as Json};

use crate::prelude::*;

mod common;
mod windows;
mod linux;

pub mod prelude {
    pub use super::common::Settings as CommonGeneralProfileSettings;
    pub use super::windows::Settings as WindowsGeneralProfileSettings;
    pub use super::linux::Settings as LinuxGeneralProfileSettings;

    pub use super::General as GeneralProfileSettings;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// General settings specify environment flags
/// and additional applications used to launch the game
/// depending on the target platform's operation system.
pub enum General {
    Windows {
        common: common::Settings,
        windows: windows::Settings
    },

    Linux {
        common: common::Settings,
        linux: linux::Settings
    },

    Unknown(common::Settings)
}

impl Default for General {
    fn default() -> Self {
        match *CURRENT_PLATFORM {
            Some(TargetPlatform::X86_64_windows_native) => Self::Windows {
                common: common::Settings::default(),
                windows: windows::Settings::default()
            },

            Some(TargetPlatform::X86_64_linux_native) |
            Some(TargetPlatform::X86_64_linux_wine32) |
            Some(TargetPlatform::X86_64_linux_wine64) => Self::Linux {
                common: common::Settings::default(),
                linux: linux::Settings::default()
            },

            None => Self::Unknown(common::Settings::default())
        }
    }
}

impl General {
    pub fn to_json(&self) -> Result<Json, AsJsonError> {
        match self {
            Self::Windows { common, windows } => Ok(json!({
                "common": common.to_json()?,
                "windows": windows.to_json()?
            })),

            Self::Linux { common, linux } => Ok(json!({
                "common": common.to_json()?,
                "linux": linux.to_json()?
            })),

            Self::Unknown(settings) => Ok(json!({
                "common": settings.to_json()?
            }))
        }
    }

    pub fn from_json(target_platform: &TargetPlatform, json: &Json) -> Result<Self, AsJsonError> {
        match target_platform {
            TargetPlatform::X86_64_windows_native => Ok(Self::Windows {
                common: json.get("common")
                    .ok_or_else(|| AsJsonError::FieldNotFound("general.common"))
                    .and_then(common::Settings::from_json)?,

                windows: json.get("windows")
                    .ok_or_else(|| AsJsonError::FieldNotFound("general.windows"))
                    .and_then(windows::Settings::from_json)?
            }),

            TargetPlatform::X86_64_linux_native |
            TargetPlatform::X86_64_linux_wine32 |
            TargetPlatform::X86_64_linux_wine64 => Ok(Self::Linux {
                common: json.get("common")
                    .ok_or_else(|| AsJsonError::FieldNotFound("general.common"))
                    .and_then(common::Settings::from_json)?,

                linux: json.get("linux")
                    .ok_or_else(|| AsJsonError::FieldNotFound("general.linux"))
                    .and_then(linux::Settings::from_json)?
            })
        }
    }
}
