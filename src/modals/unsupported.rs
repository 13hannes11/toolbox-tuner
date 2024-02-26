use adw::StatusPage;
use gtk::prelude::GtkWindowExt;
use relm4::view;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};
use relm4_icons::icon_name;
pub struct UnsupportedDialog {}

impl SimpleComponent for UnsupportedDialog {
    type Init = ();
    type Widgets = adw::Window;
    type Input = ();
    type Output = ();
    type Root = adw::Window;

    fn init_root() -> Self::Root {
        adw::Window::builder().build()
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        view! {
                    widgets = root.clone() {
                        set_hide_on_close: true,
                        set_modal: true,
                        set_resizable: false,
                        set_default_width: 400,

                        StatusPage::new() {
                            set_icon_name: Some(icon_name::ISSUE),
                            set_title: "Missing requirements",
                            set_description: Some("Make sure Toolbx and Gnome Terminal are installed."),
                            set_child: Some(&gtk::Button::with_label("Goodbye!")),
                        },
                }
        }
        // TODO: when dialgue closes close application
        // TODO: close application on button press
        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}
