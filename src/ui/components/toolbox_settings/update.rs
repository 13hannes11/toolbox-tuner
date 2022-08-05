use relm4::{ComponentUpdate, Sender};

use crate::ui::app::{messages::AppMsg, model::AppModel};

use super::{
    components::SettingsComponents, messages::ToolboxSettingsDialogMsg,
    model::ToolboxSettingsDialogModel, workers::AsyncSettingsHandlerMsg,
};

impl ComponentUpdate<AppModel> for ToolboxSettingsDialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        ToolboxSettingsDialogModel {
            hidden: true,
            window_title: None,
            current_toolbox_container: None,
            current_toolbx_settings: None,
        }
    }

    fn update(
        &mut self,
        msg: ToolboxSettingsDialogMsg,
        components: &SettingsComponents,
        _sender: Sender<ToolboxSettingsDialogMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            ToolboxSettingsDialogMsg::ReplyToolbxSettings(settings) => {
                self.current_toolbx_settings = Some(settings);
                println!("Received Toolbox Settings");
            }
            ToolboxSettingsDialogMsg::Show(toolbx_container) => {
                self.hidden = false;
                self.window_title = Some(format!(
                    "Toolbx Preferences: {}",
                    toolbx_container.name.clone()
                ));
                // TODO: create settings handling for each toolbox
                // probably settings for a toolbox should be stored in a HashMap and loaded based on that
                components
                    .async_handler
                    .sender()
                    .blocking_send(AsyncSettingsHandlerMsg::RequestToolbxSettings(
                        toolbx_container.clone(),
                    ))
                    .expect("Receiver dropped");
                self.current_toolbox_container = Some(toolbx_container);
            }
            ToolboxSettingsDialogMsg::Close => {
                self.current_toolbx_settings.as_ref().map(|settings| {
                    components
                        .async_handler
                        .sender()
                        .blocking_send(AsyncSettingsHandlerMsg::SaveToolboxSettings(
                            settings.clone(),
                        ))
                        .expect("Receiver dropped");
                });

                self.hidden = true;
                self.window_title = None;
                self.current_toolbox_container = None;
                self.current_toolbx_settings = None;
            }
        }
    }
}
