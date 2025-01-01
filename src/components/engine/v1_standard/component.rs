use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub title: LocalizableString,
    pub description: Option<LocalizableString>
}

impl<'lua> AsLua<'lua> for Component {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let table = lua.create_table_with_capacity(0, 3)?;

        table.set("name", lua.create_string(&self.name)?)?;
        table.set("title", self.title.to_lua(lua)?)?;

        if let Some(description) = &self.description {
            table.set("description", description.to_lua(lua)?)?;
        }

        Ok(LuaValue::Table(table))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("<component group>"))?;

        Ok(Self {
            name: value.get("name")
                .map_err(|_| AsLuaError::InvalidFieldValue("name"))?,

            title: value.get::<_, LuaValue>("title")
                .map_err(|_| AsLuaError::InvalidFieldValue("title"))
                .and_then(|title| LocalizableString::from_lua(&title))?,

            description: value.get::<_, LuaValue>("description")
                .map(|desc| -> Result<Option<LocalizableString>, AsLuaError> {
                    if desc.is_nil() || desc.is_null() {
                        Ok(None)
                    } else {
                        Ok(Some(LocalizableString::from_lua(&desc)?))
                    }
                })
                .unwrap_or(Ok(None))?
        })
    }
}
