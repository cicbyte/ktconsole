# 隧道提供商 CLI 功能参考

> 本文档梳理 ngrok 和 cpolar 命令行工具支持的完整功能，为后续改造提供依据。
> 最后更新：2026-03-30

---

## 一、ngrok

**当前版本**：Agent 3.x（最新 3.23.0）

### 1.1 子命令总览

| 子命令 | 说明 |
|--------|------|
| `ngrok http [addr]` | 创建 HTTP 隧道 |
| `ngrok tcp [addr]` | 创建 TCP 隧道 |
| `ngrok tls [addr]` | 创建 TLS 隧道 |
| `ngrok start [name...]` | 从配置文件启动指定隧道 |
| `ngrok config add-authtoken` | 保存认证令牌到配置文件 |
| `ngrok config add-api-key` | 保存 API Key 到配置文件 |
| `ngrok config check` | 检查配置文件合法性 |
| `ngrok config edit` | 用编辑器打开配置文件 |
| `ngrok config upgrade` | 升级配置文件格式版本 |
| `ngrok service install/start/stop` | 安装为系统服务 |
| `ngrok diagnose` | 诊断连接问题 |
| `ngrok update` | 自更新 |
| `ngrok version` | 显示版本 |

### 1.2 `ngrok http` — HTTP 隧道

**用法**：`ngrok http [address:port | port] [flags]`

**支持的协议**：HTTP、HTTPS（通过同一个 http 子命令，自动分配 http+https 端点）

| Flag | 说明 | 状态 |
|------|------|------|
| `--url` | 指定端点 URL（替代已废弃的 `--domain`/`--subdomain`） | 推荐 |
| `--domain` | 指定域名 | 已废弃，用 `--url` |
| `--subdomain` | 指定子域名 | 已废弃，用 `--domain`→`--url` |
| `--authtoken` | 指定认证令牌 | 可用 |
| `--region` | 指定区域（auto/us/eu/au/ap/sa/jp/in） | 已废弃，自动选择最低延迟 |
| `--basic-auth` | HTTP Basic Auth | 已废弃，建议用 Traffic Policy |
| `--compression` | 启用 Gzip 压缩 | 已废弃，建议用 Traffic Policy |
| `--oauth` | OAuth 认证（Google/GitHub/Microsoft） | 已废弃，建议用 Traffic Policy |
| `--oidc` | OIDC 认证 | 已废弃，建议用 Traffic Policy |
| `--cidr-allow` | IP 白名单（CIDR） | 已废弃，建议用 Traffic Policy |
| `--cidr-deny` | IP 黑名单（CIDR） | 已废弃，建议用 Traffic Policy |
| `--circuit-breaker` | 熔断器（0.5 = 50% 失败率断开） | 已废弃，建议用 Traffic Policy |
| `--request-header-add` | 添加请求头 | 已废弃，建议用 Traffic Policy |
| `--request-header-remove` | 删除请求头 | 已废弃，建议用 Traffic Policy |
| `--response-header-add` | 添加响应头 | 已废弃，建议用 Traffic Policy |
| `--response-header-remove` | 删除响应头 | 已废弃，建议用 Traffic Policy |
| `--host-header` | 重写 Host 头 | 已废弃，建议用 Traffic Policy |
| `--inspect` | 启用/禁用 HTTP 请求检查（默认 true） | 可用 |
| `--traffic-policy-file` | Traffic Policy 配置文件路径（YAML/JSON） | 推荐 |
| `--traffic-policy-url` | Traffic Policy 配置 URL | 推荐 |
| `--pooling-enabled` | 启用负载均衡池 | 可用 |
| `--binding` | 入口绑定（如 kubernetes） | 可用 |
| `--name` | 端点名称 | 可用 |
| `--description` | 端点描述 | 可用 |
| `--metadata` | 用户自定义元数据 | 可用 |
| `--verify-webhook` | Webhook 验证（slack/strip/github 等） | 已废弃，建议用 Traffic Policy |
| `--verify-webhook-secret` | Webhook 密钥 | 已废弃 |
| `--mutual-tls-cas` | 双向 TLS CA 证书 | 已废弃，建议用 Traffic Policy |
| `--upstream-protocol` | 上游协议（http/https） | 已废弃 |
| `--upstream-proxy-protocol` | 上游代理协议 | 已废弃 |
| `--upstream-tls-verify` | 上游 TLS 验证 | 已废弃 |
| `--upstream-tls-verify-cas` | 上游 TLS CA | 已废弃 |
| `--ua-filter-allow` | User-Agent 白名单 | 已废弃 |
| `--ua-filter-deny` | User-Agent 黑名单 | 已废弃 |
| `--websocket-tcp-converter` | WebSocket→TCP 转换 | 已废弃 |
| `--config` | 配置文件路径 | 可用 |
| `--scheme` | 协议（http/https） | 已废弃，用 `--url` |

