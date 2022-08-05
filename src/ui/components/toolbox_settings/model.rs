use relm4::Model;

use super::{
    components::SettingsComponents, messages::ToolboxSettingsDialogMsg,
    widgets::ToolboxSettingsDialogWidgets,
};
use crate::toolbx::ToolbxContainer;

pub struct ToolboxSettingsDialogModel {
    pub hidden: bool,
    pub window_title: Option<String>,
    pub current_toolbox_container: Option<ToolbxContainer>,
    pub current_toolbx_settings: Option<ToolboxPreferences>,
}

impl Model for ToolboxSettingsDialogModel {
    type Msg = ToolboxSettingsDialogMsg;
    type Widgets = ToolboxSettingsDialogWidgets;
    type Components = SettingsComponents;
}

#[derive(Debug, Clone)]
pub struct ToolboxPreferences {}
