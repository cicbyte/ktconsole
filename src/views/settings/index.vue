<template>
  <div class="settings-page">
    <!-- 一级菜单 -->
    <div class="settings-menu">
      <div
        v-for="item in menuItems"
        :key="item.key"
        class="menu-item"
        :class="{ active: activeMenu === item.key }"
        @click="activeMenu = item.key"
      >
        <component :is="item.icon" class="menu-icon" />
        <span class="menu-label">{{ item.label }}</span>
      </div>
    </div>

    <!-- 二级菜单（仅提供商管理时显示） -->
    <div v-if="activeMenu === 'providers'" class="settings-submenu">
      <div class="submenu-title">提供商</div>
      <div
        v-for="p in providers"
        :key="p.id"
        class="submenu-item"
        :class="{ active: activeProvider === p.id }"
        @click="activeProvider = p.id"
      >
        <div class="submenu-provider-avatar">{{ p.name[0].toUpperCase() }}</div>
        <div class="submenu-provider-info">
          <span class="submenu-provider-name">{{ p.name }}</span>
          <span class="submenu-provider-status" :class="p.status">{{ statusLabel(p.status) }}</span>
        </div>
      </div>
    </div>

    <!-- 内容面板 -->
    <div class="settings-content">
      <!-- 通用设置 -->
      <div v-if="activeMenu === 'general'" class="content-panel">
        <h3 class="panel-title">通用设置</h3>
        <div class="form-group">
          <div class="form-item">
            <div class="form-label">
              <span>自动重连</span>
              <span class="form-desc">隧道断开后是否自动重新连接</span>
            </div>
            <a-switch v-model:checked="config.autoReconnect" />
          </div>
          <div class="form-item">
            <div class="form-label">
              <span>日志级别</span>
              <span class="form-desc">控制日志输出的详细程度</span>
            </div>
            <a-select v-model:value="config.logLevel" style="width: 200px">
              <a-select-option value="verbose">Verbose</a-select-option>
              <a-select-option value="information">Information</a-select-option>
              <a-select-option value="warning">Warning</a-select-option>
              <a-select-option value="error">Error</a-select-option>
            </a-select>
          </div>
          <div class="form-actions">
            <a-button type="primary" @click="saveConfig">保存</a-button>
          </div>
        </div>
      </div>

      <!-- 提供商详情 -->
      <div v-if="activeMenu === 'providers' && currentProvider" class="content-panel">
        <div class="provider-detail-header">
          <div class="provider-detail-avatar">{{ currentProvider.name[0].toUpperCase() }}</div>
          <div>
            <h3 class="panel-title" style="margin-bottom: 2px; border: none; padding: 0;">{{ currentProvider.name }}</h3>
            <span class="provider-status" :class="currentProvider.status">{{ statusLabel(currentProvider.status) }}</span>
          </div>
          <a-tag v-if="currentProvider.version" size="small" style="margin-left: 8px;">v{{ currentProvider.version }}</a-tag>
        </div>

        <!-- FRP 提供商配置 -->
        <div v-if="currentProvider.id === 'frp'" class="provider-config">
          <div class="config-section">
            <h4 class="config-section-title">frpc 路径</h4>
            <div class="config-field">
              <label class="config-label">可执行文件路径</label>
              <div class="path-row">
                <a-input
                  v-model:value="frpConfig.frpcPath"
                  placeholder="手动指定或点击自动检测 frpc 路径"
                  style="flex: 1"
                />
                <a-button @click="selectFrpcPath">浏览</a-button>
                <a-button @click="detectFrpc" :loading="detecting">自动检测</a-button>
              </div>
            </div>
          </div>
          <div class="config-section">
            <h4 class="config-section-title">服务器配置</h4>
            <div class="config-field">
              <label class="config-label">服务器地址</label>
              <a-input
                v-model:value="frpConfig.serverAddr"
                placeholder="FRP 服务器 IP 或域名"
              />
            </div>
            <div class="config-field">
              <label class="config-label">服务器端口</label>
              <a-input-number
                v-model:value="frpConfig.serverPort"
                :min="1"
                :max="65535"
                placeholder="默认 7000"
                style="width: 200px"
              />
            </div>
          </div>
          <div class="config-section">
            <h4 class="config-section-title">连接测试</h4>
            <div class="config-actions">
              <a-button @click="testConnection(currentProvider.id)">测试连接</a-button>
              <a-button type="primary" @click="saveFrpConfig">保存配置</a-button>
            </div>
          </div>
        </div>

        <!-- Gradio 提供商配置 -->
        <div v-else-if="currentProvider.id === 'gradio'" class="provider-config">
          <div class="config-section">
            <h4 class="config-section-title">frpc 依赖状态</h4>
            <div class="config-field">
              <label class="config-label">frpc 可执行文件</label>
              <div class="path-row">
                <a-input
                  v-model:value="gradioConfig.frpcPath"
                  placeholder="手动指定或点击自动检测 frpc 路径"
                  style="flex: 1"
                />
                <a-button @click="selectGradioFrpcPath">浏览</a-button>
                <a-button @click="detectGradioFrpc" :loading="detectingGradio">自动检测</a-button>
              </div>
            </div>
          </div>
          <div class="config-section">
            <h4 class="config-section-title">Gradio API 配置</h4>
            <div class="config-field">
              <label class="config-label">API 地址</label>
              <a-input
                v-model:value="gradioConfig.apiEndpoint"
                placeholder="https://api.gradio.app/v2/tunnel-request"
              />
            </div>
          </div>
          <div class="config-section">
            <h4 class="config-section-title">连接测试</h4>
            <div class="config-actions">
              <a-button @click="testConnection(currentProvider.id)">测试连接</a-button>
              <a-button type="primary" @click="saveGradioConfig">保存配置</a-button>
            </div>
          </div>
        </div>

        <!-- ngrok / cpolar 提供商配置 -->
        <div v-else-if="currentProvider.installed && providerConfigs[currentProvider.id]" class="provider-config">
          <div class="config-section">
            <h4 class="config-section-title">认证配置</h4>
            <div class="config-field">
              <label class="config-label">认证令牌</label>
              <a-input-password
                v-model:value="providerConfigs[currentProvider.id].authToken"
                placeholder="输入认证令牌"
              />
            </div>
          </div>
          <div class="config-section">
            <h4 class="config-section-title">区域设置</h4>
            <div class="config-field">
              <label class="config-label">区域</label>
              <a-select
                v-model:value="providerConfigs[currentProvider.id].region"
                placeholder="选择区域"
                style="width: 100%"
              >
                <a-select-option v-for="region in regions" :key="region.value" :value="region.value">
                  {{ region.label }}
                </a-select-option>
              </a-select>
            </div>
          </div>
          <div class="config-section">
            <h4 class="config-section-title">连接测试</h4>
            <div class="config-actions">
              <a-button @click="testConnection(currentProvider.id)">测试连接</a-button>
              <a-button type="primary" @click="saveProviderConfig(currentProvider.id)">保存配置</a-button>
            </div>
          </div>
        </div>
        <div v-else class="provider-not-installed">
          <p class="not-installed-text">{{ currentProvider.name }} 未安装</p>
          <p class="not-installed-hint">请先安装 {{ currentProvider.name }} 后再进行配置</p>
        </div>
      </div>

      <!-- 提供商管理无选中 -->
      <div v-if="activeMenu === 'providers' && !currentProvider" class="content-panel empty">
        <p class="empty-hint">请从左侧选择一个提供商</p>
      </div>

      <!-- 关于 -->
      <div v-if="activeMenu === 'about'" class="content-panel">
        <h3 class="panel-title">关于</h3>
        <div class="about-info">
          <div class="about-row">
            <span class="about-label">应用名称</span>
            <span class="about-value">Kinetic Console</span>
          </div>
          <div class="about-row">
            <span class="about-label">版本</span>
            <span class="about-value">v0.1.0 (Build 2026.03)</span>
          </div>
          <div class="about-row">
            <span class="about-label">许可证</span>
            <span class="about-value">MIT License</span>
          </div>
          <div class="about-row">
            <span class="about-label">开发者</span>
            <span class="about-value">CicleByte</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import {
  SettingOutlined,
  ApiOutlined,
  InfoCircleOutlined,
} from '@ant-design/icons-vue'
import { appConfigService, providerConfigService, tunnelService } from '@/services/database'
import { REGION_OPTIONS } from '@/types'
import { message } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'

