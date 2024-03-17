use adw::StatusPage;
use gtk::prelude::{ButtonExt, GtkWindowExt};
use relm4::view;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};
use relm4_icons::icon_names;

pub struct UnsupportedDialog {}

#[derive(Debug)]
pub enum UnsupportedDialogOutput {
    CloseApplication,
}

impl SimpleComponent for UnsupportedDialog {
    type Init = ();
    type Widgets = adw::Window;
    type Input = ();
    type Output = UnsupportedDialogOutput;
    type Root = adw::Window;

    fn init_root() -> Self::Root {
        adw::Window::builder().build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        view! {
                    widgets = root.clone() {
                        set_hide_on_close: true,
                        set_modal: true,
                        set_resizable: false,
                        set_default_width: 400,


                        StatusPage::new() {
                            set_icon_name: Some(icon_names::ISSUE),
                            set_title: "Missing requirements",
                            set_description: Some("Make sure Toolbox and Gnome Terminal are installed."),

                            #[name = "btn_close"]
                            gtk::Button::with_label("Goodbye!") {
                                connect_clicked[sender] => move |_| {
                                    sender.output(UnsupportedDialogOutput::CloseApplication).unwrap()
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
