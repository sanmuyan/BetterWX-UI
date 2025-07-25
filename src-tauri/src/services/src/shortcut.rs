use crate::errors::Result;
use utils::shortcut::ShortCutArgs;
use utils::shortcut::to_desktop;
use utils::shortcut::to_startup;

pub fn shortcut_to_desktop(
    file: Option<&str>,
    name: Option<&str>,
    icon: Option<&str>,
    args: Option<ShortCutArgs>,
) -> Result<()> {
    Ok(to_desktop(file, name, icon, args)?)
}

pub fn shortcut_to_startup(
    file: Option<&str>,
    name: Option<&str>,
    icon: Option<&str>,
    args: Option<ShortCutArgs>,
) -> Result<()> {
    Ok(to_startup(file, name, icon, args)?)
}
