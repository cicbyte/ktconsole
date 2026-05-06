//! 隧道管理器 - 全局状态管理

use super::provider::*;
use super::ngrok::NgrokProvider;
use super::cpolar::CpolarProvider;
use super::frp::FrpProvider;
use super::gradio::GradioProvider;
use crate::db::models::{SavedTunnel};
use crate::db::connection::get_connection;
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

/// 全局隧道管理器
static MANAGER: OnceLock<Mutex<TunnelManager>> = OnceLock::new();

pub struct TunnelManager {
    /// 运行中的隧道（ID -> Tunnel）
    tunnels: HashMap<String, Tunnel>,
    providers: HashMap<String, Box<dyn TunnelProvider>>,
    logs: Vec<LogEntry>,
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub tunnel_id: Option<String>,
}

use serde::{Deserialize, Serialize};

/// 将字符串协议解析为 Protocol 枚举
fn parse_protocol(s: &str) -> Protocol {
    match s.to_uppercase().as_str() {
        "HTTP" => Protocol::HTTP,
        "HTTPS" => Protocol::HTTPS,
        "TCP" => Protocol::TCP,
        "UDP" => Protocol::UDP,
        "TLS" => Protocol::TLS,
        _ => Protocol::HTTP,
    }
}

impl TunnelManager {
    pub fn new() -> Self {
        let mut providers: HashMap<String, Box<dyn TunnelProvider>> = HashMap::new();
        providers.insert("ngrok".to_string(), Box::new(NgrokProvider::new()));
        providers.insert("cpolar".to_string(), Box::new(CpolarProvider::new()));
        providers.insert("frp".to_string(), Box::new(FrpProvider::new()));
        providers.insert("gradio".to_string(), Box::new(GradioProvider::new()));

        Self {
            tunnels: HashMap::new(),
            providers,
            logs: Vec::new(),
        }
    }

    /// 检测所有提供商
    pub fn detect_providers(&self) -> Vec<ProviderStatus> {
        self.providers.values().map(|p| p.detect()).collect()
    }

    /// 创建并启动隧道
    pub fn start_tunnel(&mut self, config: TunnelConfig) -> Result<Tunnel, String> {
        let provider = self.providers.get(&config.provider)
            .ok_or_else(|| format!("未知提供商: {}", config.provider))?;

        let tunnel = provider.start_tunnel(&config)?;
        let tunnel_id = tunnel.id.clone();

        self.add_log("INFO", &format!("隧道 {} 正在启动...", tunnel.name), Some(&tunnel_id));

        // 持久化到数据库
        let saved = SavedTunnel {
            id: tunnel_id.clone(),
            name: tunnel.name.clone(),
            provider: tunnel.provider.clone(),
            protocol: format!("{:?}", tunnel.protocol).to_uppercase(),
            local_port: tunnel.local_port.clone(),
            local_host: config.local_host.clone(),
            custom_domain: config.custom_domain.clone(),
            basic_auth: config.basic_auth.unwrap_or(false),
            basic_auth_user: config.basic_auth_user.clone(),
            basic_auth_pass: config.basic_auth_pass.clone(),
            ip_whitelist: config.ip_whitelist.unwrap_or(false),
            compression: config.compression.unwrap_or(false),
            inspect: config.inspect.unwrap_or(false),
            created_at: tunnel.created_at,
            updated_at: chrono::Utc::now().timestamp_millis(),
        };

        if let Ok(conn) = get_connection() {
            if let Err(e) = crate::db::models::settings::save_tunnel(&conn, &saved) {
                eprintln!("保存隧道配置到数据库失败: {}", e);
            }
        }

        self.tunnels.insert(tunnel_id, tunnel.clone());
        Ok(tunnel)
    }

    /// 更新隧道公网地址（由后台输出读取线程调用）
    pub fn update_tunnel_url(&mut self, tunnel_id: &str, url: &str) {
        if let Some(tunnel) = self.tunnels.get_mut(tunnel_id) {
            tunnel.public_url = url.to_string();
            tunnel.status = TunnelStatus::Online;
        }
    }

    /// 更新隧道 Web Interface 地址（由后台输出读取线程调用）
    pub fn update_tunnel_web_interface(&mut self, tunnel_id: &str, url: &str) {
        if let Some(tunnel) = self.tunnels.get_mut(tunnel_id) {
            tunnel.web_interface_url = url.to_string();
        }
    }

    /// 停止隧道（仅停进程，DB 保留）
    pub fn stop_tunnel(&mut self, tunnel_id: &str) -> Result<(), String> {
        // 优先从内存获取运行中隧道
        let tunnel = if let Some(t) = self.tunnels.get(tunnel_id) {
            t.clone()
        } else {
            // 从 DB 获取（已离线的隧道停止无意义，但不应报错）
            return Ok(());
        };

        let provider = self.providers.get(&tunnel.provider)
            .ok_or_else(|| format!("提供商不存在: {}", tunnel.provider))?;

        provider.stop_tunnel(tunnel_id)?;
        self.add_log("INFO", &format!("隧道 {} 已停止", tunnel.name), Some(tunnel_id));

        // 从内存移除（DB 保留）
        self.tunnels.remove(tunnel_id);

        Ok(())
    }

