use std::str::FromStr;

use serde_json::{json, Value as Json};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub title: LocalizableString,
    pub description: LocalizableString,
    pub category: ComponentCategory
}

impl AsJson for Variant {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "name": self.name,
            "title": self.title.to_json()?,
            "description": self.description.to_json()?,
            "category": self.category.to_string()
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            name: json.get("name")
                .ok_or_else(|| AsJsonError::FieldNotFound("variant.name"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("variant.name"))?
                .to_string(),

            title: json.get("title")
                .ok_or_else(|| AsJsonError::FieldNotFound("variant.title"))
                .and_then(LocalizableString::from_json)?,

            description: json.get("description")
                .ok_or_else(|| AsJsonError::FieldNotFound("variant.description"))
                .and_then(LocalizableString::from_json)?,

            category: json.get("category")
                .ok_or_else(|| AsJsonError::FieldNotFound("variant.category"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("variant.category"))
                .map(ComponentCategory::from_str)?
                .map_err(|err| AsJsonError::Other(err.into()))?
        })
    }
}

impl AsHash for Variant {
    fn hash(&self) -> Hash {
        self.name.hash()
            .chain(self.title.hash())
            .chain(self.description.hash())
            .chain(self.category.hash())
    }
}
