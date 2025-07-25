use crate::errors::Result;
use crate::version::Version;
use std::fs::copy;
use std::path::Path;
use thiserror::Error;
use winsys::fileinfo::FileInfo;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("文件不存在：{0}")]
    FileNotExists(String),

    #[error("文件无效：{0}")]
    FileInvalidError(String),
}

pub fn check_file_exists(file: &str) -> Result<()> {
    if file.is_empty() || !Path::new(file).exists() {
        return Err(FileError::FileNotExists(file.to_string()).into());
    }
    Ok(())
}

pub fn get_file_name(file: &str) -> Result<String> {
    let file_name = Path::new(file)
        .file_name()
        .ok_or(FileError::FileInvalidError(file.to_string()))?;
    let file_name = file_name.to_string_lossy().to_string();
    Ok(file_name)
}

pub fn remove_file(file: &str) -> Result<()> {
    if Path::new(file).exists() {
        std::fs::remove_file(file)?;
    }
    Ok(())
}

pub fn back_file(from: &str, to: &str) -> Result<()> {
    if !Path::new(to).exists() {
        copy(from, to)?;
    } else {
        if !file_is_equal(from, to)? {
            copy(from, to)?;
        }
    }
    Ok(())
}

pub fn file_is_equal(from: &str, to: &str) -> Result<bool> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    if !from_path.exists() || !to_path.exists() {
        return Ok(false);
    }
    let from = &FileInfo::new(from);
    let to = &FileInfo::new(to);
    let from_ver = Version::new(from.get_version()?.as_str());
    let to_ver = Version::new(to.get_version()?.as_str());
    let from_size = from.get_size()?;
    let to_size = to.get_size()?;
    Ok(to_ver == from_ver && to_size == from_size)
}