const activeMenu = ref('general')
const activeProvider = ref('ngrok')
const regions = REGION_OPTIONS

const config = reactive({
  autoReconnect: false,
  logLevel: 'information',
})

const providers = ref([])
const providerConfigs = reactive({})

const frpConfig = reactive({
  serverAddr: '',
  serverPort: 7000,
  frpcPath: '',
})

const detecting = ref(false)
const detectingGradio = ref(false)

const gradioConfig = reactive({
  apiEndpoint: 'https://api.gradio.app/v2/tunnel-request',
  frpcPath: '',
})

const menuItems = [
  { key: 'general', label: '通用设置', icon: SettingOutlined },
  { key: 'providers', label: '提供商管理', icon: ApiOutlined },
  { key: 'about', label: '关于', icon: InfoCircleOutlined },
]

const currentProvider = computed(() =>
  providers.value.find(p => p.id === activeProvider.value) || null
)

const statusLabel = (status) => {
  const map = { connected: '已连接', standby: '待机', failed: '失败', not_installed: '未安装' }
  return map[status] || status
}

const saveConfig = async () => {
  try {
    await appConfigService.update({
      autoReconnect: config.autoReconnect,
      logLevel: config.logLevel,
    })
    message.success('设置已保存')
  } catch (error) {
    message.error('保存失败: ' + error)
  }
}

