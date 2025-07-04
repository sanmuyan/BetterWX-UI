use super::file::is_file_exists;
#[warn(dead_code)]
use anyhow::{anyhow, Result};
use log::debug;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::{thread, time};
use uiautomation::{UIAutomation, UIElement, UITreeWalker};
use windows::core::PCWSTR;
use windows::core::{BOOL, PWSTR};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::ERROR_ALREADY_EXISTS;
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND, LPARAM, MAX_PATH, RECT, WPARAM};
use windows::Win32::Security::{
    DuplicateTokenEx, SecurityImpersonation, TokenPrimary, TOKEN_ALL_ACCESS, TOKEN_DUPLICATE,
    TOKEN_QUERY,
};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::System::Threading::TerminateProcess;
use windows::Win32::System::Threading::PROCESS_TERMINATE;
use windows::Win32::System::Threading::{
    CreateProcessW, CreateProcessWithTokenW, OpenProcess, OpenProcessToken, CREATE_NO_WINDOW,
    CREATE_PROCESS_LOGON_FLAGS, PROCESS_ALL_ACCESS, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION,
    PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, STARTF_USESHOWWINDOW, STARTF_USESTDHANDLES,
    STARTUPINFOW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetSystemMetrics, GetWindowRect, GetWindowTextW,
    GetWindowThreadProcessId, IsWindowVisible, PostMessageW, SetWindowPos, HWND_TOPMOST,
    SM_CXSCREEN, SM_CYSCREEN, SWP_NOSIZE, SWP_SHOWWINDOW, SW_HIDE, WM_KEYDOWN, WM_KEYUP,
};
use windows::Win32::UI::WindowsAndMessaging::{GetShellWindow, WM_LBUTTONDOWN, WM_LBUTTONUP};

pub fn get_runtime_path() -> Result<String> {
    let path = std::env::current_exe()
        .map_err(|_| anyhow!("获取运行程序所在目录失败"))?
        .parent()
        .ok_or_else(|| anyhow!("获取运行程序所在目录失败"))?
        .to_path_buf();
    Ok(path.to_string_lossy().into_owned())
}

pub fn get_runtime_file() -> Result<String> {
    let path = std::env::current_exe()
        .map_err(|_| anyhow!("获取程序路径失败"))?
        .to_path_buf();
    Ok(path.to_string_lossy().into_owned())
}

/**
 * @description: 运行应用程序
 */
pub fn run_app(file: &str) -> Result<()> {
    if !is_file_exists(file) {
        return Err(anyhow!("应用程序不存在: {}", file));
    }
    Command::new(&file)
        .spawn()
        .map_err(|e| anyhow!("运行应用程序失败: {}", e))?;
    Ok(())
}

/**
 * @description: 打开URL
 */
pub fn open_url(url: &str) -> Result<()> {
    Command::new("cmd.exe")
        .creation_flags(0x08000000)
        .arg("/C")
        .arg("start")
        .arg(&url)
        .spawn()
        .map_err(|e| anyhow!("打开网址失败: {}", e))?;
    Ok(())
}

/**
 * @description: 打开文件夹
 */
pub fn open_folder(file: &str) -> Result<()> {
    Command::new("explorer")
        .args([&file])
        .spawn()
        .map_err(|e| anyhow!("打开文件夹失败: {}", e))?;
    Ok(())
}

/**
 * 获取程序所在目录
 */
#[allow(dead_code)]
pub fn get_exe_dir() -> Result<String> {
    let path = std::env::current_exe()?
        .parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "无法获取程序所在目录"))?
        .to_path_buf();
    Ok(path.to_string_lossy().into_owned())
}

#[derive(Debug, Clone)]
pub struct PidPath {
    pub pid: u32,
    pub path: String,
}

struct WindowFinder {
    pids: Vec<u32>,
    hwnds: Vec<(u32, HWND)>,
}

/**
 * 批量启动App
 * @param apps 文件路径
 * @return 窗口句柄列表
 * @throws anyhow::Error 启动失败
 */
