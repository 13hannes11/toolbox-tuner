use relm4::factory::DynamicIndex;

use super::model::ToolbxEntry;

pub enum AppMsg {
    ShowToolboxSettingsRequest,
    ShowToolboxAppsRequest,
    ToolbxContainerToggleStartStop(DynamicIndex),
    ToolbxContainerChanged(DynamicIndex, ToolbxEntry),
}