### 1.3 `ngrok tcp` — TCP 隧道

**用法**：`ngrok tcp [address:port | port] [flags]`

| Flag | 说明 | 状态 |
|------|------|------|
| `--url` | 绑定保留的 TCP 地址 | 推荐 |
| `--remote-addr` | 绑定远程地址 | 已废弃，用 `--url` |
| `--authtoken` | 认证令牌 | 可用 |
| `--region` | 区域 | 已废弃 |
| `--cidr-allow` | IP 白名单 | 可用 |
| `--cidr-deny` | IP 黑名单 | 可用 |
| `--traffic-policy-file` | Traffic Policy 文件 | 推荐 |
| `--traffic-policy-url` | Traffic Policy URL | 推荐 |
| `--pooling-enabled` | 负载均衡池 | 可用 |
| `--binding` | 入口绑定 | 可用 |
| `--name` / `--description` / `--metadata` | 元信息 | 可用 |
| `--upstream-proxy-protocol` | 上游代理协议 | 已废弃 |

### 1.4 `ngrok tls` — TLS 隧道

**用法**：`ngrok tls [address:port | port] [flags]`

| Flag | 说明 | 状态 |
|------|------|------|
| `--url` | 指定域名 | 推荐 |
| `--crt` | TLS 证书路径 | 可用 |
| `--key` | TLS 私钥路径 | 可用 |
| `--mutual-tls-cas` | 双向 TLS CA | 可用 |
| `--cidr-allow` / `--cidr-deny` | IP 过滤 | 可用 |
| 其他 | 同 TCP 的通用 flag | — |

### 1.5 ngrok Traffic Policy（流量策略）

ngrok 3.x 将大量高级功能统一到 Traffic Policy YAML 文件中，包括：

- **Basic Auth** — HTTP 基本认证
- **IP 限制** — CIDR 白名单/黑名单
- **OAuth/OIDC** — 第三方登录保护
- **压缩** — Gzip 压缩
- **请求头操作** — 添加/删除请求头和响应头
- **Host 重写** — 修改 Host 头
- **Webhook 验证** — 验证 Slack/GitHub/Stripe 等 Webhook
- **熔断器** — 按失败率断开
- **User-Agent 过滤** — UA 白名单/黑名单
- **WebSocket→TCP 转换**

### 1.6 ngrok 配置文件

路径：`~/.ngrok/ngrok.yml`（Linux/Mac）或 `%USERPROFILE%\.ngrok\ngrok.yml`（Windows）

```yaml
version: "3"
agent:
  authtoken: <TOKEN>

endpoints:
  - name: my-web
    url: https://my-app.ngrok.dev
    traffic_policy:
      on_http_request:
        - actions:
            - type: basic-auth
              config:
                credentials:
                  - "user:password"

tunnels:
  - name: my-web
    bindings:
      - proxy_proto: http
        url: https://my-app.ngrok.dev
    labels:
      - "edge=my-web"
```

### 1.7 区域

| 区域代码 | 位置 |
|----------|------|
| `auto` | 自动选择（默认） |
| `us` | 美国 |
| `eu` | 欧洲 |
| `au` | 澳大利亚 |
| `ap` | 亚太 |
| `sa` | 南美 |
| `jp` | 日本 |
| `in` | 印度 |
| `us-cal-1` | 美国加州 |
| `eu-lon-1` | 欧洲伦敦 |

