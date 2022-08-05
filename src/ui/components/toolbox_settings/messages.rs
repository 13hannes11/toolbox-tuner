use crate::toolbx::ToolbxContainer;

use super::model::ToolboxPreferences;

pub enum ToolboxSettingsDialogMsg {
    Show(ToolbxContainer),
    ReplyToolbxSettings(ToolboxPreferences),
    Close,
}
