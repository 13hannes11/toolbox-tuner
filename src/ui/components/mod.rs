use relm4::Sender;
use relm4::RelmComponent;

use self::{toolbox_settings::model::ToolboxSettingsDialogModel, toolbox_apps::model::ToolboxAppDialogModel};

use super::app::model::AppModel;

pub mod toolbox_settings;
pub mod toolbox_apps;

#[derive(relm4::Components)]
pub struct AppComponents {
    pub toolbox_settings_dialog: RelmComponent<ToolboxSettingsDialogModel, AppModel>,
    pub toolbox_apps_dialog: RelmComponent<ToolboxAppDialogModel, AppModel>,
}