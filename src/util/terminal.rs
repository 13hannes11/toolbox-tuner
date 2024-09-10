use crate::util::toolbox::ToolbxError;
use std::process::Command;

#[derive(Debug, Default)]
pub enum TerminalType {
    #[default]
    GnomeTerminal,
    Konsole,
}

impl From<TerminalType> for String {
    fn from(value: TerminalType) -> Self {
        match value {
            TerminalType::GnomeTerminal => "GnomeTerminal".to_string(),
            TerminalType::Konsole => "Konsole".to_string(),
        }
    }
}

impl TryFrom<&str> for TerminalType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(TerminalType::GnomeTerminal)
    }
}

pub fn get_installed_terminals() -> Result<Vec<TerminalType>, ToolbxError> {
    let output = Command::new("flatpak-spawn")
        .arg("--host")
        .arg("gnome-terminal")
        .arg("--version")
        .output();

    if output.is_err() {
        return Err(ToolbxError::CommandExecutionError(
            output.unwrap_err().to_string(),
        ));
    }
    let output = output.unwrap();

    if output.status.code() == Some(0) {
        Ok(vec![TerminalType::GnomeTerminal, TerminalType::Konsole])
    } else {
        Err(ToolbxError::CommandUnsuccessfulError(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}
