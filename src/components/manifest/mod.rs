use serde_json::{json, Value as Json};

use crate::prelude::*;

pub mod variant;
pub mod package;
pub mod category;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsVariantManifest {
    pub standard: u64,
    pub variant: variant::Variant,
    pub package: package::Package
}

impl AsJson for ComponentsVariantManifest {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "standard": self.standard,
            "variant": self.variant.to_json()?,
            "package": self.package.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            standard: json.get("standard")
                .ok_or_else(|| AsJsonError::FieldNotFound("standard"))?
                .as_u64()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("standard"))?,

            variant: json.get("variant")
                .ok_or_else(|| AsJsonError::FieldNotFound("variant"))
                .and_then(variant::Variant::from_json)?,

            package: json.get("package")
                .ok_or_else(|| AsJsonError::FieldNotFound("package"))
                .and_then(package::Package::from_json)?
        })
    }
}

impl AsHash for ComponentsVariantManifest {
    fn hash(&self) -> Hash {
        self.standard.hash()
            .chain(self.variant.hash())
            .chain(self.package.hash())
    }
}
