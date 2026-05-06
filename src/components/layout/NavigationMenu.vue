<template>
  <a-menu
    :selectedKeys="selectedKeys"
    mode="inline"
    class="navigation-menu"
    :inline-collapsed="layoutStore.sidebarCollapsed"
    @click="handleMenuClick"
  >
    <a-menu-item key="/dashboard">
      <template #icon><DashboardOutlined /></template>
      <span>首页</span>
    </a-menu-item>
    <a-menu-item key="/tunnels">
      <template #icon><ApiOutlined /></template>
      <span>隧道管理</span>
    </a-menu-item>
    <a-menu-item key="/logs">
      <template #icon><FileTextOutlined /></template>
      <span>实时日志</span>
    </a-menu-item>
  </a-menu>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useLayoutStore } from '@/stores/layout'
import {
  DashboardOutlined,
  ApiOutlined,
  FileTextOutlined,
} from '@ant-design/icons-vue'

const router = useRouter()
const route = useRoute()
const layoutStore = useLayoutStore()

const selectedKeys = computed(() => [route.path])

const handleMenuClick = ({ key }) => {
  if (key !== route.path) {
    router.push(key)
  }
}
</script>

<style scoped>
.navigation-menu {
  border: none !important;
  background: transparent;
}

.navigation-menu :deep(.ant-menu-item) {
  margin: 2px 0;
  border-radius: var(--border-radius-md);
  color: var(--color-text-secondary);
  height: 36px;
  line-height: 36px;
  transition: all var(--transition-fast);
}

.navigation-menu :deep(.ant-menu-item:hover) {
  background: var(--color-sidebar-hover) !important;
  color: var(--color-primary) !important;
}

.navigation-menu :deep(.ant-menu-item-selected) {
  background: var(--color-primary-dim) !important;
  color: var(--color-primary) !important;
}

.navigation-menu :deep(.ant-menu-item .anticon) {
  color: inherit;
}

.navigation-menu :deep(.ant-menu-item-selected .anticon) {
  color: var(--color-primary) !important;
}
</style>