### 1.8 进程管理

- **Web Interface**：默认 `http://127.0.0.1:4040`，提供流量检查面板
- **停止**：通过 API `DELETE http://127.0.0.1:4040/api/tunnels` 或直接 kill 进程
- **日志**：`--log stdout` 将日志输出到标准输出

---

## 二、cpolar

**当前版本**：3.3.18

### 2.1 子命令总览

| 子命令 | 说明 |
|--------|------|
| `cpolar http [addr]` | 创建 HTTP 隧道 |
| `cpolar tcp [addr]` | 创建 TCP 隧道 |
| `cpolar tls [addr]` | 创建 TLS 隧道 |
| `cpolar ftp [addr]` | 创建 FTP 隧道 |
| `cpolar start [name...]` | 从配置文件启动指定隧道 |
| `cpolar start-all` | 启动配置文件中所有隧道 |
| `cpolar list` | 列出配置文件中的隧道名称 |
| `cpolar authtoken` | 保存认证令牌到配置文件 |
| `cpolar service install/start/stop/uninstall` | 安装为系统服务 |
| `cpolar version` | 显示版本 |
| `cpolar help` | 帮助 |

### 2.2 `cpolar http` — HTTP 隧道

**用法**：`cpolar http [local port or address] [flags]`

**支持的协议**：HTTP、HTTPS（同一 http 子命令，`-proto` 可选 `http`/`https`/`tcp`）

| Flag | 说明 | 对应需求 |
|------|------|----------|
| `-authtoken` | 认证令牌 | 已支持（ProviderConfig.authToken） |
| `-config` | 配置文件路径 | — |
| `-region` | 区域（默认 `us`，支持 `cn`/`cn_vip`/`us`/`eu` 等） | 已支持（ProviderConfig.region） |
| `-subdomain` | 自定义子域名（HTTP only） | 待实现 |
| `-hostname` | 自定义域名（需 CNAME 解析，HTTP only） | 可映射为 customDomain |
| `-httpauth` | HTTP Basic Auth（格式 `user:password`） | 待实现 |
| `-host-header` | 重写 Host 头 | 待实现 |
| `-proto` | 协议（`http`/`https`/`tcp`，默认 `http+https`） | 已支持 |
| `-log` | 日志输出（`stdout`/`none`/文件路径） | 已使用 `-log stdout` |
| `-log-level` | 日志级别（`DEBUG`/`INFO`/`WARNING`/`ERROR`） | — |
| `-redirect-https` | HTTP 重定向到 HTTPS | 待实现 |
| `-inspect-addr` | 检查面板地址（默认 `127.0.0.1:4040`，`false` 关闭） | 待实现 |
| `-dashboard` | Web UI 开关 | — |
| `-daemon` | 后台运行 | — |
| `-processMode` | 进程模式（single/master） | — |
| `-client-cas` | TLS 客户端 CA 证书 | — |
| `-crt` | TLS 证书 | — |
| `-key` | TLS 私钥 | — |
| `-remote-addr` | 绑定保留的固定 TCP 地址 | 待实现（TCP 隧道） |
| `-tunnelName` | 隧道名称 | — |
| `-disable-keep-alives` | 禁用 Keep-Alive | — |
| `-logRotationCount` | 日志轮转数量 | — |

### 2.3 `cpolar tcp` — TCP 隧道

**用法**：`cpolar tcp [local port or address] [flags]`

与 `cpolar http` 共享全部 Flag，额外关注：

| Flag | 说明 |
|------|------|
| `-remote-addr` | 绑定保留的固定 TCP 地址（需在官网预留） |

### 2.4 `cpolar tls` — TLS 隧道

**用法**：`cpolar tls [local port or address] [flags]`

与 `cpolar http` 共享全部 Flag，额外关注：

| Flag | 说明 |
|------|------|
| `-crt` | TLS 证书路径 |
| `-key` | TLS 私钥路径 |
| `-client-cas` | 客户端 CA 证书（双向 TLS） |
| `-hostname` | 自定义域名 |

### 2.5 `cpolar ftp` — FTP 隧道

**用法**：`cpolar ftp [local port] [flags]`