const testConnection = async (providerId) => {
  try {
    const result = await tunnelService.testProvider(providerId)
    message.info(result ? `${providerId} 连接成功` : `${providerId} 连接失败`)
  } catch (error) {
    message.error('测试失败: ' + error)
  }
}

const saveProviderConfig = async (providerId) => {
  try {
    const pc = providerConfigs[providerId]
    await providerConfigService.update(providerId, {
      authToken: pc.authToken,
      region: pc.region,
    })
    message.success(`${providerId} 配置已保存`)
  } catch (error) {
    message.error('保存失败: ' + error)
  }
}

const selectFrpcPath = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: '可执行文件', extensions: ['exe', ''] }],
    })
    if (selected) {
      frpConfig.frpcPath = selected
      updateFrpStatus()
    }
  } catch {
    // 用户取消选择
  }
}

const detectFrpc = async () => {
  detecting.value = true
  try {
    const results = await tunnelService.detectProviders()
    const frp = results.find(p => p.id === 'frp')
    if (frp?.path) {
      frpConfig.frpcPath = frp.path
      updateFrpStatus()
      message.success('检测到 frpc: ' + frp.path)
    } else {
      message.info('未检测到 frpc，请手动指定路径')
    }
  } catch (error) {
    message.error('检测失败: ' + error)
  } finally {
    detecting.value = false
  }
}

const updateFrpStatus = () => {
  const frp = providers.value.find(p => p.id === 'frp')
  if (frp && frpConfig.frpcPath) {
    frp.installed = true
    frp.status = 'standby'
    frp.path = frpConfig.frpcPath
  }
}

