use std::collections::HashSet;

use serde_json::Value as Json;

use crate::prelude::*;

/// Get list of URLs to the component variants' manifest
/// from the components registries specified in the config file.
///
/// Return error only in critical situations.
/// Broken links and json files will be logged and skipped.
pub async fn fetch_components() -> anyhow::Result<HashSet<String>> {
    let client = STARTUP_CONFIG.general.network.builder()?.build()?;

    let mut registries_tasks = Vec::with_capacity(STARTUP_CONFIG.components.registries.len());

    // Start fetching the registries.
    tracing::debug!("Fetching components registries");

    for url in STARTUP_CONFIG.components.registries.clone() {
        let request = client.get(&url);

        let task = tokio::spawn(async move {
            let response = request.send().await?
                .bytes().await?;

            let manifest = serde_json::from_slice::<Json>(&response)?;
            let manifest = ComponentsRegistryManifest::from_json(&manifest)?;

            Ok::<_, anyhow::Error>(manifest)
        });

        registries_tasks.push((url, task));
    }

    // Await registries fetching.
    let mut components = HashSet::new();

    for (url, task) in registries_tasks.drain(..) {
        tracing::trace!(?url, "Awaiting components registry");

        match task.await {
            Ok(Ok(manifest)) => {
                tracing::trace!(
                    ?url,
                    title = manifest.title.default_translation(),
                    "Added components registry"
                );

                let iter = manifest.translation_components.into_iter()
                    .chain(manifest.virtualisation_components.into_iter())
                    .chain(manifest.runtime_components.into_iter())
                    .chain(manifest.general_components.into_iter());

                for url in iter {
                    components.insert(url);
                }
            }

            Err(err) => tracing::error!(?url, ?err, "Failed to await fetching components registry"),
            Ok(Err(err)) => tracing::error!(?url, ?err, "Failed to fetch components registry")
        }
    }

    Ok::<_, anyhow::Error>(components)
}
