use crate::errors::Result;
use std::os::windows::process::CommandExt;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("运行 {0} 失败")]
    RunAppError(String),

    #[error("关闭 {0} 失败")]
    CloseAppError(String),

    #[error("打开URL失败")]
    OpenUrlError,

    #[error("打开文件浏览器失败")]
    OpenExplorerError,
}
pub struct Cmd {
    arg: String,
}

impl Cmd {
    pub fn new<S: Into<String>>(arg: S) -> Self {
        Self { arg: arg.into() }
    }

    pub fn close_app(&self) -> Result<()> {
        Command::new("cmd.exe")
            .creation_flags(0x08000000)
            .arg("/C")
            .arg("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg(&self.arg)
            .spawn()
            .map_err(|_| CmdError::CloseAppError(self.arg.clone()))?;
        Ok(())
    }

    pub fn run_app(&self) -> Result<()> {
        Command::new(&self.arg)
            .creation_flags(0x08000000)
            .spawn()
            .map_err(|_| CmdError::RunAppError(self.arg.clone()))?;
        Ok(())
    }

    pub fn open_url(&self) -> Result<()> {
        Command::new("cmd.exe")
            .creation_flags(0x08000000)
            .arg("/C")
            .arg("start")
            .arg(&self.arg)
            .spawn()
            .map_err(|_| CmdError::OpenUrlError)?;
        Ok(())
    }

    pub fn open_folder(&self) -> Result<()> {
        Command::new("explorer")
            .args([&self.arg])
            .spawn()
            .map_err(|_| CmdError::OpenExplorerError)?;
        Ok(())
    }
}
