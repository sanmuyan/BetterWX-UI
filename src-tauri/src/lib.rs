mod app;
mod error;
mod structs;
mod utils;

use crate::error::MyError;
use crate::structs::config::patches::Patches;
use crate::structs::config::rules::Rule;
use crate::structs::config::Config;
use crate::structs::files_info::{FileInfo, FilesInfo};
use crate::utils::{file, process};
use tauri::Manager;

use anyhow::Result;
use tauri_plugin_window_state::{StateFlags, WindowExt};

/**
 * 检查文件是否存在
 * @param path 文件路径
 * @return 如果文件存在返回true，否则返回false
 */
#[tauri::command(async)]
fn is_files_exists(files: Vec<String>) -> bool {
    file::is_files_exists(&files)
}

/**
 * @description: 删除文件
 */
#[tauri::command(async)]
fn del_files(files: Vec<String>) -> Result<(), MyError> {
    Ok(file::del_files(files)?)
}

/**
 * @description: 备份一组文件
 */
#[tauri::command(async)]
fn backup_files(files: Vec<String>) -> Result<(), MyError> {
    Ok(file::backup_files(files)?)
}

/**
 * @description: 运行应用
 */
#[tauri::command(async)]
fn run_app(file: &str) -> Result<(), MyError> {
    Ok(process::run_app(file)?)
}

/**
 * @description: 打开网页
 */
#[tauri::command(async)]
fn open_url(url: &str) -> Result<(), MyError> {
    Ok(process::open_url(url)?)
}

/**
 * @description: 打开文件夹
 */
#[tauri::command(async)]
fn open_folder(folder: &str) -> Result<(), MyError> {
    Ok(process::open_folder(folder)?)
}

/**
 * @description: 解析 config 规则
 */
#[tauri::command(async)]
fn check_config(config: Config) -> Result<Config, MyError> {
    let mut config = config;
    app::check_config(&mut config)?;
    Ok(config)
}

/**
 * @description: 解析 rule 规则
 */
#[tauri::command(async)]
fn parse_rule(rule: Rule) -> Result<Rule, MyError> {
    let mut rule: Rule = rule;
    app::process_rule(&mut rule)?;
    Ok(rule)
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

/**
 * @description: 备份一组文件
 */
#[tauri::command(async)]
fn remove_patches_backup_files(patches: Patches) -> Result<(), MyError> {
    let files = patches.get_bak_files();
    println!("remove_patches_backup_files    {:?}", files);
    Ok(file::del_files(files)?)
}

/*
* @description: 应用补丁
* @return {*} 返回修补基址后的rule
*/
#[tauri::command(async)]
fn build_file_info_by_num(rule: Rule, num: i32) -> Result<FileInfo, MyError> {
    Ok(rule.build_file_info_by_num(num)?)
}

/*
* @description: build_feature_file_info
*/
#[tauri::command(async)]
fn build_feature_file_info(rule: Rule) -> Result<FileInfo, MyError> {
    Ok(rule.build_feature_file_info()?)
}

/*
* @description: 运行所有选中程序
*/
#[tauri::command(async)]
fn run_apps(files: Vec<String>, login: bool, close: bool) -> Result<(), MyError> {
    Ok(process::run_apps(&files, login, close)?)
}

/*
* @description: 关闭所有选中程序
*/
#[tauri::command(async)]
fn close_apps(files: Vec<String>) -> Result<(), MyError> {
    Ok(process::close_apps(&files)?)
}

/*
* @description: 创建快捷方式
*/
#[tauri::command(async)]
fn create_shortcut_to_desktop(exe: &str, name: &str, args: Option<&str>) -> Result<(), MyError> {
    Ok(file::create_shortcut_to_desktop(&exe, &name, args)?)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_config,
            parse_rule,
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
            run_apps,
            close_apps,
            build_feature_file_info,
            remove_patches_backup_files,
            create_shortcut_to_desktop
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            let _ = main_window.restore_state(StateFlags::all());

            // 获取窗口当前尺寸
            if let Ok(size) = main_window.inner_size() {
                const MIN_WIDTH: u32 = 720;
                const MIN_HEIGHT: u32 = 360;
                // 如果窗口尺寸小于最小值，则设置为最小值
                if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
                    let size = tauri::Size::Logical(tauri::LogicalSize {
                        width: MIN_WIDTH as f64,
                        height: MIN_HEIGHT as f64,
                    });
                    main_window.set_min_size(Some(size))?;
                    main_window.set_size(size)?;
                }
            }
            main_window.show()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
