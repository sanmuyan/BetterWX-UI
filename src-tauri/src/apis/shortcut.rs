use crate::errors::Result;
use services::shortcut;
use utils::shortcut::ShortCutArgs;

#[tauri::command(async)]
pub fn shortcut_to_desktop(
    file: Option<&str>,
    name: Option<&str>,
    icon: Option<&str>,
    args: Option<ShortCutArgs>,
) -> Result<()> {
    Ok(shortcut::shortcut_to_desktop(file, name, icon, args)?)
}

#[tauri::command(async)]
pub fn shortcut_to_startup(
    file: Option<&str>,
    name: Option<&str>,
    icon: Option<&str>,
    args: Option<ShortCutArgs>,
) -> Result<()> {
    Ok(shortcut::shortcut_to_startup(file, name, icon, args)?)
}
