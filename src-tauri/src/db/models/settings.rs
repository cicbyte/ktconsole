//! 应用配置模型

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// 应用配置（单例模式）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub id: i32,
    pub auto_reconnect: bool,
    pub log_level: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 应用配置更新
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigUpdate {
    pub auto_reconnect: Option<bool>,
    pub log_level: Option<String>,
}

/// 提供商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfig {
    pub id: i32,
    pub provider_id: String,
    pub name: String,
    pub auth_token: String,
    pub region: String,
    pub extra_config: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 提供商配置更新
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfigUpdate {
    pub auth_token: Option<String>,
    pub region: Option<String>,
    pub extra_config: Option<String>,
}

/// 获取应用配置
pub fn get_app_config(conn: &Connection) -> Result<AppConfig, String> {
    conn.query_row(
        "SELECT id, auto_reconnect, log_level, created_at, updated_at
         FROM app_config WHERE id = 1",
        [],
        |row| Ok(AppConfig {
            id: row.get(0)?,
            auto_reconnect: row.get::<_, i32>(1)? != 0,
            log_level: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    ).map_err(|e| format!("获取应用配置失败: {}", e))
}

/// 更新应用配置
pub fn update_app_config(conn: &Connection, update: AppConfigUpdate) -> Result<AppConfig, String> {
    let now = Utc::now().timestamp_millis();
    let current = get_app_config(conn)?;

    let auto_reconnect = update.auto_reconnect.unwrap_or(current.auto_reconnect);
    let log_level = update.log_level.unwrap_or(current.log_level);

    conn.execute(
        "UPDATE app_config SET auto_reconnect = ?, log_level = ?, updated_at = ? WHERE id = 1",
        params![auto_reconnect as i32, log_level, now]
    ).map_err(|e| format!("更新应用配置失败: {}", e))?;

    get_app_config(conn)
}

/// 获取所有提供商配置
pub fn get_all_provider_configs(conn: &Connection) -> Result<Vec<ProviderConfig>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, provider_id, name, auth_token, region, extra_config, created_at, updated_at
         FROM provider_configs ORDER BY id"
    ).map_err(|e| format!("查询提供商配置失败: {}", e))?;

    let configs = stmt.query_map([], |row| Ok(ProviderConfig {
        id: row.get(0)?,
        provider_id: row.get(1)?,
        name: row.get(2)?,
        auth_token: row.get(3)?,
        region: row.get(4)?,
        extra_config: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })).map_err(|e| format!("查询提供商配置失败: {}", e))?
    .collect::<Result<Vec<_>, _>>().map_err(|e| format!("解析提供商配置失败: {}", e))?;

    Ok(configs)
}

/// 获取单个提供商配置
pub fn get_provider_config(conn: &Connection, provider_id: &str) -> Result<ProviderConfig, String> {
    conn.query_row(
        "SELECT id, provider_id, name, auth_token, region, extra_config, created_at, updated_at
         FROM provider_configs WHERE provider_id = ?",
        [provider_id],
        |row| Ok(ProviderConfig {
            id: row.get(0)?,
            provider_id: row.get(1)?,
            name: row.get(2)?,
            auth_token: row.get(3)?,
            region: row.get(4)?,
            extra_config: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    ).map_err(|e| format!("获取提供商配置失败: {}", e))
}

/// 已保存的隧道配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedTunnel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub protocol: String,
    pub local_port: String,
    pub local_host: String,
    pub custom_domain: Option<String>,
    pub basic_auth: bool,
    pub basic_auth_user: Option<String>,
    pub basic_auth_pass: Option<String>,
    pub ip_whitelist: bool,
    pub compression: bool,
    pub inspect: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 保存隧道配置
pub fn save_tunnel(conn: &Connection, tunnel: &SavedTunnel) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO tunnels (id, name, provider, protocol, local_port, local_host, custom_domain, basic_auth, basic_auth_user, basic_auth_pass, ip_whitelist, compression, inspect, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            tunnel.id,
            tunnel.name,
            tunnel.provider,
            tunnel.protocol,
            tunnel.local_port,
            tunnel.local_host,
            tunnel.custom_domain,
            tunnel.basic_auth as i32,
            tunnel.basic_auth_user,
            tunnel.basic_auth_pass,
            tunnel.ip_whitelist as i32,
            tunnel.compression as i32,
            tunnel.inspect as i32,
            tunnel.created_at,
            tunnel.updated_at,
        ]
    ).map_err(|e| format!("保存隧道配置失败: {}", e))?;

    Ok(())
}

