use crate::errors::Result;
use services::cmd;

#[tauri::command(async)]
pub fn cmd_close_app(name: &str) -> Result<()> {
    Ok(cmd::cmd_close_app(name)?)
}

#[tauri::command(async)]
pub fn cmd_run_app(file: &str) -> Result<()> {
    Ok(cmd::cmd_run_app(file)?)
}

#[tauri::command(async)]
pub fn cmd_open_url(url: &str) -> Result<()> {
    Ok(cmd::cmd_open_url(url)?)
}

#[tauri::command(async)]
pub fn cmd_open_folder(path: &str) -> Result<()> {
    Ok(cmd::cmd_open_folder(path)?)
}