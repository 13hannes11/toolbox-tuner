use relm4::factory::DynamicIndex;

use super::model::ToolbxEntry;

pub enum AppMsg {
    ShowToolboxSettingsRequest,
    ShowToolboxAppsRequest,
    ToolbxContainerToggleStartStop(DynamicIndex),
    OpenToolbxTerminal(DynamicIndex),
    ToolbxContainerChanged(DynamicIndex, ToolbxEntry),
}
