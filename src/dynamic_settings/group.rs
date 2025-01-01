use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsGroup {
    pub title: Option<LocalizableString>,
    pub description: Option<LocalizableString>,
    pub entries: Vec<SettingsEntry>
}

impl<'lua> AsLua<'lua> for SettingsGroup {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let table = lua.create_table_with_capacity(0, 3)?;

        if let Some(title) = &self.title {
            table.set("title", title.to_lua(lua)?)?;
        }

        if let Some(desc) = &self.description {
            table.set("description", desc.to_lua(lua)?)?;
        }

        let entries = lua.create_table_with_capacity(self.entries.len(), 0)?;

        for entry in &self.entries {
            entries.push(entry.to_lua(lua)?)?;
        }

        table.set("entries", entries)?;

        Ok(LuaValue::Table(table))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("settings[]"))?;

        Ok(Self {
            title: value.get::<_, LuaValue>("title")
                .map(|title| -> Result<Option<LocalizableString>, AsLuaError> {
                    if title.is_nil() || title.is_null() {
                        Ok(None)
                    } else {
                        Ok(Some(LocalizableString::from_lua(&title)?))
                    }
                })
                .unwrap_or(Ok(None))?,

            description: value.get::<_, LuaValue>("description")
                .map(|desc| -> Result<Option<LocalizableString>, AsLuaError> {
                    if desc.is_nil() || desc.is_null() {
                        Ok(None)
                    } else {
                        Ok(Some(LocalizableString::from_lua(&desc)?))
                    }
                })
                .unwrap_or(Ok(None))?,

            entries: value.get::<_, Vec<LuaValue>>("entries")
                .map_err(|_| AsLuaError::InvalidFieldValue("settings[].entries"))?
                .iter()
                .map(SettingsEntry::from_lua)
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}
