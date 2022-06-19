use relm4::factory::DynamicIndex;

use crate::toolbx::ToolbxContainer;

use super::model::ToolbxEntry;

pub enum AppMsg {
    ShowToolboxSettingsRequest,
    ShowToolboxAppsRequest,
    ToolbxListUpdate(Vec<ToolbxContainer>),
    ToolbxContainerToggleStartStop(DynamicIndex),
    OpenToolbxTerminal(DynamicIndex),
    ToolbxContainerChanged(DynamicIndex, ToolbxEntry),
}
