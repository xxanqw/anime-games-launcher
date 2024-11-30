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
pub struct ProfileBuilder {
    id: Hash,
    name: String,

    source: TargetPlatform,
    target: Option<TargetPlatform>,

    translation: (),
    virtualisation: (),
    general: Option<GeneralProfileSettings>,
    runtime: Option<RuntimeProfileSettings>
}

impl Default for ProfileBuilder {
    fn default() -> Self {
        let source = CURRENT_PLATFORM.unwrap_or_default();

        Self {
            id: Hash::rand(),
            name: String::from("New profile"),

            source,
            target: None,

            translation: (),
            virtualisation: (),
            general: None,
            runtime: None
        }
    }
}

impl ProfileBuilder {
    #[inline]
    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();

        self
    }

    #[inline]
    pub fn with_source_platform(mut self, platform: TargetPlatform) -> Self {
        self.source = platform;

        self
    }

    #[inline]
    pub fn with_target_platform(mut self, platform: TargetPlatform) -> Self {
        self.target = Some(platform);

        self
    }

    #[inline]
    pub fn with_general(mut self, settings: GeneralProfileSettings) -> Self {
        self.general = Some(settings);

        self
    }

    #[inline]
    pub fn with_runtime(mut self, settings: RuntimeProfileSettings) -> Self {
        self.runtime = Some(settings);

        self
    }

    #[inline]
    pub fn build(self) -> Profile {
        Profile {
            id: self.id,
            name: self.name,
            source: self.source,

            // Predict target platform of the profile
            // depending on the current source platform.
            target: match self.target {
                Some(target) => target,
                None => self.source.suggested_emulation()
            },

            translation: (),
            virtualisation: (),

            general: self.general.unwrap_or_default(),
            runtime: self.runtime.unwrap_or_default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// ID of the current profile.
    id: Hash,

    /// Name of the current profile.
    pub name: String,

    /// Platform on which current platform should work.
    pub source: TargetPlatform,

    /// Platform which current profile should emulate.
    pub target: TargetPlatform,

    /// Soruce -> target CPU instructions translation settings.
    ///
    /// Not yet supported (and unsure if ever will be).
    pub translation: (),

    /// Source -> target OS virtualisation settings.
    ///
    /// Not yet supported (and unsure if ever will be).
    pub virtualisation: (),

    /// General and OS-specific profile settings.
    pub general: GeneralProfileSettings,

    /// Source -> target environment compatibility settings.
    pub runtime: RuntimeProfileSettings
}

impl Default for Profile {
    #[inline]
    fn default() -> Self {
        ProfileBuilder::default().build()
    }
}

impl Profile {
    /// Create new profile with specified name
    /// and default settings depending on the current
    /// platform.
    ///
    /// Fallback to default source platform if current one
    /// is unknown.
    pub fn new(name: impl ToString) -> Self {
        ProfileBuilder::default()
            .with_name(name)
            .build()
    }

    #[inline]
    /// Create profile builder.
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }

    #[inline]
    /// Get profile ID.
    pub fn id(&self) -> &Hash {
        &self.id
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
