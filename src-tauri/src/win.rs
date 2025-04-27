use crate::structs::config::regedit::Regedit;

use anyhow::{anyhow, Result};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::{fs, thread, time};
use windows_registry::LOCAL_MACHINE;

use windows::core::{BOOL, HSTRING, PWSTR};
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, RECT, WPARAM};
use windows::Win32::System::Threading::{
    CreateProcessW, CREATE_NEW_CONSOLE, PROCESS_INFORMATION, STARTUPINFOW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetSystemMetrics, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId,
    IsWindowVisible, PostMessageW, SetWindowPos, HWND_TOP, SM_CXSCREEN, SM_CYSCREEN,
    SWP_SHOWWINDOW, WM_KEYDOWN, WM_KEYUP,
};

/**
 * @description: 通过注册表获取安装路径
 * @param regedit 注册表配置
 */
pub fn get_install_variables(regedit: &mut Regedit) -> Result<()> {
    //打开注册表路径
    let key = LOCAL_MACHINE.open(regedit.path.as_str())?;
    regedit.fields.0.iter_mut().try_for_each(|field| {
        //禁用跳过
        field.value = key
            .get_string(&field.value)
            .map_err(|_| return anyhow!("读取注册表字段失败: {}", &field.value))?;
        Ok(())
    })
}

/**
 * @description: 检查文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
pub fn is_file_exists(file: &str) -> bool {
    Path::new(file).exists()
}

/**
 * @description: 检查一组文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
pub fn is_files_exists(files: &Vec<String>) -> bool {
    println!("检查文件是否存在 : {:?}  : {:?}", files, &files.is_empty());
    if files.is_empty() {
        return false;
    }
    let mut result = true;
    for file in files {
        result = result && is_file_exists(&file);
    }
    result
}

/**
 * @description: 检查一组文件是否全部存在
 * @param path 文件路径
 * @return 返回存在的文件路径
 */
pub fn filter_files_is_exists(files: &Vec<String>) -> (bool, Vec<String>) {
    let mut result = Vec::new();
    for file in files {
        if is_file_exists(&file) {
            result.push(file.to_string());
        }
    }
    (files.len() == result.len(), result)
}

/**
 * @description: 删除一组文件
 */
pub fn del_files(files: Vec<String>) -> Result<()> {
    for file in files {
        if !is_file_exists(&file) {
            return Err(anyhow!("应用程序不存在: {}", file));
        }
        fs::remove_file(file)
            .map_err(|_| anyhow!("删除文件失败，文件被占用，或者以管理员模式启动"))?;
    }
    Ok(())
}

/**
 * @description: 备份一组文件
 */
pub fn backup_files(files: Vec<String>) -> Result<()> {
    for file in files {
        if !is_file_exists(&file) {
            return Err(anyhow!("文件不存在: {}", &file));
        }
        let backup_file = format!("{}.bak", &file);
        fs::copy(&file, &backup_file)
            .map_err(|_| anyhow!("备份文件失败，文件被占用，或者以管理员模式启动"))?;
    }
    Ok(())
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
pub fn get_exe_dir() -> Result<String> {
    let path = std::env::current_exe()?
        .parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "无法获取程序所在目录"))?
        .to_path_buf();
    Ok(path.to_string_lossy().into_owned())
}

/**
 * 批量启动App
 * @param files 文件路径
 * @return 窗口句柄列表
 * @throws anyhow::Error 启动失败
 */
pub fn run_apps(files: &[String]) -> Result<Vec<HWND>> {
    let mut pids = vec![];
    for (i, file) in files.iter().enumerate() {
        println!("run_apps: {} {}", i, file);
        match create_process_w(file) {
            Ok(pid) => {
                println!("run_apps success: {}", pid);
                pids.push(pid);
            }
            Err(e) => {
                println!("run_apps error: {}", e);
            }
        }
        thread::sleep(time::Duration::from_millis(500));
    }
    thread::sleep(time::Duration::from_millis(1000));
    let hwnds = get_hwnds_by_pids(pids)?;
    if hwnds.is_empty() {
        return Err(anyhow!("运行App失败"));
    }
    //sort_apps(&mut hwnds)?;
    Ok(hwnds)
}

