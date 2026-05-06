<template>
  <div class="sidebar">
    <!-- Brand Section -->
    <div class="sidebar-brand">
      <div class="brand-content">
        <div class="brand-icon">
          <svg width="24" height="24" viewBox="0 0 32 32" fill="none">
            <defs>
              <linearGradient id="kntGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color: #00d4aa; stop-opacity: 1" />
                <stop offset="100%" style="stop-color: #4facfe; stop-opacity: 1" />
              </linearGradient>
            </defs>
            <rect width="32" height="32" rx="7" fill="url(#kntGradient)" />
            <!-- Tunnel pipe -->
            <rect x="9" y="13" width="14" height="6" rx="3" fill="none" stroke="#0a0e14" stroke-width="1.2" opacity="0.5"/>
            <!-- Left node (local) -->
            <circle cx="8" cy="16" r="3" fill="#0a0e14" opacity="0.35"/>
            <!-- Right node (public) -->
            <circle cx="24" cy="16" r="3" fill="#0a0e14" opacity="0.35"/>
            <!-- Center connection point -->
            <circle cx="16" cy="16" r="1.5" fill="#fff" opacity="0.9"/>
          </svg>
        </div>
        <transition name="fade">
          <div v-show="!layoutStore.sidebarCollapsed" class="brand-text">
            <span class="brand-name">Kinetic <span class="brand-accent">Console</span></span>
          </div>
        </transition>
      </div>
    </div>

    <!-- Navigation -->
    <div class="sidebar-nav">
      <NavigationMenu />
    </div>

    <!-- Bottom: Settings -->
    <div class="sidebar-bottom">
      <a-tooltip v-if="layoutStore.sidebarCollapsed" placement="right" title="设置">
        <a-button type="text" class="bottom-btn" @click="openSettings">
          <template #icon><SettingOutlined /></template>
        </a-button>
      </a-tooltip>
      <a-button v-else type="text" class="bottom-btn" @click="openSettings">
        <template #icon><SettingOutlined /></template>
      </a-button>
    </div>
  </div>
</template>

<script setup>
import { useRouter, useRoute } from 'vue-router'
import { useLayoutStore } from '@/stores/layout'
import { SettingOutlined } from '@ant-design/icons-vue'
import NavigationMenu from './NavigationMenu.vue'

const router = useRouter()
const route = useRoute()
const layoutStore = useLayoutStore()

const openSettings = () => {
  if (route.path !== '/settings') {
    router.push('/settings')
  }
}
</script>

<style scoped>
.sidebar {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-sidebar);
}

.sidebar-brand {
  height: var(--navbar-height);
  padding: 0 16px;
  flex-shrink: 0;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
}

.brand-content {
  display: flex;
  align-items: center;
  gap: 10px;
}

.brand-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.brand-text {
  display: flex;
  align-items: center;
  white-space: nowrap;
}

.brand-name {
  font-size: 0.95rem;
  font-weight: 700;
  letter-spacing: 0.5px;
  color: var(--color-text);
}

.brand-accent {
  color: var(--color-primary);
}

.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  padding: 8px 10px;
}

.sidebar-bottom {
  height: var(--statusbar-height);
  padding: 0 10px;
  flex-shrink: 0;
  border-top: 1px solid var(--color-border);
  display: flex;
  align-items: center;
}

.bottom-btn {
  width: 100%;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--color-text-secondary);
  border-radius: var(--border-radius-md);
  font-size: 0.8rem;
}

.bottom-btn:hover {
  color: var(--color-primary) !important;
  background: var(--color-hover);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--transition-fast);
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.sidebar-nav::-webkit-scrollbar {
  width: 4px;
}

.sidebar-nav::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar-nav::-webkit-scrollbar-thumb {
  background: var(--color-border-light);
  border-radius: 2px;
}
</style>
