//! 隧道管理 Tauri 命令

use super::provider::{TunnelConfig, Tunnel, ProviderStatus};
use super::manager::{LogEntry, with_manager};

/// 检测所有提供商
#[tauri::command]
pub async fn detect_providers() -> Result<Vec<ProviderStatus>, String> {
    with_manager(|m| m.detect_providers())
}

/// 创建并启动隧道
#[tauri::command]
pub async fn create_tunnel(config: TunnelConfig) -> Result<Tunnel, String> {
    with_manager(|m| m.start_tunnel(config))?
}

/// 停止隧道
#[tauri::command]
pub async fn stop_tunnel(tunnel_id: String) -> Result<(), String> {
    with_manager(|m| m.stop_tunnel(&tunnel_id))?
}

/// 删除隧道
#[tauri::command]
pub async fn remove_tunnel(tunnel_id: String) -> Result<(), String> {
    with_manager(|m| m.remove_tunnel(&tunnel_id))?
}

/// 重启隧道
#[tauri::command]
pub async fn restart_tunnel(tunnel_id: String) -> Result<Tunnel, String> {
    with_manager(|m| m.restart_tunnel(&tunnel_id))?
}

/// 获取所有隧道
#[tauri::command]
pub async fn get_tunnels() -> Result<Vec<Tunnel>, String> {
    with_manager(|m| m.get_tunnels())
}

/// 获取日志
#[tauri::command]
pub async fn get_logs(tunnel_id: Option<String>) -> Result<Vec<LogEntry>, String> {
    with_manager(|m| m.get_logs(tunnel_id.as_deref()))
}

/// 清除日志
#[tauri::command]
pub async fn clear_logs() -> Result<(), String> {
    with_manager(|m| m.clear_logs())
}

/// 测试提供商连接
#[tauri::command]
pub async fn test_provider(provider_id: String) -> Result<bool, String> {
    with_manager(|m| m.test_provider(&provider_id))?
}

/// 获取系统信息（CPU/RAM）
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    let mut sys = crate::SYS_INFO.lock().map_err(|e| e.to_string())?;
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_usage() as u32;
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let mem_percent = if total_mem > 0.0 { (used_mem / total_mem * 100.0) as u32 } else { 0 };

    Ok(serde_json::json!({
        "cpuUsage": cpu_usage,
        "ramUsage": mem_percent,
        "ramTotal": format!("{:.1} GB", total_mem),
        "ramUsed": format!("{:.1} GB", used_mem),
    }))
}

/// 生成 Session ID
#[tauri::command]
pub async fn generate_session_id() -> Result<String, String> {
    use chrono::Utc;
    let now = Utc::now();
    let ts = now.timestamp_millis() as u32;
    let prefix = "KNT";
    let s1 = format!("{:04}", ts % 10000);
    let s2 = format!("{}", (ts / 100) % 10);
    let s3 = format!("{:04}", (ts / 1000) % 10000);
    Ok(format!("{}-{}-{}-{}", prefix, s1, s2, s3))
}