pub fn run_apps(paths: &[String], login: &str, close_first: bool) -> Result<()> {
    debug!("计划启动App: {} 个", paths.len());
    let mut all_hwnds = Vec::new();
    if paths.is_empty() {
        return Err(anyhow!("启动失败，没有指定App"));
    }
    for path in paths {
        if path.is_empty() || !is_file_exists(&path) {
            return Err(anyhow!("启动失败，App路径不存在：{}", &path));
        }
    }
    if close_first {
        let _ = close_apps(&paths);
        thread::sleep(time::Duration::from_millis(200));
    }
    if login.len() > 0 {
        let (hwnds, missing) = run_apps_and_check(&paths)?;
        all_hwnds.extend(&hwnds);
        let _ = sort_apps(&all_hwnds);
        thread::sleep(time::Duration::from_millis(200));
        // if auto_login {
        //     let _ = send_enter_to_apps(&all_hwnds);
        // }
        //发送登录
        // let finder = UIFinder::from(login);
        // let _ = &hwnds.iter().for_each(|item| {
        //     debug!("登录UI元素匹配条件:{}   pid {}", finder.to_string(), item.0);
        //     if let Ok(ui) = ui_finder_by_hwnd(item.1, &finder) {
        //         debug!("找到UI元素{}", ui);
        //         let _ = ui.set_focus();
        //         //let _ = ui.send_keys("{Enter}", 10);
        //         let _ = send_enter(item.1);
        //     } else {
        //         debug!("未找到UI元素{}", item.0);
        //     }
        // });
        //发送登录
        let _ = send_mouse_click_to_apps(&all_hwnds,login);
        if missing.is_empty() {
            return Ok(());
        }
        return Err(anyhow!(
            "启动失败，有 {} 个App未启动或已登录",
            missing.len()
        ));
    } else {
        let _ = run_apps_and_notcheck(&paths);
    }
    Ok(())
}

/**
 * 关闭App
 */
pub fn close_apps(paths: &[String]) -> Result<()> {
    for path in paths {
        let exe_name = path.split("\\").last().unwrap_or("");
        if exe_name.is_empty() {
            continue;
        }
        if exe_name.ends_with(".exe") {
            let process = find_pid_by_name_all(exe_name);
            if let Ok(process) = process {
                let pids = process.get_pids();
                pids.iter().for_each(|pid| {
                    let _ = terminate_process_by_pid(*pid);
                });
            }
            //let _ = close_app(exe_name);
        }
    }
    Ok(())
}

pub fn terminate_process_by_pid(pid: u32) -> Result<()> {
    unsafe {
        let h_process = OpenProcess(PROCESS_TERMINATE, false, pid)?;
        TerminateProcess(h_process, 1)?;
        CloseHandle(h_process)?;
        Ok(())
    }
}

/**
 * 关闭App
 */
#[allow(dead_code)]
pub fn close_app(exe_name: &str) -> Result<()> {
    Command::new("cmd.exe")
        .creation_flags(0x08000000)
        .arg("/C")
        .arg(format!("taskkill /F /IM {}", exe_name))
        .spawn()
        .map_err(|e| anyhow!("结束程序失败: {}", e))?;
    Ok(())
}

/**
 * 启动App
 * @param apps 文件路径
 * @return 窗口句柄列表
 * @throws anyhow::Error 启动失败
 */
pub fn run_apps_and_notcheck(paths: &[String]) -> Result<Vec<u32>> {
    let mut process_infos = vec![];
    let mut pids = vec![];
    for path in paths {
        match try_run_as_user(path) {
            Ok(pinfo) => {
                process_infos.push(PidPath {
                    pid: pinfo,
                    path: path.clone(),
                });
                pids.push(pinfo);
            }
            Err(e) => {
                debug!("启动 {} 失败: {}", path, e);
            }
        }
    }
    Ok(pids)
}

