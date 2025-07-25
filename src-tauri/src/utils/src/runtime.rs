use super::errors::Result;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("获取当前运行程序失败")]
    GetCurrentExeError,

    #[error("获取当前运行路径失败")]
    GetCurrentPathError,
}

pub struct Runtime;

impl Runtime {
    pub fn current_exe() -> Result<PathBuf> {
        let path = std::env::current_exe()
            .map_err(|_| RuntimeError::GetCurrentExeError)?
            .to_path_buf();
        Ok(path)
    }

    pub fn current_dir() -> Result<PathBuf> {
        let path = Self::current_exe()?
            .parent()
            .ok_or(RuntimeError::GetCurrentPathError)?
            .to_path_buf();
        Ok(path)
    }
}
