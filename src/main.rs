use relm4::{factory::FactoryVec, RelmApp};
use ui::app::model::{AppModel, ToolboxContainer, ToolboxStatus};

mod ui;

fn main() {
    let mut factory_vec = FactoryVec::new();
    factory_vec.push(ToolboxContainer {
        name: "fedora-toolbox-35".to_string(),
        status: ToolboxStatus::Running,
        update_available: false,
    });
    factory_vec.push(ToolboxContainer {
        name: "Latex".to_string(),
        status: ToolboxStatus::Running,
        update_available: false,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });
    factory_vec.push(ToolboxContainer {
        name: "Rust".to_string(),
        status: ToolboxStatus::Stopped,
        update_available: true,
    });

    let model = AppModel {
        toolboxes: factory_vec,
    };
    let app = RelmApp::new(model);
    app.run();
}
