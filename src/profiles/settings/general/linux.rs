use serde_json::{json, Value as Json};

use crate::prelude::*;

// TODO: bubblewrap sandbox settings

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// General settings for linux.
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
