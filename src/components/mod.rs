pub mod manifest;
pub mod registry;

pub mod prelude {
    pub use super::manifest::ComponentsVariantManifest;
    pub use super::manifest::category::ComponentCategory;

    pub use super::registry::Manifest as ComponentsRegistryManifest;
}
