use serde_json::Value as Json;

use crate::prelude::*;

mod windows_native;
mod linux_native;
mod linux_wine;

pub use windows_native::Settings as WindowsNativeProfileRuntimeSettings;
pub use linux_native::Settings as LinuxNativeProfileRuntimeSettings;
pub use linux_wine::Settings as LinuxWineProfileRuntimeSettings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runtime {
    WindowsNative(windows_native::Settings),
    LinuxNative(linux_native::Settings),
    LinuxWine(linux_wine::Settings)
}

impl Runtime {
    pub fn to_json(&self) -> Result<Json, AsJsonError> {
        match self {
            Self::WindowsNative(settings) => settings.to_json(),
            Self::LinuxNative(settings) => settings.to_json(),
            Self::LinuxWine(settings) => settings.to_json()
        }
    }

    pub fn from_json(target_platform: &TargetPlatform, json: &Json) -> Result<Self, AsJsonError> {
        match target_platform {
            TargetPlatform::X86_64_windows_native => Ok(Self::WindowsNative(windows_native::Settings::from_json(json)?)),
            TargetPlatform::X86_64_linux_native => Ok(Self::LinuxNative(linux_native::Settings::from_json(json)?)),

            TargetPlatform::X86_64_linux_wine32 |
            TargetPlatform::X86_64_linux_wine64 => Ok(Self::LinuxWine(linux_wine::Settings::from_json(json)?))
        }
    }
}

impl From<windows_native::Settings> for Runtime {
    #[inline]
    fn from(value: windows_native::Settings) -> Self {
        Self::WindowsNative(value)
    }
}

impl From<linux_native::Settings> for Runtime {
    #[inline]
    fn from(value: linux_native::Settings) -> Self {
        Self::LinuxNative(value)
    }
}

impl From<linux_wine::Settings> for Runtime {
    #[inline]
    fn from(value: linux_wine::Settings) -> Self {
        Self::LinuxWine(value)
    }
}
