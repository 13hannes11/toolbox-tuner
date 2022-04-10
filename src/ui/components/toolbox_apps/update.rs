use relm4::{ComponentUpdate, Sender};

use crate::ui::app::{messages::AppMsg, model::AppModel};

use super::{messages::ToolboxAppDialogMsg, model::ToolboxAppDialogModel};

impl ComponentUpdate<AppModel> for ToolboxAppDialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ToolboxAppDialogModel { hidden: true }
    }

    fn update(
        &mut self,
        msg: ToolboxAppDialogMsg,
        _components: &(),
        _sender: Sender<ToolboxAppDialogMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ToolboxAppDialogMsg::Show => self.hidden = false,
            ToolboxAppDialogMsg::Close => self.hidden = true,
        }
    }
}
