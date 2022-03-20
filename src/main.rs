use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{adw::{self, traits::{PreferencesRowExt, ActionRowExt}, prelude::{ListBoxRowExt, WidgetExt}}, gtk::{self, SelectionMode}, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

const START_ICON : &str = r#"media-playback-start-symbolic"#;
const START_TOOLTIP : &str = r#"Start toolbox"#;

const SHUTDOWN_ICON : &str = r#"system-shutdown-symbolic"#;
const SHUTDOWN_TOOLTIP : &str = r#"Stop toolbox"#;

const UPDATE_ICON : &str = r#"software-update-available-symbolic"#;
const UPDATE_TOOLTIP : &str = r#"Update all applications inside of the toolbox"#;

const APP_ICON : &str = r#"view-grid-symbolic"#;
const APP_TOOLTIP : &str = r#"Select applications to showup in the application menu"#;

const TERMINAL_ICON : &str = r#"utilities-terminal-symbolic"#;
const TERMINAL_TOOLTIP : &str = r#"Open terminal inside of toolbox"#;

const SETTINGS_ICON : &str = r#"applications-system-symbolic"#;
const SETTINGS_TOOLTIP : &str = r#"Open toolbox settings"#;

#[derive(Default)]
struct AppModel {
}

enum AppMsg {
    Increment,
    Decrement,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Increment => {
            }
            AppMsg::Decrement => {
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 600,

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,

                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "Toolbox Tuner",
                    }
                },
                
                append = &gtk::ListBox {
                    set_selection_mode: SelectionMode::None,
                    set_margin_all: 30,
                    set_css_classes: &["boxed-list"],
                    append = &adw::ActionRow {
                        set_title: "Latex",
                        set_subtitle: "additional information",
                        add_prefix = &gtk::Box {
                            append = &gtk::Button::from_icon_name(START_ICON) {
                                set_margin_top: 10,
                                set_margin_bottom: 10,
                                set_tooltip_text: Some(START_TOOLTIP),
                                set_css_classes: &["circular"],
                            },
                        }, 
                        
                        add_suffix = &gtk::Button::from_icon_name(UPDATE_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_margin_end: 10,
                            set_tooltip_text: Some(UPDATE_TOOLTIP),
                            set_css_classes: &["suggested-action"],
                        },
                        add_suffix = &gtk::Button::from_icon_name(APP_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(APP_TOOLTIP),
                            set_css_classes: &["flat"],
                        },                        
                        add_suffix = &gtk::Button::from_icon_name(TERMINAL_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(TERMINAL_TOOLTIP),
                            set_css_classes: &["flat"],
                        },
                        add_suffix = &gtk::Button::from_icon_name(SETTINGS_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(SETTINGS_TOOLTIP),
                            set_css_classes: &["circular"],
                        },
                    },
                    append = &adw::ActionRow {
                        set_title: "Rust",
                        set_subtitle: "additional information",
                        add_prefix = &gtk::Box {
                            append = &gtk::Button::from_icon_name(SHUTDOWN_ICON) {
                                set_margin_top: 10,
                                set_margin_bottom: 10,
                                set_tooltip_text: Some(SHUTDOWN_TOOLTIP),
                                set_css_classes: &["circular"],
                            },
                        },
                        add_suffix = &gtk::Button::from_icon_name(APP_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(APP_TOOLTIP),
                            set_css_classes: &["flat"],
                        },
                        add_suffix = &gtk::Button::from_icon_name(TERMINAL_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(TERMINAL_TOOLTIP),
                            set_css_classes: &["flat"],
                        },
                        add_suffix = &gtk::Button::from_icon_name(SETTINGS_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(SETTINGS_TOOLTIP),
                            set_css_classes: &["circular"],
                        },
                    },
                    append = &adw::ActionRow {
                        set_title: "Python",
                        set_subtitle: "additional information",
                        add_prefix = &gtk::Box {
                            append = &gtk::Button::from_icon_name(SHUTDOWN_ICON) {
                                set_margin_top: 10,
                                set_margin_bottom: 10,
                                set_tooltip_text: Some(SHUTDOWN_TOOLTIP),
                                set_css_classes: &["circular"],
                            },
                        }, 
                        add_suffix = &gtk::Button::from_icon_name(APP_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(APP_TOOLTIP),
                            set_css_classes: &["flat"],
                        },
                        add_suffix = &gtk::Button::from_icon_name(TERMINAL_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(TERMINAL_TOOLTIP),
                            set_css_classes: &["flat"],
                        },
                        add_suffix = &gtk::Button::from_icon_name(SETTINGS_ICON) {
                            set_margin_top: 10,
                            set_margin_bottom: 10,
                            set_tooltip_text: Some(SETTINGS_TOOLTIP),
                            set_css_classes: &["circular"],
                        },
                    },
                },
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
