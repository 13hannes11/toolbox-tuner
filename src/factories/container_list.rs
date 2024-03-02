use crate::app::AppMsg;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::adw;
use relm4::adw::prelude::ActionRowExt;
use relm4::adw::prelude::PreferencesRowExt;
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
pub struct Container {
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
    type ParentInput = AppMsg;
    type ParentWidget = gtk::ListBox;

    view! {
        root = adw::ActionRow {
            #[watch]
            set_title: &self.value.to_string(),


            #[name(add_button)]
            add_prefix = &gtk::Button {
                set_label: "+",
                connect_clicked => ContainerMsg::Start,
            },

            #[name(remove_button)]
            add_suffix = &gtk::Button {
                set_label: "-",
                connect_clicked => ContainerMsg::Start,
            },
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { value }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            ContainerMsg::Start => {
                self.value = self.value.wrapping_add(1);
            }
        }
    }
}
