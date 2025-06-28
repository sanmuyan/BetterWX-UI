use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use windows::core::{w, BOOL, HSTRING, PCWSTR};
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
};

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
                    println!(
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
