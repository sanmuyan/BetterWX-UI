#[warn(dead_code)]
use anyhow::{anyhow, Result};
use known_folders::{get_known_folder_path, KnownFolder};
use log::debug;
use std::fs;
use std::path::Path;
use windows::core::Interface;
use windows::core::{w, BOOL, GUID, HSTRING, PCWSTR};
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::IShellLinkW;
/**
 * @description: 检查文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
pub fn is_file_exists(file: &str) -> bool {
    Path::new(file).exists()
}

/**
 * @description: 检查一组文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
pub fn is_files_exists(files: &[String]) -> bool {
    if files.is_empty() {
        return false;
    }
    let mut result = true;
    for file in files {
        result = result && is_file_exists(&file);
    }
    result
}

/**
 * @description: 检查一组文件是否全部存在
 * @param path 文件路径
 * @return 返回存在的文件路径
 */
pub fn filter_files_is_exists(files: &Vec<String>) -> (bool, Vec<String>) {
    let mut result = Vec::new();
    for file in files {
        if is_file_exists(&file) {
            result.push(file.to_string());
        }
    }
    (files.len() == result.len(), result)
}

/**
 * @description: 删除一组文件
 */
pub fn del_files(files: Vec<String>) -> Result<()> {
    for file in files {
        if !is_file_exists(&file) {
            return Err(anyhow!("应用程序不存在: {}", file));
        }
        fs::remove_file(file)
            .map_err(|_| anyhow!("删除文件失败，请先关闭所有目标程序，或者尝试以管理员模式启动"))?;
    }
    Ok(())
}

/**
 * @description: 备份一组文件
 */
pub fn backup_files(files: Vec<String>) -> Result<()> {
    let mut all_exists = true;
    for file in &files {
        let bak_file = format!("{}.bak", file);
        all_exists = all_exists
            && if is_file_exists(&bak_file) {
                let file_ver = get_file_version_retry(file)?;
                let bak_file_ver = get_file_version_retry(&bak_file)?;
                if file_ver != bak_file_ver {
                    debug!(
                        "备份文件版本号不一致,备份文件版本:{},目标文件版本:{} ",
                        bak_file_ver, file_ver
                    );
                }
                file_ver == bak_file_ver
            } else {
                false
            };
    }
    if !all_exists {
        for file in &files {
            if !is_file_exists(file) {
                return Err(anyhow!("文件不存在: {}", &file));
            }
            let backup_file = format!("{}.bak", &file);
            fs::copy(&file, &backup_file).map_err(|_| {
                anyhow!("备份文件失败，请先关闭所有目标程序，或者尝试以管理员模式启动")
            })?;
        }
    }
    Ok(())
}

pub fn get_file_version_retry(file: &str) -> Result<String> {
    for _ in 0..3 {
        let ver = get_file_version(file);
        if ver.is_ok() {
            return ver;
        }
    }
    Err(anyhow!("获取文件版本号失败，文件：{file}"))
}

/**
 * @description: 获取程序版本号
 */
pub fn get_file_version(file: &str) -> Result<String> {
    let lpname = PCWSTR(HSTRING::from(file).as_ptr() as _);
    // 第一步：获取版本信息大小
    let mut dummy = 0;
    let info_size = unsafe { GetFileVersionInfoSizeW(lpname, Some(&mut dummy)) };
    if info_size == 0 {
        return Err(anyhow!("无法获取版本信息大小"));
    }
    // 第二步：分配缓冲区并获取版本信息
    let mut buffer: Vec<u8> = vec![0; info_size as usize];
    let _ =
        unsafe { GetFileVersionInfoW(lpname, Some(0), info_size, buffer.as_mut_ptr() as *mut _) }?;
    // 第三步：查询固定文件信息
    let mut fixed_info_ptr = std::ptr::null_mut();
    let mut fixed_info_len = 0;
    let success = unsafe {
        VerQueryValueW(
            buffer.as_ptr() as *const _,
            w!("\\"),
            &mut fixed_info_ptr,
            &mut fixed_info_len,
        )
    };
    if success != BOOL(1) {
        return Err(anyhow!("无法获取版本信息大小"));
    }
    // 第四步：提取版本号
    let fixed_info = unsafe { &*(fixed_info_ptr as *const VS_FIXEDFILEINFO) };
    let major = (fixed_info.dwFileVersionMS >> 16) as u16;
    let minor = (fixed_info.dwFileVersionMS & 0xFFFF) as u16;
    let build = (fixed_info.dwFileVersionLS >> 16) as u16;
    let revision = (fixed_info.dwFileVersionLS & 0xFFFF) as u16;
    Ok(format!("{}.{}.{}.{}", major, minor, build, revision))
}

