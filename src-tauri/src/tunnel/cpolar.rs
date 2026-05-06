//! cpolar 隧道提供商实现

use super::provider::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

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

pub struct CpolarProvider;

impl CpolarProvider {
    pub fn new() -> Self {
        Self
    }

    fn find_cpolar(&self) -> Option<String> {
        let search_names = if cfg!(target_os = "windows") {
            vec!["cpolar.exe", "cpolar"]
        } else {
            vec!["cpolar"]
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
                r"C:\Program Files\cpolar\cpolar.exe".to_string(),
                r"C:\cpolar\cpolar.exe".to_string(),
                format!(r"{}\cpolar\cpolar.exe", std::env::var("USERPROFILE").unwrap_or_default()),
            ]
        } else {
            vec![
                "/usr/local/bin/cpolar".to_string(),
                "/usr/bin/cpolar".to_string(),
                format!("{}/.local/bin/cpolar", std::env::var("HOME").unwrap_or_default()),
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
        let output = c.arg("version").output().ok()?;
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            let first_line = version_str.lines().next()?.to_string();
            Some(first_line)
        } else {
            None
        }
    }

    /// 从输出行中提取公网 URL
    /// 终端输出: Forwarding          http://4b8726ae.r23.cpolar.top -> localhost://localhost:80
    /// 日志输出: ... url=http://xxx.cpolar.top ...
    fn extract_url_from_line(line: &str) -> Option<String> {
        for prefix in &["https://", "http://", "tcp://"] {
            if let Some(pos) = line.find(prefix) {
                let rest = &line[pos + prefix.len()..];
                let end = rest.find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '-' || c == ',')
                    .unwrap_or(rest.len());
                let host = &rest[..end];
                if !host.is_empty() && !host.contains("localhost") && !host.contains("127.0.0.1") {
                    return Some(format!("{}{}", prefix, host));
                }
            }
        }
        None
    }

    /// 从输出行中提取 Web Interface 地址
    /// cpolar: Web Interface       127.0.0.1:4042
    fn extract_web_interface(line: &str) -> Option<String> {
        if !line.contains("Web Interface") && !line.contains("web interface") {
            return None;
        }
        // 查找 127.0.0.1:PORT 或 localhost:PORT
        for pattern in &["127.0.0.1:", "localhost:"] {
            if let Some(pos) = line.find(pattern) {
                let rest = &line[pos..];
                let end = rest.find(|c: char| c.is_whitespace())
                    .unwrap_or(rest.len());
                let addr = &rest[..end];
                if !addr.is_empty() {
                    // 补全为 http:// 地址
                    if addr.starts_with("http") {
                        return Some(addr.to_string());
                    }
                    return Some(format!("http://{}", addr));
                }
            }
        }
        None
    }
}

impl TunnelProvider for CpolarProvider {
    fn id(&self) -> &str { "cpolar" }
    fn name(&self) -> &str { "cpolar" }

    fn detect(&self) -> ProviderStatus {
        if let Some(path) = self.find_cpolar() {
            let version = self.get_version(&path);
            ProviderStatus {
                id: "cpolar".to_string(),
                name: "cpolar".to_string(),
                installed: true,
                version,
                path: Some(path),
                status: "standby".to_string(),
            }
        } else {
            ProviderStatus {
                id: "cpolar".to_string(),
                name: "cpolar".to_string(),
                installed: false,
                version: None,
                path: None,
                status: "not_installed".to_string(),
            }
        }
    }

    fn start_tunnel(&self, config: &TunnelConfig) -> Result<Tunnel, String> {
        let cpolar_path = self.find_cpolar()
            .ok_or("cpolar 未安装")?;

        let proto = match config.protocol {
            Protocol::HTTP | Protocol::HTTPS => "http",
            Protocol::TCP => "tcp",
            Protocol::TLS => "tls",
            _ => "http",
        };

        let addr = format!("{}:{}", config.local_host, config.local_port);

        let mut cmd = Command::new(&cpolar_path);
        cmd.arg(proto).arg(&addr);

        // 自定义域名 / 子域名（仅 HTTP 协议）
        if let Some(ref domain) = config.custom_domain {
            if proto == "http" {
                if domain.contains('.') {
                    cmd.arg("-hostname").arg(domain);
                } else {
                    cmd.arg("-subdomain").arg(domain);
                }
            } else if proto == "tcp" {
                cmd.arg("-remote-addr").arg(domain);
            }
        }

        // 从 ProviderConfig 读取 authtoken 和 region
        if let Ok(conn) = crate::db::connection::get_connection() {
            if let Ok(pc) = crate::db::models::settings::get_provider_config(&conn, "cpolar") {
                if !pc.auth_token.is_empty() {
                    cmd.arg("-authtoken").arg(&pc.auth_token);
                }
                if !pc.region.is_empty() {
                    cmd.arg("-region").arg(&pc.region);
                }
            }
        }

        // HTTP Basic Auth
        if config.basic_auth.unwrap_or(false) {
            let user = config.basic_auth_user.as_deref().unwrap_or("admin");
            let pass = config.basic_auth_pass.as_deref().unwrap_or("");
            if !pass.is_empty() {
                cmd.arg("-httpauth").arg(format!("{}:{}", user, pass));
            }
        }

        // 请求检查
        if !config.inspect.unwrap_or(true) {
            cmd.arg("-inspect-addr").arg("false");
        }

        cmd.arg("-log").arg("stdout");
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd_no_window!(cmd);

        let mut child = cmd.spawn()
            .map_err(|e| format!("启动 cpolar 失败: {}", e))?;

        let tunnel_id = config.tunnel_id.clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let tunnel_name = config.name.clone().unwrap_or_else(|| format!("cpolar-{}", config.local_port));
        let tid = tunnel_id.clone();
        let tname = tunnel_name.clone();

        // 读取 stdout
        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    eprintln!("[cpolar:{}] {}", tid, line);
                    let _ = super::manager::with_manager(|m| {
                        m.add_log("INFO", &line, Some(&tid));
                    });
                    if let Some(url) = Self::extract_url_from_line(&line) {
                        eprintln!("[cpolar] 获取到公网地址: {}", url);
                        let _ = super::manager::with_manager(|m| {
                            m.update_tunnel_url(&tid, &url);
                            m.add_log("INFO", &format!("隧道 {} 已上线: {}", tname, url), Some(&tid));
                        });
                    }
                    if let Some(wi_url) = Self::extract_web_interface(&line) {
                        eprintln!("[cpolar] 获取到 Web Interface: {}", wi_url);
                        let _ = super::manager::with_manager(|m| {
                            m.update_tunnel_web_interface(&tid, &wi_url);
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
                    eprintln!("[cpolar:{}] ERR {}", tid2, line);
                    let _ = super::manager::with_manager(|m| {
                        m.add_log("WARN", &line, Some(&tid2));
                    });
                }
            });
        }

        Ok(Tunnel {
            id: tunnel_id,
            name: tunnel_name,
            provider: "cpolar".to_string(),
            protocol: config.protocol.clone(),
            local_port: config.local_port.clone(),
            public_url: "获取中...".to_string(),
            web_interface_url: "http://127.0.0.1:4042".to_string(),
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
                c.args(["/F", "/IM", "cpolar.exe"]).output()
            };
        } else {
            let _ = Command::new("pkill").arg("cpolar").output();
        }
        Ok(())
    }

    fn test_connection(&self) -> Result<bool, String> {
        match self.find_cpolar() {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn supported_protocols(&self) -> Vec<Protocol> {
        vec![Protocol::HTTP, Protocol::TCP]
    }
}
