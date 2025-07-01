use crate::app;
use crate::error::MyError;
use crate::structs::config::patches::Patches;
use crate::structs::config::rules::Rule;
use crate::structs::config::Config;
use crate::structs::files_info::{FileInfo, FilesInfo};
use crate::utils::{file, process};
use anyhow::Result;
/**
 * 检查文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
#[tauri::command(async)]
pub fn is_files_exists(files: Vec<String>) -> bool {
    file::is_files_exists(&files)
}

/**
 * @description: 删除文件
 */
#[tauri::command(async)]
pub fn del_files(files: Vec<String>) -> Result<(), MyError> {
    Ok(file::del_files(files)?)
}

/**
 * @description: 备份一组文件
 */
#[tauri::command(async)]
pub fn backup_files(files: Vec<String>) -> Result<(), MyError> {
    Ok(file::backup_files(files)?)
}

/**
 * @description: 运行应用
 */
#[tauri::command(async)]
pub fn run_app(file: &str) -> Result<(), MyError> {
    Ok(process::run_app(file)?)
}

/**
 * @description: 打开网页
 */
#[tauri::command(async)]
pub fn open_url(url: &str) -> Result<(), MyError> {
    Ok(process::open_url(url)?)
}

/**
 * @description: 打开文件夹
 */
#[tauri::command(async)]
pub fn open_folder(folder: &str) -> Result<(), MyError> {
    Ok(process::open_folder(folder)?)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
pub fn check_config(config: Config) -> Result<Config, MyError> {
    let mut config = config;
    app::check_config(&mut config)?;
    Ok(config)
}

/**
 * @description: 解析 rule 规则
 */
#[tauri::command(async)]
pub fn parse_rule(rule: Rule) -> Result<Rule, MyError> {
    let mut rule: Rule = rule;
    app::process_rule(&mut rule)?;
    Ok(rule)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
pub fn search_base_address(rule: Rule) -> Result<Rule, MyError> {
    let mut rule = rule;
    app::search_base_address(&mut rule)?;
    Ok(rule)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
pub fn refresh_files_info(rule: Rule) -> Result<FilesInfo, MyError> {
    Ok(app::refresh_files_info(&rule)?)
}

/*
* @description: 应用补丁
* @return {*} 返回修补基址后的rule
*/
#[tauri::command(async)]
pub fn apply_patch(patches: Patches) -> Result<Patches, MyError> {
    let mut patches = patches;
    app::apply_patch(&mut patches)?;
    Ok(patches)
}

/**
 * @description: 备份一组文件
 */
#[tauri::command(async)]
pub fn remove_patches_backup_files(patches: Patches) -> Result<(), MyError> {
    let files = patches.get_bak_files();
    println!("remove_patches_backup_files    {:?}", files);
    Ok(file::del_files(files)?)
}

/*
* @description: 应用补丁
* @return {*} 返回修补基址后的rule
*/
#[tauri::command(async)]
pub fn build_file_info_by_num(rule: Rule, num: i32) -> Result<FileInfo, MyError> {
    Ok(rule.build_file_info_by_num(num)?)
}

/*
* @description: build_feature_file_info
*/
#[tauri::command(async)]
pub fn build_feature_file_info(rule: Rule) -> Result<FileInfo, MyError> {
    Ok(rule.build_feature_file_info()?)
}

/*
* @description: 运行所有选中程序
*/
#[tauri::command(async)]
pub fn run_apps(files: Vec<String>, login: bool, close: bool) -> Result<(), MyError> {
    Ok(process::run_apps(&files, login, close)?)
}

/*
* @description: 关闭所有选中程序
*/
#[tauri::command(async)]
pub fn close_apps(files: Vec<String>) -> Result<(), MyError> {
    Ok(process::close_apps(&files)?)
}

/*
* @description: 创建快捷方式
*/
#[tauri::command(async)]
pub fn create_shortcut_to_desktop(
    exe: &str,
    name: &str,
    icon: Option<&str>,
    args: Option<&str>,
) -> Result<(), MyError> {
    Ok(file::create_shortcut_to_desktop(&exe, &name,icon, args)?)
}

/*
* @description: 创建快捷方式
*/
#[tauri::command(async)]
pub fn get_runtime_file( 
) -> Result<String, MyError> {
    Ok(process::get_runtime_file()?)
}

