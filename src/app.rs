use crate::gtk::Align;
use crate::util::prerequisit::get_installed_terminals;
use crate::util::prerequisit::is_toolbox_installed;

use crate::modals::settings::SettingsMsg;
use crate::util::toolbox::ToolbxContainer;
use relm4::adw::prelude::PreferencesGroupExt;
use relm4::factory::FactoryHashMap;
use relm4::gtk::PolicyType;
use relm4::RelmWidgetExt;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender,
    Controller,
};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use crate::config::{APP_ID, PROFILE};
use crate::factories::container_list::Container;
use crate::factories::container_list::ContainerStatus;
use crate::modals::about::AboutDialog;
use crate::modals::settings::SettingsDialog;
use crate::modals::unsupported::UnsupportedDialog;
use crate::modals::unsupported::UnsupportedDialogOutput;
use crate::util::toolbox::ToolbxStatus;
use gtk::prelude::{
    ApplicationExt, ApplicationWindowExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
};
use gtk::{gio, glib};

pub(super) struct App {
    settings_dialog: Controller<SettingsDialog>,
    unsupported_dialog: Controller<UnsupportedDialog>,
    about_dialog: Controller<AboutDialog>,
    containers: FactoryHashMap<String, Container>,
    spinning: bool,
}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
}

#[derive(Debug)]
pub(super) enum AppCommandMsg {
    PrerequisitsInstalled(bool),
    UpdateToolboxes(Vec<ToolbxContainer>),
    InitiateRefresh,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
use crate::factories::container_list::ContainerInit;

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppMsg;
    type CommandOutput = AppCommandMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Toolbox Tuner" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/org/kuchelmeister/ToolboxTuner/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                gtk::ScrolledWindow {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_hscrollbar_policy: PolicyType::Never,

                    adw::PreferencesGroup{
                        set_title: "Toolboxes",
                        set_margin_all: 30,



                        #[wrap(Some)]
                        set_header_suffix: refresh_spinner  = &gtk::Spinner {
                            #[watch]
                            set_spinning: model.spinning
                        },

                        #[local_ref]
                        container_box -> gtk::ListBox {
                            set_size_request: (200, -1),
                            set_selection_mode: gtk::SelectionMode::None,
                            set_valign: Align::Start,
                            set_css_classes: &["boxed-list"],
                        },
                    },

                },
            }

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(&root)
            .launch(())
            .detach();

        let settings_dialog = SettingsDialog::builder()
            .transient_for(&root)
            .launch(())
            .detach();

        let unsupported_dialog = UnsupportedDialog::builder()
            .transient_for(&root)
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                UnsupportedDialogOutput::CloseApplication => AppMsg::Quit,
            });

        let containers = FactoryHashMap::builder().launch_default().detach();

        let model = Self {
            settings_dialog,
            about_dialog,
            unsupported_dialog,
            containers,
            spinning: true,
        };

        let container_box = model.containers.widget();
        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let preference_action = {
            let settings = model.settings_dialog.sender().clone();
            RelmAction::<PreferencesAction>::new_stateless(move |_| {
                settings.send(SettingsMsg::OpenSettings).unwrap();
            })
        };

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap();
            })
        };

        sender.spawn_oneshot_command(|| {
            let terminals = get_installed_terminals().unwrap_or_default();
            let toolbox_installed = is_toolbox_installed().unwrap_or(false);
            AppCommandMsg::PrerequisitsInstalled(terminals.len() > 0 && toolbox_installed)
        });

        actions.add_action(preference_action);
        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AppMsg::Quit => main_application().quit(),
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        match message {
            AppCommandMsg::PrerequisitsInstalled(false) => {
                self.unsupported_dialog.sender().clone().send(()).unwrap()
            }

            AppCommandMsg::PrerequisitsInstalled(true) | AppCommandMsg::InitiateRefresh => {
                self.spinning = true;
                sender.spawn_oneshot_command(|| {
                    AppCommandMsg::UpdateToolboxes(ToolbxContainer::get_toolboxes())
                })
            }
            AppCommandMsg::UpdateToolboxes(toolboxes) => {
                let mut updated_containers = HashSet::<String>::new();
                toolboxes.iter().for_each(|toolbox| {
                    self.containers.insert(
                        toolbox.id.clone(),
                        ContainerInit {
                            name: toolbox.name.clone(),
                            status: match toolbox.status {
                                ToolbxStatus::Running => ContainerStatus::Running,
                                _ => ContainerStatus::NotRunning,
                            },
                        },
                    );
                    updated_containers.insert(toolbox.id.clone());
                });
                let obsolete_containers: Vec<String> = self
                    .containers
                    .iter()
                    .map(|(hash, _)| hash.clone())
                    .filter(|hash| !updated_containers.contains(hash))
                    .collect();
                obsolete_containers.into_iter().for_each(|hash| {
                    self.containers.remove(&hash);
                });

                self.spinning = false;

                sender.spawn_oneshot_command(|| {
                    sleep(Duration::from_millis(2000));
                    AppCommandMsg::InitiateRefresh
                });
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        if PROFILE != "Screenshot" {
            settings.set_int("window-width", width)?;
            settings.set_int("window-height", height)?;

            settings.set_boolean("is-maximized", self.main_window.is_maximized())?;
        }
        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        if PROFILE == "Screenshot" {
            self.main_window.set_default_size(778, 478);
            return;
        }

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
