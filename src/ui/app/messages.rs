use relm4::factory::DynamicIndex;

use crate::util::toolbx::ToolbxContainer;

use super::model::ToolbxEntry;

pub enum AppMsg {
    ToolbxListUpdate(Vec<ToolbxContainer>),
    ToolbxContainerToggleStartStop(DynamicIndex),
    OpenToolbxTerminal(DynamicIndex),
    ToolbxContainerChanged(DynamicIndex, ToolbxEntry),
}
