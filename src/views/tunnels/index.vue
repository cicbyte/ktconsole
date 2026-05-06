<template>
  <div class="tunnels-page">
    <div class="page-header">
      <h2 class="page-title">隧道管理</h2>
      <a-button v-if="tunnels.length > 0" type="primary" @click="showCreateModal = true">
        <template #icon><PlusOutlined /></template>
        新建隧道
      </a-button>
    </div>

    <!-- Tunnel Grid -->
    <div v-if="tunnels.length > 0" class="tunnel-grid">
      <div v-for="tunnel in tunnels" :key="tunnel.id" class="tunnel-card">
        <!-- Provider Header -->
        <div class="card-header">
          <div class="provider-badge">
            <div class="provider-avatar">{{ tunnel.provider[0].toUpperCase() }}</div>
            <div class="provider-info">
              <span class="provider-name">{{ tunnel.provider }}</span>
              <span class="provider-label">{{ tunnel.name }}</span>
            </div>
          </div>
          <div class="card-header-right">
            <a-tag :color="getProtocolColor(tunnel.protocol)" size="small">{{ tunnel.protocol }}</a-tag>
            <a-popconfirm
              title="确定删除此隧道？"
              ok-text="删除"
              cancel-text="取消"
              @confirm="deleteTunnel(tunnel)"
            >
              <a-button type="text" size="small" class="delete-btn">
                <template #icon><DeleteOutlined /></template>
              </a-button>
            </a-popconfirm>
          </div>
        </div>

        <!-- Card Body -->
        <div class="card-body">
          <div class="info-row">
            <span class="info-label">本地端口</span>
            <span class="info-value mono">{{ tunnel.localPort }}</span>
          </div>
          <div class="info-row public-url-row">
            <span class="info-label">公网地址</span>
            <span class="info-value mono public-url-value" :title="tunnel.publicUrl || '--'">{{ tunnel.publicUrl || '--' }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">运行时间</span>
            <span class="info-value mono">{{ formatUptime(tunnel) }}</span>
          </div>
        </div>

        <!-- Card Footer -->
        <div class="card-footer">
          <div class="footer-left">
            <a-tooltip title="访问链接">
              <a-button type="text" size="small" :disabled="!tunnel.publicUrl" @click="handleOpenUrl(tunnel.publicUrl)">
                <template #icon><LinkOutlined /></template>
              </a-button>
            </a-tooltip>
            <a-tooltip title="复制地址">
              <a-button type="text" size="small" :disabled="!tunnel.publicUrl" @click="copyUrl(tunnel.publicUrl)">
                <template #icon><CopyOutlined /></template>
              </a-button>
            </a-tooltip>
            <a-tooltip title="查看日志">
              <a-button type="text" size="small" @click="viewLogs(tunnel)">
                <template #icon><FileTextOutlined /></template>
              </a-button>
            </a-tooltip>
            <a-tooltip v-if="tunnel.webInterfaceUrl" title="Web 管理界面">
              <a-button type="text" size="small" class="web-interface-btn" @click="handleOpenUrl(tunnel.webInterfaceUrl)">
                <template #icon><GlobalOutlined /></template>
              </a-button>
            </a-tooltip>
          </div>
          <div class="status-indicator" :class="tunnel.status">
            <span class="status-dot"></span>
            <span class="status-text">{{ tunnel.status === 'online' ? '在线' : '离线' }}</span>
          </div>
          <a-button type="text" size="small"
            @click="toggleTunnel(tunnel)"
            :class="{ 'stop-btn': tunnel.status === 'online' }">
            <template #icon>
              <StopOutlined v-if="tunnel.status === 'online'" />
              <CaretRightOutlined v-else />
            </template>
          </a-button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="empty-state">
      <div class="empty-illustration">
        <ApiOutlined class="empty-icon" />
      </div>
      <h3 class="empty-title">还没有隧道</h3>
      <p class="empty-desc">创建一个隧道，将本地服务暴露到公网</p>
      <a-button type="primary" @click="showCreateModal = true">
        <template #icon><PlusOutlined /></template>
        新建隧道
      </a-button>
    </div>

    <CreateTunnelModal v-model:open="showCreateModal" @created="loadTunnels" />
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { tunnelService } from '@/services/database'
import { message } from 'ant-design-vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import {
  PlusOutlined,
  ApiOutlined,
  FileTextOutlined,
  StopOutlined,
  CaretRightOutlined,
  DeleteOutlined,
  GlobalOutlined,
  LinkOutlined,
  CopyOutlined,
} from '@ant-design/icons-vue'
import CreateTunnelModal from '@/components/CreateTunnelModal.vue'

