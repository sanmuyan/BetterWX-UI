use crate::errors::Result;
use crate::types::wstr::WSTR;
use known_folders::KnownFolder;
use known_folders::get_known_folder_path;
use std::path::Path;
use thiserror::Error;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
use windows::Win32::System::Com::COINIT_APARTMENTTHREADED;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::CoUninitialize;
use windows::Win32::System::Com::IPersistFile;
use windows::Win32::UI::Shell::IShellLinkW;
use windows::core::GUID;
use windows::core::Interface;

const ERROR_PRFIX: &str = "创建快捷方式失败";

#[derive(Debug, Error)]
pub enum ShortcutError {
    #[error("{ERROR_PRFIX}")]
    CreateInstanceError,

    #[error("{ERROR_PRFIX}，目标文件不能为空")]
    FileEmptyError,

    #[error("{ERROR_PRFIX}，获取目标文件名失败")]
    GetFileNameError,

    #[error("{ERROR_PRFIX}，获取目标所在目录失败")]
    GetFilePathError,

    #[error("{ERROR_PRFIX}，获取桌面目录失败")]
    GetDesktopPathError,

    #[error("{ERROR_PRFIX}，获取开机启动目录失败")]
    GetStartUpPathError,

    #[error("{ERROR_PRFIX}，设置目标文件失败")]
    SetPathError,

    #[error("{ERROR_PRFIX}，设置参数失败")]
    SetArgsError,

    #[error("{ERROR_PRFIX}，设置图标失败")]
    SetIconError,

    #[error("{ERROR_PRFIX}，设置工作目录失败")]
    SetWorkDirError,

    #[error("{ERROR_PRFIX}，保存失败")]
    SaveError,

    #[error("{ERROR_PRFIX}，保存文件权限不足")]
    SaveAccessDeniedError,
}

