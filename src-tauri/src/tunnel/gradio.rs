//! Gradio 隧道提供商实现
//! 通过 Gradio API 获取免费 FRP 服务器地址，使用 frpc 建立隧道

use super::provider::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use serde::Deserialize;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

macro_rules! cmd_no_window {
    ($cmd:expr) => {
        #[cfg(target_os = "windows")]
        $cmd.creation_flags(CREATE_NO_WINDOW);
    };
}

/// Gradio API 返回的服务器信息
#[derive(Deserialize)]
struct TunnelServer {
    host: String,
    port: u16,
}

pub struct GradioProvider {
    default_api_url: String,
}

impl GradioProvider {
    pub fn new() -> Self {
        Self {
            default_api_url: "https://api.gradio.app/v2/tunnel-request".to_string(),
        }
    }

    fn find_frpc(&self) -> Option<String> {
        let search_names = if cfg!(target_os = "windows") {
            vec!["frpc.exe", "frpc"]
        } else {
            vec!["frpc"]
        };

        for name in &search_names {
            let finder = if cfg!(target_os = "windows") { "where" } else { "which" };
            if let Ok(output) = {
                let mut c = Command::new(finder);
                cmd_no_window!(c);
                c.arg(name).output()
            } {
                if output.status.success() {
                    for line in String::from_utf8_lossy(&output.stdout).lines() {
                        let path = line.trim().to_string();
                        if !path.is_empty() && !path.ends_with(".cmd") {
                            return Some(path);
                        }
                    }
                }
            }
        }

        let candidates = if cfg!(target_os = "windows") {
            vec![
                r"C:\Program Files\frp\frpc.exe".to_string(),
                r"C:\frp\frpc.exe".to_string(),
                format!(r"{}\frp\frpc.exe", std::env::var("USERPROFILE").unwrap_or_default()),
            ]
        } else {
            vec![
                "/usr/local/bin/frpc".to_string(),
                "/usr/bin/frpc".to_string(),
                format!("{}/.local/bin/frpc", std::env::var("HOME").unwrap_or_default()),
            ]
        };

        for path in candidates {
            if std::path::Path::new(&path).exists() {
                return Some(path);
            }
        }
        None
    }

    fn get_version(&self, path: &str) -> Option<String> {
        let mut c = Command::new(path);
        cmd_no_window!(c);
        let output = c.arg("-v").output().ok()?;
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            let v = version_str.trim().to_string();
            if v.is_empty() { None } else { Some(v) }
        } else {
            None
        }
    }

    /// 调用 Gradio API 获取 FRP 服务器地址
    /// 使用独立线程执行，避免 reqwest::blocking 与 tokio 运行时冲突
    fn fetch_server_info(api_url: &str) -> Result<(String, u16), String> {
        let url = api_url.to_string();
        let handle = std::thread::spawn(move || {
            let response = reqwest::blocking::Client::new()
                .get(&url)
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .map_err(|e| format!("请求 Gradio API 失败: {}", e))?;

            if !response.status().is_success() {
                return Err(format!("Gradio API 返回错误状态: {}", response.status()));
            }

            let servers: Vec<TunnelServer> = response
                .json()
                .map_err(|e| format!("解析 Gradio API 响应失败: {}", e))?;

            let server = servers.into_iter().next()
                .ok_or("Gradio API 未返回可用的服务器地址")?;

            Ok((server.host, server.port))
        });
        handle.join().map_err(|_| "Gradio API 请求线程异常".to_string())?
    }

    /// 从数据库配置中读取 API 地址
    fn get_api_url(&self) -> String {
        if let Some(url) = Self::read_extra_config_field("apiEndpoint") {
            return url;
        }
        self.default_api_url.clone()
    }

    /// 获取 frpc 路径：优先数据库配置，再查系统 PATH
    fn get_frpc_path(&self) -> Option<String> {
        // 先从数据库读取手动配置的路径
        if let Some(path) = Self::read_extra_config_field("frpcPath") {
            if std::path::Path::new(&path).exists() {
                return Some(path);
            }
        }
        // 回退到系统检测
        self.find_frpc()
    }

    /// 从数据库 extra_config 中读取指定字段
    fn read_extra_config_field(field: &str) -> Option<String> {
        let conn = crate::db::connection::get_connection().ok()?;
        let config = crate::db::models::settings::get_provider_config(&conn, "gradio").ok()?;
        if config.extra_config.is_empty() || config.extra_config == "{}" {
            return None;
        }
        let extra = serde_json::from_str::<serde_json::Value>(&config.extra_config).ok()?;
        let value = extra.get(field)?.as_str()?.to_string();
        if value.is_empty() { None } else { Some(value) }
    }
}

impl TunnelProvider for GradioProvider {
    fn id(&self) -> &str { "gradio" }
    fn name(&self) -> &str { "Gradio" }