/**
 * 启动App并检查
 * @param apps 文件路径
 * @return 窗口句柄列表
 * @throws anyhow::Error 启动失败
 */
pub fn run_apps_and_check(paths: &[String]) -> Result<(Vec<(u32, HWND)>, Vec<String>)> {
    let mut process_infos = vec![];
    let mut pids = vec![];
    for path in paths {
        match try_run_as_user(path) {
            Ok(pinfo) => {
                process_infos.push(PidPath {
                    pid: pinfo,
                    path: path.clone(),
                });
                pids.push(pinfo);
            }
            Err(e) => {
                debug!("启动 {} 失败: {}", path, e);
            }
        }
    }
    thread::sleep(time::Duration::from_millis(500));
    let hwnds = get_hwnds_by_pids(pids)?;
    let missing_apps = check_missing_processes(&process_infos, &hwnds);
    Ok((hwnds, missing_apps))
}

/**
 * 检查未成功创建窗口的进程
 * @param pids 所有尝试启动的进程ID列表
 * @param hwnds 成功获取窗口的(pid, hwnd)列表
 */
pub fn check_missing_processes(process_infos: &[PidPath], hwnds: &[(u32, HWND)]) -> Vec<String> {
    let found_pids: Vec<u32> = hwnds.iter().map(|(pid, _)| *pid).collect();
    let mut apps = vec![];
    process_infos.iter().for_each(|info| {
        if !found_pids.contains(&info.pid) {
            apps.push(info.path.clone());
        }
    });
    apps
}

/**
 * 按行堆叠窗口
 * @param hwnds 窗口句柄列表
 * @throws anyhow::Error 堆叠失败
 */
pub fn sort_apps(hwnds: &[(u32, HWND)]) -> Result<Vec<(HWND,i32,i32)>> {
    let mut  result = Vec::new();
    if hwnds.is_empty() {
        return Err(anyhow!("窗口句柄列表为空"));
    }
    let total = hwnds.len() as i32;
    let (sw, sh) = get_screen_size()?;
    let app_size = get_window_size(hwnds[0].1)?;
    let (mut w, h) = app_size;
    // 计算最大行列数
    let max_col_num = sw / w;
    let max_row_num = sh / h;
    if max_col_num < 1 || max_row_num < 1 {
        return Err(anyhow!("屏幕尺寸不足,无法排列"));
    }
    // 计算起始位置
    let mut row_num = 1;
    // 计算需要使用的行数，totol >= row_num 平方数
    for i in (0..max_row_num).rev() {
        let num = i + 1;
        if total >= num * num {
            row_num = num;
            break;
        }
    }
    //进行堆叠
    if total > row_num * max_col_num {
        // 行最大数量
        let temp_col_num = total / row_num + total % (total / row_num);
        // 超过屏幕部分宽度
        let diff_w = temp_col_num * w - sw;
        // 对窗口叠加
        w = w - diff_w / (temp_col_num - 1);
        debug!(
            "temp_col_num {} diff_w {} 窗口宽度: {},高度: {}",
            temp_col_num, diff_w, w, h
        )
    }
    // 计算列数
    let col_num = total / row_num;
    for (i, (_, hwnd)) in hwnds.iter().enumerate() {
        // 当前行
        let index = i as i32;
        let mut row_index = index / col_num;
        if row_index >= max_row_num {
            row_index = max_row_num - 1;
        }
        // 当前列
        let col_index = index - (row_index * col_num);
        // 当前列 最后一行
        let now_col_num = if row_index >= row_num - 1 {
            row_index = row_num - 1;
            total - (row_num - 1) * col_num
        } else {
            col_num
        };
        let start_x = (sw - (now_col_num - 1) * w - app_size.0) / 2;
        let start_x = if start_x < 0 { 0 } else { start_x };
        let start_y = (sh - row_num * h) / 2;
        let x = start_x + col_index * w;
        let y = start_y + row_index * h;
        // debug!(
        //     "第 {} 个窗口,行: {},列: {},x: {},y: {}",
        //     i, row_index, col_index, x, y
        // );
        set_window_position(*hwnd, x, y, app_size.0, app_size.1)?;
        result.push((*hwnd,x, y));
    }
    Ok(result)
}

