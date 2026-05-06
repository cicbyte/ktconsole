/**
 * Kinetic Console - 数据服务层
 * 封装所有 Tauri invoke 调用
 */
import { invoke } from '@tauri-apps/api/core'
import type {
  AppConfig,
  AppConfigUpdate,
  ProviderConfig,
  ProviderConfigUpdate,
  Tunnel,
  TunnelConfig,
  LogEntry,
  ProviderStatus,
  SystemInfo,
} from '@/types'

// ==================== 应用配置 ====================

export const appConfigService = {
  get: (): Promise<AppConfig> => invoke('db_get_app_config'),
  update: (config: AppConfigUpdate): Promise<AppConfig> => invoke('db_update_app_config', { config }),
}

// ==================== 提供商配置 ====================

export const providerConfigService = {
  getAll: (): Promise<ProviderConfig[]> => invoke('db_get_all_provider_configs'),
  get: (providerId: string): Promise<ProviderConfig> => invoke('db_get_provider_config', { providerId }),
  update: (providerId: string, config: ProviderConfigUpdate): Promise<ProviderConfig> =>
    invoke('db_update_provider_config', { providerId, config }),
}

// ==================== 隧道管理 ====================

export const tunnelService = {
  detectProviders: (): Promise<ProviderStatus[]> => invoke('detect_providers'),
  create: (config: TunnelConfig): Promise<Tunnel> => invoke('create_tunnel', { config }),
  stop: (tunnelId: string): Promise<void> => invoke('stop_tunnel', { tunnelId }),
  getAll: (): Promise<Tunnel[]> => invoke('get_tunnels'),
  remove: (tunnelId: string): Promise<void> => invoke('remove_tunnel', { tunnelId }),
  restart: (tunnelId: string): Promise<Tunnel> => invoke('restart_tunnel', { tunnelId }),
  testProvider: (providerId: string): Promise<boolean> => invoke('test_provider', { providerId }),
}

// ==================== 日志管理 ====================

export const logService = {
  get: (tunnelId?: string): Promise<LogEntry[]> => invoke('get_logs', { tunnelId: tunnelId || null }),
  clear: (): Promise<void> => invoke('clear_logs'),
}

// ==================== 系统信息 ====================

export const systemService = {
  getInfo: (): Promise<SystemInfo> => invoke('get_system_info'),
  getSessionId: (): Promise<string> => invoke('generate_session_id'),
}

// ==================== 窗口控制 ====================

export const windowService = {
  minimize: (): Promise<void> => invoke('window_minimize'),
  maximize: (): Promise<void> => invoke('window_maximize'),
  close: (): Promise<void> => invoke('window_close'),
  getPlatform: (): Promise<string> => invoke('get_platform'),
}
