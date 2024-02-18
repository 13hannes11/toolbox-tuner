use std::collections::VecDeque;

use relm4::gtk::{Application, ApplicationWindow};
use relm4::{factory::FactoryVecDeque, RelmApp};
use ui::app::model::AppModel;
use util::toolbx::ToolbxContainer;

mod ui;
mod util;

fn main() {
    let mut model = AppModel {
        toolboxes: FactoryVecDeque::new(),
    };
    let app = RelmApp::new(model);
    app.run();
}