pub fn try_run_as_user(exe_path: &str) -> Result<u32> {
    match run_as_user(&exe_path) {
        Ok(hwnd) => return Ok(hwnd),
        Err(_) => return create_process_w(&exe_path, false),
    }
}

pub fn create_process_w(file: &str, hidden: bool) -> Result<u32> {
    let mut file_wide: Vec<u16> = std::ffi::OsStr::new(file)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let file_ptr = Some(unsafe { std::mem::transmute(file_wide.as_mut_ptr()) });
    let mut startup_info = STARTUPINFOW::default();
    if hidden {
        startup_info.dwFlags = STARTF_USESHOWWINDOW;
        startup_info.wShowWindow = SW_HIDE.0 as u16;
    }
    let creation_flags = if hidden {
        CREATE_NO_WINDOW
    } else {
        PROCESS_CREATION_FLAGS(0)
    };
    let mut process_info = PROCESS_INFORMATION::default();
    unsafe {
        //尝试设置父进程为explorer.exe
        let pid = get_explorer_pid();
        if let Ok(pid) = pid {
            let parent_handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid);
            if let Ok(parent_handle) = parent_handle {
                startup_info.hStdInput = parent_handle;
                startup_info.hStdOutput = parent_handle;
                startup_info.hStdError = parent_handle;
                startup_info.dwFlags |= STARTF_USESTDHANDLES;
            }
        }
        CreateProcessW(
            None,
            file_ptr,
            None,
            None,
            false,
            creation_flags,
            None,
            None,
            &startup_info,
            &mut process_info,
        )
        .map_err(|e| anyhow!("启动App失败，{}", e))?;
        delayed_close_handles(
            process_info.hThread.0 as _,
            process_info.hProcess.0 as _,
            2000,
        );
    }
    Ok(process_info.dwProcessId)
}

pub fn run_as_user(exe_path: &str) -> Result<u32> {
    let pid = get_explorer_pid().map_err(|_| anyhow!("降权运行失败"))?;
    let explorer_process = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, pid)? };
    let mut explorer_token = HANDLE::default();
    unsafe {
        OpenProcessToken(
            explorer_process,
            TOKEN_DUPLICATE | TOKEN_QUERY,
            &mut explorer_token,
        )?
    };
    let mut new_token = HANDLE::default();
    unsafe {
        DuplicateTokenEx(
            explorer_token,
            TOKEN_ALL_ACCESS,
            None,
            SecurityImpersonation,
            TokenPrimary,
            &mut new_token,
        )?
    };
    let mut startup_info = STARTUPINFOW::default();
    startup_info.hStdInput = explorer_process;
    startup_info.hStdOutput = explorer_process;
    startup_info.hStdError = explorer_process;
    startup_info.dwFlags |= STARTF_USESTDHANDLES;
    let mut process_info = PROCESS_INFORMATION::default();
    let mut exe_wide: Vec<u16> = std::ffi::OsStr::new(exe_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let exe_ptr = PWSTR::from_raw(exe_wide.as_mut_ptr());
    unsafe {
        CreateProcessWithTokenW(
            new_token,
            CREATE_PROCESS_LOGON_FLAGS(0),
            None,
            Some(exe_ptr),
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &startup_info,
            &mut process_info,
        )?;
        let thread_handle = process_info.hThread;
        let process_handle = process_info.hProcess;
        delayed_close_handles(thread_handle.0 as _, process_handle.0 as _, 2000);
        let _ = CloseHandle(new_token);
        let _ = CloseHandle(explorer_token);
        let _ = CloseHandle(explorer_process);
    }
    Ok(process_info.dwProcessId as u32)
}

/**
 * 获取窗口句柄
 * @param pid 进程ID
 * @return 窗口句柄
 * @throws anyhow::Error 未找到窗口
 */
