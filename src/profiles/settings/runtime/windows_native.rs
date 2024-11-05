use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Runtime settings for native windows.
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