#[derive(Debug, Default, Clone)]
pub struct ShotCutArgs {
    pub code: String,
    pub name: String,
    pub path: String,
    pub list: String,
    pub level: String,
    pub login: String,
}

impl ShotCutArgs {
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<String>,
        list: impl Into<String>,
        level: impl Into<String>,
        login: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            path: path.into(),
            list: list.into(),
            level: level.into(),
            login: login.into(),
        }
    }

    pub fn to_cmd_args(&self) -> String {
        format!(
            "code=\"{}\" name=\"{}\" path=\"{}\" list=\"{}\"",
            self.code, self.name, self.path, self.list
        )
    }

    pub fn check(&self) -> bool {
        self.code.len() > 0 && self.name.len() > 0 && self.path.len() > 0 && self.list.len() > 0
    }
}

impl From<Vec<String>> for ShotCutArgs {
    fn from(args: Vec<String>) -> Self {
        let mut result = Self::default();
        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                match key {
                    "code" => result.code = value.to_string(),
                    "name" => result.name = value.to_string(),
                    "path" => result.path = value.to_string(),
                    "list" => result.list = value.to_string(),
                    "level" => result.level = value.to_string(),
                    "login" => result.login = value.to_string(),
                    _ => continue,
                }
            }
        }

        result
    }
}
/**
 * @description: 创建快捷方式到桌面
 */
pub fn create_shortcut_to_desktop(
    exe_path: &str,
    shortcut_name: &str,
    icon: Option<&str>,
    args: Option<&str>,
) -> Result<()> {
    let desktop_path = get_known_folder_path(KnownFolder::Desktop)
        .ok_or_else(|| anyhow!("创建快捷方式失败，无法获取桌面路径"))?
        .to_str()
        .ok_or_else(|| anyhow!("创建快捷方式失败，桌面路径包含无效字符"))?
        .to_string();
    let shortcut_path = format!("{}\\{}.lnk", desktop_path, shortcut_name);
    create_shortcut(&exe_path, &shortcut_path, icon, args)
        .map_err(|e| anyhow!("创建快捷方式失败，{}", e))
}

const CLSID_SHELLLINK: GUID = GUID::from_values(
    0x00021401,
    0x0000,
    0x0000,
    [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
);

/**
 * @description: 创建快捷方式到桌面
 */
pub fn create_shortcut(
    exe_path: &str,
    shortcut_path: &str,
    icon: Option<&str>,
    args: Option<&str>,
) -> Result<()> {
    unsafe {
        // 初始化COM库
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        // 创建IShellLink对象
        let shell_link: IShellLinkW =
            CoCreateInstance(&CLSID_SHELLLINK, None, CLSCTX_INPROC_SERVER)?;
        // 设置快捷方式属性
        shell_link.SetPath(PCWSTR(HSTRING::from(exe_path).as_ptr()))?;
        if let Some(arguments) = args {
            shell_link.SetArguments(PCWSTR(HSTRING::from(arguments).as_ptr()))?;
        }
        if let Some(icon) = icon {
            // 新增：设置快捷方式图标（使用exe文件自身的图标）
            shell_link.SetIconLocation(PCWSTR(HSTRING::from(icon).as_ptr()), 0)?;
        }
        // 获取工作目录
        let work_dir = Path::new(exe_path)
            .parent()
            .and_then(|p| p.to_str())
            .ok_or(anyhow!("无法获取可执行文件在所在目录"))?;
        shell_link.SetWorkingDirectory(PCWSTR(HSTRING::from(work_dir).as_ptr()))?;
        // 保存快捷方式
        let persist_file: IPersistFile = shell_link.cast()?;
        persist_file.Save(PCWSTR(HSTRING::from(shortcut_path).as_ptr()), true)?;
        // 释放COM库
        CoUninitialize();
    }
    Ok(())
}