pub fn get_hwnds_by_pids(pids: Vec<u32>) -> Result<Vec<(u32, HWND)>> {
    let mut finder = WindowFinder {
        pids,
        hwnds: Vec::new(),
    };
    let lparam = LPARAM(&mut finder as *mut WindowFinder as isize);
    unsafe {
        for _ in 0..20 {
            let _ = EnumWindows(Some(enum_windows_proc), lparam);
            if finder.hwnds.len() == finder.pids.len() {
                break;
            }
            thread::sleep(time::Duration::from_millis(500));
        }
    };
    if finder.hwnds.is_empty() {
        return Err(anyhow!("启动失败，未找到窗口"));
    }
    Ok(finder.hwnds)
}

/**
 * 枚举窗口回调函数
 * @param hwnd 窗口句柄
 * @param lparam 自定义参数
 * @return 是否继续枚举
 */
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let finder = &mut *(lparam.0 as *mut WindowFinder);
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        let hasnot_hwnd = finder.hwnds.iter().find(|(_, h)| h == &hwnd).is_none();
        if hasnot_hwnd && finder.pids.contains(&pid) && IsWindowVisible(hwnd).as_bool() {
            //检查窗口是否有标题（非空）
            let mut title = [0u16; 256];
            let len = GetWindowTextW(hwnd, &mut title);
            // 检查窗口尺寸（可选）
            let has_rect = get_window_size(hwnd).is_ok();
            if len > 0 && has_rect {
                finder.hwnds.push((pid, hwnd));
            }
        }
    }
    BOOL::from(true)
}

#[derive(Debug)]
pub struct ProcessInfos(pub Vec<ProcessInfo>);
impl ProcessInfos {
    #[allow(dead_code)]
    pub fn retain_process_info(
        &self,
        class_name: Option<&str>,
        window_name: Option<&str>,
    ) -> ProcessInfos {
        let p = self
            .0
            .iter()
            .filter(|p| {
                p.class_name.contains(class_name.unwrap_or(""))
                    && p.window_name.contains(window_name.unwrap_or(""))
            })
            .cloned()
            .collect::<Vec<_>>();
        Self(p)
    }
    pub fn first(&self) -> Option<Box<&ProcessInfo>> {
        if self.0.is_empty() {
            return None;
        }
        Some(Box::new(&self.0[0]))
    }

    pub fn get_pids(&self) -> Vec<u32> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        self.0
            .iter()
            .map(|p| p.pid)
            .filter(|&pid| set.insert(pid))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub hwnd: u32,
    pub class_name: String,
    pub window_name: String,
}

struct WindowFinder2 {
    process_name: String,
    process_infos: ProcessInfos,
    all: bool,
}

pub fn get_explorer_pid() -> Result<u32> {
    let shell_window = unsafe { GetShellWindow() };
    if !shell_window.is_invalid() {
        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(shell_window, Some(&mut pid)) };
        if pid != 0 {
            return Ok(pid);
        }
    }
    Err(anyhow!("获取explorer_pid失败"))
}

pub fn find_pid_by_name(process_name: &str) -> Result<ProcessInfos> {
    find_pids_by_name(process_name, false)
}

pub fn find_pid_by_name_all(process_name: &str) -> Result<ProcessInfos> {
    find_pids_by_name(process_name, true)
}

