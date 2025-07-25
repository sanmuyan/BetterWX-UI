use crate::errors::Result;
use services::process;
use winsys::process::pid::Pid;

#[tauri::command]
pub fn process_run_apps(paths: Vec<String>, login: Option<String>) -> Result<()> {
    Ok(process::process_run_apps(&paths,&login)?)
}

#[tauri::command]
pub fn process_run_app(file: String) -> Result<Vec<Pid>> {
    Ok(process::process_run_app(file.as_str())?)
}

#[tauri::command]
pub fn process_close_apps(files: Vec<String>,delay:u64) -> Result<()> {
    Ok(process::process_close_apps(&files,delay)?)
}

#[tauri::command]
pub fn process_close_app(file_name: String,delay:u64) -> Result<()> {
    Ok(process::process_close_app(file_name.as_str(),delay)?)
}