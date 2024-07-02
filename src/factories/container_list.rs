use crate::util::toolbox::open_toolbox_container_in_terminal;
use crate::util::toolbox::start_toolbox_container;
use crate::util::toolbox::stop_toolbox_container;
use gtk::prelude::ButtonExt;
use relm4::adw;
use relm4::adw::prelude::ActionRowExt;
use relm4::adw::prelude::PreferencesRowExt;
use relm4::factory::{FactoryComponent, FactorySender};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4_icons::icon_names;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum ContainerStatus {
    Running,
    NotRunning,
    Refreshing,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum ContainerOperation {
    LaunchingTerminal,
}

#[derive(Debug)]
pub struct Container {
    hash: String,
    value: String,
    status: ContainerStatus,
    running_operations: HashSet<ContainerOperation>,
}

#[derive(Debug)]
pub enum ContainerMsg {
    Start,
    Stop,
    OpenTerminal,
}

#[derive(Debug)]
pub enum CommandMessage {
    SetStarted,
    SetStopped,
    FinishLaunchTerminal,
}

pub struct ContainerInit {
    pub name: String,
    pub status: ContainerStatus,
}

#[relm4::factory(pub)]
impl FactoryComponent for Container {
    type Init = ContainerInit;
    type Input = ContainerMsg;
    type Output = ();
    type CommandOutput = CommandMessage;
    type Widgets = ContainerWidgets;
    type ParentWidget = gtk::ListBox;
    type Index = String;

    view! {
        root = adw::ActionRow {
            #[watch]
            set_title: &self.value,
            #[watch]
            set_subtitle: &self.hash,

            add_prefix = &gtk::Box{
                gtk::AspectFrame{
                    set_ratio: 1.0,
                    gtk::Box{
                        gtk::Button {
                            #[watch]
                            set_visible: self.status == ContainerStatus::Refreshing,
                            #[wrap(Some)]
                            set_child = &gtk::Spinner {
                                #[watch]
                                set_spinning: self.status == ContainerStatus::Refreshing,
                            },
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_css_classes: &["circular"],
                        },
                        gtk::Button {
                            #[watch]
                            set_visible: self.status == ContainerStatus::NotRunning,
                            set_icon_name: icon_names::PLAY,
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_css_classes: &["circular"],
                            connect_clicked => ContainerMsg::Start,
                        },
                        gtk::Button {
                            #[watch]
                            set_visible: self.status == ContainerStatus::Running,
                            set_icon_name: icon_names::PAUSE,
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_css_classes: &["circular"],
                            connect_clicked => ContainerMsg::Stop,
                        },
                    },
                },
            },
            add_prefix = &gtk::Box{
                gtk::AspectFrame{
                    set_ratio: 1.0,

                },
            },

            add_suffix = &gtk::Box{
                gtk::AspectFrame{
                    #[watch]
                    set_visible: !self.running_operations.contains(&ContainerOperation::LaunchingTerminal),
                    set_ratio: 1.0,
                    gtk::Button {
                        set_icon_name: icon_names::TERMINAL,
                        set_margin_start: 10,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_css_classes: &["flat"],
                        connect_clicked => ContainerMsg::OpenTerminal,
                    },

                },
                gtk::AspectFrame{
                    #[watch]
                    set_visible: self.running_operations.contains(&ContainerOperation::LaunchingTerminal),
                    set_ratio: 1.0,
                    gtk::Button {
                        set_margin_start: 10,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_css_classes: &["flat"],
                        #[wrap(Some)]
                        set_child = &gtk::Spinner {
                            #[watch]
                            set_spinning: self.running_operations.contains(&ContainerOperation::LaunchingTerminal),
                        },
                    },
                },
            },
        }
    }

    fn init_model(value: Self::Init, index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            hash: index.clone(),
            value: value.name.clone(),
            status: value.status,
            running_operations: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            ContainerMsg::Start => {
                self.status = ContainerStatus::Refreshing;
                let hash = (&self.hash).clone();
                sender.spawn_oneshot_command(move || match start_toolbox_container(&hash) {
                    Ok(_) => CommandMessage::SetStarted,
                    Err(_) => CommandMessage::SetStopped,
                });
            }
            ContainerMsg::Stop => {
                self.status = ContainerStatus::Refreshing;
                let hash = (&self.hash).clone();
                sender.spawn_oneshot_command(move || match stop_toolbox_container(&hash) {
                    Ok(_) => CommandMessage::SetStopped,
                    Err(_) => CommandMessage::SetStarted,
                });
            }
            ContainerMsg::OpenTerminal => {
                self.running_operations
                    .insert(ContainerOperation::LaunchingTerminal);
                let hash = (&self.hash).clone();
                sender.spawn_oneshot_command(move || {
                    match open_toolbox_container_in_terminal(&hash) {
                        _ => CommandMessage::FinishLaunchTerminal,
                    }
                });
            }
        }
    }

    fn update_cmd(&mut self, message: Self::CommandOutput, _sender: FactorySender<Self>) {
        match message {
            CommandMessage::SetStarted => self.status = ContainerStatus::Running,
            CommandMessage::SetStopped => self.status = ContainerStatus::NotRunning,
            CommandMessage::FinishLaunchTerminal => {
                self.running_operations
                    .remove(&ContainerOperation::LaunchingTerminal);
            }
        };
    }
}
