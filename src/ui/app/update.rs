use std::process::Command;

use relm4::{AppUpdate, Sender};

use crate::{ui::components::AppComponents, util::toolbx::ToolbxStatus};

use super::{messages::AppMsg, model::AppModel, workers::AsyncHandlerMsg};

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::ToolbxContainerToggleStartStop(index) => {
                if let Some(toolbx_container) = self.toolboxes.get_mut(index.current_index()) {
                    match toolbx_container.toolbx_container.status {
                        ToolbxStatus::Exited | ToolbxStatus::Configured | ToolbxStatus::Created => {
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
            AppMsg::ToolbxListUpdate(tbx_vec) => {
                println!("Updating Toolbox List");
                self.update_toolbxes(tbx_vec.into_iter());
            }

            AppMsg::OpenToolbxTerminal(index) => {
                if let Some(toolbx_container) = self.toolboxes.get_mut(index.current_index()) {
                    // TODO: support many terminals and check which are installed
                    let output = Command::new("flatpak-spawn")
                        .arg("--host")
                        .arg("gnome-terminal") //Command::new("gnome-terminal")
                        .arg("--")
                        .arg("toolbox")
                        .arg("enter")
                        .arg(toolbx_container.toolbx_container.name.clone())
                        .output();

                    println!("{:?}", output);

                    // TODO: update status on worker and add refresh spinner in the meantime
                    toolbx_container.toolbx_container.update_status();
                }
            }
        }
        true
    }
}
