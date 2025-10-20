pub mod apis;
pub mod errors;
use services::process::process_run_by_cmd;
use tauri::Manager;
use utils::shortcut::ShortCutArgs;
use winsys::process::mutex::Mutex;
use winsys::win::keep_running;
use winsys::win::message_box;
use setting::DEBUG_MODEL;

const MY_UI_APP_MUTEX_NAME: &str = "My_BXUI_App_Instance_Identity_Mutex_Name";

pub fn run() {
    let args: Vec<_> = std::env::args().collect();
    let cmd_args: ShortCutArgs = ShortCutArgs::from(args);
    if DEBUG_MODEL {
        let _ = logger::init(Some("debug"));
    } else {
        let _ = logger::init(cmd_args.level.clone());
    }
    if cmd_args.check() {
        run_without_ui(&cmd_args);
    } else {
        run_with_ui();
    }
    keep_running();
}

pub fn run_without_ui(args: &ShortCutArgs) {
    let mutex_name = format!("{}_{}", MY_UI_APP_MUTEX_NAME, args.code);
    if let Err(_) = Mutex::create(mutex_name) {
        let _ = message_box(
            "错误",
            &format!("请等待其他 {} 快捷方式执行完毕!", args.name),
        );
        std::process::exit(1);
    }
    if let Err(e) = process_run_by_cmd(args) {
        let _ = message_box("错误", &format!("执行 {} 快捷方式失败:{}", args.name, e));
    }
    std::process::exit(1);
    //keep_running();
}

pub fn run_with_ui() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .invoke_handler(tauri::generate_handler![
            apis::update::update_check,
            apis::update::update_config_check,
            apis::update::update_readme_check,
            apis::rule::rule_get_path,
            apis::rule::rule_search_address,
            apis::rule::rule_walk_files,
            apis::rule::rule_patch,
            apis::rule::rule_make_coexist,
            apis::rule::rule_del_coexist,
            apis::rule::rule_read_orignal,
            apis::rule::rule_patch_by_replace,
            apis::cmd::cmd_close_app,
            apis::cmd::cmd_run_app,
            apis::cmd::cmd_open_url,
            apis::cmd::cmd_open_folder,
            apis::shortcut::shortcut_to_desktop,
            apis::shortcut::shortcut_to_startup,
            apis::process::process_run_app,
            apis::process::process_close_app,
            apis::process::process_run_apps,
            apis::process::process_close_apps,
            apis::store::store_read,
            apis::store::store_save,
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.show()?;
            let window_clone = main_window.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(15));
                let _ = window_clone.close();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
use ctor::ctor;

#[cfg(test)]
#[ctor]
fn test_init() {
    let _ = logger::init(Some("debug"));
}