pub fn sort_apps(hwnds: &[HWND]) -> Result<()> {
    if hwnds.is_empty() {
        return Err(anyhow!("窗口句柄列表为空"));
    }
    if hwnds.len() == 1 {
        return Ok(());
    }
    let total = hwnds.len() as i32;
    let (sw, sh) = get_screen_size()?;
    let app_size = get_window_size(hwnds[0])?;
    let (mut w, h) = app_size;
    let mut positions = Vec::new();
    // 计算最大行列数
    let max_col_num = sw / w;
    let max_row_num = sh / h;
    if max_col_num < 1 || max_row_num < 1 {
        return Err(anyhow!("窗口尺寸不足无法排列"));
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
        let temp_col_num = total / row_num + total % (total / row_num);
        let diff_w = temp_col_num * w - sw;
        w = w - diff_w / (total / row_num - 1);
    }
    // 计算列数
    let col_num = total / row_num;
    for (i, hwnd) in hwnds.iter().enumerate() {
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
        let start_x = (sw - now_col_num * w) / 2;
        let start_y = (sh - row_num * h) / 2;
        let x = start_x + col_index * w;
        let y = start_y + row_index * h;
        positions.push((x, y));
        set_window_position(*hwnd, x, y, app_size.0, app_size.1)?;
    }
    Ok(())
}

/**
 * 启动App
 * @param command_line 文件路径
 * @return 进程ID
 * @throws anyhow::Error 启动失败
 */
pub fn create_process_w(command_line: &str) -> Result<u32> {
    let command_line = Some(PWSTR(HSTRING::from(command_line).as_ptr() as _));
    let startup_info = STARTUPINFOW::default();
    let mut process_info = PROCESS_INFORMATION::default();
    unsafe {
        CreateProcessW(
            None,
            command_line,
            None,
            None,
            false,
            CREATE_NEW_CONSOLE,
            None,
            None,
            &startup_info,
            &mut process_info,
        )
        .map_err(|e| anyhow!("启动App失败{}", e))?;
        let _ = CloseHandle(process_info.hThread);
        let _ = CloseHandle(process_info.hProcess);
    }
    Ok(process_info.dwProcessId)
}

struct WindowFinder {
    pids: Vec<u32>,
    hwnds: Vec<HWND>,
}

/**
 * 获取窗口句柄
 * @param pid 进程ID
 * @return 窗口句柄
 * @throws anyhow::Error 未找到窗口
 */
pub fn get_hwnds_by_pids(pids: Vec<u32>) -> Result<Vec<HWND>> {
    let mut finder = WindowFinder {
        pids,
        hwnds: Vec::new(),
    };
    let lparam = LPARAM(&mut finder as *mut WindowFinder as isize);
    unsafe {
        for _ in 0..20 {
            let _ = EnumWindows(Some(enum_windows_proc), lparam);
            println!("hwnds: {:?}", finder.hwnds);
            if !finder.hwnds.is_empty() {
                break;
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    };
    if finder.hwnds.is_empty() {
        return Err(anyhow!("未找到窗口"));
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
        if finder.pids.contains(&pid) && IsWindowVisible(hwnd).as_bool() {
            //检查窗口是否有标题（非空）
            let mut title = [0u16; 256];
            let len = GetWindowTextW(hwnd, &mut title);
            // 检查窗口尺寸（可选）
            let mut rect = RECT::default();
            let has_rect = GetWindowRect(hwnd, &mut rect).is_ok();
            if len > 0 && has_rect {
                finder.hwnds.push(hwnd);
            }
        }
    }
    BOOL::from(true)
}

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
pub fn send_enter_to_apps(hwnds: &[HWND]) -> Result<()> {
    for hwnd in hwnds {
        send_enter(*hwnd)?;
    }
    Ok(())
}

/**
 * 向窗口发送回车键消息
 * @param hwnd 目标窗口句柄
 * @return Result<()> 操作结果
 */
pub fn send_enter(hwnd: HWND) -> Result<()> {
    unsafe {
        // 发送按键按下消息
        PostMessageW(Some(hwnd), WM_KEYDOWN, WPARAM(13), LPARAM(0))
            .map_err(|e| anyhow!("发送回车键按下消息失败: {}", e))?;

        // 短暂延迟，模拟真实按键
        thread::sleep(time::Duration::from_millis(50));

        // 发送按键释放消息
        PostMessageW(Some(hwnd), WM_KEYUP, WPARAM(13), LPARAM(0))
            .map_err(|e| anyhow!("发送回车键释放消息失败: {}", e))?;
    }
    Ok(())
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
        SetWindowPos(hwnd, Some(HWND_TOP), x, y, width, height, SWP_SHOWWINDOW)
            .map_err(|e| anyhow!("设置窗口位置失败: {}", e))?;
    }
    Ok(())
}
