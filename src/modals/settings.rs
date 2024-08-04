use crate::util::prerequisit::get_installed_terminals;
use relm4::adw::prelude::{
    ComboRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt, PreferencesWindowExt,
};
use relm4::gtk::prelude::WidgetExt;
use relm4::gtk::prelude::GtkWindowExt;
use relm4::gtk::prelude::ListModelExt;
use relm4::view;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};
pub struct SettingsDialog {
    hidden: bool,
    terminal: String,
}
use crate::APP_ID;
use gtk::gio;
use relm4::gtk::prelude::CastNone;
use relm4::gtk::prelude::SettingsExt;

#[derive(Debug)]
pub enum SettingsMsg {
    OpenSettings,
    TerminalSelectionChanged(String),
}

impl SimpleComponent for SettingsDialog {
    type Init = ();
    type Widgets = adw::PreferencesWindow;
    type Input = SettingsMsg;
    type Output = ();
    type Root = adw::PreferencesWindow;

    fn init_root() -> Self::Root {
        adw::PreferencesWindow::new()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            hidden: true,
            terminal: "".to_string(),
        };
        let terminal_selection_model = gtk::gio::ListStore::new::<gtk::StringObject>();

        let sort_function = |to_insert: &gtk::glib::Object,
                             existing: &gtk::glib::Object|
         -> std::cmp::Ordering { to_insert.cmp(existing) };

        let terminals = get_installed_terminals().unwrap_or_default();

        terminals.iter().for_each(|t| {
            terminal_selection_model.insert_sorted(
                &gtk::StringObject::new(format!("{:?}", t).as_str()),
                &sort_function,
            );
        });

        let settings = gio::Settings::new(APP_ID);
        let terminal = settings.string("terminal");

        let terminal_selection = terminal_selection_model
            .find_with_equal_func(|obj| obj == &gtk::StringObject::new(terminal.as_str()))
            .unwrap_or(0);

        let str_object: String = terminal_selection_model
            .item(terminal_selection)
            .and_downcast::<gtk::StringObject>()
            .expect("The item has to be an `StringObject`.")
            .into();
        sender.input(SettingsMsg::TerminalSelectionChanged(str_object));

        view! {
            widgets = root.clone() {

                add = &adw::PreferencesPage::new() {
                    add = &adw::PreferencesGroup::new() {
                    set_title: "Terminal settings",
                        add = &adw::ComboRow::new() {
                            set_title: "Terminal",
                            #[wrap(Some)]
                            set_model = &gtk::SingleSelection::new(Some(terminal_selection_model)),
                            #[watch]
                            set_selected: terminal_selection,
                            connect_selected_item_notify[sender] => move |combo_row| {
                                let str_object: String = combo_row.selected_item().and_downcast::<gtk::StringObject>().expect("The item has to be an `StringObject`.").into();
                                sender.input(SettingsMsg::TerminalSelectionChanged(str_object));
                            },
                        },
                    },
                },
            }
        }
        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.show();
    }
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            SettingsMsg::OpenSettings => {    }
            SettingsMsg::TerminalSelectionChanged(terminal) => {
                let settings = gio::Settings::new(APP_ID);
                settings.set_string("terminal", &terminal);
            }
        }
    }
}
