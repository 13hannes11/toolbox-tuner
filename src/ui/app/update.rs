use relm4::{AppUpdate, Sender};

use crate::ui::components::toolbox_settings::{
    messages::ToolboxSettingsDialogMsg, model::AppComponents,
};

use super::{messages::AppMsg, model::AppModel};

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::ShowToolboxSettingsRequest => {
                components
                    .toolbox_settings_dialog
                    .send(ToolboxSettingsDialogMsg::Show)
                    .unwrap();
            }
        }
        true
    }
}
