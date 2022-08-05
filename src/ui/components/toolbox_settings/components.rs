use relm4::RelmComponent;
use relm4::RelmMsgHandler;
use relm4::Sender;

use super::model::ToolboxSettingsDialogModel;
use crate::ui::components::toolbox_settings::workers::AsyncSettingsHandler;

#[derive(relm4::Components)]
pub struct SettingsComponents {
    pub async_handler: RelmMsgHandler<AsyncSettingsHandler, ToolboxSettingsDialogModel>,
}
