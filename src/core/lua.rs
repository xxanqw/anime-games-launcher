use mlua::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AsLuaError {
    #[error("Field not found: {0}")]
    FieldNotFound(&'static str),

    #[error("Invalid field value: {0}")]
    InvalidFieldValue(&'static str),

    #[error("Unsupported format version: {0}")]
    UnsupportedFormat(u64),

    #[error(transparent)]
    LuaError(#[from] LuaError),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>)
}

impl From<AsLuaError> for LuaError {
    #[inline]
    fn from(value: AsLuaError) -> Self {
        LuaError::external(value)
    }
}

pub trait AsLua<'lua> {
    fn to_lua(&self, lua: &'lua Lua) -> Result<LuaValue<'lua>, AsLuaError>;
    fn from_lua(value: &'lua LuaValue<'lua>) -> Result<Self, AsLuaError> where Self: Sized;
}
