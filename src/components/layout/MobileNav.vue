<template>
  <div class="mobile-nav">
    <div
      v-for="item in navItems"
      :key="item.path"
      class="mobile-nav-item"
      :class="{ active: route.path === item.path }"
      @click="router.push(item.path)"
    >
      <component :is="item.icon" class="nav-icon" />
      <span class="nav-label">{{ item.label }}</span>
    </div>
    <!-- Center FAB for new tunnel -->
    <div class="mobile-nav-fab" @click="showCreateModal = true">
      <PlusOutlined />
    </div>
    <CreateTunnelModal v-model:open="showCreateModal" />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  DashboardOutlined,
  ApiOutlined,
  FileTextOutlined,
  SettingOutlined,
  PlusOutlined,
} from '@ant-design/icons-vue'
import CreateTunnelModal from '@/components/CreateTunnelModal.vue'

const router = useRouter()
const route = useRoute()
const showCreateModal = ref(false)

const navItems = [
  { path: '/dashboard', label: '首页', icon: DashboardOutlined },
  { path: '/tunnels', label: '隧道', icon: ApiOutlined },
  { path: '/logs', label: '日志', icon: FileTextOutlined },
  { path: '/settings', label: '设置', icon: SettingOutlined },
]
</script>

<style scoped>
.mobile-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 56px;
  background: var(--color-sidebar);
  border-top: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-around;
  z-index: 1000;
  padding: 0 10px;
}

.mobile-nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  cursor: pointer;
  color: var(--color-text-dim);
  transition: color 0.2s ease;
  padding: 4px 8px;
}

.mobile-nav-item.active {
  color: var(--color-primary);
}

.mobile-nav-item:hover {
  color: var(--color-text-secondary);
}

.nav-icon {
  font-size: 18px;
}

.nav-label {
  font-size: 0.65rem;
}

.mobile-nav-fab {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: var(--color-primary);
  color: #0a0e14;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(0, 212, 170, 0.4);
  transition: transform 0.2s ease;
}

.mobile-nav-fab:hover {
  transform: translateX(-50%) scale(1.1);
}
</style>
