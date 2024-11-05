pub mod general;
pub mod runtime;

pub mod prelude {
    pub use super::general::Settings as GeneralProfileSettings;

    pub use super::runtime::{
        Runtime as ProfileRuntime,
        WindowsNativeProfileRuntimeSettings,
        LinuxNativeProfileRuntimeSettings,
        LinuxWineProfileRuntimeSettings
    };
}
