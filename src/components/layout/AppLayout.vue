<template>
  <a-layout class="app-layout">
    <!-- Sidebar -->
    <a-layout-sider
      v-model:collapsed="layoutStore.sidebarCollapsed"
      :width="layoutStore.sidebarWidth"
      :collapsed-width="60"
      breakpoint="lg"
      class="app-sidebar"
      v-show="!layoutStore.isMobile"
    >
      <Sidebar />
    </a-layout-sider>

    <!-- Main Content Area -->
    <a-layout class="app-main">
      <!-- Navbar -->
      <a-layout-header class="app-navbar">
        <Navbar />
      </a-layout-header>

      <!-- Content -->
      <a-layout-content class="app-content">
        <MainContent />
      </a-layout-content>

      <!-- Status Bar -->
      <a-layout-footer class="app-statusbar">
        <StatusBar />
      </a-layout-footer>
    </a-layout>

    <!-- Mobile Bottom Navigation -->
    <MobileNav v-if="layoutStore.isMobile" />
  </a-layout>
</template>

<script setup>
import { onMounted, onUnmounted } from 'vue'
import { useLayoutStore } from '@/stores/layout'
import Sidebar from './Sidebar.vue'
import Navbar from './Navbar.vue'
import MainContent from './MainContent.vue'
import StatusBar from './StatusBar.vue'
import MobileNav from './MobileNav.vue'

const layoutStore = useLayoutStore()

let resizeObserver = null

onMounted(() => {
  updateWindowSize()
  resizeObserver = new ResizeObserver(debounce(updateWindowSize, 200))
  resizeObserver.observe(document.body)
})

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect()
  }
})

const updateWindowSize = () => {
  layoutStore.updateWindowSize({
    width: window.innerWidth,
    height: window.innerHeight
  })
}

const debounce = (func, wait) => {
  let timeout
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout)
      func(...args)
    }
    clearTimeout(timeout)
    timeout = setTimeout(later, wait)
  }
}
</script>

<style scoped>
.app-layout {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--color-background);
}

.app-sidebar {
  background: var(--color-sidebar) !important;
  border-right: 1px solid var(--color-border);
}

.app-main {
  background: var(--color-background);
  flex: 1;
  overflow: hidden;
}

.app-navbar {
  background: var(--color-navbar) !important;
  border-bottom: 1px solid var(--color-border);
  padding: 0;
  height: var(--navbar-height);
  line-height: var(--navbar-height);
  width: 100%;
}

.app-content {
  background: var(--color-background);
  overflow: hidden;
  height: calc(100vh - var(--navbar-height) - var(--statusbar-height));
  width: 100%;
}

.app-statusbar {
  background: var(--color-statusbar) !important;
  border-top: 1px solid var(--color-border);
  padding: 0 16px;
  height: var(--statusbar-height);
  line-height: var(--statusbar-height);
  display: flex;
  align-items: center;
}

@media (max-width: 768px) {
  .app-content {
    height: calc(100vh - var(--navbar-height) - var(--statusbar-height) - 56px);
  }

  .app-statusbar {
    display: none;
  }
}
</style>