/// 获取所有隧道配置
pub fn get_all_tunnels(conn: &Connection) -> Result<Vec<SavedTunnel>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, name, provider, protocol, local_port, local_host, custom_domain, basic_auth, basic_auth_user, basic_auth_pass, ip_whitelist, compression, inspect, created_at, updated_at
         FROM tunnels ORDER BY created_at"
    ).map_err(|e| format!("查询隧道配置失败: {}", e))?;

    let tunnels = stmt.query_map([], |row| Ok(SavedTunnel {
        id: row.get(0)?,
        name: row.get(1)?,
        provider: row.get(2)?,
        protocol: row.get(3)?,
        local_port: row.get(4)?,
        local_host: row.get(5)?,
        custom_domain: row.get(6)?,
        basic_auth: row.get::<_, i32>(7)? != 0,
        basic_auth_user: row.get(8)?,
        basic_auth_pass: row.get(9)?,
        ip_whitelist: row.get::<_, i32>(10)? != 0,
        compression: row.get::<_, i32>(11)? != 0,
        inspect: row.get::<_, i32>(12)? != 0,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })).map_err(|e| format!("查询隧道配置失败: {}", e))?
    .collect::<Result<Vec<_>, _>>().map_err(|e| format!("解析隧道配置失败: {}", e))?;

    Ok(tunnels)
}

/// 获取单个隧道配置
pub fn get_tunnel(conn: &Connection, id: &str) -> Result<SavedTunnel, String> {
    conn.query_row(
        "SELECT id, name, provider, protocol, local_port, local_host, custom_domain, basic_auth, basic_auth_user, basic_auth_pass, ip_whitelist, compression, inspect, created_at, updated_at
         FROM tunnels WHERE id = ?",
        [id],
        |row| Ok(SavedTunnel {
            id: row.get(0)?,
            name: row.get(1)?,
            provider: row.get(2)?,
            protocol: row.get(3)?,
            local_port: row.get(4)?,
            local_host: row.get(5)?,
            custom_domain: row.get(6)?,
            basic_auth: row.get::<_, i32>(7)? != 0,
            basic_auth_user: row.get(8)?,
            basic_auth_pass: row.get(9)?,
            ip_whitelist: row.get::<_, i32>(10)? != 0,
            compression: row.get::<_, i32>(11)? != 0,
            inspect: row.get::<_, i32>(12)? != 0,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
        })
    ).map_err(|e| format!("获取隧道配置失败: {}", e))
}

/// 删除隧道配置
pub fn delete_tunnel(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute(
        "DELETE FROM tunnels WHERE id = ?",
        [id]
    ).map_err(|e| format!("删除隧道配置失败: {}", e))?;
    Ok(())
}

/// 更新提供商配置
pub fn update_provider_config(conn: &Connection, provider_id: &str, update: ProviderConfigUpdate) -> Result<ProviderConfig, String> {
    let now = Utc::now().timestamp_millis();
    let current = get_provider_config(conn, provider_id)?;

    let auth_token = update.auth_token.unwrap_or(current.auth_token);
    let region = update.region.unwrap_or(current.region);
    let extra_config = update.extra_config.unwrap_or(current.extra_config);

    conn.execute(
        "UPDATE provider_configs SET auth_token = ?, region = ?, extra_config = ?, updated_at = ? WHERE provider_id = ?",
        params![auth_token, region, extra_config, now, provider_id]
    ).map_err(|e| format!("更新提供商配置失败: {}", e))?;

    get_provider_config(conn, provider_id)
}
