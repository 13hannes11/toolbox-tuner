use relm4::{ComponentUpdate, Sender};

use crate::ui::app::{messages::AppMsg, model::AppModel};

use super::{messages::ToolboxSettingsDialogMsg, model::ToolboxSettingsDialogModel};

impl ComponentUpdate<AppModel> for ToolboxSettingsDialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ToolboxSettingsDialogModel { hidden: true }
    }

    fn update(
        &mut self,
        msg: ToolboxSettingsDialogMsg,
        _components: &(),
        _sender: Sender<ToolboxSettingsDialogMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ToolboxSettingsDialogMsg::Show => self.hidden = false,
            ToolboxSettingsDialogMsg::Close => self.hidden = true,
        }
    }
}
