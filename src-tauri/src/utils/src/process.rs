use crate::cmd::Cmd;
use crate::errors::Result;
use thiserror::Error;
use winsys::process::hwnd::Hwnd;
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
    let cmd = Cmd::new(file);
    cmd.run_app()?;
    Ok(())
}

pub fn close_app_by_pid(file_name: &str) -> Result<()> {
    let pids = Pid::find_all_by_process_name(&file_name);
    if let Ok(pids) = pids {
        for pid in pids {
            pid.terminate()?;
        }
    }
    Ok(())
}

pub fn close_app_by_cmd(file_name: &str) -> Result<()> {
    let cmd = Cmd::new(file_name);
    cmd.close_app()?;
    Ok(())
}

pub fn sort_apps(pids: &[Pid]) -> Result<()> {
    if pids.is_empty() || pids.len() == 1 {
        return Err(ProcessError::HwndsEmptyError.into());
    }
    let total = pids.len() as i32;
    let (sw, sh) = get_screen_size()?;
    let mut hwnds = Vec::new();
    for pid in pids {
        let hwnd = Hwnd::from(pid);
        hwnds.push(hwnd);
    }
    
    // 多次尝试，避免失败
    let mut app_size = (0, 0);
    for i in 0..hwnds.len() {
        let hwnd = &hwnds[i];
        match hwnd.get_app_size() {
            Ok(s) => {
                app_size = s;
                break;
            }
            Err(e) => {
                if i == hwnds.len() - 1 {
                    return Err(e.into());
                }
                sleep(500);
            }
        }
    }
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
        let start_x = (sw - (now_col_num - 1) * w - app_size.0) / 2;
        let start_x = if start_x < 0 { 0 } else { start_x };
        let start_y = (sh - row_num * h) / 2;
        let x = start_x + col_index * w;
        let y = start_y + row_index * h;
        hwnd.set_window_pos(x, y)?;
    }
    Ok(())
}

pub fn send_mouse_click_to_apps_use_scale(hwnds: &[Hwnd], x: i32, y: i32) -> Result<()> {
    if hwnds.is_empty() {
        return Err(ProcessError::HwndsEmptyError.into());
    }
    let mut scale = 1.0;
    for i in 0..hwnds.len() {
        let hwnd = &hwnds[i];
        match hwnd.get_app_scale() {
            Ok(s) => {
                scale = s;
                break;
            }
            Err(e) => {
                if i == hwnds.len() - 1 {
                    return Err(e.into());
                }
                sleep(300);
            }
        }
    }
    let real_x = (scale * x as f32) as i32;
    let real_y = (scale * y as f32) as i32;
    for hwnd in hwnds {
        let _ = hwnd.send_mouse_click(real_x, real_y);
    }
    Ok(())
}