fn find_pids_by_name(process_name: &str, all: bool) -> Result<ProcessInfos> {
    let mut finder = WindowFinder2 {
        process_name: process_name.to_string(),
        process_infos: ProcessInfos(Vec::new()),
        all,
    };
    let lparam = LPARAM(&mut finder as *mut WindowFinder2 as isize);
    unsafe {
        let _ = EnumWindows(Some(enum_windows_callback), lparam);
    }
    if finder.process_infos.0.is_empty() {
        return Err(anyhow!("未找到进程: {}", process_name));
    }
    Ok(finder.process_infos)
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // 获取窗口大小
    let mut rect = RECT::default();
    unsafe {
        let _ = GetWindowRect(hwnd, &mut rect);
    };
    let finder = unsafe { &mut *(lparam.0 as *mut WindowFinder2) };
    let mut window_pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut window_pid)) };
    if window_pid != 0 {
        let process = unsafe {
            OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                false,
                window_pid,
            )
            .ok()
        };
        if let Some(process) = process {
            let mut module_name = [0u16; MAX_PATH as usize];
            if unsafe { GetModuleFileNameExW(Some(process), None, &mut module_name) } != 0 {
                let module_path = String::from_utf16_lossy(&module_name);
                let exe_name = Path::new(&module_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if exe_name
                    .to_lowercase()
                    .contains(&finder.process_name.to_lowercase())
                {
                    // 获取窗口类名
                    let mut class_buf = [0u16; 256];
                    let len = unsafe { GetClassNameW(hwnd, &mut class_buf) };
                    let class_name = if len == 0 {
                        String::new()
                    } else {
                        String::from_utf16_lossy(&class_buf[..len as usize])
                    };
                    // 获取窗口标题
                    let mut title_buf = [0u16; 256];
                    let len = unsafe { GetWindowTextW(hwnd, &mut title_buf) };
                    let window_name = if len == 0 {
                        String::new()
                    } else {
                        String::from_utf16_lossy(&title_buf[..len as usize])
                    };
                    let hwnd_u32 = hwnd.0 as u32;
                    if !finder
                        .process_infos
                        .0
                        .iter()
                        .find(|p| p.hwnd == hwnd_u32)
                        .is_some()
                    {
                        let process_info = ProcessInfo {
                            pid: window_pid,
                            hwnd: hwnd_u32,
                            class_name,
                            window_name,
                        };
                        finder.process_infos.0.push(process_info);
                    }
                    if !finder.all {
                        let _ = unsafe { CloseHandle(process) };
                        return BOOL(0); // 找到匹配进程，停止枚举
                    }
                }
            }
            let _ = unsafe { CloseHandle(process) };
        }
    }
    BOOL(1) // 继续枚举
}

/**
 * 设置窗口位置和大小
 * @param hwnd 窗口句柄
 * @param x 窗口左上角x坐标
 * @param y 窗口左上角y坐标
 * @param width 窗口宽度
 * @param height 窗口高度
 * @param repaint 是否重绘窗口
 * @return Result<()> 操作结果
 */
pub fn set_window_position(hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
    unsafe {
        SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            x,
            y,
            width,
            height,
            SWP_SHOWWINDOW | SWP_NOSIZE,
        )
        .map_err(|e| anyhow!("设置窗口位置失败: {}", e))?;
    }
    Ok(())
}

pub fn delayed_close_handles(h_thread: isize, h_process: isize, delay_ms: u64) {
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(delay_ms));
        unsafe {
            let _ = CloseHandle(HANDLE(h_thread as _));
            let _ = CloseHandle(HANDLE(h_process as _));
        }
    });
}

