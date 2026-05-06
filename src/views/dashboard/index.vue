<template>
  <div class="dashboard">
    <!-- Active Tunnels Section -->
    <div class="section">
      <div class="section-header">
        <h3 class="section-title">活跃隧道</h3>
        <div class="section-actions">
          <a-button size="small">
            <template #icon><FilterOutlined /></template>
            筛选: 全部
          </a-button>
          <a-button size="small">
            <template #icon><SortAscendingOutlined /></template>
            排序: 最新
          </a-button>
        </div>
      </div>

      <!-- Tunnel List -->
      <div v-if="tunnels.length > 0" class="tunnel-list">
        <div v-for="tunnel in tunnels" :key="tunnel.id" class="tunnel-row">
          <div class="tunnel-info">
            <div class="tunnel-status-dot" :class="tunnel.status"></div>
            <div class="tunnel-details">
              <span class="tunnel-name">{{ tunnel.name }}</span>
              <span class="tunnel-meta">
                <a-tag :color="getProtocolColor(tunnel.protocol)" size="small">{{ tunnel.protocol }}</a-tag>
                <span class="tunnel-port">{{ tunnel.localPort }}</span>
              </span>
            </div>
          </div>
          <div class="tunnel-url mono">{{ tunnel.publicUrl }}</div>
          <div class="tunnel-actions">
            <a-button type="text" size="small" @click="viewLogs(tunnel)">
              <template #icon><FileTextOutlined /></template>
            </a-button>
            <a-button type="text" size="small"
              :danger="tunnel.status === 'online'"
              @click="toggleTunnel(tunnel)">
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
        <ApiOutlined class="empty-icon" />
        <p class="empty-text">暂无活跃隧道</p>
        <p class="empty-hint">点击左侧"新建隧道"按钮开始创建</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { tunnelService } from '@/services/database'
import { message } from 'ant-design-vue'
import {
  ThunderboltOutlined,
  FilterOutlined,
  SortAscendingOutlined,
  ApiOutlined,
  FileTextOutlined,
  StopOutlined,
  CaretRightOutlined,
} from '@ant-design/icons-vue'

const router = useRouter()
const tunnels = ref([])

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
      message.info('请通过新建隧道重新启动')
    }
    await loadData()
  } catch (error) {
    message.error('操作失败: ' + error)
  }
}

const loadData = async () => {
  try {
    tunnels.value = await tunnelService.getAll()
  } catch (error) {
    console.error('加载数据失败:', error)
  }
}

let refreshTimer = null

onMounted(() => {
  loadData()
  refreshTimer = setInterval(loadData, 3000)
})

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer)
    refreshTimer = null
  }
})
</script>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

/* Section */
.section {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius-md);
  padding: var(--spacing-md);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.section-title {
  margin: 0;
  font-size: 0.95rem;
  color: var(--color-text);
}

.section-actions {
  display: flex;
  gap: 8px;
}

/* Tunnel List */
.tunnel-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.tunnel-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--color-surface-alt);
  border-radius: var(--border-radius-sm);
  transition: background 0.2s;
}

.tunnel-row:hover {
  background: var(--color-surface-hover);
}

.tunnel-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
}

.tunnel-status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.tunnel-status-dot.online {
  background: var(--color-online);
  box-shadow: 0 0 6px rgba(0, 212, 170, 0.5);
}

.tunnel-status-dot.offline {
  background: var(--color-offline);
}

.tunnel-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.tunnel-name {
  font-size: 0.85rem;
  color: var(--color-text);
  font-weight: 500;
}

.tunnel-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.75rem;
}

.tunnel-port {
  color: var(--color-text-dim);
  font-family: var(--font-mono);
}

.tunnel-url {
  color: var(--color-primary);
  font-size: 0.8rem;
  flex: 2;
  text-align: center;
}

.tunnel-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--color-text-dim);
}

.empty-state.small {
  padding: var(--spacing-md);
}

.empty-icon {
  font-size: 32px;
  margin-bottom: 8px;
  color: var(--color-text-dim);
}

.empty-text {
  font-size: 0.9rem;
  margin: 0 0 4px;
  color: var(--color-text-secondary);
}

.empty-hint {
  font-size: 0.8rem;
  color: var(--color-text-dim);
  margin: 0;
}

.mono {
  font-family: var(--font-mono);
}

@media (max-width: 768px) {
  .tunnel-url {
    display: none;
  }
}
</style>
