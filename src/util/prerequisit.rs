use crate::util::toolbox::ToolbxError;
use std::process::Command;

pub enum TerminalType {
    GnomeTerminal,
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
        Ok(vec![TerminalType::GnomeTerminal])
    } else {
        Err(ToolbxError::CommandUnsuccessfulError(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

pub fn is_toolbox_installed() -> Result<bool, ToolbxError> {
    let output = Command::new("flatpak-spawn")
        .arg("--host")
        .arg("toolbox")
        .arg("--version")
        .output();

    if output.is_err() {
        return Err(ToolbxError::CommandExecutionError(
            output.unwrap_err().to_string(),
        ));
    }
    let output = output.unwrap();
    if output.status.code() == Some(0) {
        Ok(true)
    } else {
        Err(ToolbxError::CommandUnsuccessfulError(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}
