use relm4::adw;
use relm4::adw::prelude::BoxExt;
use relm4::adw::prelude::GtkWindowExt;
use relm4::adw::prelude::ListBoxRowExt;
use relm4::adw::prelude::WidgetExt;
use relm4::adw::traits::ActionRowExt;
use relm4::adw::traits::PreferencesGroupExt;
use relm4::adw::traits::PreferencesPageExt;
use relm4::adw::traits::PreferencesRowExt;
use relm4::adw::traits::PreferencesWindowExt;
use relm4::gtk;
use relm4::send;
use relm4::WidgetPlus;
use relm4::Widgets;

use crate::ui::app::model::AppModel;
use crate::ui::components::toolbox_settings::messages::ToolboxSettingsDialogMsg;
use crate::ui::ui_strings::FOLDER_PICKER_ICON;
use crate::ui::ui_strings::FOLDER_PICKER_TOOLTIP;

use super::model::ToolboxSettingsDialogModel;

#[relm4::widget(pub)]
impl Widgets<ToolboxSettingsDialogModel, AppModel> for ToolboxSettingsDialogWidgets {
    view! {
        adw::PreferencesWindow {
            set_title:  watch!{ model.window_title.as_ref().map( |x| x.as_str() ) },
            set_transient_for: parent!{Some(&parent_widgets.main_window)},
            set_modal: true,
            set_visible: watch!(!model.hidden),
            connect_close_request(sender) => move |_| {
                send!(sender, ToolboxSettingsDialogMsg::Close);
                gtk::Inhibit(true)
            },
            add = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_title: "Updates",
                    add = &adw::PreferencesRow {
                        set_title: "Update Policy",
                        set_child = Some(&adw::ActionRow) {
                            set_title: "Update Policy",
                            add_suffix = &gtk::Box {
                                append = &gtk::DropDown::from_strings(&[
                                        "Update automatically",
                                        "Notify about updates",
                                        "Do nothing"
                                     ]) {
                                    set_margin_all: 15,
                                },
                            }
                        },
                    },
            },
                add = &adw::PreferencesGroup {
                    set_title: "Home Folder",
                    add = &adw::PreferencesRow {
                        set_title: "Seperate Home Folder",
                        set_child = Some(&adw::ActionRow) {
                            set_title: "Use separate home folder",
                            add_suffix = &gtk::Box {
                                append = &gtk::Switch {
                                    set_margin_all: 15,
                                    set_tooltip_text: Some("Use separate home folder"),
                                },
                            }
                        },
                    },
                    add = &adw::PreferencesRow {
                        set_title: "Home Folder Path",
                        set_child = Some(&adw::ActionRow) {
                            set_title: "Home folder path",
                            add_suffix = &gtk::Box {
                                set_margin_all: 15,
                                add_css_class: "linked",
                                append = &gtk::Entry {
                                    set_hexpand: true,
                                },
                                append = &gtk::Button::from_icon_name(FOLDER_PICKER_ICON) {
                                    set_tooltip_text: Some(FOLDER_PICKER_TOOLTIP),
                                }
                            }
                        },
                    },

                }
            }
        }
    }
}
