use crate::cmd::Cmd;
use crate::errors::Result;
use crate::file_pid_hwnd::FilesPid;
use log::debug;
use thiserror::Error;
use winsys::process::pid::Pid;
use winsys::win::get_screen_size;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("窗口句柄列表为空")]
    HwndsEmptyError,

    #[error("屏幕尺寸不足,无法排列")]
    ScreenSizeError,
}

pub fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

pub fn run_app_by_cmd(file: &str) -> Result<()> {
    debug!("使用命令行启动程序: {:?}", file);
    let cmd = Cmd::new(file);
    cmd.run_app()?;
    Ok(())
}

pub fn close_app_by_pid(file_name: &str) -> Result<bool> {
    let pids = Pid::find_all_by_process_name(&file_name);
    debug!(
        "关闭程序 close_app_by_pid : {:?}, 进程ID: {:?}",
        file_name, pids
    );
    if let Ok(pids) = pids {
        for pid in pids {
            pid.terminate()?;
        }
        return Ok(true);
    }
    Ok(false)
}

pub fn close_app_by_cmd(file_name: &str, delay: u64) -> Result<()> {
    debug!("使用命令行关闭程序: {:?}", file_name);
    let cmd = Cmd::new(file_name);
    cmd.close_app()?;
    if delay > 0 {
        sleep(delay);
    }
    Ok(())
}

pub fn sort_apps(fpids: &FilesPid) -> Result<()> {
    if fpids.is_empty() || fpids.len() == 1 {
        return Err(ProcessError::HwndsEmptyError.into());
    }
    let total = fpids.len() as i32;
    let (sw, sh) = get_screen_size()?;
    // 多次尝试，避免失败
    let app_size = try_get_app_size(fpids)?;
    let (mut w, h) = app_size;
    // 计算最大行列数
    let max_col_num = sw / w;
    let max_row_num = sh / h;
    if max_col_num < 1 || max_row_num < 1 {
        return Err(ProcessError::ScreenSizeError.into());
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
    }
    // 计算列数
    let col_num = total / row_num;
    for (i, fpid) in fpids.0.iter().enumerate() {
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
        fpid.hwnd.set_window_pos(x, y)?;
    }
    Ok(())
}

fn try_get_app_size(fpids: &FilesPid) -> Result<(i32, i32)> {
    let mut app_size = (0, 0);
    for (i,fpid) in fpids.0.iter().enumerate() {
        let hwnd = &fpid.hwnd;
        match hwnd.get_app_size() {
            Ok(s) => {
                app_size = s;
                break;
            }
            Err(e) => {
                if i == fpids.len() - 1 {
                    return Err(e.into());
                }
            }
        }
    }
    Ok(app_size)
}

pub fn send_mouse_click_to_apps_use_scale(fpids: &FilesPid, x: i32, y: i32) -> Result<()> {
    if fpids.is_empty() {
        return Err(ProcessError::HwndsEmptyError.into());
    }
    let scale = try_get_app_scale(fpids)?;
    let real_x = (scale * x as f32) as i32;
    let real_y = (scale * y as f32) as i32;
    for fpid in &fpids.0 {
        let _ = fpid.hwnd.send_mouse_click(real_x, real_y);
    }
    Ok(())
}

fn try_get_app_scale(fpids: &FilesPid) -> Result<f32> {
    let mut scale = 1.0;
    for (i,fpid) in fpids.0.iter().enumerate() {
        let hwnd = &fpid.hwnd;
        match hwnd.get_app_scale() {
            Ok(s) => {
                scale = s;
                break;
            }
            Err(e) => {
                if i == fpids.len() - 1 {
                    return Err(e.into());
                }
            }
        }
    }
    Ok(scale)
}