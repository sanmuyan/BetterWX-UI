use anyhow::Result;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::{ MessageBoxW, MB_ICONINFORMATION, MB_OK,};


/**
 * 显示消息框
 * @param title 标题
 * @param message 消息
 * @throws anyhow::Error 显示失败
 */
#[allow(dead_code)]
pub fn message_box(title: &str, message: &str) -> Result<()> {
    let title = HSTRING::from(title);
    let message = HSTRING::from(message);
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }
    Ok(())
}
