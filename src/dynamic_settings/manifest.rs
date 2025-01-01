use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicSettingsManifest {
    pub standard: u64,
    pub settings: Vec<SettingsGroup>
}

impl<'lua> AsLua<'lua> for DynamicSettingsManifest {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let table = lua.create_table_with_capacity(0, 2)?;
        let settings = lua.create_table_with_capacity(self.settings.len(), 0)?;

        table.set("standard", self.standard)?;

        for group in &self.settings {
            settings.push(group.to_lua(lua)?)?;
        }

        table.set("settings", settings)?;

        Ok(LuaValue::Table(table))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("<dynamic settings>"))?;

        Ok(Self {
            standard: value.get("standard")
                .map_err(|_| AsLuaError::InvalidFieldValue("standard"))?,

            settings: value.get::<_, Vec<LuaValue>>("settings")
                .map_err(|_| AsLuaError::InvalidFieldValue("settings"))?
                .iter()
                .map(SettingsGroup::from_lua)
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}
