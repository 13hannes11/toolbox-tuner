use std::collections::VecDeque;

use relm4::{
    factory::{FactoryVecDeque},
    RelmApp,
};
use toolbx::ToolbxContainer;
use ui::app::model::AppModel;

mod toolbx;
mod ui;

fn main() {
    let toolbx_list = VecDeque::from(ToolbxContainer::get_toolboxes());
    let factory_vec = FactoryVecDeque::from_vec_deque(toolbx_list);

    let model = AppModel {
        toolboxes: factory_vec,
    };
    let app = RelmApp::new(model);
    app.run();
}
