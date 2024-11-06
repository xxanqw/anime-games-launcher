use std::str::FromStr;

use serde_json::{json, Value as Json};

use crate::prelude::*;

// TODO: wine settings

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WineSync {
    /// `WINEESYNC=1`
    Esync,

    /// `WINEFSYNC=1`
    Fsync
}

impl std::fmt::Display for WineSync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: replace all write!(f, ..) to f.write_str(..)
            Self::Esync => f.write_str("esync"),
            Self::Fsync => f.write_str("fsync")
        }
    }
}

impl FromStr for WineSync {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "esync" => Ok(Self::Esync),
            "fsync" => Ok(Self::Fsync),

            _ => anyhow::bail!("Unsupported wine sync value: {s}")
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
/// Runtime settings for linux wine.
pub struct Settings {
    /// Wine events synchronization method.
    /// 
    /// TODO: add source and details.
    pub winesync: Option<WineSync>
}

impl AsJson for Settings {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "winesync": self.winesync.as_ref()
                .map(WineSync::to_string)
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            winesync: match json.get("winesync") {
                Some(value) if value.is_null() => None,

                Some(value) => value.as_str()
                    .ok_or_else(|| AsJsonError::InvalidFieldValue("runtime.winesync"))
                    .map(WineSync::from_str)?
                    .map(Some)
                    .map_err(|err| AsJsonError::Other(err.into()))?,

                None => None
            }
        })
    }
}
