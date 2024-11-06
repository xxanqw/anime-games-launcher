use std::collections::HashMap;

use serde_json::{json, Value as Json};

use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// Settings common to all the systems.
pub struct Settings {
    /// Environment variables passed to the executable.
    pub environment: Option<HashMap<String, String>>,

    /// Additional arguments to be passed to the executable.
    pub extra_arguments: Option<Vec<String>>,

    /// Launch the game with a terminal window.
    pub show_terminal: bool
}

impl AsJson for Settings {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "environment": self.environment,
            "extra_arguments": self.extra_arguments,
            "show_terminal": self.show_terminal
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            environment: json.get("environment")
                .and_then(Json::as_object)
                .map(|environment| {
                    environment.iter()
                        .map(|(key, value)| {
                            value.as_str().map(|value| (key.to_string(), value.to_string()))
                        })
                        .collect::<Option<HashMap<String, String>>>()
                        .ok_or_else(|| AsJsonError::InvalidFieldValue("general.common.environment[]"))
                })
                .transpose()?,

            extra_arguments: json.get("extra_arguments")
                .and_then(Json::as_array)
                .map(|extra_args| {
                    extra_args.iter()
                        .map(|arg| arg.as_str().map(String::from))
                        .collect::<Option<Vec<String>>>()
                        .ok_or_else(|| AsJsonError::InvalidFieldValue("general.common.extra_arguments[]"))
                })
                .transpose()?,

            show_terminal: json.get("show_terminal")
                .ok_or_else(|| AsJsonError::FieldNotFound("general.common.show_terminal"))?
                .as_bool()
                .ok_or_else(|| AsJsonError::InvalidFieldValue("general.common.show_terminal"))?
        })
    }
}
