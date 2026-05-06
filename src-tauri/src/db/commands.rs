//! 数据库 Tauri 命令

use super::connection::get_connection;
use super::models::*;
use super::models::settings as settings_model;
use super::backup::BackupResult;

// ==================== 应用配置命令 ====================

#[tauri::command]
pub async fn db_get_app_config() -> Result<AppConfig, String> {
    let conn = get_connection()?;
    settings_model::get_app_config(&conn)
}

#[tauri::command]
pub async fn db_update_app_config(config: AppConfigUpdate) -> Result<AppConfig, String> {
    let conn = get_connection()?;
    settings_model::update_app_config(&conn, config)
}

// ==================== 提供商配置命令 ====================

#[tauri::command]
pub async fn db_get_all_provider_configs() -> Result<Vec<ProviderConfig>, String> {
    let conn = get_connection()?;
    settings_model::get_all_provider_configs(&conn)
}

#[tauri::command]
pub async fn db_get_provider_config(provider_id: String) -> Result<ProviderConfig, String> {
    let conn = get_connection()?;
    settings_model::get_provider_config(&conn, &provider_id)
}

#[tauri::command]
pub async fn db_update_provider_config(provider_id: String, config: ProviderConfigUpdate) -> Result<ProviderConfig, String> {
    let conn = get_connection()?;
    settings_model::update_provider_config(&conn, &provider_id, config)
}

// ==================== 备份与恢复命令 ====================

#[tauri::command]
pub async fn db_export_database(backup_path: String) -> Result<BackupResult, String> {
    super::backup::export_database(&backup_path)
}

#[tauri::command]
pub async fn db_import_database(backup_path: String) -> Result<BackupResult, String> {
    super::backup::import_database(&backup_path)
}

#[tauri::command]
pub async fn db_clear_database() -> Result<(), String> {
    super::backup::clear_database()
}
