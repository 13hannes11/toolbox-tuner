use relm4::{Model, factory::{FactoryVec}};


use super::{messages::ToolboxAppDialogMsg, widgets::{ToolboxAppDialogWidgets}};

#[derive(Default)]
pub struct DotDesktopApplication {
    pub name: String,
    pub selected: bool,
    pub icon_path: String,
}
pub struct ToolboxAppDialogModel {
    pub hidden: bool,
    pub apps: FactoryVec<DotDesktopApplication>,
}

impl Model for ToolboxAppDialogModel {
    type Msg = ToolboxAppDialogMsg;
    type Widgets = ToolboxAppDialogWidgets;
    type Components = ();
}
