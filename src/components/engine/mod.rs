use mlua::prelude::*;

use crate::prelude::*;

pub mod v1_standard;

pub use v1_standard::{
    ComponentGroup,
    Component
};

#[derive(Debug, Clone)]
/// Unified wrapper around game integration standards.
pub enum GameEngine<'lua> {
    V1(v1_standard::ComponentIntegration<'lua>)
}

impl<'lua> GameEngine<'lua> {
    pub fn from_lua(lua: &'lua Lua, table: &LuaTable<'lua>) -> Result<Self, LuaError> {
        match table.get::<_, u32>("standard")? {
            1 => Ok(Self::V1(v1_standard::ComponentIntegration::from_lua(lua, table)?)),

            _ => Err(LuaError::external("unsupported component integration standard"))
        }
    }

    #[inline]
    /// Get list of available component groups.
    pub fn groups(&self, source_platform: TargetPlatform, target_platform: TargetPlatform) -> Result<Vec<ComponentGroup>, AsLuaError> {
        match self {
            Self::V1(engine) => engine.groups(source_platform, target_platform)
        }
    }

    #[inline]
    /// Get list of components within the group.
    pub fn components(&self, group_name: impl AsRef<str>) -> Result<Vec<Component>, AsLuaError> {
        match self {
            Self::V1(engine) => engine.components(group_name)
        }
    }

    #[inline]
    /// Get status of the component installation.
    pub fn component_status(&self, component_name: impl AsRef<str>) -> Result<InstallationStatus, LuaError> {
        match self {
            Self::V1(engine) => engine.component_status(component_name)
        }
    }

    #[inline]
    /// Get component installation diff.
    pub fn component_diff(&self, component_name: impl AsRef<str>) -> Result<Option<InstallationDiff>, LuaError> {
        match self {
            Self::V1(engine) => engine.component_diff(component_name)
        }
    }

    #[inline]
    /// Apply component to the game launch environment.
    pub fn component_apply(&self, component_name: impl AsRef<str>, launch_info: GameLaunchInfo) -> Result<Option<GameLaunchInfo>, AsLuaError> {
        match self {
            Self::V1(engine) => engine.component_apply(component_name, launch_info)
        }
    }

    #[inline]
    /// Get dynamic settings of the current components variant.
    pub fn settings(&self) -> Option<&DynamicSettingsManifest> {
        match self {
            Self::V1(engine) => engine.settings()
        }
    }
}
