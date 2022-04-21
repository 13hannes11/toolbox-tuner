use relm4::{
    adw::{
        self,
        prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt},
        traits::AdwApplicationWindowExt,
    },
    gtk::{self, SelectionMode, Align, PolicyType},
    WidgetPlus, Widgets,
};

use super::model::AppModel;

#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 600,
            set_content : main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Toolbox Tuner",
                    }
                },

                append = &gtk::ScrolledWindow {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_hscrollbar_policy: PolicyType::Never,
                    set_child = Some(&gtk::ListBox) {
                        set_valign: Align::Start,
                        set_selection_mode: SelectionMode::None,
                        set_margin_all: 30,
                        set_css_classes: &["boxed-list"],
                        factory!(model.toolboxes)
                    }
                }
            }
        }
    }
}
