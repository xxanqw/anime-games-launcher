use std::collections::HashMap;

use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsEntryFormat {
    Switch {
        default: bool
    },

    Text {
        default: String
    },

    Enum {
        values: HashMap<String, LocalizableString>,
        default: String
    }
}

impl<'lua> AsLua<'lua> for SettingsEntryFormat {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let table = lua.create_table_with_capacity(0, 3)?;

        match self {
            Self::Switch { default } => {
                table.set("format", "switch")?;
                table.set("default", *default)?;
            }

            Self::Text { default } => {
                table.set("format", "text")?;
                table.set("default", lua.create_string(default)?)?;
            }

            Self::Enum { values, default } => {
                let enum_values = lua.create_table_with_capacity(0, values.len())?;

                table.set("format", "enum")?;
                table.set("values", enum_values.clone())?;
                table.set("default", lua.create_string(default)?)?;

                for (key, value) in values {
                    enum_values.set(key.as_str(), value.to_lua(lua)?)?;
                }
            }
        }

        Ok(LuaValue::Table(table))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("settings.entries[].entry"))?;

        let format = value.get::<_, LuaString>("format")
            .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].entry.format"))?;

        match format.as_bytes() {
            b"switch" => Ok(Self::Switch {
                default: value.get("default")
                    .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].entry.default"))?
            }),

            b"text" => Ok(Self::Text {
                default: value.get("default")
                    .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].entry.default"))?
            }),

            b"enum" => Ok(Self::Enum {
                values: value.get::<_, LuaTable>("values")
                    .and_then(|values| {
                        let mut table = HashMap::new();

                        for pair in values.pairs::<LuaString, LuaValue>() {
                            let (key, value) = pair?;

                            table.insert(key.to_string_lossy().to_string(), LocalizableString::from_lua(&value)?);
                        }

                        Ok(table)
                    })
                    .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].entry.values"))?,

                default: value.get("default")
                    .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].entry.default"))?
            }),

            _ => Err(AsLuaError::InvalidFieldValue("settings.entries[].entry.format"))
        }
    }
}
