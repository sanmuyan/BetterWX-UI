use crate::errors::Result;
use crate::process::pid::Pid;
use crate::types::wstr::WSTR;
use std::borrow::Borrow;
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;
use windows::Win32::UI::WindowsAndMessaging::GWL_STYLE;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongA;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
use windows::Win32::UI::WindowsAndMessaging::GetWindowTextW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
use windows::Win32::UI::WindowsAndMessaging::HWND_NOTOPMOST;
use windows::Win32::UI::WindowsAndMessaging::IsWindowVisible;
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER;
use windows::Win32::UI::WindowsAndMessaging::SWP_SHOWWINDOW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONUP;
use windows::Win32::UI::WindowsAndMessaging::WS_CHILD;
use windows::core::BOOL;

#[derive(Debug, Error)]
pub enum HwndError {
    #[error("class_name和window_name不能同时为空")]
    FindWindowsWArgsBothNone,

    #[error("根据进程名查找窗口失败")]
    WindowsNotFind,

    #[error("枚举窗口失败")]
    EnumWindowsError,

    #[error("设置窗口位置失败")]
    SetWindowPosError,

    #[error("根据进程名查找窗口失败")]
    FindWindowWError,

    #[error("获取窗口尺寸失败")]
    GetAppSizeError,

    #[error("发送鼠标消息失败")]
    SendMouseKeyError,

    #[error("获取缩放比例失败")]
    GetDpiScaleError,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hwnd(HWND);

impl Default for Hwnd {
    fn default() -> Self {
        Self(HWND::default())
    }
}

impl Hwnd {
    pub fn new<T: Into<HWND>>(handle: T) -> Self {
        Self(handle.into())
    }

    pub fn get_hwnd(&self) -> HWND {
        self.0
    }

    pub fn is_invalid(&self) -> bool {
        self.0.is_invalid()
    }

    pub fn find_windows_w(class_name: Option<&str>, window_name: Option<&str>) -> Result<Self> {
        if class_name.is_none() && window_name.is_none() {
            return Err(HwndError::FindWindowsWArgsBothNone.into());
        }
        let class_wstr = WSTR::new(class_name);
        let window_wstr = WSTR::new(window_name);
        let hwnd = unsafe {
            FindWindowW(class_wstr.to_pcwstr(), window_wstr.to_pcwstr())
                .map_err(|_| HwndError::FindWindowWError)?
        };
        Ok(Hwnd(hwnd))
    }

    pub fn find_all_by_process_name(process_name: &str) -> Result<Vec<Self>> {
        let mut hf = HwndFinderByProcessName::new(process_name);
        let lparam = LPARAM(&mut hf as *mut HwndFinderByProcessName as isize);
        unsafe {
            EnumWindows(Some(enum_windows_by_process_name_proc), lparam)
                .map_err(|_| HwndError::EnumWindowsError)?;
        }
        if !hf.hwnds.is_empty() {
            return Ok(hf.hwnds);
        }
        Err(HwndError::WindowsNotFind.into())
    }
    
    pub fn get_app_scale(&self) -> Result<f32> {
        unsafe {
            let dpi = GetDpiForWindow(self.get_hwnd());
            if dpi == 0 {
                return Err(HwndError::GetDpiScaleError.into());
            }
            Ok(dpi as f32 / 96.0)
        }
    }

    pub fn get_app_size(&self) -> Result<(i32, i32)> {
        let mut rect = RECT::default();
        unsafe {
            GetWindowRect(self.get_hwnd(), &mut rect).map_err(|_| HwndError::GetAppSizeError)?;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        Ok((width, height))
    }

    pub fn set_window_pos(&self, x: i32, y: i32) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.0,
                Some(HWND_NOTOPMOST),
                x,
                y,
                0,
                0,
                SWP_SHOWWINDOW | SWP_NOSIZE | SWP_NOZORDER,
            )
            .map_err(|_| HwndError::SetWindowPosError)?;
        }
        Ok(())
    }

    pub fn send_mouse_click(&self, x: i32, y: i32) -> Result<()> {
        unsafe {
            let hwnd = self.get_hwnd();

            let lparam = LPARAM(((y << 16) | x) as isize);

            PostMessageW(Some(hwnd), WM_LBUTTONDOWN, WPARAM(1), lparam)
                .map_err(|_| HwndError::SendMouseKeyError)?;

            sleep(Duration::from_millis(50));

            PostMessageW(Some(hwnd), WM_LBUTTONUP, WPARAM(0), lparam)
                .map_err(|_| HwndError::SendMouseKeyError)?;
        }
        Ok(())
    }
}

impl<T> From<T> for Hwnd
where
    T: Borrow<Pid>,
{
    fn from(pid: T) -> Self {
        let pid_ref = pid.borrow();
        let mut finder = HwndFinderByPid::new(pid_ref.clone());
        let lparam = LPARAM(&mut finder as *mut HwndFinderByPid as isize);
        let _ = unsafe { EnumWindows(Some(enum_windows_by_pid_proc), lparam) };
        finder.hwnd
    }
}

struct HwndFinderByPid {
    pid: Pid,
    hwnd: Hwnd,
}

impl HwndFinderByPid {
    fn new(pid: Pid) -> Self {
        Self {
            pid,
            hwnd: Hwnd::default(),
        }
    }
}

unsafe extern "system" fn enum_windows_by_pid_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let finder = unsafe { &mut *(lparam.0 as *mut HwndFinderByPid) };
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == finder.pid.get_u32() {
        let mut title = [0u16; 256];
        // 最好传入更多匹配参数，暂时默认为可见 有标题的 窗口
        let style = unsafe { GetWindowLongA(hwnd, GWL_STYLE) };
        let is_top = (style & WS_CHILD.0 as i32) == 0;
        let len = unsafe { GetWindowTextW(hwnd, &mut title) };
        let h = Hwnd::new(hwnd);
        let isvisible = unsafe { IsWindowVisible(hwnd).as_bool() };
        if is_top && len > 0 && h.get_app_size().is_ok() && isvisible && title.len()>0 {
            log::debug!(
                "find hwnd by pid : pid {}, hwnd{:?}, title {},is_top {}, isvisible {}",
                finder.pid.get_u32(),
                h,
                String::from_utf16_lossy(&title),
                is_top,
                isvisible
            );
            finder.hwnd = h;
            return BOOL::from(false);
        }
    }
    BOOL::from(true)
}

struct HwndFinderByProcessName {
    process_name: String,
    hwnds: Vec<Hwnd>,
}

impl HwndFinderByProcessName {
    fn new(process_name: &str) -> Self {
        Self {
            process_name: process_name.to_string(),
            hwnds: Vec::new(),
        }
    }
}

unsafe extern "system" fn enum_windows_by_process_name_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let hf = unsafe { &mut *(lparam.0 as *mut HwndFinderByProcessName) };
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    let style = unsafe { GetWindowLongA(hwnd, GWL_STYLE) };
    if (style & WS_CHILD.0 as i32) == 0 {
        if let Some(process_name) = Pid::new(pid).get_process_name() {
            if process_name.trim().to_ascii_lowercase()
                == hf.process_name.trim().to_ascii_lowercase()
            {
                hf.hwnds.push(Hwnd::new(hwnd));
            }
        }
    }
    BOOL::from(true)
}
