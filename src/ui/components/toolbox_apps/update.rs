use relm4::{factory::FactoryVec, ComponentUpdate, Sender};

use crate::ui::app::{messages::AppMsg, model::AppModel};

use super::{
    messages::ToolboxAppDialogMsg,
    model::{DotDesktopApplication, ToolboxAppDialogModel},
};

impl ComponentUpdate<AppModel> for ToolboxAppDialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        let mut factory_vec = FactoryVec::new();
        factory_vec.push(DotDesktopApplication {
            name: "Firefox".to_string(),
            selected: true,
            icon_path: "".to_string(),
        });
        factory_vec.push(DotDesktopApplication {
            name: "Firefox".to_string(),
            selected: false,
            icon_path: "".to_string(),
        });
        factory_vec.push(DotDesktopApplication {
            name: "Firefox".to_string(),
            selected: true,
            icon_path: "".to_string(),
        });
        factory_vec.push(DotDesktopApplication {
            name: "Firefox".to_string(),
            selected: false,
            icon_path: "".to_string(),
        });
        ToolboxAppDialogModel {
            hidden: true,
            apps: factory_vec,
        }
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
