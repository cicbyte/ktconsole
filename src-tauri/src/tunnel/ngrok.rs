//! ngrok 隧道提供商实现

use super::provider::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// 在 Windows 上为 Command 添加 CREATE_NO_WINDOW 标志，防止弹出控制台窗口
macro_rules! cmd_no_window {
    ($cmd:expr) => {
        #[cfg(target_os = "windows")]
        $cmd.creation_flags(CREATE_NO_WINDOW);
    };
}

pub struct NgrokProvider;

impl NgrokProvider {
    pub fn new() -> Self {
        Self
    }

    fn find_ngrok(&self) -> Option<String> {
        let search_names = if cfg!(target_os = "windows") {
            vec!["ngrok.exe", "ngrok"]
        } else {
            vec!["ngrok"]
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
            let mut paths = vec![
                r"C:\Program Files\ngrok\ngrok.exe".to_string(),
                format!(r"{}\ngrok\ngrok.exe", std::env::var("USERPROFILE").unwrap_or_default()),
                format!(r"{}\AppData\Local\ngrok\ngrok.exe", std::env::var("USERPROFILE").unwrap_or_default()),
            ];
            if let Ok(npm_root) = {
                let mut c = Command::new("npm");
                cmd_no_window!(c);
                c.args(["root", "-g"]).output()
            } {
                if npm_root.status.success() {
                    let root = String::from_utf8_lossy(&npm_root.stdout).trim().to_string();
                    paths.push(format!(r"{}\ngrok\bin\ngrok.exe", root));
                }
            }
            paths
        } else {
            vec![
                "/usr/local/bin/ngrok".to_string(),
                "/usr/bin/ngrok".to_string(),
                format!("{}/.local/bin/ngrok", std::env::var("HOME").unwrap_or_default()),
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

    /// 从输出行提取公网 URL
    /// ngrok 输出格式: Forwarding  https://xxxx.ngrok-free.app -> http://localhost:8080
    fn extract_url_from_line(line: &str) -> Option<String> {
        if !line.contains("Forwarding") && !line.contains("forwarding") {
            return None;
        }
        for prefix in &["https://", "http://", "tcp://", "tls://"] {
            if let Some(start) = line.find(prefix) {
                let rest = &line[start..];
                let end = rest.find(|c: char| c.is_whitespace() || c == '"' || c == '\'')
                    .unwrap_or(rest.len());
                let url = rest[..end].to_string();
                if url.len() > prefix.len() && !url.contains("localhost") && !url.contains("127.0.0.1") {
                    return Some(url);
                }
            }
        }
        None
    }

    /// 从输出行中提取 Web Interface 地址
    /// ngrok: Web Interface        http://127.0.0.1:4040
    fn extract_web_interface(line: &str) -> Option<String> {
        if !line.contains("Web Interface") && !line.contains("web interface") {
            return None;
        }
        for pattern in &["127.0.0.1:", "localhost:"] {
            if let Some(pos) = line.find(pattern) {
                let rest = &line[pos..];
                let end = rest.find(|c: char| c.is_whitespace())
                    .unwrap_or(rest.len());
                let addr = &rest[..end];
                if !addr.is_empty() {
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

impl TunnelProvider for NgrokProvider {
    fn id(&self) -> &str { "ngrok" }
    fn name(&self) -> &str { "ngrok" }

    fn detect(&self) -> ProviderStatus {
        if let Some(path) = self.find_ngrok() {
            let version = self.get_version(&path);
            ProviderStatus {
                id: "ngrok".to_string(),
                name: "ngrok".to_string(),
                installed: true,
                version,
                path: Some(path),
                status: "standby".to_string(),
            }
        } else {
            ProviderStatus {
                id: "ngrok".to_string(),
                name: "ngrok".to_string(),
                installed: false,
                version: None,
                path: None,
                status: "not_installed".to_string(),
            }
        }
    }

    fn start_tunnel(&self, config: &TunnelConfig) -> Result<Tunnel, String> {
        let ngrok_path = self.find_ngrok()
            .ok_or("ngrok 未安装")?;

        let proto = match config.protocol {
            Protocol::HTTP | Protocol::HTTPS => "http",
            Protocol::TCP => "tcp",
            Protocol::UDP => "udp",
            Protocol::TLS => "tls",
        };

        let addr = format!("{}:{}", config.local_host, config.local_port);

        let mut cmd = Command::new(&ngrok_path);
        cmd.arg(proto).arg(&addr);

        if let Some(ref domain) = config.custom_domain {
            // ngrok 3.x 推荐 --url，自动补全协议前缀
            match config.protocol {
                Protocol::TCP => cmd.arg("--url").arg(domain),
                _ => {
                    let url = if domain.starts_with("http://") || domain.starts_with("https://") || domain.starts_with("tcp://") || domain.starts_with("tls://") {
                        domain.clone()
                    } else {
                        format!("https://{}", domain)
                    };
                    cmd.arg("--url").arg(url)
                }
            };
        }

        // HTTP Basic Auth
        if config.basic_auth.unwrap_or(false) {
            let user = config.basic_auth_user.as_deref().unwrap_or("admin");
            let pass = config.basic_auth_pass.as_deref().unwrap_or("");
            if !pass.is_empty() {
                cmd.arg("--basic-auth").arg(format!("{}:{}", user, pass));
            }
        }

        // 请求检查
        if !config.inspect.unwrap_or(true) {
            cmd.arg("--inspect").arg("false");
        }

        // 用 --log stdout 让 ngrok 输出日志到 stdout
        cmd.arg("--log").arg("stdout");
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd_no_window!(cmd);

        let mut child = cmd.spawn()
            .map_err(|e| format!("启动 ngrok 失败: {}", e))?;

        let tunnel_id = config.tunnel_id.clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let tunnel_name = config.name.clone().unwrap_or_else(|| format!("ngrok-{}", config.local_port));
        let tid = tunnel_id.clone();
        let tname = tunnel_name.clone();

        // 读取 stdout - ngrok 会输出日志和 URL
        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    // ngrok log 格式: lvl=info msg="..." 等
                    if !line.is_empty() {
                        let _ = super::manager::with_manager(|m| {
                            m.add_log("INFO", &line, Some(&tid));
                        });
                        if let Some(url) = Self::extract_url_from_line(&line) {
                            eprintln!("[ngrok] 获取到公网地址: {}", url);
                            let _ = super::manager::with_manager(|m| {
                                m.update_tunnel_url(&tid, &url);
                                m.add_log("INFO", &format!("隧道 {} 已上线: {}", tname, url), Some(&tid));
                            });
                        }
                        if let Some(wi_url) = Self::extract_web_interface(&line) {
                            eprintln!("[ngrok] 获取到 Web Interface: {}", wi_url);
                            let _ = super::manager::with_manager(|m| {
                                m.update_tunnel_web_interface(&tid, &wi_url);
                            });
                        }
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
                    if !line.is_empty() {
                        eprintln!("[ngrok:{}] ERR {}", tid2, line);
                        let _ = super::manager::with_manager(|m| {
                            m.add_log("WARN", &line, Some(&tid2));
                        });
                    }
                }
            });
        }

        Ok(Tunnel {
            id: tunnel_id,
            name: tunnel_name,
            provider: "ngrok".to_string(),
            protocol: config.protocol.clone(),
            local_port: config.local_port.clone(),
            public_url: "获取中...".to_string(),
            web_interface_url: "http://127.0.0.1:4040".to_string(),
            status: TunnelStatus::Starting,
            uptime: "00:00:00".to_string(),
            traffic: vec![0; 12],
            created_at: chrono::Utc::now().timestamp_millis(),
        })
    }

    fn stop_tunnel(&self, _tunnel_id: &str) -> Result<(), String> {
        // 尝试通过 API 停止
        let _ = {
            let mut c = Command::new("curl");
            cmd_no_window!(c);
            c.args(["-s", "-X", "DELETE", "http://127.0.0.1:4040/api/tunnels"]).output()
        };
        // 兜底：kill 进程
        if cfg!(target_os = "windows") {
            let _ = {
                let mut c = Command::new("taskkill");
                cmd_no_window!(c);
                c.args(["/F", "/IM", "ngrok.exe"]).output()
            };
        } else {
            let _ = Command::new("pkill").arg("ngrok").output();
        }
        Ok(())
    }

    fn test_connection(&self) -> Result<bool, String> {
        match self.find_ngrok() {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn supported_protocols(&self) -> Vec<Protocol> {
        vec![Protocol::HTTP, Protocol::HTTPS, Protocol::TCP, Protocol::UDP, Protocol::TLS]
    }
}
