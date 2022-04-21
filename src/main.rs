use relm4::{factory::FactoryVec, RelmApp};
use toolbx::ToolbxContainer;
use ui::app::model::{AppModel};

mod ui;
mod toolbx;

fn main() {
    let toolbx_list = ToolbxContainer::get_toolboxes();
    let mut factory_vec = FactoryVec::from_vec(toolbx_list);

    let model = AppModel {
        toolboxes: factory_vec,
    };
    let app = RelmApp::new(model);
    app.run();
}
