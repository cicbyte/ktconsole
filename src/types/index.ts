// Kinetic Console - Type Definitions

/** 隧道协议类型 */
export type Protocol = 'HTTP' | 'HTTPS' | 'TCP' | 'UDP' | 'TLS'

/** 隧道状态 */
export type TunnelStatus = 'online' | 'offline' | 'starting' | 'error'

/** 日志级别 */
export type LogLevel = 'INFO' | 'DEBUG' | 'WARN' | 'ERROR'

/** 提供商连接状态 */
export type ProviderConnectStatus = 'connected' | 'failed' | 'standby' | 'not_installed'

/** 隧道信息 */
export interface Tunnel {
  id: string
  name: string
  provider: string
  protocol: Protocol
  localPort: string
  publicUrl: string
  webInterfaceUrl: string
  status: TunnelStatus
  uptime: string
  traffic: number[]
  createdAt: number
}

/** 隧道创建配置 */
export interface TunnelConfig {
  tunnelId?: string
  name?: string
  provider: string
  protocol: Protocol
  localPort: string
  localHost: string
  customDomain?: string
  basicAuth?: boolean
  basicAuthUser?: string
  basicAuthPass?: string
  ipWhitelist?: boolean
  compression?: boolean
  inspect?: boolean
}

/** 日志条目 */
export interface LogEntry {
  id: string
  timestamp: string
  level: LogLevel
  message: string
  tunnelId?: string
}

/** 提供商状态 */
export interface ProviderStatus {
  id: string
  name: string
  installed: boolean
  version?: string
  path?: string
  status: ProviderConnectStatus
}

/** 应用配置 */
export interface AppConfig {
  id: number
  autoReconnect: boolean
  logLevel: 'verbose' | 'information' | 'warning' | 'error'
  createdAt: number
  updatedAt: number
}

/** 应用配置更新 */
export interface AppConfigUpdate {
  autoReconnect?: boolean
  logLevel?: string
}

/** 提供商配置 */
export interface ProviderConfig {
  id: number
  providerId: string
  name: string
  authToken: string
  region: string
  extraConfig: string
  createdAt: number
  updatedAt: number
}

/** 提供商配置更新 */
export interface ProviderConfigUpdate {
  authToken?: string
  region?: string
  extraConfig?: string
}

/** FRP 扩展配置 */
export interface FrpExtraConfig {
  serverAddr: string
  serverPort: number
  frpcPath?: string
}

/** Gradio 扩展配置 */
export interface GradioExtraConfig {
  apiEndpoint: string
}

/** 系统信息 */
export interface SystemInfo {
  cpuUsage: number
  ramUsage: number
  ramTotal: string
  ramUsed: string
}

/** 统计卡片数据 */
export interface StatCard {
  title: string
  value: string | number
  total?: string
  unit?: string
  icon: string
  color: string
}

/** 路由页面标识 */
export type ViewRoute = 'dashboard' | 'tunnels' | 'logs' | 'settings'

/** 区域选项 */
export interface RegionOption {
  value: string
  label: string
}

/** 提供商区域选项 */
export const REGION_OPTIONS: RegionOption[] = [
  { value: 'us-east-2', label: '美国 - 俄亥俄' },
  { value: 'eu-central-1', label: '欧洲 - 法兰克福' },
  { value: 'ap-northeast-1', label: '亚太 - 东京' },
  { value: 'ap-southeast-1', label: '亚太 - 新加坡' },
]