pub fn create_mutex_w(name: &str) -> bool {
    let mut mutex_name_wide: Vec<u16> = std::ffi::OsStr::new(name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mutex_name_ptr = PCWSTR::from_raw(mutex_name_wide.as_mut_ptr());
    unsafe {
        let a = CreateMutexW(None, true, mutex_name_ptr);
        match a {
            Ok(_) => GetLastError() == ERROR_ALREADY_EXISTS,
            Err(_) => false,
        }
    }
}

// ==================== 互斥体操作 ====================
#[allow(dead_code)]
pub fn check_mutex(name: &str) -> bool {
    let mut mutex_name_wide: Vec<u16> = std::ffi::OsStr::new(name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mutex_name_ptr = PCWSTR::from_raw(mutex_name_wide.as_mut_ptr());
    unsafe {
        let a = CreateMutexW(None, false, mutex_name_ptr);
        match a {
            Ok(handle) => {
                let is_exists = GetLastError() == ERROR_ALREADY_EXISTS;
                let _ = CloseHandle(handle);
                is_exists
            }
            Err(_) => false,
        }
    }
}

// ==================== 窗口UI ====================

/**
 * 获取窗口大小
 * @param hwnd 窗口句柄
 * @return 窗口大小
 * @throws anyhow::Error 获取窗口大小失败
 */
pub fn get_window_size(hwnd: HWND) -> Result<(i32, i32)> {
    let mut rect = RECT::default();
    unsafe {
        GetWindowRect(hwnd, &mut rect).map_err(|e| anyhow!("获取窗口大小失败: {}", e))?;
    }
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    Ok((width, height))
}

/**
 * 获取主显示器屏幕尺寸
 * @return (宽度, 高度)
 * @throws anyhow::Error 获取失败
 */
pub fn get_screen_size() -> Result<(i32, i32)> {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        if width == 0 || height == 0 {
            return Err(anyhow!("获取屏幕尺寸失败"));
        }
        Ok((width, height))
    }
}

/**
 * 向窗口发送回车键消息
 * @param hwnds 目标窗口句柄列表
 * @return Result<()> 操作结果
 */
pub fn send_enter_to_apps(hwnds: &[(u32, HWND)]) -> Result<()> {
    for (_, hwnd) in hwnds {
        let _ = send_enter(*hwnd);
    }
    Ok(())
}

/**
 * 向窗口发送回车键消息
 * @param hwnd 目标窗口句柄
 * @return Result<()> 操作结果
 */
pub fn send_enter( hwnd:HWND) -> Result<()> {
    unsafe {
        PostMessageW(Some(hwnd), WM_KEYDOWN, WPARAM(13), LPARAM(0))
            .map_err(|e| anyhow!("发送回车键按下消息失败: {}", e))?;
        thread::sleep(time::Duration::from_millis(50));
        PostMessageW(Some(hwnd), WM_KEYUP, WPARAM(13), LPARAM(0))
            .map_err(|e| anyhow!("发送回车键释放消息失败: {}", e))?;
    }
    Ok(())
}

/**
 * 向窗口列表发送鼠标左键单击消息
 * @param hwnds 目标窗口句柄列表
 * @return Result<()> 操作结果
 */
pub fn send_mouse_click_to_apps(hwnds: &[(u32, HWND)],pos:&str) -> Result<()> {
    if !hwnds.is_empty(){
    let rect = get_window_size(hwnds[0].1)?;
    let pos_vec:Vec<&str> = pos.split(",").collect();
    let w = pos_vec[0].parse::<f64>()?;
    let h = pos_vec[1].parse::<f64>()?;
    let x = pos_vec[2].parse::<f64>()?;
    let y = pos_vec[3].parse::<f64>()?;
    let real_x = rect.0  as f64  / w * x;
    let real_y = rect.1  as f64  /h * y;
    debug!("获取窗口大小:{:?}  输入窗体：{}，{} 点击位置：{}，{} 输入坐标:{},{}",rect,w,h,real_x,real_y,x,y);
    for (_, hwnd) in hwnds {
        let _ = send_mouse_click(*hwnd, real_x  as i32, real_y as i32);
    }
    }
    Ok(())
}

/**
 * 向窗口发送鼠标左键单击消息(带坐标)
 * @param hwnd 目标窗口句柄
 * @param x 点击位置的x坐标(相对于窗口)
 * @param y 点击位置的y坐标(相对于窗口)
 * @return Result<()> 操作结果
 */
pub fn send_mouse_click(hwnd: HWND, x: i32, y: i32) -> Result<()> {
    unsafe {
        // 将坐标转换为LPARAM格式 (y << 16 | x)
        let lparam = LPARAM(((y << 16) | x) as isize);

        // 发送鼠标左键按下消息
        PostMessageW(Some(hwnd), WM_LBUTTONDOWN, WPARAM(1), lparam)
            .map_err(|e| anyhow!("发送鼠标左键按下消息失败: {}", e))?;

        thread::sleep(time::Duration::from_millis(50));

        // 发送鼠标左键释放消息
        PostMessageW(Some(hwnd), WM_LBUTTONUP, WPARAM(0), lparam)
            .map_err(|e| anyhow!("发送鼠标左键释放消息失败: {}", e))?;
    }
    Ok(())
}

pub struct UIFinder {
    name: Option<String>,
    class_name: Option<String>,
    control_type: Option<String>,
}

impl UIFinder {
    pub fn new(name: Option<&str>, class_name: Option<&str>, control_type: Option<&str>) -> Self {
        Self {
            name: name.map(|e| e.to_string()),
            class_name: class_name.map(|e| e.to_string()),
            control_type: control_type.map(|e| e.to_string()),
        }
    }
    pub fn matched(&self, element: &UIElement) -> bool {
        if let Some(name) = &self.name {
            if element
                .get_name()
                .map(|e_name| !e_name.contains(name))
                .unwrap_or(true)
            {
                return false;
            }
        }
        if let Some(class_name) = &self.class_name {
            if element
                .get_classname()
                .map(|e_class| !e_class.contains(class_name))
                .unwrap_or(true)
            {
                return false;
            }
        }
        if let Some(control_type) = &self.control_type {
            if element
                .get_control_type()
                .map(|e_type| !e_type.to_string().contains(control_type))
                .unwrap_or(true)
            {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(name) = &self.name {
            result.push_str(&format!("name={},", name));
        }
        if let Some(class_name) = &self.class_name {
            result.push_str(&format!("class_name={},", class_name));
        }
        if let Some(control_type) = &self.control_type {
            result.push_str(&format!("control_type={}", control_type));
        }
        result
    }
}

impl From<&str> for UIFinder {
    fn from(value: &str) -> Self {
        let values = value.split(",");
        let mut finder = Self::new(None, None, None);
        for v in values {
            let mut kv = v.split("=");
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();
            match key {
                "name" => finder.name = Some(value.to_string()),
                "class_name" => finder.class_name = Some(value.to_string()),
                "control_type" => finder.control_type = Some(value.to_string()),
                _ => {}
            }
        }
        finder
    }
}

pub fn ui_finder_by_hwnd(hwnd: HWND, finder: &UIFinder) -> Result<UIElement> {
    let automation = UIAutomation::new()?;
    let root = automation.element_from_handle(hwnd.into())?;
    let walker = automation.get_control_view_walker()?;
    if let Some(ui) = walker_element(&walker, &root, 0, finder) {
        return Ok(ui);
    }
    Err(anyhow!("未找到匹配的UI元素"))
}

pub fn ui_finder_by_pid(pid: u32, finder: &UIFinder) -> Result<UIElement> {
    let automation = UIAutomation::new()?;
    let walker = automation.get_control_view_walker()?;
    let root = automation.get_root_element()?;
    let matcher = automation.create_matcher().from(root).process_id(pid);
    if let Ok(ui) = matcher.find_first() {
        if let Some(ui) = walker_element(&walker, &ui, 0, finder) {
            return Ok(ui);
        }
    }
    Err(anyhow!("未找到匹配的UI元素"))
}

fn walker_element(
    walker: &UITreeWalker,
    element: &UIElement,
    level: usize,
    finder: &UIFinder,
) -> Option<UIElement> {
    // 首先检查当前元素是否匹配
    if finder.matched(element) {
        return Some(element.clone());
    }
    // 递归检查子元素
    if let Ok(child) = walker.get_first_child(element) {
        // 检查第一个子元素
        if let Some(found) = walker_element(walker, &child, level + 1, finder) {
            return Some(found);
        }
        // 检查兄弟元素
        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            if let Some(found) = walker_element(walker, &sibling, level + 1, finder) {
                return Some(found);
            }
            next = sibling;
        }
    }
    None
}
