use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::{json, Value as Json};

use super::*;

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
    fn hash(&self) -> Hash {
        self.url.hash()
            .chain(self.output.hash())
            .chain(self.runtime.hash())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageRuntime {
    pub source_platforms: Vec<TargetPlatform>,
    pub target_platforms: Vec<TargetPlatform>
}

impl AsJson for PackageRuntime {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "source_platforms": self.source_platforms.iter()
                .map(TargetPlatform::to_string)
                .collect::<Vec<String>>(),

            "target_platforms": self.target_platforms.iter()
                .map(TargetPlatform::to_string)
                .collect::<Vec<String>>()
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            source_platforms: json.get("source_platforms")
                .and_then(Json::as_array)
                .ok_or_else(|| AsJsonError::FieldNotFound("package.runtime.source_platforms"))?
                .iter()
                .map(|platform| {
                    platform.as_str()
                        .ok_or_else(|| AsJsonError::InvalidFieldValue("package.runtime.source_platforms[]"))
                        .and_then(|platform| {
                            TargetPlatform::from_str(platform)
                                .map_err(|err| AsJsonError::Other(err.into()))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?,

            target_platforms: json.get("target_platforms")
                .and_then(Json::as_array)
                .ok_or_else(|| AsJsonError::FieldNotFound("package.runtime.target_platforms"))?
                .iter()
                .map(|platform| {
                    platform.as_str()
                        .ok_or_else(|| AsJsonError::InvalidFieldValue("package.runtime.target_platforms[]"))
                        .and_then(|platform| {
                            TargetPlatform::from_str(platform)
                                .map_err(|err| AsJsonError::Other(err.into()))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}

impl AsHash for PackageRuntime {
    fn hash(&self) -> Hash {
        self.source_platforms.hash()
            .chain(self.target_platforms.hash())
    }
}
