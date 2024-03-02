use relm4::factory::FactoryVecDeque;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender,
    Controller,
};

use gtk::prelude::{
    ApplicationExt, ApplicationWindowExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
};
use gtk::{gio, glib};

use crate::config::{APP_ID, PROFILE};
use crate::factories::container_list::Container;
use crate::modals::about::AboutDialog;
use crate::modals::unsupported::UnsupportedDialog;
use crate::modals::unsupported::UnsupportedDialogOutput;

pub(super) struct App {
    unsupported_dialog: Controller<UnsupportedDialog>,
    about_dialog: Controller<AboutDialog>,
    containers: FactoryVecDeque<Container>,
}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
}

#[derive(Debug)]
pub(super) enum AppCommandMsg {
    PrerequisitsInstalled(bool),
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
//relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

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
                //"_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Toolbox Tuner" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                gtk::Inhibit(true)
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


                #[local_ref]
                container_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                }
            }

        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(root)
            .launch(())
            .detach();

        let unsupported_dialog = UnsupportedDialog::builder()
            .transient_for(root)
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                UnsupportedDialogOutput::CloseApplication => AppMsg::Quit,
            });

        let mut containers = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        containers.guard().push_back(3);

        let model = Self {
            about_dialog,
            unsupported_dialog,
            containers,
        };

        let container_box = model.containers.widget();
        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

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
            // TODO: actually check for compatibility
            AppCommandMsg::PrerequisitsInstalled(true)
        });

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
        _sender: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        match message {
            AppCommandMsg::PrerequisitsInstalled(false) => {
                self.unsupported_dialog.sender().clone().send(()).unwrap()
            }
            AppCommandMsg::PrerequisitsInstalled(true) => {
                // TODO: start process of fetching toolboxes
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

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}

