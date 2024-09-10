use crate::gio;
use crate::util::terminal::TerminalType;
use crate::APP_ID;
use relm4::gtk::prelude::SettingsExt;

pub fn get_terminal() -> TerminalType {
    let settings = gio::Settings::new(APP_ID);
    let terminal = settings.string("terminal");
    terminal.as_str().try_into().unwrap_or_default()
}

pub fn set_terminal(terminal: TerminalType) {
    let settings = gio::Settings::new(APP_ID);
    let terminal: String = terminal.into();
    settings.set_string("terminal", &terminal);
}
