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
    toolbx::ToolbxStatus,
    ui::ui_strings::{
        APP_ICON, APP_TOOLTIP, SETTINGS_ICON, SETTINGS_TOOLTIP, SHUTDOWN_ICON, SHUTDOWN_TOOLTIP,
        START_ICON, START_TOOLTIP, TERMINAL_ICON, TERMINAL_TOOLTIP, UPDATE_ICON, UPDATE_TOOLTIP,
    },
};

use super::{messages::AppMsg, model::ToolbxEntry};

#[derive(Debug)]
pub struct FactoryWidgets {
    pub action_row: adw::ActionRow,
    status_button: gtk::Button,
    status_spinner: gtk::Spinner,
}

impl FactoryPrototype for ToolbxEntry {
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

        match self.toolbx_container.status {
            ToolbxStatus::Running => {
                status_button_tooltip = SHUTDOWN_TOOLTIP;
                status_button_icon = SHUTDOWN_ICON;
            }
            _ => {
                status_button_tooltip = START_TOOLTIP;
                status_button_icon = START_ICON;
            }
        }

        let subtitle = format!(
            "created {}\n{}",
            self.toolbx_container.created, self.toolbx_container.image
        );

        let index = key.clone();

        view! {
            status_spinner = &gtk::Spinner {
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_tooltip_text: Some(status_button_tooltip),
                set_css_classes: &["circular"],

            }
        };
        //status_spinner.start();

        view! {
            status_button = &gtk::Button::from_icon_name(status_button_icon) {
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_tooltip_text: Some(status_button_tooltip),
                set_css_classes: &["circular"],
                connect_clicked(sender) => move |btn| {
                    // Disable button
                    btn.set_sensitive(false);
                    send!(sender, AppMsg::ToolbxContainerToggleStartStop(index.clone()));
                },
            }
        };

        view! {
            action_row = &adw::ActionRow {
                set_title: &self.toolbx_container.name,
                set_subtitle: subtitle.as_str(),
                add_prefix = &gtk::Box {
                    append = &gtk::AspectFrame{
                        set_ratio: 1.0,
                        set_child: Some(&status_button),
                    }
                },
                add_suffix: &suffix_box,
            }

        };
        FactoryWidgets {
            action_row,
            status_button,
            status_spinner,
        }
    }

    fn view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        println!("updated {}", key.current_index());

        // fixme: IDEALY this is would be done with message handling and only if the request actually is done

        if self.changing_status {
            widgets.status_button.set_sensitive(false);
            widgets
                .status_button
                .set_child(Some(&widgets.status_spinner));
            widgets.status_spinner.start();
        } else {
            match self.toolbx_container.status {
                ToolbxStatus::Running => {
                    widgets.status_button.set_icon_name(SHUTDOWN_ICON);
                    widgets
                        .status_button
                        .set_tooltip_text(Some(SHUTDOWN_TOOLTIP));
                }
                _ => {
                    widgets.status_button.set_icon_name(START_ICON);
                    widgets.status_button.set_tooltip_text(Some(START_TOOLTIP));
                }
            }
            widgets.status_button.set_sensitive(true);
            widgets.status_spinner.stop();
        }
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
