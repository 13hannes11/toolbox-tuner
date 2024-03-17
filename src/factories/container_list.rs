use gtk::prelude::ButtonExt;
use relm4::adw;
use relm4::adw::prelude::ActionRowExt;
use relm4::adw::prelude::PreferencesRowExt;
use relm4::factory::{FactoryComponent, FactorySender};
use relm4::gtk;

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
            set_title: &self.hash,

            #[name(add_button)]
            add_suffix = &gtk::Button {
                #[watch]
                set_label: &self.value.to_string(),
                connect_clicked => ContainerMsg::Start,
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
