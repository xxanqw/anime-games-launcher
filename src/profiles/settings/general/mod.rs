use serde_json::Value as Json;

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
pub enum General {
    Windows(windows::Settings),
    Linux(linux::Settings)
}

impl General {
    pub fn to_json(&self) -> Result<Json, AsJsonError> {
        match self {
            Self::Windows(settings) => settings.to_json(),
            Self::Linux(settings) => settings.to_json()
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
