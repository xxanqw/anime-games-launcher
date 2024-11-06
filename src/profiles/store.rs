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
    AsJson(#[from] AsJsonError),

    #[error("Failed to await profile reading task: {0}")]
    Async(#[from] tokio::task::JoinError)
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
    pub async fn list(&self) -> Result<Vec<Profile>, StoreError> {
        let mut tasks = tokio::task::JoinSet::new();

        for entry in std::fs::read_dir(&self.folder)? {
            let entry = entry?;

            if entry.path().is_file() {
                tasks.spawn(async move {
                    let profile = tokio::fs::read(entry.path()).await?;
                    let profile = serde_json::from_slice::<Json>(&profile)?;
                    let profile = Profile::from_json(&profile)?;

                    Ok::<_, StoreError>(profile)
                });
            }
        }

        let mut profiles = Vec::with_capacity(tasks.len());

        while let Some(profile) = tasks.join_next().await {
            profiles.push(profile??);
        }

        // Sort all the profiles so they have consistent positioning.
        profiles.sort_by(|a, b| {
            a.name.cmp(&b.name)
        });

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
