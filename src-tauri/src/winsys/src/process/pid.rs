use crate::close_handle;
use crate::errors::Result;
use crate::process::hwnd::Hwnd;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Borrow;
use thiserror::Error;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::ProcessStatus::GetProcessImageFileNameW;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
use windows::Win32::System::Threading::PROCESS_TERMINATE;
use windows::Win32::System::Threading::PROCESS_VM_READ;
use windows::Win32::System::Threading::TerminateProcess;
use windows::Win32::UI::WindowsAndMessaging::GetShellWindow;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

#[derive(Debug, Error)]
pub enum PidError {
    #[error("获取文件浏览器PID失败")]
    GetExplorerError,

    #[error("关闭程序失败")]
    TerminateError,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Pid(pub u32);

impl Pid {
    pub fn new(pid: u32) -> Self {
        Self(pid)
    }

    pub fn get_u32(&self) -> u32 {
        self.0
    }

    pub fn get_pid(&self) -> Pid {
        Self::new(self.0)
    }

    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }

    pub fn terminate(self) -> Result<()> {
        let h_process = unsafe {
            OpenProcess(PROCESS_TERMINATE, false, self.0).map_err(|_| PidError::TerminateError)?
        };
        unsafe { TerminateProcess(h_process, 1).map_err(|_| PidError::TerminateError)? };
        close_handle!(h_process);
        Ok(())
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
        Err(PidError::GetExplorerError.into())
    }

    pub fn get_process_name(self) -> Option<String> {
        let h_process = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, self.0).ok()
        }?;
        let mut buffer = [0u16; MAX_PATH as usize];
        let size = unsafe { GetProcessImageFileNameW(h_process, &mut buffer) };
        close_handle!(h_process);
        if size > 0 {
            let path = String::from_utf16_lossy(&buffer[..size as usize]);
            Some(path.split('\\').last()?.to_string())
        } else {
            None
        }
    }

    pub fn find_all_by_process_name(process_name: &str) -> Result<Vec<Self>> {
        let mut pids = Vec::new();
        let hwnds = Hwnd::find_all_by_process_name(process_name)?;
        for hwnd in hwnds {
            let pid = Pid::from(hwnd);
            if !pids.contains(&pid) {
                pids.push(pid);
            }
        }
        Ok(pids)
    }
}

impl<T> From<T> for Pid
where
    T: Borrow<Hwnd>,
{
    fn from(hwnd: T) -> Self {
        let hwnd_ref = hwnd.borrow();
        let mut pid = 0;
        unsafe {
            GetWindowThreadProcessId(hwnd_ref.get_hwnd(), Some(&mut pid));
        }
        Pid::new(pid)
    }
}
