use gtk::prelude::ButtonExt;
use relm4::adw;
use relm4::adw::prelude::ActionRowExt;
use relm4::adw::prelude::PreferencesRowExt;
use relm4::factory::{FactoryComponent, FactorySender};
use relm4::gtk;
use relm4::gtk::prelude::WidgetExt;
use relm4_icons::icon_names;

#[derive(Debug)]
pub struct Container {
    hash: String,
    value: u8,
}

#[derive(Debug)]
pub enum ContainerMsg {
    Start,
}

#[relm4::factory(pub)]
impl FactoryComponent for Container {
    type Init = u8;
    type Input = ContainerMsg;
    type Output = ();
    type CommandOutput = ();
    type Widgets = ContainerWidgets;
    type ParentWidget = gtk::ListBox;
    type Index = String;

    view! {
        root = adw::ActionRow {
            #[watch]
            set_title: format!{"{}: {}", self.hash, self.value.to_string()}.as_str(),

            add_prefix = &gtk::Box{
                gtk::AspectFrame{
                    set_ratio: 1.0,
                    #[name(play_button)]
                    gtk::Button {
                        // TODO: make component with state that either is waiting, play or pause
                        set_icon_name: icon_names::PLAY,
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
                        connect_clicked => ContainerMsg::Start,
                    },
                },
            },
        }
    }

    fn init_model(value: Self::Init, index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            hash: index.clone(),
            value,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            ContainerMsg::Start => {
                self.value = self.value.wrapping_add(1);
            }
        }
    }
}
