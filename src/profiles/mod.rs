use std::str::FromStr;

use serde_json::{json, Value as Json};

use crate::prelude::*;

pub mod settings;
pub mod store;

pub mod prelude {
    pub use super::settings::prelude::*;

    pub use super::store::{
        Store as ProfilesStore,
        StoreError as ProfilesStoreError
    };

    pub use super::Profile;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// ID of the current profile.
    id: Hash,

    /// Name of the current profile.
    name: String,

    /// Platform on which current platform should work.
    source: TargetPlatform,

    /// Platform which current profile should emulate.
    target: TargetPlatform,

    /// Soruce -> target CPU instructions translation settings.
    /// 
    /// Not yet supported (and unsure if ever will be).
    translation: (),

    /// Source -> target OS virtualisation settings.
    /// 
    /// Not yet supported (and unsure if ever will be).
    virtualisation: (),

    /// General and OS-specific profile settings.
    general: GeneralProfileSettings,

    /// Source -> target environment compatibility settings.
    runtime: RuntimeProfileSettings
}

impl Profile {
    /// Create new profile with specified name
    /// and default settings depending on the current
    /// platform.
    /// 
    /// Fallback to default source platform if current one
    /// is unknown.
    pub fn new(name: impl ToString) -> Self {
        let source = CURRENT_PLATFORM.unwrap_or_default();

        Self {
            id: Hash::rand(),
            name: name.to_string(),
            source,

            // Predict target platform of the profile
            // depending on the current source platform.
            target: match source {
                TargetPlatform::X86_64_windows_native => TargetPlatform::X86_64_windows_native,
                TargetPlatform::X86_64_linux_native   => TargetPlatform::X86_64_linux_wine64,
                TargetPlatform::X86_64_linux_wine32   => TargetPlatform::X86_64_linux_wine32,
                TargetPlatform::X86_64_linux_wine64   => TargetPlatform::X86_64_linux_wine64
            },

            translation: (),
            virtualisation: (),
            general: GeneralProfileSettings::default(),
            runtime: RuntimeProfileSettings::default()
        }
    }

    #[inline]
    /// Get profile ID.
    pub fn id(&self) -> &Hash {
        &self.id
    }

    #[inline]
    /// Get profile name.
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    /// Get source platform.
    pub fn source_platform(&self) -> &TargetPlatform {
        &self.source
    }

    #[inline]
    /// Get target platform.
    pub fn target_platform(&self) -> &TargetPlatform {
        &self.target
    }
}

impl AsJson for Profile {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "profile": {
                "id": self.id.to_base32(),
                "name": self.name,
                "source": self.source.to_string(),
                "target": self.target.to_string()
            },

            "translation": Json::Null,
            "virtualisation": Json::Null,

            "general": self.general.to_json()?,
            "runtime": self.runtime.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        let profile = json.get("profile")
            .ok_or_else(|| AsJsonError::FieldNotFound("profile"))?;

        let target = profile.get("target")
            .ok_or_else(|| AsJsonError::FieldNotFound("profile.target"))?
            .as_str()
            .map(TargetPlatform::from_str)
            .ok_or_else(|| AsJsonError::InvalidFieldValue("profile.target"))?
            .map_err(|err| AsJsonError::Other(err.into()))?;

        let general = json.get("general")
            .map(|general| GeneralProfileSettings::from_json(&target, general))
            .ok_or_else(|| AsJsonError::FieldNotFound("general"))??;

        let runtime = json.get("runtime")
            .map(|runtime| RuntimeProfileSettings::from_json(&target, runtime))
            .ok_or_else(|| AsJsonError::FieldNotFound("runtime"))??;

        Ok(Self {
            id: profile.get("id")
                .ok_or_else(|| AsJsonError::FieldNotFound("profile.id"))?
                .as_str()
                .and_then(Hash::from_base32)
                .ok_or_else(|| AsJsonError::InvalidFieldValue("profile.id"))?,

            name: profile.get("name")
                .ok_or_else(|| AsJsonError::FieldNotFound("profile.name"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("profile.name"))?
                .to_string(),

            source: profile.get("source")
                .ok_or_else(|| AsJsonError::FieldNotFound("profile.source"))?
                .as_str()
                .map(TargetPlatform::from_str)
                .ok_or_else(|| AsJsonError::InvalidFieldValue("profile.source"))?
                .map_err(|err| AsJsonError::Other(err.into()))?,

            target,

            translation: (),
            virtualisation: (),

            general,
            runtime
        })
    }
}
