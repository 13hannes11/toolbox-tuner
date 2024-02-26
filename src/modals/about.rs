use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutWindow;
    type Input = ();
    type Output = ();
    type Root = adw::AboutWindow;

    fn init_root() -> Self::Root {
        adw::AboutWindow::builder()
            .application_icon(APP_ID)
            // Insert your license of choice here
            .license_type(gtk::License::Lgpl30)
            // Insert your website here
            .website("https://github.com/13hannes11/toolbox-tuner")
            // Insert your Issues page
            .issue_url("https://github.com/13hannes11/toolbox-tuner/issues")
            // Insert your application name here
            .application_name("Toolbox Tuner")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2022-2024 Hannes Kuchelmeister")
            .developers(vec!["Hannes Kuchelmeister"])
            .designers(vec!["Hannes Kuchelmeister"])
            .build()
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();
        widgets.set_hide_on_close(true);
        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}