共享通用 Flag。cpolar 特有功能，ngrok 不支持。

### 2.6 cpolar 配置文件

路径：
- **Windows**：`C:\Users\<用户名>\.cpolar\cpolar.yml`
- **Linux**：`/usr/local/etc/cpolar/cpolar.yml` 或 `~/.cpolar/cpolar.yml`
- **macOS**：`~/.cpolar/cpolar.yml`

```yaml
authtoken: xxxxxxxxxxxx

tunnels:
  web:
    proto: http
    addr: 8080
    subdomain: myapp
    region: cn_vip

  ssh:
    proto: tcp
    addr: 22
    region: cn_vip
    remote-addr: 1.tcp.cpolar.cn:20000

  website:
    proto: http
    addr: 5000
    hostname: dev.example.com
    region: cn_vip
    httpauth: "admin:secretpass"
```

### 2.7 区域

| 区域代码 | 位置 |
|----------|------|
| `us` | 美国（默认） |
| `eu` | 欧洲 |
| `cn` | 中国 |
| `cn_vip` | 中国 VIP |
| `cn_top` | 中国顶级线路 |
| `cn_geo` | 中国多线路 |

### 2.8 进程管理

- **Web Interface**：默认 `http://127.0.0.1:4042`（与 ngrok 的 4040 不同）
- **停止**：直接 kill 进程（Windows: `taskkill /F /IM cpolar.exe`）
- **日志**：`-log stdout` 输出到标准输出
- **后台服务**：`cpolar service install` + `cpolar service start`

---

## 三、功能对比矩阵

| 功能 | ngrok | cpolar | 我们当前支持 |
|------|-------|--------|-------------|
| **HTTP 隧道** | `ngrok http` | `cpolar http` | 两者都支持 |
| **TCP 隧道** | `ngrok tcp` | `cpolar tcp` | 仅 ngrok |
| **TLS 隧道** | `ngrok tls` | `cpolar tls` | 仅 ngrok |
| **FTP 隧道** | 不支持 | `cpolar ftp` | 不支持 |
| **UDP 隧道** | 不支持 | 不支持 | 仅类型定义 |
| **自定义域名** | `--url` | `-hostname` | 两者都已对接 |
| **子域名** | 已废弃→`--url` | `-subdomain` | cpolar 已对接 |
| **HTTP Basic Auth** | `--basic-auth`（已废弃但仍可用） | `-httpauth user:pass` | 两者都已对接 |
| **IP 白名单** | `--cidr-allow`（已废弃）/ Traffic Policy | 不支持 CLI 参数 | 不支持 |
| **压缩** | Traffic Policy（已废弃） | 不支持 | 不支持 |
| **请求检查** | `--inspect` | `-inspect-addr` | 两者都已对接（可关闭） |
| **OAuth/OIDC** | Traffic Policy（已废弃） | 不支持 | 不支持 |
| **TLS 终止** | `--crt` + `--key` | `-crt` + `-key` | 不支持 |
| **区域选择** | 已废弃（自动） | `-region` | cpolar 已对接，ngrok 自动 |
| **认证令牌** | 配置文件 / `--authtoken` | 配置文件 / `-authtoken` | cpolar CLI 传参，ngrok 用配置文件 |
| **配置文件** | `~/.ngrok/ngrok.yml`（v3 YAML） | `~/.cpolar/cpolar.yml` | 不支持 |
| **Web Interface** | `:4040` | `:4042` | 已解析 |
| **HTTP→HTTPS 重定向** | Traffic Policy | `-redirect-https` | 不支持 |
| **Host 头重写** | Traffic Policy（已废弃） | `-host-header` | 不支持 |
| **负载均衡** | `--pooling-enabled` | 不支持 | 不支持 |
| **Traffic Policy** | YAML/JSON 文件 | 不支持 | 不支持 |
| **系统服务** | `ngrok service` | `cpolar service` | 不支持 |
| **多隧道配置** | `ngrok start` | `cpolar start` | 不支持 |
| **固定 TCP 地址** | `--url` | `-remote-addr` | 不支持 |

---

## 四、当前实现 vs CLI 能力差距

