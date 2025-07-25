use crate::close_handle;
use crate::errors::Result;
use crate::types::wstr::WSTR;
use thiserror::Error;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::ERROR_ALREADY_EXISTS;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::CreateMutexW;

#[derive(Debug, Error)]
pub enum MutexError {
    #[error("创建互斥量失败")]
    CreateMutexWError,

    #[error("互斥量已存在")]
    MutexExistsError,
}

pub struct Mutex {
    name: String,
}

impl Mutex {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn create<S: Into<String>>(name: S) -> Result<HANDLE> {
        Self::new(name).run()
    }

    pub fn check(name: &str) -> bool {
        if let Ok(mutex) = Self::create(name) {
            close_handle!(mutex);
            true;
        }
        false
    }

    pub fn run(&self) -> Result<HANDLE> {
        let mut wstr = WSTR::new(Some(&self.name));
        let result = unsafe {
            CreateMutexW(None, true, wstr.to_pwstr()).map_err(|_| MutexError::CreateMutexWError)?
        };
        if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            return Err(MutexError::MutexExistsError.into());
        }
        Ok(result)
    }
}
