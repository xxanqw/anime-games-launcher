use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// General settings for windows.
pub struct Settings;

impl AsJson for Settings {
    #[inline]
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({}))
    }

    #[inline]
    fn from_json(_json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self)
    }
}
