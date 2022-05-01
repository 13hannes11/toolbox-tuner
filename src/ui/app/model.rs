use relm4::{
    factory::{FactoryVecDeque},
    Model,
};

use crate::{toolbx::ToolbxContainer, ui::components::AppComponents};

use super::{messages::AppMsg, widgets::AppWidgets};

pub struct AppModel {
    pub toolboxes: FactoryVecDeque<ToolbxContainer>,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}
