//! Kinetic Console - 桌面端口转发和隧道管理工具

mod db;
mod tunnel;

use std::path::PathBuf;
use std::sync::Mutex;
use sysinfo::System;

/// 全局 System 实例，用于持续采集 CPU/RAM 数据
pub static SYS_INFO: once_cell::sync::Lazy<Mutex<System>> =
    once_cell::sync::Lazy::new(|| Mutex::new(System::new_all()));

/// 打开文件所在目录
#[tauri::command]
async fn open_file_location(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path.parent().unwrap_or(&path))
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    Ok(())
}

/// 窗口最小化
#[tauri::command]
async fn window_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

/// 窗口最大化/还原
#[tauri::command]
async fn window_maximize(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().map_err(|e| e.to_string())? {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

/// 窗口关闭
#[tauri::command]
async fn window_close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

/// 获取系统平台
#[tauri::command]
fn get_platform() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    #[cfg(target_os = "linux")]
    return "linux".to_string();
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            open_file_location,
            window_minimize,
            window_maximize,
            window_close,
            get_platform,
            db::commands::db_get_app_config,
            db::commands::db_update_app_config,
            db::commands::db_get_all_provider_configs,
            db::commands::db_get_provider_config,
            db::commands::db_update_provider_config,
            db::commands::db_export_database,
            db::commands::db_import_database,
            db::commands::db_clear_database,
            tunnel::commands::detect_providers,
            tunnel::commands::create_tunnel,
            tunnel::commands::stop_tunnel,
            tunnel::commands::remove_tunnel,
            tunnel::commands::restart_tunnel,
            tunnel::commands::get_tunnels,
            tunnel::commands::get_logs,
            tunnel::commands::clear_logs,
            tunnel::commands::test_provider,
            tunnel::commands::get_system_info,
            tunnel::commands::generate_session_id,
        ])
        .setup(|_app| {
            let data_dir = dirs::data_local_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("KineticConsole");

            if let Err(e) = db::init_database(&data_dir) {
                eprintln!("初始化数据库失败: {}", e);
            }

            tunnel::manager::init_manager();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
