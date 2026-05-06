//! 隧道提供商 trait 定义

use serde::{Deserialize, Serialize};

/// 隧道协议类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    HTTP,
    HTTPS,
    TCP,
    UDP,
    TLS,
}

/// 隧道状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TunnelStatus {
    Online,
    Offline,
    Starting,
    Error,
}

/// 隧道信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tunnel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub protocol: Protocol,
    pub local_port: String,
    pub public_url: String,
    pub web_interface_url: String,
    pub status: TunnelStatus,
    pub uptime: String,
    pub traffic: Vec<u32>,
    pub created_at: i64,
}

/// 隧道创建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TunnelConfig {
    pub tunnel_id: Option<String>,
    pub name: Option<String>,
    pub provider: String,
    pub protocol: Protocol,
    pub local_port: String,
    pub local_host: String,
    pub custom_domain: Option<String>,
    pub basic_auth: Option<bool>,
    pub basic_auth_user: Option<String>,
    pub basic_auth_pass: Option<String>,
    pub ip_whitelist: Option<bool>,
    pub compression: Option<bool>,
    pub inspect: Option<bool>,
}

/// 提供商安装状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStatus {
    pub id: String,
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub status: String,
}

/// 提供商 trait
pub trait TunnelProvider: Send + Sync {
    /// 提供商标识
    fn id(&self) -> &str;

    /// 提供商名称
    fn name(&self) -> &str;

    /// 检测安装状态
    fn detect(&self) -> ProviderStatus;

    /// 创建并启动隧道
    fn start_tunnel(&self, config: &TunnelConfig) -> Result<Tunnel, String>;

    /// 停止隧道
    fn stop_tunnel(&self, tunnel_id: &str) -> Result<(), String>;

    /// 测试连接
    fn test_connection(&self) -> Result<bool, String>;

    /// 获取支持的协议
    fn supported_protocols(&self) -> Vec<Protocol>;
}