    fn detect(&self) -> ProviderStatus {
        if let Some(path) = self.get_frpc_path() {
            let version = self.get_version(&path);
            ProviderStatus {
                id: "gradio".to_string(),
                name: "Gradio".to_string(),
                installed: true,
                version,
                path: Some(path),
                status: "standby".to_string(),
            }
        } else {
            ProviderStatus {
                id: "gradio".to_string(),
                name: "Gradio".to_string(),
                installed: false,
                version: None,
                path: None,
                status: "not_installed".to_string(),
            }
        }
    }

    fn start_tunnel(&self, config: &TunnelConfig) -> Result<Tunnel, String> {
        let frpc_path = self.get_frpc_path()
            .ok_or("frpc 未安装，Gradio 隧道需要 frpc 客户端")?;

        let api_url = self.get_api_url();
        let (host, port) = Self::fetch_server_info(&api_url)?;

        // 生成随机 share token
        let share_token = uuid::Uuid::new_v4().to_string();

        // 构建并启动 frpc
        let mut cmd = Command::new(&frpc_path);
        cmd_no_window!(cmd);
        let mut child = cmd
            .arg("http")
            .arg("-n").arg(&share_token)
            .arg("-l").arg(&config.local_port)
            .arg("-i").arg(&config.local_host)
            .arg("--uc")
            .arg("--sd").arg("random")
            .arg("--ue")
            .arg("--server_addr").arg(format!("{}:{}", host, port))
            .arg("--disable_log_color")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("启动 frpc (Gradio) 失败: {}", e))?;

        let tunnel_id = config.tunnel_id.clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let tunnel_name = config.name.clone().unwrap_or_else(|| format!("gradio-{}", config.local_port));
        let tid = tunnel_id.clone();
        let tname = tunnel_name.clone();

        // 读取 stdout - 解析 "start proxy success: <url>"
        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    eprintln!("[gradio:{}] {}", tid, line);
                    let _ = super::manager::with_manager(|m| {
                        m.add_log("INFO", &line, Some(&tid));
                    });
                    // 解析公网地址
                    if let Some(url) = extract_gradio_url(&line) {
                        eprintln!("[gradio] 获取到公网地址: {}", url);
                        let _ = super::manager::with_manager(|m| {
                            m.update_tunnel_url(&tid, &url);
                            m.add_log("INFO", &format!("隧道 {} 已上线: {}", tname, url), Some(&tid));
                        });
                    }
                }
            });
        }

        // 读取 stderr
        if let Some(stderr) = child.stderr.take() {
            let tid2 = tunnel_id.clone();
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().flatten() {
                    eprintln!("[gradio:{}] ERR {}", tid2, line);
                    let _ = super::manager::with_manager(|m| {
                        m.add_log("WARN", &line, Some(&tid2));
                    });
                    // stderr 中也可能包含公网地址
                    if let Some(url) = extract_gradio_url(&line) {
                        eprintln!("[gradio] 从 stderr 获取到公网地址: {}", url);
                        let _ = super::manager::with_manager(|m| {
                            m.update_tunnel_url(&tid2, &url);
                        });
                    }
                }
            });
        }

        Ok(Tunnel {
            id: tunnel_id,
            name: tunnel_name,
            provider: "gradio".to_string(),
            protocol: Protocol::HTTP,
            local_port: config.local_port.clone(),
            public_url: "获取中...".to_string(),
            web_interface_url: String::new(),
            status: TunnelStatus::Starting,
            uptime: "00:00:00".to_string(),
            traffic: vec![0; 12],
            created_at: chrono::Utc::now().timestamp_millis(),
        })
    }

    fn stop_tunnel(&self, _tunnel_id: &str) -> Result<(), String> {
        if cfg!(target_os = "windows") {
            let _ = {
                let mut c = Command::new("taskkill");
                cmd_no_window!(c);
                c.args(["/F", "/IM", "frpc.exe"]).output()
            };
        } else {
            let _ = Command::new("pkill").arg("frpc").output();
        }
        Ok(())
    }

    fn test_connection(&self) -> Result<bool, String> {
        let api_url = self.get_api_url();
        match Self::fetch_server_info(&api_url) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Gradio API 连接失败: {}", e)),
        }
    }

    fn supported_protocols(&self) -> Vec<Protocol> {
        vec![Protocol::HTTP]
    }
}

/// 从 frpc 输出行中提取公网地址
/// frpc 输出格式: "start proxy success: https://xxxxx.gradio.live"
fn extract_gradio_url(line: &str) -> Option<String> {
    let marker = "start proxy success:";
    if let Some(pos) = line.find(marker) {
        let rest = line[pos + marker.len()..].trim();
        if rest.starts_with("http://") || rest.starts_with("https://") {
            let url = rest.split_whitespace().next().unwrap_or(rest).to_string();
            if !url.is_empty() {
                return Some(url);
            }
        }
    }
    None
}
