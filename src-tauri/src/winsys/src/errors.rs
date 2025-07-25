pub type Result<T> = core::result::Result<T, WinsysError>;
use crate::fileinfo::FileInfoError;
use crate::process::hwnd::HwndError;
use crate::process::mutex::MutexError;
use crate::process::pid::PidError;
use crate::process::process::ProcessError;
use crate::registry::RegistryError;
use crate::shortcut::ShortcutError;
use crate::win::WinApiError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WinsysError {
    #[error(transparent)]
    HwndError(#[from] HwndError),

    #[error(transparent)]
    MutexError(#[from] MutexError),

    #[error(transparent)]
    FileInfoError(#[from] FileInfoError),

    #[error(transparent)]
    RegistryError(#[from] RegistryError),

    #[error(transparent)]
    PidError(#[from] PidError),

    #[error(transparent)]
    ShortcutError(#[from] ShortcutError),

    #[error(transparent)]
    WinApiError(#[from] WinApiError),

    #[error(transparent)]
    ProcessError(#[from] ProcessError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
