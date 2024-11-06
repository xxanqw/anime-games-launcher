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
    Windows(windows::Settings),
    Linux(linux::Settings),
    Unknown(common::Settings)
}

impl Default for General {
    fn default() -> Self {
        match *CURRENT_PLATFORM {
            Some(TargetPlatform::X86_64_windows_native) => Self::Windows(windows::Settings::default()),

            Some(TargetPlatform::X86_64_linux_native) |
            Some(TargetPlatform::X86_64_linux_wine32) |
            Some(TargetPlatform::X86_64_linux_wine64) => Self::Linux(linux::Settings::default()),

            None => Self::Unknown(common::Settings::default())
        }
    }
}

impl General {
    pub fn to_json(&self) -> Result<Json, AsJsonError> {
        match self {
            Self::Windows(settings) => settings.to_json(),
            Self::Linux(settings) => settings.to_json(),

            Self::Unknown(settings) => Ok(json!({
                "common": settings.to_json()?
            }))
        }
    }

    pub fn from_json(target_platform: &TargetPlatform, json: &Json) -> Result<Self, AsJsonError> {
        match target_platform {
            TargetPlatform::X86_64_windows_native => Ok(Self::Windows(windows::Settings::from_json(json)?)),

            TargetPlatform::X86_64_linux_native |
            TargetPlatform::X86_64_linux_wine32 |
            TargetPlatform::X86_64_linux_wine64 => Ok(Self::Linux(linux::Settings::from_json(json)?))
        }
    }
}

impl From<windows::Settings> for General {
    #[inline]
    fn from(value: windows::Settings) -> Self {
        Self::Windows(value)
    }
}

impl From<linux::Settings> for General {
    #[inline]
    fn from(value: linux::Settings) -> Self {
        Self::Linux(value)
    }
}
