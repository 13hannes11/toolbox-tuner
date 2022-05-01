use relm4::factory::DynamicIndex;

pub enum AppMsg {
    ShowToolboxSettingsRequest,
    ShowToolboxAppsRequest,
    ToolbxContainerToggleStartStop(DynamicIndex),
}