### 4.1 ngrok 当前实现

**已传递的 CLI 参数**：
```
ngrok <proto> <host:port> --url <url> --basic-auth <user:pass> --inspect <true|false> --log stdout
```

**未使用但有价值的功能**：

| 功能 | Flag | 优先级 | 备注 |
|------|------|--------|------|
| ~~指定 URL~~ `--url` | ✅ 已完成 | — | 替代已废弃的 `--domain` |
| ~~Basic Auth~~ | ✅ 已完成 | — | `--basic-auth user:pass` |
| ~~请求检查开关~~ | ✅ 已完成 | — | `--inspect false` 可关闭 |
| IP 白名单 | `--cidr-allow` | 中 | 已废弃，建议用 Traffic Policy |
| Traffic Policy | `--traffic-policy-file` | 低 | 高级功能，可后期支持 |

### 4.2 cpolar 当前实现

**已传递的 CLI 参数**：
```
cpolar <proto> <addr> -subdomain <name>|-hostname <domain> -authtoken <token> -region <region> -httpauth <user:pass> -inspect-addr <addr|false> -log stdout
```

**未使用但有价值的功能**：

| 功能 | Flag | 优先级 | 备注 |
|------|------|--------|------|
| ~~子域名~~ `-subdomain` | ✅ 已完成 | — | 无点号时自动使用 |
| ~~自定义域名~~ `-hostname` | ✅ 已完成 | — | 有点号时自动使用 |
| ~~HTTP Basic Auth~~ | ✅ 已完成 | — | `-httpauth user:pass` |
| ~~区域选择~~ `-region` | ✅ 已完成 | — | 从 ProviderConfig 读取 |
| ~~认证令牌~~ `-authtoken` | ✅ 已完成 | — | 从 ProviderConfig 读取 |
| ~~请求检查~~ `-inspect-addr` | ✅ 已完成 | — | 关闭时传 `false` |
| HTTP→HTTPS 重定向 | `-redirect-https` | 中 | 安全加固 |
| Host 头重写 | `-host-header` | 低 | 虚拟主机场景 |
| TLS 证书 | `-crt` + `-key` | 低 | TLS 隧道需要 |

---

## 五、建议改造优先级

### ~~P0 — 核心功能（已有字段但未对接）~~ ✅ 已完成

1. ~~**cpolar 子域名** `-subdomain`~~ ✅
2. ~~**cpolar 自定义域名** `-hostname`~~ ✅
3. ~~**cpolar 区域选择** `-region`~~ ✅
4. ~~**cpolar 认证令牌** `-authtoken`~~ ✅

### ~~P1 — 安全与认证~~ ✅ 已完成

5. ~~**cpolar HTTP Basic Auth** `-httpauth user:pass`~~ ✅
6. ~~**ngrok `--url`** 替代已废弃的 `--domain`~~ ✅
7. ~~**ngrok Basic Auth** `--basic-auth`~~ ✅

### P2 — 高级功能（待实现）

8. **cpolar HTTP→HTTPS 重定向** `-redirect-https`
9. **ngrok Traffic Policy** `--traffic-policy-file`

### P3 — 企业级功能（远期）

10. **多隧道配置文件** `ngrok start` / `cpolar start`
11. **系统服务管理** `ngrok service` / `cpolar service`
12. **TLS 证书配置** ngrok `--crt`+`--key` / cpolar `-crt`+`-key`

### P1 — 安全与认证

4. **cpolar HTTP Basic Auth** `-httpauth user:pass`：需新增用户名/密码字段
5. **ngrok URL 替代 domain** `--url` 替代已废弃的 `--domain`

### P2 — 高级功能

6. **cpolar 固定 TCP 地址** `-remote-addr`
7. **cpolar HTTP→HTTPS 重定向** `-redirect-https`
8. **请求检查开关** ngrok `--inspect` / cpolar `-inspect-addr`

### P3 — 企业级功能

9. **ngrok Traffic Policy** `--traffic-policy-file`
10. **多隧道配置文件** `ngrok start` / `cpolar start`
11. **系统服务管理** `ngrok service` / `cpolar service`
