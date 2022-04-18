use relm4::{
    adw::prelude::{BoxExt, OrientableExt, WidgetExt},
    factory::{FactoryPrototype, FactoryVec},
    gtk::{self, Orientation},
    view, Sender, WidgetPlus,
};

use super::{
    messages::ToolboxAppDialogMsg, model::DotDesktopApplication, widgets::AppFactoryWidgets,
};

impl FactoryPrototype for DotDesktopApplication {
    type Factory = FactoryVec<Self>;
    type Widgets = AppFactoryWidgets;
    type Root = gtk::Box;
    type View = gtk::FlowBox;
    type Msg = ToolboxAppDialogMsg;

    fn init_view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        sender: Sender<Self::Msg>,
    ) -> Self::Widgets {
        view! {
            app_box = &gtk::Box {
                set_css_classes: &[&"card"],
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                set_orientation: Orientation::Vertical,
                set_hexpand: false,
                set_vexpand: false,
                set_width_request: 100,
                set_height_request: 100,

                append = &gtk::Switch {
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_hexpand: false,
                    set_vexpand: false,
                    set_margin_all: 10,
                    set_active: self.selected,
                },
                append = &gtk::Image::from_file(&self.icon_path) {
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_width_request: 64,
                    set_height_request: 64,
                    set_margin_start: 36,
                    set_margin_end: 36,
                    set_hexpand: false,
                    set_vexpand: false,
                },
                append = &gtk::Label {
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_text: &self.name,
                    set_margin_all: 5,
                    set_hexpand: false,
                    set_vexpand: false,
                }
            }
        }
        AppFactoryWidgets { app_box }
    }

    fn position(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    ) -> <Self::View as relm4::factory::FactoryView<Self::Root>>::Position {
    }

    fn view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.app_box
    }
}
