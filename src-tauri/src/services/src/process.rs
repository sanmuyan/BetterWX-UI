use crate::errors::Result;
use crate::errors::ServicesError;
use log::debug;
use std::path::PathBuf;
use utils::file;
use utils::file_pid_hwnd::FilePid;
use utils::file_pid_hwnd::FilesPid;
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
    let closed = process_close_apps(&paths)?;
    debug!("关闭程序成功: {:?}", closed);
    if closed {
        sleep(1000);
    }
    let mut pids = try_run_apps(paths)?;
    if let Some(login) = login {
        sort_and_click(&mut pids, login).map_err(|e| {
            debug!("一键启动失败，请重试。{:?}", e);
            e
        })?;
    }
    debug!("批量运行程序成功: {:?}", pids);
    Ok(())
}

fn try_run_apps(paths: &[String]) -> Result<FilesPid> {
    let mut last_error = None;
    let mut success_pids = FilesPid::new();
    let mut run_paths = paths.to_vec();
    for i in 0..3 {
        let pids = run_apps(&run_paths);
        match pids {
            Ok(pids) => {
                success_pids.extend(pids);
                run_paths = filter_run_failed(paths, &success_pids);
                if run_paths.is_empty() {
                    break;
                }
                debug!("第 {} 次 try_run_apps，运行失败的: {:?}", i + 1, run_paths);
            }
            Err(e) => {
                debug!("第 {} 次 try_run_apps，运行出错: {:?}", i + 1, e);
                last_error = Some(e);
            }
        }
        if i == 2 {
            break;
        }
    }
    return if success_pids.len() == paths.len() {
        Ok(success_pids)
    } else {
        let last_error = match last_error {
            Some(e) => e.to_string(),
            None => ServicesError::UnkonwError.to_string(),
        };
        Err(ServicesError::RunAppError(
            paths.len(),
            success_pids.len(),
            last_error,
        ))
    };
}

fn run_apps(paths: &[String]) -> Result<FilesPid> {
    for path in paths {
        process_run_app(path)?;
    }
    let pids = try_get_pids_by_paths(paths);
    return Ok(pids);
}

fn filter_run_failed(paths: &[String], pids: &FilesPid) -> Vec<String> {
    let mut fialed_paths = Vec::new();
    for path in paths {
        let f = pids.0.iter().find(|x| &x.file == path).is_none();
        if f {
            fialed_paths.push(path.to_string());
        }
    }
    debug!("启动失败的程序，再次尝运行: {:?}", fialed_paths);
    return fialed_paths;
}

fn sort_and_click(fpids: &FilesPid, pos: &str) -> Result<()> {
    process::sort_apps(fpids).map_err(|e| {
        debug!("排列窗口失败: {:?}", e);
        ServicesError::ArrangeWindowError
    })?;
    sleep(1000);
    send_mouse_click_to_apps(fpids, pos).map_err(|e| {
        debug!("发送点击事件失败: {:?}", e);
        ServicesError::SendClickEventError
    })
}

fn send_mouse_click_to_apps(fpids: &FilesPid, pos: &str) -> Result<()> {
    let pos = pos.split(",").collect::<Vec<_>>();
    if pos.len() != 4 {
        return Err(ServicesError::InvalidShortcutError);
    }
    let w = pos[0].trim().parse::<i32>().unwrap_or(0);
    let h = pos[1].trim().parse::<i32>().unwrap_or(0);
    let x = pos[2].trim().parse::<i32>().unwrap_or(0);
    let y = pos[3].trim().parse::<i32>().unwrap_or(0);
    if w <= 0 || h <= 0 || x <= 0 || y <= 0{
        return Err(ServicesError::InvalidShortcutError);
    }
    debug!("发送点击事件坐标: {:?}，初始窗口尺寸: {:?}", (x, y), (w, h));
    process::send_mouse_click_to_apps_use_scale(fpids,w,h, x, y)?;
    Ok(())
}
fn try_get_pids_by_paths(files: &[String]) -> FilesPid {
    for i in 0..10 {
        debug!("第 {} 次，try_get_pids_by_paths 尝试获取程序进程ID", i + 1);
        let fpids = get_pids_by_paths(&files);
        if fpids.len() == files.len() || i == 9 {
            return fpids;
        }
        sleep(500);
    }
    return FilesPid(vec![]);
}

fn get_pids_by_paths(files: &[String]) -> FilesPid {
    let mut fpids = FilesPid::new();
    for file in files {
        if let Ok(p) = get_pid_by_path(&file) {
            fpids.insert(p);
        }
    }
    return fpids;
}

fn get_pid_by_path(file: &str) -> Result<FilePid> {
    let file_name = file::get_file_name(file)?;
    if let Ok(pids) = Pid::find_all_by_process_name(&file_name)
        && !pids.is_empty()
    {
        let hwnd = Hwnd::from(pids[0]);
        if !hwnd.is_invalid() {
            return Ok(FilePid {
                file: file.to_string(),
                pid: pids[0],
                hwnd,
            });
        }
    }
    return Err(ServicesError::UnkonwError);
}

pub fn process_run_app(file: &str) -> Result<()> {
    file::check_file_exists(file)?;
    if let Ok(_) = Process::try_create_as_user(file) {
        return Ok(());
    }
    if let Ok(_) = process::run_app_by_cmd(file) {
        return Ok(());
    } else {
        return Err(ServicesError::RunAppFailed(file.to_string()).into());
    }
}

pub fn process_close_apps(files: &[String]) -> Result<bool> {
    let mut closed = Vec::new();
    for file in files {
        let file_name = file::get_file_name(file)?;
        closed.push(process_close_app(&file_name)?);
    }
    Ok(closed.iter().any(|x| *x))
}

pub fn process_close_app(file_name: &str) -> Result<bool> {
    let _ = match process::close_app_by_pid(file_name) {
        Ok(closed) => {
            return Ok(closed);
        }
        Err(_) => process::close_app_by_cmd(file_name, 0),
    };
    Ok(true)
}
