use relm4::{factory::FactoryVec, Model};

use crate::ui::components::AppComponents;

use super::{messages::AppMsg, widgets::AppWidgets};

pub struct AppModel {
    pub toolboxes: FactoryVec<ToolboxContainer>,
}

#[derive(Default)]
pub struct ToolboxContainer {
    pub name: String,
    pub status: ToolboxStatus,
    pub update_available: bool,
}
impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

pub enum ToolboxStatus {
    Stopped,
    Running,
}
impl Default for ToolboxStatus {
    fn default() -> Self {
        ToolboxStatus::Stopped
    }
}
