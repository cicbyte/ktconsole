**[English](README.en.md)** | 中文

# Kinetic Console

> 桌面端口转发与隧道管理工具 —— 统一管理 ngrok、cpolar、FRP、Gradio 等多种隧道服务，可视化创建、监控和操控内网穿透隧道。

## 功能特性

- **多提供商支持** — ngrok、cpolar、FRP、Gradio 四种隧道服务统一管理，自动检测本地安装状态
- **可视化隧道管理** — 通过图形界面创建、启动、停止、重启和删除隧道，无需记忆 CLI 参数
- **多协议覆盖** — HTTP、HTTPS、TCP、UDP、TLS 协议一键配置
- **实时日志监控** — 按隧道或全局查看运行日志，快速定位问题
- **系统状态仪表盘** — 实时展示 CPU 和内存占用，掌握系统资源状况
- **灵活配置** — 自定义域名、Basic Auth 认证、IP 白名单、数据压缩等高级选项
- **数据持久化** — 所有配置和隧道信息存储在本地 SQLite 数据库，支持导出/导入/清空
- **深色主题** — 精心设计的暗色 UI，长时间使用不疲劳

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/) 8+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+（含 cargo）
- 至少安装一种隧道工具（ngrok / cpolar / FRP）

### 克隆并运行

```bash
git clone https://github.com/cicbyte/ktconsole.git
cd ktconsole
pnpm install
pnpm tauri dev
```

`pnpm tauri dev` 会同时启动前端开发服务器（端口 1420）和 Rust 后端，支持前后端热更新。

### 构建生产版本

```bash
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`，包含安装包和可执行文件。

## 技术栈

| 层级 | 技术 |
|---|---|
| 前端框架 | Vue 3（Composition API）+ TypeScript |
| UI 组件库 | Ant Design Vue 4 |
| 状态管理 | Pinia 3 |
| 构建工具 | Vite 6 |
| 桌面框架 | Tauri 2 |
| 后端语言 | Rust |
| 数据库 | SQLite（rusqlite + r2d2 连接池） |
| 系统监控 | sysinfo |

## 使用方法

### 创建隧道

1. 在左侧导航进入「隧道管理」
2. 点击「创建隧道」按钮
3. 选择提供商、协议类型，填写本地端口
4. 按需配置自定义域名、认证等高级选项
5. 点击确认，隧道自动启动

### 管理隧道

在隧道列表中对每个隧道执行启动、停止、重启或删除操作。隧道状态实时更新。

### 查看日志

进入「实时日志」页面，可按隧道筛选日志，查看运行详情和错误信息。

### 系统设置

在「系统设置」中配置各提供商的认证令牌（Auth Token）、区域等参数，支持数据库导出和导入。

## 项目结构

```
src/                    # Vue 3 前端
├── components/         # 通用组件（布局、弹窗）
├── views/              # 页面（dashboard、tunnels、logs、settings）
├── services/           # Tauri invoke 封装层
├── stores/             # Pinia 状态管理
├── router/             # 路由配置
├── types/              # TypeScript 类型定义
└── utils/              # 工具函数

src-tauri/              # Rust 后端
├── src/
│   ├── db/             # 数据库（schema、模型、命令）
│   ├── tunnel/         # 隧道管理（提供商抽象、生命周期管理）
│   │   ├── provider.rs # TunnelProvider trait 定义
│   │   ├── manager.rs  # 隧道管理器
│   │   ├── ngrok.rs    # ngrok 实现
│   │   ├── cpolar.rs   # cpolar 实现
│   │   ├── frp.rs      # FRP 实现
│   │   └── gradio.rs   # Gradio 实现
│   └── lib.rs          # Tauri 入口，命令注册
└── tauri.conf.json     # Tauri 配置
```

### 扩展新的隧道提供商

实现 `TunnelProvider` trait 并在 `manager.rs` 中注册即可：

```rust
pub trait TunnelProvider: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn detect(&self) -> ProviderStatus;
    fn start_tunnel(&self, config: &TunnelConfig) -> Result<Tunnel, String>;
    fn stop_tunnel(&self, tunnel_id: &str) -> Result<(), String>;
    fn test_connection(&self) -> Result<bool, String>;
    fn supported_protocols(&self) -> Vec<Protocol>;
}
```

同时在 `src-tauri/src/db/schema.rs` 的 `provider_configs` 表中插入默认配置行。

## 参与贡献

欢迎提交 Issue 和 Pull Request。

## 开源许可证

[MIT](LICENSE)