const saveFrpConfig = async () => {
  try {
    const extraConfig = JSON.stringify({
      serverAddr: frpConfig.serverAddr,
      serverPort: frpConfig.serverPort,
      frpcPath: frpConfig.frpcPath || undefined,
    })
    await providerConfigService.update('frp', { extraConfig })
    updateFrpStatus()
    message.success('FRP 配置已保存')
  } catch (error) {
    message.error('保存失败: ' + error)
  }
}

const selectGradioFrpcPath = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: '可执行文件', extensions: ['exe', ''] }],
    })
    if (selected) {
      gradioConfig.frpcPath = selected
      updateGradioStatus()
    }
  } catch {
    // 用户取消选择
  }
}

const detectGradioFrpc = async () => {
  detectingGradio.value = true
  try {
    const results = await tunnelService.detectProviders()
    const gradio = results.find(p => p.id === 'gradio')
    if (gradio?.path) {
      gradioConfig.frpcPath = gradio.path
      updateGradioStatus()
      message.success('检测到 frpc: ' + gradio.path)
    } else {
      message.info('未检测到 frpc，请手动指定路径')
    }
  } catch (error) {
    message.error('检测失败: ' + error)
  } finally {
    detectingGradio.value = false
  }
}

const updateGradioStatus = () => {
  const gradio = providers.value.find(p => p.id === 'gradio')
  if (gradio && gradioConfig.frpcPath) {
    gradio.installed = true
    gradio.status = 'standby'
    gradio.path = gradioConfig.frpcPath
  }
}

const saveGradioConfig = async () => {
  try {
    const extraConfig = JSON.stringify({
      apiEndpoint: gradioConfig.apiEndpoint,
      frpcPath: gradioConfig.frpcPath || undefined,
    })
    await providerConfigService.update('gradio', { extraConfig })
    updateGradioStatus()
    message.success('Gradio 配置已保存')
  } catch (error) {
    message.error('保存失败: ' + error)
  }
}

