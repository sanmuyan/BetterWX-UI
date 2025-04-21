use crate::structs::config::regedit::Regedit;

use anyhow::{anyhow,Result};
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

use windows_registry::LOCAL_MACHINE;

/**
 * @description: 通过注册表获取安装路径
 * @param regedit 注册表配置
 */
pub fn get_install_variables(regedit: &mut Regedit) -> Result<()> {
    //打开注册表路径
    let key = LOCAL_MACHINE.open(regedit.path.as_str())?;
    regedit.fields.0.iter_mut().try_for_each(|field| {
        //禁用跳过
        field.value = key
            .get_string(&field.value)
            .map_err(|_| return anyhow!("读取注册表字段失败: {}", &field.value))?;
        Ok(())
    })
}



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
pub fn is_files_exists(files: &Vec<String>) -> bool {
    println!("检查文件是否存在 : {:?}  : {:?}", files,&files.is_empty());
    if files.is_empty() {
        return false;
    }
    let mut result= true;
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
pub fn filter_files_is_exists(files: &Vec<String>) -> (bool,Vec<String>) {
    
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
    for  file in files {
        if !is_file_exists(&file){
            return Err(anyhow!("应用程序不存在: {}",file));
        }
        fs::remove_file( file).map_err(|_|  anyhow!("删除文件失败，文件被占用，或者以管理员模式启动"))?;
    }
    Ok(())
}

 /**
  * @description: 备份一组文件
  */
pub fn backup_files(files: Vec<String>) -> Result<()> {
    for file in files {
        if !is_file_exists(&file){
            return Err(anyhow!("文件不存在: {}",&file));
        }
        let backup_file = format!("{}.bak", &file);
        fs::copy(&file, &backup_file).map_err(|_|  anyhow!("备份文件失败，文件被占用，或者以管理员模式启动"))?;
    }
    Ok(())
}

/**
 * @description: 运行应用程序
 */
pub fn run_app(file: &str) -> Result<()> {
    if !is_file_exists(file){
        return Err(anyhow!("应用程序不存在: {}",file));
    }
    Command::new(&file)
        .spawn().map_err(|e|  anyhow!("运行应用程序失败: {}",e))?;
    Ok(())
}

/**
 * @description: 打开URL
 */
pub fn open_url(url: &str) -> Result<()> {
    Command::new("cmd.exe")
        .creation_flags(0x08000000)
        .arg("/C")
        .arg("start")
        .arg(&url)
        .spawn()
        .map_err(|e|  anyhow!("打开网址失败: {}",e))?;
    Ok(())
}

/**
 * @description: 打开文件夹
 */
pub fn open_folder(file: &str) -> Result<()> {
    Command::new("explorer")
        .args([&file])
        .spawn()
        .map_err(|e|  anyhow!("打开文件夹失败: {}",e))?;
    Ok(())
}
