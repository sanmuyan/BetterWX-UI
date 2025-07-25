use crate::errors::Result;
use crate::types::wstr::WSTR;
use thiserror::Error;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN;
use windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN;

#[derive(Debug, Error)]
pub enum WinApiError {
    #[error("获取屏幕尺寸失败")]
    GetScreenSizeError,
}

pub fn message_box(title: &str, message: &str) -> Result<()> {
    let title = WSTR::new(Some(title));
    let message = WSTR::new(Some(message));
    unsafe {
        MessageBoxW(
            None,
            message.to_pcwstr(),
            title.to_pcwstr(),
            MB_OK | MB_ICONINFORMATION,
        );
    }
    Ok(())
}

pub fn get_screen_size() -> Result<(i32, i32)> {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        if width == 0 || height == 0 {
            return Err(WinApiError::GetScreenSizeError.into());
        }
        Ok((width, height))
    }
}

pub fn keep_running() {
    let _ = unsafe { GetMessageW(&mut MSG::default(), None, 0, 0) };
}
