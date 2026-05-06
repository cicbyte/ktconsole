English | **[中文](README.md)**

# Kinetic Console

> A desktop port forwarding and tunnel management tool — manage ngrok, cpolar, FRP, Gradio and more through a unified GUI to create, monitor, and control reverse tunnels.

## Features

- **Multi-Provider Support** — Manage ngrok, cpolar, FRP, and Gradio tunnels from a single interface with automatic local installation detection
- **Visual Tunnel Management** — Create, start, stop, restart, and remove tunnels via GUI — no CLI memorization needed
- **Multi-Protocol Coverage** — One-click configuration for HTTP, HTTPS, TCP, UDP, and TLS protocols
- **Real-Time Log Monitoring** — Filter logs by tunnel or view globally to quickly diagnose issues
- **System Dashboard** — Live CPU and memory usage stats at a glance
- **Flexible Configuration** — Custom domains, Basic Auth, IP whitelist, compression, and other advanced options
- **Data Persistence** — All configs and tunnel info stored in a local SQLite database with export/import/clear support
- **Dark Theme** — Carefully crafted dark UI for extended use without eye strain

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/) 8+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+ (with cargo)
- At least one tunneling tool installed (ngrok / cpolar / FRP)

### Clone and Run

```bash
git clone https://github.com/cicbyte/ktconsole.git
cd ktconsole
pnpm install
pnpm tauri dev
```

`pnpm tauri dev` starts both the frontend dev server (port 1420) and the Rust backend with hot-reload support.

### Production Build

```bash
pnpm tauri build
```

Build artifacts are located in `src-tauri/target/release/bundle/`, including installers and executables.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Vue 3 (Composition API) + TypeScript |
| UI Library | Ant Design Vue 4 |
| State Management | Pinia 3 |
| Build Tool | Vite 6 |
| Desktop Framework | Tauri 2 |
| Backend | Rust |
| Database | SQLite (rusqlite + r2d2 connection pool) |
| System Monitoring | sysinfo |

## Usage

### Creating a Tunnel

1. Navigate to "Tunnels" in the sidebar
2. Click the "Create Tunnel" button
3. Select a provider and protocol, then enter the local port
4. Optionally configure advanced settings (custom domain, auth, etc.)
5. Confirm — the tunnel starts automatically

### Managing Tunnels

Start, stop, restart, or remove any tunnel from the tunnel list. Status updates in real time.

### Viewing Logs

Go to the "Logs" page to filter logs by tunnel, inspect runtime details, and troubleshoot errors.

### System Settings

Configure auth tokens, regions, and other provider-specific parameters. Supports database export and import.

## Project Structure

```
src/                    # Vue 3 frontend
├── components/         # Shared components (layout, modals)
├── views/              # Pages (dashboard, tunnels, logs, settings)
├── services/           # Tauri invoke wrapper layer
├── stores/             # Pinia state management
├── router/             # Route configuration
├── types/              # TypeScript type definitions
└── utils/              # Utility functions

src-tauri/              # Rust backend
├── src/
│   ├── db/             # Database (schema, models, commands)
│   ├── tunnel/         # Tunnel management (provider abstraction, lifecycle)
│   │   ├── provider.rs # TunnelProvider trait definition
│   │   ├── manager.rs  # Tunnel manager
│   │   ├── ngrok.rs    # ngrok implementation
│   │   ├── cpolar.rs   # cpolar implementation
│   │   ├── frp.rs      # FRP implementation
│   │   └── gradio.rs   # Gradio implementation
│   └── lib.rs          # Tauri entry point, command registration
└── tauri.conf.json     # Tauri configuration
```

### Adding a New Tunnel Provider

Implement the `TunnelProvider` trait and register it in `manager.rs`:

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

Also insert a default row into the `provider_configs` table in `src-tauri/src/db/schema.rs`.

## Contributing

Issues and Pull Requests are welcome.

## License

[MIT](LICENSE)