    /// 重启隧道（从 DB 加载配置重新启动）
    pub fn restart_tunnel(&mut self, tunnel_id: &str) -> Result<Tunnel, String> {
        let conn = get_connection().map_err(|e| format!("获取数据库连接失败: {}", e))?;
        let saved = crate::db::models::settings::get_tunnel(&conn, tunnel_id)?;

        // 如果正在运行，先停止
        if self.tunnels.contains_key(tunnel_id) {
            self.stop_tunnel(tunnel_id)?;
        }

        let config = TunnelConfig {
            tunnel_id: Some(saved.id.clone()),
            name: Some(saved.name.clone()),
            provider: saved.provider.clone(),
            protocol: parse_protocol(&saved.protocol),
            local_port: saved.local_port.clone(),
            local_host: saved.local_host.clone(),
            custom_domain: saved.custom_domain.clone(),
            basic_auth: Some(saved.basic_auth),
            basic_auth_user: saved.basic_auth_user.clone(),
            basic_auth_pass: saved.basic_auth_pass.clone(),
            ip_whitelist: Some(saved.ip_whitelist),
            compression: Some(saved.compression),
            inspect: Some(saved.inspect),
        };

        let provider = self.providers.get(&config.provider)
            .ok_or_else(|| format!("未知提供商: {}", config.provider))?;

        let tunnel = provider.start_tunnel(&config)?;

        self.add_log("INFO", &format!("隧道 {} 正在重启...", tunnel.name), Some(&tunnel.id));

        self.tunnels.insert(tunnel.id.clone(), tunnel.clone());
        Ok(tunnel)
    }

    /// 获取所有隧道（合并 DB 配置 + 运行中状态）
    pub fn get_tunnels(&self) -> Vec<Tunnel> {
        let saved_tunnels = match get_connection() {
            Ok(conn) => crate::db::models::settings::get_all_tunnels(&conn).unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        saved_tunnels.into_iter().map(|s| {
            if let Some(runtime) = self.tunnels.get(&s.id) {
                runtime.clone()
            } else {
                Tunnel {
                    id: s.id,
                    name: s.name,
                    provider: s.provider,
                    protocol: parse_protocol(&s.protocol),
                    local_port: s.local_port,
                    public_url: String::new(),
                    web_interface_url: String::new(),
                    status: TunnelStatus::Offline,
                    uptime: "00:00:00".to_string(),
                    traffic: vec![0; 12],
                    created_at: s.created_at,
                }
            }
        }).collect()
    }

    /// 删除隧道（停进程 + 删 DB）
    pub fn remove_tunnel(&mut self, id: &str) -> Result<(), String> {
        // 如果正在运行，先停止
        if self.tunnels.contains_key(id) {
            let tunnel = self.tunnels.get(id).cloned().unwrap();
            let provider = self.providers.get(&tunnel.provider)
                .ok_or_else(|| format!("提供商不存在: {}", tunnel.provider))?;
            let _ = provider.stop_tunnel(id);
        }

        self.tunnels.remove(id);

        // 从 DB 删除
        if let Ok(conn) = get_connection() {
            crate::db::models::settings::delete_tunnel(&conn, id)?;
        }

        Ok(())
    }

    /// 添加日志
    pub fn add_log(&mut self, level: &str, message: &str, tunnel_id: Option<&str>) {
        let entry = LogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().format("%H:%M:%S").to_string(),
            level: level.to_string(),
            message: message.to_string(),
            tunnel_id: tunnel_id.map(|s| s.to_string()),
        };
        self.logs.push(entry);
        if self.logs.len() > 500 {
            self.logs = self.logs.split_off(self.logs.len() - 500);
        }
    }

    /// 获取日志
    pub fn get_logs(&self, tunnel_id: Option<&str>) -> Vec<LogEntry> {
        match tunnel_id {
            Some(id) => self.logs.iter().filter(|l| l.tunnel_id.as_deref() == Some(id)).cloned().collect(),
            None => self.logs.clone(),
        }
    }

    /// 清除日志
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// 测试提供商连接
    pub fn test_provider(&self, provider_id: &str) -> Result<bool, String> {
        let provider = self.providers.get(provider_id)
            .ok_or_else(|| format!("未知提供商: {}", provider_id))?;
        provider.test_connection()
    }
}

/// 初始化全局管理器
pub fn init_manager() {
    MANAGER.get_or_init(|| Mutex::new(TunnelManager::new()));
}

/// 获取管理器（用于命令调用）
pub fn with_manager<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&mut TunnelManager) -> R,
{
    let manager = MANAGER.get().ok_or("管理器未初始化")?;
    let mut guard = manager.lock().map_err(|e| format!("锁定管理器失败: {}", e))?;
    Ok(f(&mut guard))
}
