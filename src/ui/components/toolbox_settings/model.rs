use relm4::Sender;
use relm4::{Model, RelmComponent};

use crate::ui::app::model::AppModel;

use super::{messages::ToolboxSettingsDialogMsg, widgets::ToolboxSettingsDialogWidgets};

#[derive(relm4::Components)]
pub struct AppComponents {
    pub toolbox_settings_dialog: RelmComponent<ToolboxSettingsDialogModel, AppModel>,
}
pub struct ToolboxSettingsDialogModel {
    pub hidden: bool,
}

impl Model for ToolboxSettingsDialogModel {
    type Msg = ToolboxSettingsDialogMsg;
    type Widgets = ToolboxSettingsDialogWidgets;
    type Components = ();
}