/// CLSID_SHELLLINK is missing in windows create 0.60+
const CLSID_SHELLLINK: GUID = GUID::from_values(
    0x00021401,
    0x0000,
    0x0000,
    [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);

#[derive(Default)]
pub struct Shortcut {
    file: String,
    target: Option<String>,
    name: Option<String>,
    work_dir: Option<String>,
    icon: Option<String>,
    args: Option<String>,
}

impl Shortcut {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
            target: None,
            name: None,
            icon: None,
            args: None,
            work_dir: None,
        }
    }

    fn get_file(&self) -> Result<String> {
        let file = match self.file.is_empty() {
            true => std::env::current_exe()?.to_string_lossy().to_string(),
            false => self.file.to_string(),
        };
        Ok(file)
    }

    fn get_filename(&self) -> Result<String> {
        let path = Path::new(&self.file)
            .file_name()
            .ok_or(ShortcutError::GetFileNameError)?;
        Ok(path.to_string_lossy().to_string())
    }

    pub fn get_name(&self) -> String {
        let name = match &self.name {
            Some(name) => name.to_string(),
            None => self.get_filename().unwrap_or("default".into()),
        };
        match name.to_ascii_lowercase().ends_with(".lnk") {
            true => name,
            false => format!("{}.lnk", name),
        }
    }

    pub fn get_target(&self) -> Result<String> {
        let target = match &self.target {
            Some(target) => target.to_string(),
            None => self.get_filepath()?,
        };
        Ok(target)
    }

    pub fn get_target_file(&self) -> Result<String> {
        let name = self.get_name();
        let target = self.get_target()?;
        Ok(Path::new(&target).join(name).to_string_lossy().into())
    }

    pub fn get_work_dir(&self) -> Result<String> {
        let work_dir = match &self.work_dir {
            Some(work_dir) => work_dir.to_string(),
            None => self.get_filepath()?,
        };
        Ok(work_dir)
    }

    pub fn get_filepath(&self) -> Result<String> {
        let file = self.get_file()?;
        let path = Path::new(&file)
            .parent()
            .ok_or(ShortcutError::GetFilePathError)?
            .to_string_lossy()
            .to_string();
        Ok(path)
    }

    pub fn set_file<S: Into<String>>(mut self, file: S) -> Self {
        self.file = file.into();
        self
    }

    pub fn set_target<S: Into<String>>(mut self, target: S) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn set_filepath_as_target_(mut self) -> Result<Self> {
        self.target = Some(self.get_filepath()?);
        Ok(self)
    }

    pub fn set_desktop_as_target(mut self) -> Result<Self> {
        let path = get_known_folder_path(KnownFolder::Desktop)
            .ok_or(ShortcutError::GetDesktopPathError)?;
        self.target = Some(path.to_string_lossy().to_string());
        Ok(self)
    }

    pub fn set_startup_as_target(mut self) -> Result<Self> {
        let path = get_known_folder_path(KnownFolder::Startup)
            .ok_or(ShortcutError::GetStartUpPathError)?;
        self.target = Some(path.to_string_lossy().to_string());
        Ok(self)
    }

    pub fn set_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn set_icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn set_args<S: Into<String>>(mut self, args: S) -> Self {
        self.args = Some(args.into());
        self
    }

    pub fn set_work_dir<S: Into<String>>(mut self, work_dir: S) -> Self {
        self.work_dir = Some(work_dir.into());
        self
    }

    pub fn set_filepath_as_work_dir(mut self) -> Result<Self> {
        self.work_dir = Some(self.get_filepath()?);
        Ok(self)
    }

    pub fn run(&self) -> Result<()> {
        // init com
        let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };

        // crete IShellLink object
        let shell_link: IShellLinkW = unsafe {
            CoCreateInstance(&CLSID_SHELLLINK, None, CLSCTX_INPROC_SERVER)
                .map_err(|_| ShortcutError::CreateInstanceError)?
        };

        // set shortcut file
        let file = self.get_file()?;

        let path_wstr = WSTR::new(Some(&file));
        unsafe {
            shell_link
                .SetPath(path_wstr.to_pcwstr())
                .map_err(|_| ShortcutError::SetPathError)?
        };

        // set shortcut args
        if let Some(args) = &self.args {
            let args_wstr = WSTR::new(Some(&args));
            unsafe {
                shell_link
                    .SetArguments(args_wstr.to_pcwstr())
                    .map_err(|_| ShortcutError::SetArgsError)?
            };
        }

        // set shortcut icon
        let icon = match &self.icon {
            Some(icon) => icon,
            None => &self.file,
        };
        let icon_wstr = WSTR::new(Some(icon));
        unsafe {
            shell_link
                .SetIconLocation(icon_wstr.to_pcwstr(), 0)
                .map_err(|_| ShortcutError::SetIconError)?
        };

        // set shortcut work_dir
        let work_dir = self.get_work_dir()?;
        let work_dir_wstr = WSTR::new(Some(&work_dir));
        unsafe {
            shell_link
                .SetWorkingDirectory(work_dir_wstr.to_pcwstr())
                .map_err(|_| ShortcutError::SetWorkDirError)?
        };

        // shell_link cast need import "use windows_core::interface::Interface";
        let persist_file: IPersistFile = shell_link
            .cast()
            .map_err(|_| ShortcutError::CreateInstanceError)?;

        // set shortcut target_file
        let shortcut_path = self.get_target_file()?;
        let shortcut_path_wstr = WSTR::new(Some(&shortcut_path));

        // save shortcut to target_file
        unsafe {
            persist_file
                .Save(shortcut_path_wstr.to_pcwstr(), true)
                .map_err(|e| match e.code().0 as u32 {
                    0x80070005 => ShortcutError::SaveAccessDeniedError,
                    _ => ShortcutError::SaveError,
                })
        }?;

        // release COM
        unsafe { CoUninitialize() };
        Ok(())
    }
}
