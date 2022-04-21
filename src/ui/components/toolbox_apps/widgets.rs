use gtk::Orientation;
use gtk::PolicyType;
use relm4::adw;
use relm4::adw::prelude::GtkWindowExt;
use relm4::adw::prelude::WidgetExt;
use relm4::adw::traits::PreferencesGroupExt;
use relm4::adw::traits::PreferencesPageExt;
use relm4::adw::traits::PreferencesWindowExt;
use relm4::gtk;
use relm4::send;
use relm4::WidgetPlus;
use relm4::Widgets;

use gtk::prelude::*;

use crate::ui::app::model::AppModel;
use crate::ui::components::toolbox_apps::messages::ToolboxAppDialogMsg;

use super::model::ToolboxAppDialogModel;

#[derive(Debug)]
pub struct AppFactoryWidgets {
    pub app_box: gtk::Box,
}

#[relm4::widget(pub)]
impl Widgets<ToolboxAppDialogModel, AppModel> for ToolboxAppDialogWidgets {
    view! {
        adw::PreferencesWindow {
            set_title: Some("Applications: <Toolbox_name>"),
            set_transient_for: parent!{Some(&parent_widgets.main_window)},
            set_modal: true,
            set_search_enabled: false,
            set_visible: watch!(!model.hidden),
            connect_close_request(sender) => move |_| {
                send!(sender, ToolboxAppDialogMsg::Close);
                gtk::Inhibit(true)
            },
            add = &adw::PreferencesPage {
                set_hexpand: true,
                set_vexpand: false,
                add = &adw::PreferencesGroup {
                    add = &gtk::ScrolledWindow {
                        set_hscrollbar_policy: PolicyType::Never,
                        set_hexpand: false,
                        set_vexpand: true,
                        set_child = Some(&gtk::FlowBox) {
                            set_halign: gtk::Align::Fill,
                            set_valign: gtk::Align::Start,
                            set_selection_mode: gtk::SelectionMode::None,
                            set_margin_all: 5,
                            set_column_spacing: 5,
                            set_row_spacing: 5,
                            set_orientation: Orientation::Horizontal,
                            factory!(model.apps),
                        }
                    }

                }
            }
        }
    }
}
