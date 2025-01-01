use std::str::FromStr;

use mlua::prelude::*;

use crate::prelude::*;

pub mod components_group;
pub mod component;

pub use components_group::*;
pub use component::*;

#[derive(Debug, Clone)]
pub struct ComponentIntegration<'lua> {
    lua: &'lua Lua,

    groups: LuaFunction<'lua>,
    components: LuaFunction<'lua>,
    compontn_get_status: LuaFunction<'lua>,
    component_get_diff: LuaFunction<'lua>,
    component_apply: LuaFunction<'lua>,

    settings: Option<DynamicSettingsManifest>
}

impl<'lua> ComponentIntegration<'lua> {
    pub fn from_lua(lua: &'lua Lua, table: &LuaTable<'lua>) -> Result<Self, LuaError> {
        if table.get::<_, u32>("standard")? != 1 {
            return Err(LuaError::external("invalid game integration standard, v1 expected"));
        }

        let component = table.get::<_, LuaTable>("component")?;

        Ok(Self {
            lua,

            groups: table.get("groups")?,
            components: table.get("components")?,
            compontn_get_status: component.get("get_status")?,
            component_get_diff: component.get("get_diff")?,
            component_apply: component.get("apply")?,

            settings: table.get::<_, LuaValue>("settings").ok()
                .map(|settings| DynamicSettingsManifest::from_lua(&settings))
                .transpose()?
        })
    }

    /// Get list of available component groups.
    pub fn groups(&self, source_platform: TargetPlatform, target_platform: TargetPlatform) -> Result<Vec<ProfileComponentsGroup>, AsLuaError> {
        self.groups.call::<_, Vec<LuaValue>>((source_platform.to_string(), target_platform.to_string()))
            .map_err(AsLuaError::LuaError)
            .and_then(|groups| {
                groups.iter()
                    .map(ProfileComponentsGroup::from_lua)
                    .collect::<Result<Vec<_>, _>>()
            })
    }

    /// Get list of components within the group.
    pub fn components(&self, group_name: impl AsRef<str>) -> Result<Vec<ProfileComponent>, AsLuaError> {
        self.components.call::<_, Vec<LuaValue>>(group_name.as_ref())
            .map_err(AsLuaError::LuaError)
            .and_then(|components| {
                components.iter()
                    .map(ProfileComponent::from_lua)
                    .collect::<Result<Vec<_>, _>>()
            })
    }

    /// Get status of the component installation.
    pub fn component_status(&self, component_name: impl AsRef<str>) -> Result<InstallationStatus, LuaError> {
        self.compontn_get_status.call::<_, LuaString>(component_name.as_ref())
            .and_then(|status| InstallationStatus::from_str(&status.to_string_lossy()))
    }

    /// Get component installation diff.
    pub fn component_diff(&self, component_name: impl AsRef<str>) -> Result<Option<InstallationDiff>, LuaError> {
        self.component_get_diff.call::<_, Option<LuaTable>>(component_name.as_ref())
            .and_then(|diff| {
                diff.map(|diff| InstallationDiff::from_lua(self.lua, &diff))
                    .transpose()
            })
    }

    /// Apply component to the game launch environment.
    pub fn component_apply(&self, component_name: impl AsRef<str>, launch_info: GameLaunchInfo) -> Result<Option<GameLaunchInfo>, AsLuaError> {
        self.component_apply.call::<_, LuaValue>((component_name.as_ref(), launch_info.to_lua(self.lua)?))
            .map_err(AsLuaError::LuaError)
            .and_then(|info| {
                if info.is_nil() || info.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(GameLaunchInfo::from_lua(&info)?))
                }
            })
    }

    #[inline]
    /// Get dynamic settings of the current components variant.
    pub fn settings(&self) -> Option<&DynamicSettingsManifest> {
        self.settings.as_ref()
    }
}
