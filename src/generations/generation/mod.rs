use std::collections::HashSet;

use serde_json::Value as Json;

use crate::prelude::*;

pub mod manifest;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to deserialize game manifest: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Failed to decode game manifest: {0}")]
    AsJson(#[from] AsJsonError),

    #[error("Failed to download game manifest: {0}")]
    DownloaderError(#[from] DownloaderError),

    #[error("Failed to build lock file for game packages: {0}")]
    LockFileError(#[from] LockFileError)
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Generation {
    /// URLs to the games manifests.
    games_manifests: HashSet<String>,

    /// URLs to the components manifests.
    components_manifests: HashSet<String>
}

impl Generation {
    #[inline]
    /// Create new generation file from the provided links.
    ///
    /// Use `Generation::default` if you want an empty generation.
    pub fn new<T: ToString>(games: impl IntoIterator<Item = T>, components: impl IntoIterator<Item = T>) -> Self {
        Self {
            games_manifests: games.into_iter()
                .map(|url| url.to_string())
                .collect(),

            components_manifests: components.into_iter()
                .map(|url| url.to_string())
                .collect()
        }
    }

    /// Change URLs to the games manifests.
    pub fn with_games<T: ToString>(mut self, urls: impl IntoIterator<Item = T>) -> Self {
        self.games_manifests = urls.into_iter()
            .map(|url| url.to_string())
            .collect();

        self
    }

    /// Change URLs to the components manifests.
    pub fn with_components<T: ToString>(mut self, urls: impl IntoIterator<Item = T>) -> Self {
        self.components_manifests = urls.into_iter()
            .map(|url| url.to_string())
            .collect();

        self
    }

    /// Build new generation from provided URLs.
    ///
    /// Note: This is a heavy function which executes
    /// lock file building internally. It's expected
    /// to run it in background.
    pub async fn build(&self, packages_store: &PackagesStore, generations_store: &GenerationsStore) -> Result<GenerationManifest, GenerationError> {
        // Prepare set of packages to be locked.
        let mut packages = HashSet::with_capacity(
            self.games_manifests.len() +
            self.components_manifests.len()
        );

        // Start downloading all added games' manifests.
        let mut games_contexts = Vec::with_capacity(self.games_manifests.len());

        for url in &self.games_manifests {
            let temp_hash = Hash::rand();
            let temp_path = generations_store.get_temp_path(&temp_hash);

            let context = Downloader::new(url)?
                .with_continue_downloading(false)
                .with_output_file(&temp_path)
                .download(|_, _, _| {})
                .await?;

            games_contexts.push((url.clone(), temp_path, context));
        }

        // Await games' manifests and store game packages URLs.
        let mut games = Vec::with_capacity(games_contexts.len());

        for (url, temp_path, context) in games_contexts.drain(..) {
            // Await manifest download finish.
            context.wait()?;

            // Parse the manifest file.
            let manifest = std::fs::read(&temp_path)?;
            let manifest = serde_json::from_slice::<Json>(&manifest)?;
            let manifest = GameManifest::from_json(&manifest)?;

            // Delete it.
            std::fs::remove_file(temp_path)?;

            // Store the manifest and the game package's URL
            // to build a lock file.
            packages.insert(manifest.package.url.clone());

            games.push(GenerationGameLock {
                url,
                manifest
            });
        }

        // Start downloading all added components' manifests.
        let mut components_contexts = Vec::with_capacity(self.components_manifests.len());

        for url in &self.components_manifests {
            let temp_hash = Hash::rand();
            let temp_path = generations_store.get_temp_path(&temp_hash);

            let context = Downloader::new(url)?
                .with_continue_downloading(false)
                .with_output_file(&temp_path)
                .download(|_, _, _| {})
                .await?;

            components_contexts.push((url.clone(), temp_path, context));
        }

        // Await components' manifests and store component packages URLs.
        let mut components = Vec::with_capacity(components_contexts.len());

        for (url, temp_path, context) in components_contexts.drain(..) {
            // Await manifest download finish.
            context.wait()?;

            // Parse the manifest file.
            let manifest = std::fs::read(&temp_path)?;
            let manifest = serde_json::from_slice::<Json>(&manifest)?;
            let manifest = ComponentsVariantManifest::from_json(&manifest)?;

            // Delete it.
            std::fs::remove_file(temp_path)?;

            // Store the manifest and the component package's URL
            // to build a lock file.
            packages.insert(manifest.package.url.clone());

            components.push(GenerationComponentLock {
                url,
                manifest
            });
        }

        // Build the lock file for the game packages.
        let lock_file = LockFile::with_packages(packages)
            .build(packages_store)
            .await?;

        Ok(GenerationManifest::compose(games, components, lock_file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn build() -> Result<(), GenerationError> {
        let path = std::env::temp_dir().join(".agl-generations-test");

        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }

        std::fs::create_dir_all(&path)?;

        // Use the same folder for both packages and generations.
        let packages_store = PackagesStore::new(&path);
        let generations_store = GenerationsStore::new(&path);

        let generation = Generation::default()
            .with_games([
                "https://raw.githubusercontent.com/an-anime-team/anime-games-launcher/next/tests/games/1.json"
            ])
            .with_components([
                "https://raw.githubusercontent.com/an-anime-team/anime-games-launcher/next/tests/components/variant.json"
            ]);

        let generation = generation.build(&packages_store, &generations_store).await?;

        assert_eq!(generation.games.len(), 1);
        assert_eq!(generation.components.len(), 1);
        assert_eq!(generation.lock_file.root.iter().copied().sum::<u32>(), 1); // 0 + 1
        assert_eq!(generation.lock_file.resources.len(), 10);
        assert_eq!(Hash::for_entry(path)?, Hash(3241624005879532019));

        Ok(())
    }
}
