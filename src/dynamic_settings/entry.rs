use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsEntry {
    pub name: String,
    pub title: LocalizableString,
    pub description: Option<LocalizableString>,
    pub entry: SettingsEntryFormat
}

impl<'lua> AsLua<'lua> for SettingsEntry {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let table = lua.create_table_with_capacity(0, 4)?;

        table.set("name", lua.create_string(&self.name)?)?;
        table.set("title", self.title.to_lua(lua)?)?;
        table.set("entry", self.entry.to_lua(lua)?)?;

        if let Some(desc) = &self.description {
            table.set("description", desc.to_lua(lua)?)?;
        }

        Ok(LuaValue::Table(table))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("settings.entries[]"))?;

        Ok(Self {
            name: value.get("name")
                .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].name"))?,

            title: value.get::<_, LuaValue>("title")
                .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].title"))
                .and_then(|title| LocalizableString::from_lua(&title))?,

            description: value.get::<_, LuaValue>("description")
                .map(|desc| -> Result<Option<LocalizableString>, AsLuaError> {
                    if desc.is_nil() || desc.is_null() {
                        Ok(None)
                    } else {
                        Ok(Some(LocalizableString::from_lua(&desc)?))
                    }
                })
                .unwrap_or(Ok(None))?,

            entry: value.get::<_, LuaValue>("entry")
                .map_err(|_| AsLuaError::InvalidFieldValue("settings.entries[].entry"))
                .and_then(|title| SettingsEntryFormat::from_lua(&title))?
        })
    }
}
