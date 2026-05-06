<template>
  <div class="status-bar">
    <div class="status-item">
      <span class="status-label">CPU</span>
      <div class="status-progress">
        <div class="progress-bar" :style="{ width: cpuUsage + '%' }" :class="cpuClass"></div>
      </div>
      <span class="status-value">{{ cpuUsage }}%</span>
    </div>
    <div class="status-item">
      <span class="status-label">RAM</span>
      <div class="status-progress">
        <div class="progress-bar" :style="{ width: ramUsage + '%' }" :class="ramClass"></div>
      </div>
      <span class="status-value">{{ ramUsage }}%</span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { systemService } from '@/services/database'

const cpuUsage = ref(0)
const ramUsage = ref(0)

const cpuClass = computed(() => {
  if (cpuUsage.value > 80) return 'high'
  if (cpuUsage.value > 50) return 'medium'
  return 'normal'
})

const ramClass = computed(() => {
  if (ramUsage.value > 80) return 'high'
  if (ramUsage.value > 50) return 'medium'
  return 'normal'
})

let timer = null

const fetchSystemInfo = async () => {
  try {
    const info = await systemService.getInfo()
    cpuUsage.value = Math.min(info.cpuUsage || 0, 100)
    ramUsage.value = Math.min(info.ramUsage || 0, 100)
  } catch {
    // 模拟数据
    cpuUsage.value = Math.floor(Math.random() * 30 + 10)
    ramUsage.value = Math.floor(Math.random() * 40 + 30)
  }
}

onMounted(async () => {
  await fetchSystemInfo()
  timer = setInterval(fetchSystemInfo, 3000)
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})
</script>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  gap: 16px;
  height: 100%;
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.status-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-label {
  color: var(--color-text-dim);
  font-size: 0.7rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.status-progress {
  width: 60px;
  height: 4px;
  background: var(--color-border);
  border-radius: 2px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  border-radius: 2px;
  transition: width 0.5s ease, background 0.3s ease;
}

.progress-bar.normal {
  background: var(--color-primary);
}

.progress-bar.medium {
  background: var(--color-accent-yellow);
}

.progress-bar.high {
  background: var(--color-error);
}

.status-value {
  font-size: 0.7rem;
  min-width: 30px;
}

.mono {
  font-family: var(--font-mono);
  color: var(--color-primary);
  letter-spacing: 1px;
}

.status-separator {
  width: 1px;
  height: 12px;
  background: var(--color-border-light);
}
</style>
