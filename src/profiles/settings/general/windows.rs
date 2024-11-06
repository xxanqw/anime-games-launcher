use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// General settings for windows.
pub struct Settings {
    pub common: CommonGeneralProfileSettings
}

impl AsJson for Settings {
    #[inline]
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "common": self.common.to_json()?
        }))
    }

    #[inline]
    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            common: json.get("common")
                .ok_or_else(|| AsJsonError::FieldNotFound("general.common"))
                .and_then(CommonGeneralProfileSettings::from_json)?
        })
    }
}
