use gtk::prelude::ButtonExt;
use relm4::adw;
use relm4::adw::prelude::ActionRowExt;
use relm4::adw::prelude::PreferencesRowExt;
use relm4::factory::{FactoryComponent, FactorySender};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4_icons::icon_names;

#[derive(Debug)]
pub enum ContainerStatus {
    Running,
    NotRunning,
}

#[derive(Debug)]
pub struct Container {
    hash: String,
    value: String,
    status: ContainerStatus,
}

#[derive(Debug)]
pub enum ContainerMsg {
    Start,
    Stop,
    OpenTerminal,
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
    type CommandOutput = ();
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
                    #[name(play_button)]
                    gtk::Button {
                        #[watch]
                        set_icon_name: match &self.status {
                            ContainerStatus::NotRunning => icon_names::PLAY,
                            ContainerStatus::Running => icon_names::PAUSE,
                        },
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_css_classes: &["circular"],
                        connect_clicked => ContainerMsg::Start,
                    },
                },
            },

            add_suffix = &gtk::Box{
                gtk::AspectFrame{
                    set_ratio: 1.0,
                    #[name(add_button)]
                    gtk::Button {
                        set_icon_name: icon_names::TERMINAL,
                        set_margin_start: 10,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_css_classes: &["flat"],
                        connect_clicked => ContainerMsg::OpenTerminal,
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
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            ContainerMsg::Start => {}
            ContainerMsg::Stop => {}
            ContainerMsg::OpenTerminal => {}
        }
    }
}
