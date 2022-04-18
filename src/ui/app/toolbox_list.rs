use relm4::{
    adw::{
        self,
        prelude::{BoxExt, ButtonExt, WidgetExt},
        traits::{ActionRowExt, PreferencesRowExt},
    },
    factory::{FactoryPrototype, FactoryVec},
    gtk, send, view, Sender,
};

use crate::ui::{
    app::model::ToolboxStatus,
    ui_strings::{
        APP_ICON, APP_TOOLTIP, SETTINGS_ICON, SETTINGS_TOOLTIP, SHUTDOWN_ICON, SHUTDOWN_TOOLTIP,
        START_ICON, START_TOOLTIP, TERMINAL_ICON, TERMINAL_TOOLTIP, UPDATE_ICON, UPDATE_TOOLTIP,
    },
};

use super::{messages::AppMsg, model::ToolboxContainer};

#[derive(Debug)]
pub struct FactoryWidgets {
    pub action_row: adw::ActionRow,
}

impl FactoryPrototype for ToolboxContainer {
    type Factory = FactoryVec<Self>;
    type Widgets = FactoryWidgets;
    type Root = adw::ActionRow;
    type View = gtk::ListBox;
    type Msg = AppMsg;

    fn init_view(
        &self,
        key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
        sender: Sender<Self::Msg>,
    ) -> Self::Widgets {
        view! {
            suffix_box = &gtk::Box{
                append = &gtk::Button::from_icon_name(APP_ICON) {
                    set_margin_start: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_tooltip_text: Some(APP_TOOLTIP),
                    set_css_classes: &["flat"],
                    connect_clicked(sender) => move |btn| {
                        send!(sender, AppMsg::ShowToolboxAppsRequest);
                    },
                },
                append = &gtk::Button::from_icon_name(TERMINAL_ICON) {
                    set_margin_start: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_tooltip_text: Some(TERMINAL_TOOLTIP),
                    set_css_classes: &["flat"],
                },
                append = &gtk::Button::from_icon_name(SETTINGS_ICON) {
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
        };

        if self.update_available {
            view! {
                update_button = &gtk::Button::from_icon_name(UPDATE_ICON) {
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_end: 10,
                    set_tooltip_text: Some(UPDATE_TOOLTIP),
                    set_css_classes: &["suggested-action"],
                }
            };
            suffix_box.prepend(&update_button);
        }

        let is_on = true;

        let mut status_button_tooltip = START_TOOLTIP;
        let mut status_button_icon = START_ICON;

        match &self.status {
            &ToolboxStatus::Running => {
                status_button_tooltip = SHUTDOWN_TOOLTIP;
                status_button_icon = SHUTDOWN_ICON;
            }
            &ToolboxStatus::Stopped => {
                status_button_tooltip = START_TOOLTIP;
                status_button_icon = START_ICON;
            }
        }

        view! {
            action_row = &adw::ActionRow {
                set_title: &self.name,
                set_subtitle: "additional information",
                add_prefix = &gtk::Box {
                    append = &gtk::Button::from_icon_name(status_button_icon) {
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_tooltip_text: Some(status_button_tooltip),
                        set_css_classes: &["circular"],
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