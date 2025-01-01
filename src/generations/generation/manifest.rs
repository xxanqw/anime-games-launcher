use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Format of the generation.
    pub format: u64,

    /// UTC timestamp of the generation creation time.
    pub generated_at: u64,

    /// List of games added by the user.
    pub games: Vec<GameLock>,

    /// List of components added by the user.
    pub components: Vec<ComponentLock>,

    /// Lock file of the game integration packages.
    pub lock_file: LockFileManifest
}

impl Manifest {
    /// Compose new generation manifest from given parts.
    pub fn compose(
        games: impl IntoIterator<Item = GameLock>,
        components: impl IntoIterator<Item = ComponentLock>,
        lock_file: LockFileManifest
    ) -> Self {
        Self {
            format: 1,
            generated_at: lock_file.metadata.generated_at,
            games: games.into_iter().collect(),
            components: components.into_iter().collect(),
            lock_file
        }
    }
}

impl AsJson for Manifest {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "format": self.format,
            "generated_at": self.generated_at,

            "games": self.games.iter()
                .map(GameLock::to_json)
                .collect::<Result<Vec<_>, _>>()?,

            "components": self.components.iter()
                .map(ComponentLock::to_json)
                .collect::<Result<Vec<_>, _>>()?,

            "lock_file": self.lock_file.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            format: json.get("format")
                .ok_or_else(|| AsJsonError::FieldNotFound("format"))?
                .as_u64()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("format"))?,

            generated_at: json.get("generated_at")
                .ok_or_else(|| AsJsonError::FieldNotFound("generated_at"))?
                .as_u64()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("generated_at"))?,

            games: json.get("games")
                .ok_or_else(|| AsJsonError::FieldNotFound("games"))?
                .as_array()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("games"))?
                .iter()
                .map(GameLock::from_json)
                .collect::<Result<Vec<_>, _>>()?,

            components: json.get("components")
                .ok_or_else(|| AsJsonError::FieldNotFound("components"))?
                .as_array()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components"))?
                .iter()
                .map(ComponentLock::from_json)
                .collect::<Result<Vec<_>, _>>()?,

            lock_file: json.get("lock_file")
                .map(LockFileManifest::from_json)
                .ok_or_else(|| AsJsonError::FieldNotFound("lock_file"))??
        })
    }
}

impl AsHash for Manifest {
    fn hash(&self) -> Hash {
        self.format.hash()
            .chain(self.generated_at.hash())
            .chain(self.games.hash())
            .chain(self.components.hash())
            .chain(self.lock_file.hash())
    }

    fn partial_hash(&self) -> Hash {
        self.format.partial_hash()
            .chain(self.games.partial_hash())
            .chain(self.components.partial_hash())
            .chain(self.lock_file.partial_hash())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameLock {
    /// URL of the game's manifest.
    pub url: String,

    /// Fetched manifest of the game.
    pub manifest: GameManifest
}

impl AsJson for GameLock {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "url": self.url,
            "manifest": self.manifest.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            url: json.get("url")
                .ok_or_else(|| AsJsonError::FieldNotFound("games[].url"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("games[].url"))?
                .to_string(),

            manifest: json.get("manifest")
                .map(GameManifest::from_json)
                .ok_or_else(|| AsJsonError::FieldNotFound("games[].manifest"))??
        })
    }
}

impl AsHash for GameLock {
    #[inline]
    fn hash(&self) -> Hash {
        self.url.hash().chain(self.manifest.hash())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentLock {
    /// URL of the component's manifest.
    pub url: String,

    /// Fetched manifest of the component.
    pub manifest: ComponentsVariantManifest
}

impl AsJson for ComponentLock {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "url": self.url,
            "manifest": self.manifest.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            url: json.get("url")
                .ok_or_else(|| AsJsonError::FieldNotFound("components[].url"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components[].url"))?
                .to_string(),

            manifest: json.get("manifest")
                .map(ComponentsVariantManifest::from_json)
                .ok_or_else(|| AsJsonError::FieldNotFound("components[].manifest"))??
        })
    }
}

impl AsHash for ComponentLock {
    #[inline]
    fn hash(&self) -> Hash {
        self.url.hash().chain(self.manifest.hash())
    }
}
