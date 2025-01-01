use mlua::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ProgressReport<'lua> {
    /// Optional title of the current action.
    pub title: Option<LocalizableString>,

    /// Optional description of the current action.
    pub description: Option<LocalizableString>,

    pub progress_current: u64,
    pub progress_total: u64,

    progress_format: Option<LuaFunction<'lua>>
}

impl ProgressReport<'_> {
    #[inline]
    /// Return `current / total` fraction.
    pub fn fraction(&self) -> f64 {
        self.progress_current as f64 / self.progress_total as f64
    }

    /// Return formatted progress string if the formatter callback is specified.
    pub fn format(&self) -> Result<Option<LocalizableString>, LuaError> {
        let Some(format) = &self.progress_format else {
            return Ok(None);
        };

        let str = format.call::<_, LuaValue>(())?;

        Ok(Some(LocalizableString::from_lua(&str)?))
    }
}

impl<'lua> AsLua<'lua> for ProgressReport<'lua> {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError> {
        let progress = lua.create_table()?;

        if let Some(title) = &self.title {
            progress.set("title", title.to_lua(lua)?)?;
        }

        if let Some(description) = &self.description {
            progress.set("description", description.to_lua(lua)?)?;
        }

        let progress_details = lua.create_table()?;

        progress_details.set("current", self.progress_current)?;
        progress_details.set("total", self.progress_total)?;

        if let Some(format) = &self.progress_format {
            progress_details.set("format", format)?;
        }

        Ok(LuaValue::Table(progress))
    }

    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized {
        let value = value.as_table()
            .ok_or_else(|| AsLuaError::InvalidFieldValue("<progress report>"))?;

        let progress = value.get::<_, LuaTable>("progress")?;

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
                .map(|title| -> Result<Option<LocalizableString>, AsLuaError> {
                    if title.is_nil() || title.is_null() {
                        Ok(None)
                    } else {
                        Ok(Some(LocalizableString::from_lua(&title)?))
                    }
                })
                .unwrap_or(Ok(None))?,

            progress_current: progress.get("current")?,
            progress_total: progress.get("total")?,

            progress_format: progress.get::<_, LuaFunction>("format").ok()
        })
    }
}
