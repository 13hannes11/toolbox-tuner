use relm4::RelmComponent;
use relm4::Sender;

use self::{
    toolbox_apps::model::ToolboxAppDialogModel, toolbox_settings::model::ToolboxSettingsDialogModel,
};

use super::app::model::AppModel;

pub mod toolbox_apps;
pub mod toolbox_settings;

#[derive(relm4::Components)]
pub struct AppComponents {
    pub toolbox_settings_dialog: RelmComponent<ToolboxSettingsDialogModel, AppModel>,
    pub toolbox_apps_dialog: RelmComponent<ToolboxAppDialogModel, AppModel>,
}
