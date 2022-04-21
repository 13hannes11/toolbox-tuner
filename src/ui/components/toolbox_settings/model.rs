use relm4::Model;

use super::{messages::ToolboxSettingsDialogMsg, widgets::ToolboxSettingsDialogWidgets};

pub struct ToolboxSettingsDialogModel {
    pub hidden: bool,
}

impl Model for ToolboxSettingsDialogModel {
    type Msg = ToolboxSettingsDialogMsg;
    type Widgets = ToolboxSettingsDialogWidgets;
    type Components = ();
}
