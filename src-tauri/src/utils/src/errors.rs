pub type Result<T> = core::result::Result<T, UtilsError>;
use crate::base64::Base64Error;
use crate::cmd::CmdError;
use crate::file::FileError;
use crate::http::HttpError;
use crate::patch::errors::UPatchError;
use crate::process::ProcessError;
use crate::runtime::RuntimeError;
use crate::store::StoreError;
use crate::tools::ToolsError;
use thiserror::Error;
use winsys::errors::WinsysError;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error(transparent)]
    Base64Error(#[from] Base64Error),

    #[error(transparent)]
    CmdError(#[from] CmdError),

    #[error(transparent)]
    FileError(#[from] FileError),

    #[error(transparent)]
    HttpError(#[from] HttpError),

    #[error(transparent)]
    ProcessError(#[from] ProcessError),

    #[error(transparent)]
    UPatchError(#[from] UPatchError),

    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),

    #[error(transparent)]
    WinsysError(#[from] WinsysError),

    #[error(transparent)]
    StoreError(#[from] StoreError),

    #[error(transparent)]
    ToolsError(#[from] ToolsError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
