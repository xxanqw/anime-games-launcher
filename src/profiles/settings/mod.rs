mod general;
mod runtime;

pub use general::Settings as GeneralProfileSettings;

pub use runtime::{
    Runtime as ProfileRuntime,
    WindowsNativeProfileRuntimeSettings,
    LinuxNativeProfileRuntimeSettings,
    LinuxWineProfileRuntimeSettings
};
