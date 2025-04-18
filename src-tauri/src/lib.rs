mod app;
mod error;
mod patch;
mod structs;
mod win;

use crate::error::MyError;
use crate::structs::config::patches::Patches;
use crate::structs::config::rules::Rule;
use crate::structs::config::Config;
use crate::structs::files_info::{FileInfo, FilesInfo};

use anyhow::Result;

/**
 * 检查文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
#[tauri::command(async)]
fn is_files_exists(files: Vec<String>) -> bool {
    win::is_files_exists(&files)
}

/**
 * @description: 删除文件
 */
#[tauri::command(async)]
fn del_files(files: Vec<String>) -> Result<(), MyError> {
    Ok(win::del_files(files)?)
}

/**
 * @description: 备份一组文件
 */
#[tauri::command(async)]
fn backup_files(files: Vec<String>) -> Result<(), MyError> {
    Ok(win::backup_files(files)?)
}

/**
 * @description: 运行应用
 */
#[tauri::command(async)]
fn run_app(file: &str) -> Result<(), MyError> {
    Ok(win::run_app(file)?)
}

/**
 * @description: 打开网页
 */
#[tauri::command(async)]
fn open_url(url: &str) -> Result<(), MyError> {
    Ok(win::open_url(url)?)
}

/**
 * @description: 打开文件夹
 */
#[tauri::command(async)]
fn open_folder(folder: &str) -> Result<(), MyError> {
    Ok(win::open_folder(folder)?)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
fn parse_config(config: Config) -> Result<Config, MyError> {
    let mut config = config;
    app::process_config(&mut config)?;
    Ok(config)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
fn search_base_address(rule: Rule) -> Result<Rule, MyError> {
    let mut rule = rule;
    app::search_base_address(&mut rule)?;
    Ok(rule)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
fn refresh_files_info(rule: Rule) -> Result<FilesInfo, MyError> {
    Ok(app::refresh_files_info(&rule)?)
}

/*
* @description: 应用补丁
* @return {*} 返回修补基址后的rule
*/
#[tauri::command(async)]
fn apply_patch(patches: Patches) -> Result<Patches, MyError> {
    let mut patches = patches;
    app::apply_patch(&mut patches)?;
    Ok(patches)
}

/*
* @description: 应用补丁
* @return {*} 返回修补基址后的rule
*/
#[tauri::command(async)]
fn build_file_info_by_num(rule: Rule, num: i32) -> Result<FileInfo, MyError> {
    Ok(rule.build_file_info_by_num(num)?)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            parse_config,
            search_base_address,
            refresh_files_info,
            apply_patch,
            build_file_info_by_num,
            is_files_exists,
            del_files,
            backup_files,
            open_url,
            run_app,
            open_folder,
        ]).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
