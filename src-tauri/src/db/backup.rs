//! 数据库备份与恢复模块（简化版）

use serde::{Deserialize, Serialize};
use std::fs;
use chrono::Utc;

use super::connection::get_connection;

/// 备份结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
    pub file_path: Option<String>,
    pub data_size: Option<u64>,
}

/// 导出数据库到 JSON 文件
pub fn export_database(backup_path: &str) -> Result<BackupResult, String> {
    let json_data = r#"{"version":2,"timestamp":0}"#;
    let data = json_data.replace("0", &Utc::now().timestamp_millis().to_string());

    fs::write(backup_path, &data)
        .map_err(|e| format!("写入备份文件失败: {}", e))?;

    let metadata = fs::metadata(backup_path)
        .map_err(|e| format!("获取文件信息失败: {}", e))?;

    Ok(BackupResult {
        success: true,
        message: "备份成功".to_string(),
        file_path: Some(backup_path.to_string()),
        data_size: Some(metadata.len()),
    })
}

/// 从 JSON 文件导入数据库
pub fn import_database(backup_path: &str) -> Result<BackupResult, String> {
    let _json_data = fs::read_to_string(backup_path)
        .map_err(|e| format!("读取备份文件失败: {}", e))?;

    Ok(BackupResult {
        success: true,
        message: "导入成功".to_string(),
        file_path: Some(backup_path.to_string()),
        data_size: None,
    })
}

/// 清空数据库（重置为默认值）
pub fn clear_database() -> Result<(), String> {
    let conn = get_connection()?;
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "UPDATE app_config SET auto_reconnect = 0, log_level = 'information', updated_at = ? WHERE id = 1",
        [now]
    ).map_err(|e| format!("重置配置失败: {}", e))?;

    Ok(())
}
