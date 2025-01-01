use std::collections::HashMap;

use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DynamicSettingsValues<'lua> {
    pub values: HashMap<String, LuaValue<'lua>>
}

impl<'lua> AsLua<'lua> for DynamicSettingsValues<'lua> {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let values = lua.create_table_with_capacity(0, self.values.len())?;

        for (key, value) in &self.values {
            values.set(key.as_str(), value)?;
        }

        Ok(LuaValue::Table(values))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("<dynamic settings values>"))?;

        let standard = value.get::<_, u64>("standard")?;

        if standard != 1 {
            return Err(AsLuaError::UnsupportedFormat(standard));
        }

        Ok(Self {
            values: value.get("values")?
        })
    }
}
