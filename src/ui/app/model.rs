use relm4::{
    factory::{FactoryVecDeque},
    Model,
};

use crate::{toolbx::ToolbxContainer, ui::components::AppComponents};

use super::{messages::AppMsg, widgets::AppWidgets};

pub struct ToolbxEntry {
    pub toolbx_container: ToolbxContainer,
    pub changing_status: bool,
    // TODO: settings
}

impl ToolbxEntry {
    pub fn update_container(&mut self, container : ToolbxContainer) {
        std::mem::replace::<ToolbxContainer>(&mut self.toolbx_container, container);
    }
}

pub struct AppModel {
    pub toolboxes: FactoryVecDeque<ToolbxEntry>,
}

impl Model for AppModel { 
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppModel {
    pub fn update_toolbxes<I>(&mut self, toolbox_iter: I)
    where
    I: Iterator<Item = ToolbxContainer>, {
        // Update each toolbx entry if there were changes to it
        // TODO: deal with the removal of toolboxes
        for tbx_update in toolbox_iter {
            println!("name: {}", tbx_update.name);
            let mut exists = false;
            for (index, tbx_entry) in self.toolboxes.iter().enumerate() {
                if tbx_update.name == tbx_entry.toolbx_container.name {
                    self.toolboxes.get_mut(index).map(|x| x.update_container(tbx_update.clone()));
                    exists = true;
                    break;
                }
            }
            if !exists {
                println!("{}", tbx_update.name);
                self.toolboxes.push_back(ToolbxEntry { toolbx_container: tbx_update, changing_status: false})
            }
        }
    }
}