use std::path::{Path, PathBuf};

use serde_json::Value as Json;

use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to deserialize profile: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Failed to decode profile: {0}")]
    AsJson(#[from] AsJsonError)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Store {
    folder: PathBuf
}

impl Store {
    #[inline]
    /// Create new empty profiles store.
    pub fn new(folder: impl Into<PathBuf>) -> Self {
        Self {
            folder: folder.into()
        }
    }

    #[inline]
    /// Get path to the store folder.
    pub fn folder(&self) -> &Path {
        self.folder.as_path()
    }

    #[inline]
    /// Build path to the profile in the store.
    pub fn get_path(&self, profile: &Hash) -> PathBuf {
        self.folder.join(profile.to_base32())
    }

    #[inline]
    /// Check if profile exists in the store.
    pub fn has_profile(&self, profile: &Hash) -> bool {
        self.get_path(profile).exists()
    }

    /// Return list of all profiles with supported source target.
    pub fn list(&self) -> Result<Vec<Profile>, StoreError> {
        let profiles = std::fs::read_dir(&self.folder)?
            .map(|entry| {
                entry.map_err(StoreError::Io)
                    .and_then(|entry| {
                        if !entry.path().is_file() {
                            return Ok(None);
                        }

                        let profile = std::fs::read(entry.path())?;
                        let profile = serde_json::from_slice::<Json>(&profile)?;
                        let profile = Profile::from_json(&profile)?;

                        Ok::<_, StoreError>(Some(profile))
                    })
            })
            .filter_map(|profile| {
                match profile {
                    Ok(Some(profile)) if Some(profile.source) == *CURRENT_PLATFORM => Some(Ok(profile)),
                    Err(err) => Some(Err(err)),
                    _ => None
                }
            })
            .collect::<Result<Vec<Profile>, _>>()?;

        Ok(profiles)
    }

    /// Insert profile to the store.
    pub fn insert(&self, profile: &Profile) -> Result<(), StoreError> {
        let path = self.get_path(&profile.id);

        let profile = profile.to_json()?;
        let profile = serde_json::to_vec_pretty(&profile)?;

        std::fs::write(path, profile)?;

        Ok(())
    }

    /// Remove profile from the store.
    pub fn remove(&self, profile: &Hash) -> Result<(), StoreError> {
        let path = self.get_path(profile);

        std::fs::remove_file(path)?;

        Ok(())
    }
}
