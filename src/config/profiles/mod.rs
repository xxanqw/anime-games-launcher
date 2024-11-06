use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Profiles {
    pub store: ProfilesStore
}

impl AsJson for Profiles {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "store": self.store.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            store: json.get("store")
                .map(ProfilesStore::from_json)
                .ok_or_else(|| AsJsonError::FieldNotFound("profiles.store"))??
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfilesStore {
    pub path: PathBuf
}

impl Default for ProfilesStore {
    fn default() -> Self {
        Self {
            path: DATA_FOLDER.join("profiles")
        }
    }
}

impl AsJson for ProfilesStore {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "path": self.path
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            path: json.get("path")
                .ok_or_else(|| AsJsonError::FieldNotFound("profiles.store.path"))?
                .as_str()
                .map(PathBuf::from)
                .ok_or_else(|| AsJsonError::InvalidFieldValue("profiles.store.path"))?
        })
    }
}
