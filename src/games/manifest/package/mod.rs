use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value as Json};

use crate::core::prelude::*;
use crate::packages::prelude::*;

pub mod target_platform;

use target_platform::TargetPlatform;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub url: String,
    pub output: String,
    pub runtime: PackageRuntime
}

impl AsJson for Package {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "url": self.url,
            "output": self.output,
            "runtime": self.runtime.to_json()?
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            url: json.get("url")
                .ok_or_else(|| AsJsonError::FieldNotFound("package.url"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("package.url"))?
                .to_string(),

            output: json.get("output")
                .ok_or_else(|| AsJsonError::FieldNotFound("package.output"))?
                .as_str()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("package.output"))?
                .to_string(),

            runtime: json.get("runtime")
                .ok_or_else(|| AsJsonError::FieldNotFound("package.runtime"))
                .and_then(PackageRuntime::from_json)?
        })
    }
}

impl AsHash for Package {
    #[inline]
    fn hash(&self) -> Hash {
        self.url.hash().chain(self.output.hash())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageRuntime {
    pub platform: TargetPlatform,
    pub supported_platforms: Option<Vec<TargetPlatform>>
}

impl AsJson for PackageRuntime {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "platform": self.platform.to_string(),

            "supported_platforms": self.supported_platforms.as_ref()
                .map(|platforms| {
                    platforms.iter()
                        .map(TargetPlatform::to_string)
                        .collect::<Vec<_>>()
                })
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            platform: json.get("platform")
                .ok_or_else(|| AsJsonError::FieldNotFound("package.runtime.platform"))?
                .as_str()
                .map(TargetPlatform::from_str)
                .ok_or_else(|| AsJsonError::InvalidFieldValue("package.runtime.platform"))?
                .map_err(|err| AsJsonError::Other(err.into()))?,

            supported_platforms: match json.get("supported_platforms") {
                Some(supported) if supported.is_null() => None,

                Some(supported) => {
                    supported.as_array()
                        .ok_or_else(|| AsJsonError::InvalidFieldValue("package.runtime.supported_platforms"))?
                        .iter()
                        .map(|platform| {
                            platform.as_str()
                                .ok_or_else(|| AsJsonError::InvalidFieldValue("package.runtime.supported_platforms[]"))
                                .and_then(|platform| {
                                    TargetPlatform::from_str(platform)
                                        .map_err(|err| AsJsonError::Other(err.into()))
                                })
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(Some)?
                }

                None => None
            }
        })
    }
}

impl AsHash for PackageRuntime {
    fn hash(&self) -> Hash {
        self.platform.hash().chain(self.supported_platforms.hash())
    }
}
