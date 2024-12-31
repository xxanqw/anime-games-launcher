use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub format: u64,
    pub title: LocalizableString,
    pub translation_components: Vec<String>,
    pub virtualisation_components: Vec<String>,
    pub runtime_components: Vec<String>,
    pub general_components: Vec<String>
}

impl AsJson for Manifest {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "format": self.format,
            "title": self.title.to_json()?,
            "components": {
                "translation": self.translation_components,
                "virtualisation": self.virtualisation_components,
                "runtime": self.runtime_components,
                "general": self.general_components
            }
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        let components = json.get("components")
            .ok_or_else(|| AsJsonError::FieldNotFound("components"))?;

        Ok(Self {
            format: json.get("format")
                .ok_or_else(|| AsJsonError::FieldNotFound("format"))?
                .as_u64()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("format"))?,

            title: json.get("title")
                .ok_or_else(|| AsJsonError::FieldNotFound("title"))
                .and_then(LocalizableString::from_json)?,

            translation_components: components.get("translation")
                .ok_or_else(|| AsJsonError::FieldNotFound("components.translation"))?
                .as_array()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.translation"))?
                .iter()
                .map(|url| url.as_str().map(String::from))
                .collect::<Option<Vec<String>>>()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.translation[]"))?,

            virtualisation_components: components.get("virtualisation")
                .ok_or_else(|| AsJsonError::FieldNotFound("components.virtualisation"))?
                .as_array()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.virtualisation"))?
                .iter()
                .map(|url| url.as_str().map(String::from))
                .collect::<Option<Vec<String>>>()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.virtualisation[]"))?,

            runtime_components: components.get("runtime")
                .ok_or_else(|| AsJsonError::FieldNotFound("components.runtime"))?
                .as_array()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.runtime"))?
                .iter()
                .map(|url| url.as_str().map(String::from))
                .collect::<Option<Vec<String>>>()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.runtime[]"))?,

            general_components: components.get("general")
                .ok_or_else(|| AsJsonError::FieldNotFound("components.general"))?
                .as_array()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.general"))?
                .iter()
                .map(|url| url.as_str().map(String::from))
                .collect::<Option<Vec<String>>>()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("components.general[]"))?
        })
    }
}

impl AsHash for Manifest {
    fn hash(&self) -> Hash {
        self.format.hash()
            .chain(self.title.hash())
            .chain(self.translation_components.hash())
            .chain(self.virtualisation_components.hash())
            .chain(self.runtime_components.hash())
            .chain(self.general_components.hash())
    }
}
