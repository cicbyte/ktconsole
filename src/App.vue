<script setup>
import { onMounted, onBeforeUnmount } from 'vue'
import AppLayout from '@/components/layout/AppLayout.vue'
import { theme as antTheme } from 'ant-design-vue'

// 固定深色主题
const antThemeConfig = {
  algorithm: antTheme.darkAlgorithm,
  token: {
    colorPrimary: '#00d4aa',
    borderRadius: 6,
    colorBgContainer: '#111820',
    colorText: '#e6edf3',
    colorBorder: '#1e2a3a',
  },
}

// 禁用右键菜单
const handleGlobalContextMenu = (event) => {
  const target = event.target
  const isInEditor = target.closest('.cm-editor') ||
                     target.closest('.log-terminal')
  if (isInEditor) return
  event.preventDefault()
}

onMounted(() => {
  document.documentElement.setAttribute('data-theme', 'dark')
  document.addEventListener('contextmenu', handleGlobalContextMenu)
})

onBeforeUnmount(() => {
  document.removeEventListener('contextmenu', handleGlobalContextMenu)
})
</script>

<template>
  <a-config-provider :theme="antThemeConfig">
    <div id="app">
      <AppLayout />
    </div>
  </a-config-provider>
</template>

<style>
html, body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-color: #0a0e14;
}

#app {
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #e6edf3;
}

::selection {
  background: #00d4aa;
  color: #0a0e14;
}

:focus-visible {
  outline: 2px solid #00d4aa;
  outline-offset: 2px;
}
</style>
