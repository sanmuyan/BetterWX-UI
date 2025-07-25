use crate::errors::Result;
use crate::errors::ServicesError;
use log::debug;
use std::path::PathBuf;
use utils::file;
use utils::process;
use utils::process::sleep;
use utils::shortcut::ShortCutArgs;
use winsys::process::hwnd::Hwnd;
use winsys::process::pid::Pid;
use winsys::process::process::Process;

pub fn process_run_by_cmd(args: &ShortCutArgs) -> Result<()> {
    let files_name = args.list.split(",").collect::<Vec<_>>();
    let path = PathBuf::from(&args.path);
    let files = files_name
        .iter()
        .map(|x| {
            let name = format!("{}.exe", x.trim_end_matches(".exe"));
            path.join(name.trim()).to_string_lossy().to_string()
        })
        .collect::<Vec<_>>();
    for file in &files {
        file::check_file_exists(file)?;
    }
    process_run_apps(&files, &args.login)?;
    Ok(())
}

pub fn process_run_apps(paths: &[String], login: &Option<String>) -> Result<()> {
    debug!("开始批量运行程序: {:?}", paths);
    if paths.is_empty() {
        return Err(ServicesError::RunAppListIsEmpty.into());
    }
    for path in paths {
        file::check_file_exists(&path)?;
    }
    process_close_apps(&paths)?;
    let mut pids = run_apps(paths)?;
    sleep(2000);
    if pids.len() != paths.len() {
        return Err(ServicesError::RunAppError(paths.len(),pids.len()).into());
    }
    if let Some(login) = login
    {   
        sort_and_click(&mut pids, login)?;
    }
    Ok(())
}

fn run_apps(paths: &[String]) -> Result<Vec<Pid>> {
    let mut pids = Vec::new();
    for path in paths {
        let p = process_run_app(path)?;
        pids.extend(p);
    }
    Ok(pids)
}

fn sort_and_click(pids: &mut Vec<Pid>, login: &str) -> Result<()> {
    process::sort_apps(pids)?;
    sleep(500);
    let hwnds = pids.iter().map(|x| Hwnd::from(*x)).collect::<Vec<_>>();
    send_mouse_click_to_apps(&hwnds, login)
}

fn send_mouse_click_to_apps(hwnds: &[Hwnd], pos: &str) -> Result<()> {
    let pos = pos.split(",").collect::<Vec<_>>();
    let x = pos[0].trim().parse::<i32>().unwrap_or(0);
    let y = pos[1].trim().parse::<i32>().unwrap_or(0);
    process::send_mouse_click_to_apps_use_scale(hwnds, x, y)?;
    Ok(())
}

pub fn process_run_app(file: &str) -> Result<Vec<Pid>> {
    file::check_file_exists(file)?;
    if let Ok(p) = Process::try_create_as_user(file) {
        return Ok(vec![p.get_pid()]);
    }
    if let Ok(_) = process::run_app_by_cmd(file) {
        return Ok(get_pid_by_path(file)?);
    } else {
        return Err(ServicesError::RunAppFailed(file.to_string()).into());
    }
}

fn get_pid_by_path(file: &str) -> Result<Vec<Pid>> {
    let file_name = file::get_file_name(file)?;
    if let Ok(pids) = Pid::find_all_by_process_name(&file_name) {
        return Ok(pids);
    }
    return Ok(vec![]);
}

pub fn process_close_apps(files: &[String]) -> Result<()> {
    for file in files {
        let file_name = file::get_file_name(file)?;
        process_close_app(&file_name,0)?;
    }
    Ok(())
}

pub fn process_close_app(file_name: &str,delay:u64) -> Result<()> {
    if let Err(_) = process::close_app_by_pid(file_name,delay) {
        process::close_app_by_cmd(file_name,delay)?;
    }
    Ok(())
}
