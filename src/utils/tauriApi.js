import { invoke } from '@tauri-apps/api/core'

export const tauriApi = {
  window: {
    minimize: () => invoke('window_minimize'),
    maximize: () => invoke('window_maximize'),
    close: () => invoke('window_close')
  },

  system: {
    getPlatform: () => invoke('get_platform'),
    getSystemInfo: () => invoke('get_system_info'),
    getSessionId: () => invoke('generate_session_id'),
  },
}
