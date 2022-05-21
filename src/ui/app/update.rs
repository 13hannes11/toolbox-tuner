use relm4::{AppUpdate, Sender};

use crate::{
    toolbx::ToolbxStatus,
    ui::components::{
        toolbox_apps::messages::ToolboxAppDialogMsg,
        toolbox_settings::messages::ToolboxSettingsDialogMsg, AppComponents,
    },
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
            AppMsg::ShowToolboxAppsRequest => {
                components
                    .toolbox_apps_dialog
                    .send(ToolboxAppDialogMsg::Show)
                    .unwrap();
            }
            AppMsg::ToolbxContainerToggleStartStop(index) => {
                if let Some(toolbx_container) = self.toolboxes.get_mut(index.current_index()) {
                    
                    match toolbx_container.toolbx_container.status {
                        ToolbxStatus::Exited | ToolbxStatus::Configured => {
                            toolbx_container.changing_status = true;
                            // Send message to background worker to start
                            //toolbx_container.toolbx_container.start();
                            
                        }
                        ToolbxStatus::Running => {
                            toolbx_container.changing_status = true;
                            // send message to beackground worker to stop
                            //toolbx_container.toolbx_container.stop();
                        }
                    }
                    // TODO: tell button to reactivate somehow
                }
            }
        }
        true
    }
}
