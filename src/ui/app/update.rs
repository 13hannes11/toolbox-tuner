use relm4::{AppUpdate, Sender};

use crate::{
    toolbx::ToolbxStatus,
    ui::components::{
        toolbox_apps::messages::ToolboxAppDialogMsg,
        toolbox_settings::messages::ToolboxSettingsDialogMsg, AppComponents,
    },
};

use super::{messages::AppMsg, model::AppModel, workers::AsyncHandlerMsg};

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
                            components
                                .async_handler
                                .sender()
                                .blocking_send(AsyncHandlerMsg::StartToolbx(
                                    index,
                                    toolbx_container.clone(),
                                ))
                                .expect("Receiver dropped");
                        }
                        ToolbxStatus::Running => {
                            toolbx_container.changing_status = true;
                            components
                                .async_handler
                                .sender()
                                .blocking_send(AsyncHandlerMsg::StopToolbx(
                                    index,
                                    toolbx_container.clone(),
                                ))
                                .expect("Receiver dropped");
                        }
                    }
                    // TODO: tell button to reactivate somehow
                }
            }
            AppMsg::ToolbxContainerChanged(index, container) => {
                if let Some(toolbx_container) = self.toolboxes.get_mut(index.current_index()) {
                    toolbx_container.update_entry(container);
                }
            }

            AppMsg::OpenToolbxTerminal(index) => {
                if let Some(toolbx_container) = self.toolboxes.get_mut(index.current_index()) {
                    components
                        .async_handler
                        .sender()
                        .blocking_send(AsyncHandlerMsg::OpenToolbxTerminal(
                            index,
                            toolbx_container.clone(),
                        ))
                        .expect("Receiver dropped");
                }
            }
        }
        true
    }
}