const router = useRouter()
const tunnels = ref([])
const showCreateModal = ref(false)

const now = ref(Date.now())

const formatUptime = (tunnel) => {
  if (tunnel.status !== 'online') return '00:00:00'
  const elapsed = Math.max(0, Math.floor((now.value - tunnel.createdAt) / 1000))
  const h = Math.floor(elapsed / 3600)
  const m = Math.floor((elapsed % 3600) / 60)
  const s = elapsed % 60
  return `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
}

const getProtocolColor = (protocol) => {
  const colors = { HTTP: 'green', HTTPS: 'blue', TCP: 'orange', UDP: 'purple', TLS: 'cyan' }
  return colors[protocol] || 'default'
}

const viewLogs = (tunnel) => {
  router.push({ path: '/logs', query: { tunnelId: tunnel.id } })
}

const toggleTunnel = async (tunnel) => {
  try {
    if (tunnel.status === 'online') {
      await tunnelService.stop(tunnel.id)
      message.success(`隧道 ${tunnel.name} 已停止`)
    } else {
      // 离线隧道：直接重启（从持久化配置加载）
      await tunnelService.restart(tunnel.id)
      message.success(`隧道 ${tunnel.name} 已启动`)
    }
    await loadTunnels()
  } catch (error) {
    message.error('操作失败: ' + error)
  }
}

const deleteTunnel = async (tunnel) => {
  try {
    await tunnelService.remove(tunnel.id)
    message.success(`隧道已删除`)
    await loadTunnels()
  } catch (error) {
    message.error('删除失败: ' + error)
  }
}

const handleOpenUrl = (url) => {
  openUrl(url)
}

const copyUrl = async (url) => {
  try {
    await navigator.clipboard.writeText(url)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

const loadTunnels = async () => {
  try {
    tunnels.value = await tunnelService.getAll()
  } catch (error) {
    console.error('加载隧道失败:', error)
  }
}

let refreshTimer = null
let uptimeTimer = null

onMounted(() => {
  loadTunnels()
  // 每 2 秒刷新隧道状态（获取后台更新的 publicUrl、webInterfaceUrl 等）
  refreshTimer = setInterval(loadTunnels, 2000)
  // 每秒更新运行时间显示
  uptimeTimer = setInterval(() => { now.value = Date.now() }, 1000)
})

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
    refreshTimer = null
  }
  if (uptimeTimer) {
    clearInterval(uptimeTimer)
    uptimeTimer = null
  }
})
</script>

<style scoped>
.tunnels-page {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.page-title {
  margin: 0;
  font-size: 1.1rem;
  color: var(--color-text);
}

/* Tunnel Grid */
.tunnel-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: var(--spacing-md);
}

.tunnel-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  overflow: hidden;
  transition: transform 0.2s, box-shadow 0.2s;
}

.tunnel-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.card-header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.provider-badge {
  display: flex;
  align-items: center;
  gap: 8px;
}

.provider-avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--color-primary-dim);
  color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  font-weight: 700;
}

.provider-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.provider-name {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--color-text);
}

.provider-label {
  font-size: 0.7rem;
  color: var(--color-text-dim);
}

.card-body {
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.public-url-row {
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
}

.public-url-value {
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.info-label {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
}

.info-value {
  font-size: 0.85rem;
  color: var(--color-text);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface-alt);
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 4px;
}

.web-interface-btn {
  color: var(--color-primary) !important;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.8rem;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-indicator.online .status-dot {
  background: var(--color-online);
  box-shadow: 0 0 8px rgba(0, 212, 170, 0.5);
}

.status-indicator.online .status-text {
  color: var(--color-online);
}

.status-indicator.offline .status-dot {
  background: var(--color-offline);
}

.status-indicator.offline .status-text {
  color: var(--color-offline);
}

.stop-btn {
  color: var(--color-error) !important;
}

.delete-btn {
  color: var(--color-text-dim) !important;
}

.delete-btn:hover {
  color: var(--color-error) !important;
  background: rgba(255, 107, 107, 0.1) !important;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: var(--spacing-xl) * 2;
  color: var(--color-text-dim);
}

.empty-illustration {
  margin-bottom: 16px;
}

.empty-icon {
  font-size: 48px;
  color: var(--color-text-dim);
}

.empty-title {
  margin: 0 0 8px;
  font-size: 1.1rem;
  color: var(--color-text-secondary);
}

.empty-desc {
  margin: 0 0 16px;
  font-size: 0.9rem;
  color: var(--color-text-dim);
}

.mono {
  font-family: var(--font-mono);
}

@media (max-width: 768px) {
  .tunnel-grid {
    grid-template-columns: 1fr;
  }
}
</style>