const loadData = async () => {
  try {
    const appConfig = await appConfigService.get()
    config.autoReconnect = appConfig.autoReconnect
    config.logLevel = appConfig.logLevel

    providers.value = await tunnelService.detectProviders()

    const pConfigs = await providerConfigService.getAll()
    for (const pc of pConfigs) {
      providerConfigs[pc.providerId] = {
        authToken: pc.authToken || '',
        region: pc.region || '',
        extraConfig: pc.extraConfig || '{}',
      }
      // 解析 FRP 扩展配置
      if (pc.providerId === 'frp' && pc.extraConfig) {
        try {
          const extra = JSON.parse(pc.extraConfig)
          frpConfig.serverAddr = extra.serverAddr || ''
          frpConfig.serverPort = extra.serverPort || 7000
          frpConfig.frpcPath = extra.frpcPath || ''
        } catch {
          // JSON 解析失败，使用默认值
        }
      }
      // 解析 Gradio 扩展配置
      if (pc.providerId === 'gradio' && pc.extraConfig) {
        try {
          const extra = JSON.parse(pc.extraConfig)
          gradioConfig.apiEndpoint = extra.apiEndpoint || 'https://api.gradio.app/v2/tunnel-request'
          gradioConfig.frpcPath = extra.frpcPath || ''
        } catch {
          // JSON 解析失败，使用默认值
        }
      }
    }
    for (const p of providers.value) {
      if (!providerConfigs[p.id]) {
        providerConfigs[p.id] = { authToken: '', region: '' }
      }
    }

    // 设置 FRP 提供商状态（手动路径视为已安装）
    updateFrpStatus()

    // 设置 Gradio 提供商状态
    updateGradioStatus()

    // 默认选中第一个提供商
    const installed = providers.value.filter(p => p.id === 'ngrok' || p.id === 'cpolar')
    if (installed.length > 0 && !activeProvider.value) {
      activeProvider.value = installed[0].id
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

onMounted(loadData)
</script>

<style scoped>
.settings-page {
  display: flex;
  height: 100%;
}

/* 一级菜单 */
.settings-menu {
  width: 160px;
  flex-shrink: 0;
  border-right: 1px solid var(--color-border);
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  color: var(--color-text-secondary);
  transition: all 0.2s ease;
  font-size: 0.85rem;
}

.menu-item:hover {
  background: var(--color-hover);
  color: var(--color-text);
}

.menu-item.active {
  background: var(--color-primary-dim);
  color: var(--color-primary);
}

.menu-icon {
  font-size: 16px;
}

/* 二级菜单（提供商列表） */
.settings-submenu {
  width: 180px;
  flex-shrink: 0;
  border-right: 1px solid var(--color-border);
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.submenu-title {
  font-size: 0.75rem;
  color: var(--color-text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  padding: 6px 12px;
  margin-bottom: 4px;
}

.submenu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.submenu-item:hover {
  background: var(--color-hover);
}

.submenu-item.active {
  background: var(--color-primary-dim);
}

.submenu-provider-avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--color-primary-dim);
  color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  font-weight: 700;
  flex-shrink: 0;
}

.submenu-provider-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.submenu-provider-name {
  font-size: 0.85rem;
  color: var(--color-text);
}

.submenu-provider-status {
  font-size: 0.65rem;
}

.submenu-provider-status.connected, .submenu-provider-status.standby {
  color: var(--color-online);
}

.submenu-provider-status.not_installed {
  color: var(--color-text-dim);
}

.submenu-provider-status.failed {
  color: var(--color-error);
}

/* 内容面板 */
.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.content-panel.empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 200px;
}

.panel-title {
  margin: 0 0 var(--spacing-md);
  font-size: 1rem;
  color: var(--color-text);
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px solid var(--color-border);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--spacing-md);
}

.form-label {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.form-label span:first-child {
  font-size: 0.85rem;
  color: var(--color-text);
}

.form-desc {
  font-size: 0.75rem;
  color: var(--color-text-dim);
}

.form-actions {
  padding-top: var(--spacing-sm);
}

/* 提供商详情 */
.provider-detail-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: var(--spacing-lg);
}

.provider-detail-header .provider-status {
  font-size: 0.7rem;
  padding: 2px 6px;
  border-radius: 4px;
}

.provider-detail-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: var(--color-primary-dim);
  color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  font-weight: 700;
  flex-shrink: 0;
}

.provider-status.connected, .provider-status.standby {
  background: rgba(0, 212, 170, 0.1);
  color: var(--color-online);
}

.provider-status.not_installed {
  background: rgba(125, 141, 161, 0.1);
  color: var(--color-text-dim);
}

.provider-status.failed {
  background: rgba(255, 107, 107, 0.1);
  color: var(--color-error);
}

.provider-config {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.config-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.config-section-title {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  margin: 0;
  padding-bottom: 6px;
  border-bottom: 1px solid var(--color-border);
}

.config-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.config-label {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
}

.path-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.config-actions {
  display: flex;
  gap: 8px;
}

.provider-not-installed {
  text-align: center;
  padding: var(--spacing-xl);
}

.not-installed-text {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  margin: 0 0 4px;
}

.not-installed-hint {
  font-size: 0.8rem;
  color: var(--color-text-dim);
  margin: 0;
}

.empty-hint {
  color: var(--color-text-dim);
  font-size: 0.85rem;
}

/* 关于 */
.about-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.about-row {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--color-border);
}

.about-label {
  color: var(--color-text-secondary);
  font-size: 0.85rem;
}

.about-value {
  color: var(--color-text);
  font-size: 0.85rem;
}

@media (max-width: 768px) {
  .settings-page {
    flex-direction: column;
  }

  .settings-menu {
    width: 100%;
    flex-direction: row;
    overflow-x: auto;
    border-right: none;
    border-bottom: 1px solid var(--color-border);
  }

  .settings-submenu {
    display: none;
  }

  .menu-label {
    display: none;
  }
}
</style>
