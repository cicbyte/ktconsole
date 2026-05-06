//! FRP 隧道提供商实现（仅设置页配置支持）

use super::provider::*;
use std::process::Command;

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

pub struct FrpProvider;

impl FrpProvider {
    pub fn new() -> Self {
        Self
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
}

impl TunnelProvider for FrpProvider {
    fn id(&self) -> &str { "frp" }
    fn name(&self) -> &str { "FRP" }

    fn detect(&self) -> ProviderStatus {
        if let Some(path) = self.find_frpc() {
            let version = self.get_version(&path);
            ProviderStatus {
                id: "frp".to_string(),
                name: "FRP".to_string(),
                installed: true,
                version,
                path: Some(path),
                status: "standby".to_string(),
            }
        } else {
            ProviderStatus {
                id: "frp".to_string(),
                name: "FRP".to_string(),
                installed: false,
                version: None,
                path: None,
                status: "not_installed".to_string(),
            }
        }
    }

    fn start_tunnel(&self, _config: &TunnelConfig) -> Result<Tunnel, String> {
        Err("FRP 隧道功能开发中".to_string())
    }

    fn stop_tunnel(&self, _tunnel_id: &str) -> Result<(), String> {
        Err("FRP 隧道功能开发中".to_string())
    }

    fn test_connection(&self) -> Result<bool, String> {
        match self.find_frpc() {
            Some(path) => {
                let mut c = Command::new(&path);
                cmd_no_window!(c);
                let output = c.arg("-v").output();
                match output {
                    Ok(o) if o.status.success() => Ok(true),
                    _ => Ok(false),
                }
            }
            None => Ok(false),
        }
    }

    fn supported_protocols(&self) -> Vec<Protocol> {
        vec![Protocol::HTTP, Protocol::HTTPS, Protocol::TCP, Protocol::UDP, Protocol::TLS]
    }
}
