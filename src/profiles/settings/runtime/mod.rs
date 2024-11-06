use serde_json::Value as Json;

use crate::prelude::*;

mod linux_wine;

pub mod prelude {
    pub use super::linux_wine::Settings as LinuxWineProfileRuntimeSettings;

    pub use super::Runtime as RuntimeProfileSettings;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Runtime settings configure the runtime environment
/// specified in the target platform.
pub enum Runtime {
    LinuxWine(linux_wine::Settings),
    None
}

impl Default for Runtime {
    fn default() -> Self {
        if *CURRENT_PLATFORM == Some(TargetPlatform::X86_64_linux_native) {
            Self::LinuxWine(linux_wine::Settings::default())
        } else {
            Self::None
        }
    }
}

impl Runtime {
    pub fn to_json(&self) -> Result<Json, AsJsonError> {
        match self {
            Self::LinuxWine(settings) => settings.to_json(),
            Self::None => Ok(Json::Null)
        }
    }

    pub fn from_json(target_platform: &TargetPlatform, json: &Json) -> Result<Self, AsJsonError> {
        match target_platform {
            TargetPlatform::X86_64_linux_wine32 |
            TargetPlatform::X86_64_linux_wine64 => Ok(Self::LinuxWine(linux_wine::Settings::from_json(json)?)),

            _ => Ok(Self::None)
        }
    }
}

impl From<linux_wine::Settings> for Runtime {
    #[inline]
    fn from(value: linux_wine::Settings) -> Self {
        Self::LinuxWine(value)
    }
}
