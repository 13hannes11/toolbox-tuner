use relm4::{
    adw::{
        self,
        prelude::{BoxExt, ButtonExt, WidgetExt},
        traits::{ActionRowExt, PreferencesRowExt},
    },
    factory::{DynamicIndex, FactoryPrototype, FactoryVecDeque},
    gtk, send, view, Sender,
};

use crate::{
    toolbx::{ToolbxContainer, ToolbxStatus},
    ui::ui_strings::{
        APP_ICON, APP_TOOLTIP, SETTINGS_ICON, SETTINGS_TOOLTIP, SHUTDOWN_ICON, SHUTDOWN_TOOLTIP,
        START_ICON, START_TOOLTIP, TERMINAL_ICON, TERMINAL_TOOLTIP, UPDATE_ICON, UPDATE_TOOLTIP,
    },
};

use super::messages::AppMsg;

#[derive(Debug)]
pub struct FactoryWidgets {
    pub action_row: adw::ActionRow,
}

impl FactoryPrototype for ToolbxContainer {
    type Factory = FactoryVecDeque<Self>;
    type Widgets = FactoryWidgets;
    type Root = adw::ActionRow;
    type View = gtk::ListBox;
    type Msg = AppMsg;

    fn init_view(&self, key: &DynamicIndex, sender: Sender<Self::Msg>) -> Self::Widgets {

        view! {
            suffix_box = &gtk::Box{
                append = &gtk::AspectFrame{
                    set_ratio: 1.0,
                    set_child = Some(&gtk::Button::from_icon_name(APP_ICON)) {
                        set_margin_start: 10,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_tooltip_text: Some(APP_TOOLTIP),
                        set_css_classes: &["flat"],
                        connect_clicked(sender) => move |btn| {
                            send!(sender, AppMsg::ShowToolboxAppsRequest);
                        },
                    }
                },
                append = &gtk::AspectFrame{
                    set_ratio: 1.0,
                    set_child = Some(&gtk::Button::from_icon_name(TERMINAL_ICON)) {
                        set_margin_start: 10,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_tooltip_text: Some(TERMINAL_TOOLTIP),
                        set_css_classes: &["flat"],
                    }
                },
                append = &gtk::AspectFrame{
                    set_ratio: 1.0,
                    set_child = Some(&gtk::Button::from_icon_name(SETTINGS_ICON)) {
                        set_margin_start: 10,set_margin_start: 10,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_tooltip_text: Some(SETTINGS_TOOLTIP),
                        set_css_classes: &["circular"],
                        connect_clicked(sender) => move |btn| {
                            send!(sender, AppMsg::ShowToolboxSettingsRequest);
                        },
                    },
                }
            }
        };

        /*
        if self.update_available {
            view! {
                update_button = &gtk::AspectFrame{
                        set_ratio: 1.0,
                        set_child = Some(&gtk::Button::from_icon_name(UPDATE_ICON)) {
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_margin_end: 10,
                        set_tooltip_text: Some(UPDATE_TOOLTIP),
                        set_css_classes: &["suggested-action"],
                    }
                }
            };
            suffix_box.prepend(&update_button);
        }
        */

        let mut status_button_tooltip = START_TOOLTIP;
        let mut status_button_icon = START_ICON;

        match self.status {
            ToolbxStatus::Running => {
                status_button_tooltip = SHUTDOWN_TOOLTIP;
                status_button_icon = SHUTDOWN_ICON;
            }
            _ => {
                status_button_tooltip = START_TOOLTIP;
                status_button_icon = START_ICON;
            }
        }

        let subtitle = format!("created {}\n{}", self.created, self.image);

        let index = key.clone();

        view! {
            action_row = &adw::ActionRow {
                set_title: &self.name,
                set_subtitle: subtitle.as_str(),
                add_prefix = &gtk::Box {
                    append = &gtk::AspectFrame{
                        set_ratio: 1.0,
                        set_child = Some(&gtk::Button::from_icon_name(status_button_icon)) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(status_button_tooltip),
                            set_css_classes: &["circular"],
                            connect_clicked(sender) => move |btn| {
                                // Disable button
                                btn.set_sensitive(false);
                                send!(sender, AppMsg::ToolbxContainerToggleStartStop(index.clone()));
                            },
                        },
                    },
                },
                add_suffix: &suffix_box,
            }

        };
        FactoryWidgets { action_row }
    }

    fn view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        //widgets.action_row.set_label(&self.name.to_string());
        println!("updated {}", key.current_index());
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.action_row
    }

    fn position(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    ) -> <Self::View as relm4::factory::FactoryView<Self::Root>>::Position {
    }
}
