mod app;
mod commands;
mod error;
mod structs;
mod utils;

use commands::*;
use tauri::Manager;
use tauri_plugin_window_state::{StateFlags, WindowExt};

use crate::app::run_by_cmd;
use crate::utils::file::ShotCutArgs;
use crate::utils::process::create_mutex_w;
use crate::utils::win::message_box;

const MY_UI_APP_MUTEX_NAME: &str = "My_BXUI_App_Instance_Identity_Mutex_Name";

pub fn start() {
    let args: Vec<_> = std::env::args().collect();
    let cmd_args: ShotCutArgs = ShotCutArgs::from(args);
    if cmd_args.check() {
        run_without_ui(&cmd_args); 
    }else{
        run();
    }
}

pub fn run_without_ui(args: &ShotCutArgs) {
    if create_mutex_w(&format!("{}_{}",MY_UI_APP_MUTEX_NAME,args.code)) {
        let _ = message_box("错误", &format!("请等待其他 {} 快捷方式执行完毕!",args.name));
        std::process::exit(0);
    }
    if let Err(e)= run_by_cmd(args) {
         let _ = message_box("错误", &format!("执行 {} 快捷方式失败:{}",args.name,e));
    }
    std::process::exit(0);
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
            create_shortcut_to_desktop,
            get_runtime_file
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
