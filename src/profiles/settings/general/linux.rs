use serde_json::{json, Value as Json};

use crate::prelude::*;

// TODO: bubblewrap sandbox settings

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// General settings for linux.
pub struct Settings {
    pub gamemode: bool
}

impl AsJson for Settings {
    #[inline]
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "gamemode": self.gamemode
        }))
    }

    #[inline]
    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            gamemode: json.get("gamemode")
                .ok_or_else(|| AsJsonError::FieldNotFound("general.linux.gamemode"))?
                .as_bool()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("general.linux.gamemode"))?
        })
    }
}
