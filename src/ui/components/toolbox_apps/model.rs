use relm4::{Model};


use super::{messages::ToolboxAppDialogMsg, widgets::ToolboxAppDialogWidgets};

pub struct ToolboxAppDialogModel {
    pub hidden: bool,
}

impl Model for ToolboxAppDialogModel {
    type Msg = ToolboxAppDialogMsg;
    type Widgets = ToolboxAppDialogWidgets;
    type Components = ();
}
