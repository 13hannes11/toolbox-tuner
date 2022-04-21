use relm4::{factory::FactoryVec, Model};

use crate::{ui::components::AppComponents, toolbx::ToolbxContainer};

use super::{messages::AppMsg, widgets::AppWidgets};

pub struct AppModel {
    pub toolboxes: FactoryVec<ToolbxContainer>,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}
