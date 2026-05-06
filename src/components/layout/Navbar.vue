<template>
  <div class="navbar titlebar-drag-region">
    <div class="navbar-left">
      <a-button
        type="text"
        class="sidebar-toggle titlebar-no-drag"
        @click="layoutStore.toggleSidebar"
        v-show="!layoutStore.isMobile"
      >
        <template #icon>
          <MenuFoldOutlined v-if="!layoutStore.sidebarCollapsed" />
          <MenuUnfoldOutlined v-else />
        </template>
      </a-button>

      <div class="breadcrumb titlebar-no-drag">
        <span class="breadcrumb-home" @click="router.push('/dashboard')">首页</span>
        <span class="breadcrumb-sep" v-if="currentPageTitle !== '首页'">/</span>
        <span class="breadcrumb-current" v-if="currentPageTitle !== '首页'">{{ currentPageTitle }}</span>
      </div>
    </div>

    <div class="navbar-right">
      <div class="window-controls">
        <a-button type="text" size="small" class="window-control titlebar-no-drag" @click="minimizeWindow">
          <template #icon><MinusOutlined /></template>
        </a-button>
        <a-button type="text" size="small" class="window-control titlebar-no-drag" @click="maximizeWindow">
          <template #icon><BorderOutlined /></template>
        </a-button>
        <a-button type="text" size="small" class="window-control close titlebar-no-drag" @click="closeWindow">
          <template #icon><CloseOutlined /></template>
        </a-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useLayoutStore } from '@/stores/layout'
import { invoke } from '@tauri-apps/api/core'
import {
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  MinusOutlined,
  BorderOutlined,
  CloseOutlined,
} from '@ant-design/icons-vue'

const route = useRoute()
const router = useRouter()
const layoutStore = useLayoutStore()

const currentPageTitle = computed(() => {
  const titleMap = {
    '/dashboard': '首页',
    '/tunnels': '隧道管理',
    '/logs': '实时日志',
    '/settings': '系统设置',
  }
  return titleMap[route.path] || '首页'
})

const minimizeWindow = async () => {
  try { await invoke('window_minimize') } catch (e) { console.error(e) }
}
const maximizeWindow = async () => {
  try { await invoke('window_maximize') } catch (e) { console.error(e) }
}
const closeWindow = async () => {
  try { await invoke('window_close') } catch (e) { console.error(e) }
}
</script>

<style scoped>
.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 100%;
  background: var(--color-navbar);
  user-select: none;
}

.titlebar-drag-region {
  -webkit-app-region: drag;
  width: 100%;
  height: 100%;
}

.titlebar-no-drag {
  -webkit-app-region: no-drag;
}

.navbar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.navbar-right {
  display: flex;
  align-items: center;
}

.sidebar-toggle {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
}

.sidebar-toggle:hover {
  color: var(--color-primary);
  background: var(--color-hover);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.85rem;
}

.breadcrumb-home {
  color: var(--color-text-secondary);
  cursor: pointer;
}

.breadcrumb-home:hover {
  color: var(--color-primary);
}

.breadcrumb-sep {
  color: var(--color-text-dim);
}

.breadcrumb-current {
  color: var(--color-text);
  font-weight: 500;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 4px;
}

.window-control {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
  outline: none;
}

.window-control:hover {
  color: var(--color-primary);
  background: var(--color-hover);
}

.window-control.close:hover {
  background: #ff4757;
  color: white;
}

@media (max-width: 768px) {
  .breadcrumb {
    font-size: 0.8rem;
  }
}
</style>
