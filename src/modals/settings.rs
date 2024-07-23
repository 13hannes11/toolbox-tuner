use relm4::adw::prelude::{
    ComboRowExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt, PreferencesWindowExt,
};
use relm4::gtk::prelude::GtkWindowExt;
use relm4::view;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct SettingsDialog {}

impl SimpleComponent for SettingsDialog {
    type Init = ();
    type Widgets = adw::PreferencesWindow;
    type Input = ();
    type Output = ();
    type Root = adw::PreferencesWindow;

    fn init_root() -> Self::Root {
        adw::PreferencesWindow::new()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let terminal_selection_model = gtk::gio::ListStore::new::<gtk::StringObject>();

        let sort_function = |to_insert: &gtk::glib::Object,
                             existing: &gtk::glib::Object|
         -> std::cmp::Ordering { to_insert.cmp(existing) };
        terminal_selection_model.insert_sorted(&gtk::StringObject::new("Test"), &sort_function);
        terminal_selection_model.insert_sorted(&gtk::StringObject::new("Test2"), &sort_function);
        terminal_selection_model.insert_sorted(&gtk::StringObject::new("Test3"), &sort_function);

        let terminal_selection = 2;

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
                        },
                    },
                },
            }
        }
        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}
