import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useLayoutStore = defineStore('layout', () => {
  const sidebarCollapsed = ref(false)
  const windowSize = ref({
    width: window.innerWidth,
    height: window.innerHeight
  })

  const isMobile = computed(() => windowSize.value.width < 768)
  const isTablet = computed(() => windowSize.value.width >= 768 && windowSize.value.width < 1024)
  const isDesktop = computed(() => windowSize.value.width >= 1024)

  const sidebarWidth = computed(() => {
    if (isMobile.value) return 0
    return sidebarCollapsed.value ? 'var(--sidebar-width)' : 'var(--sidebar-width-expanded)'
  })

  const toggleSidebar = () => {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  const setSidebarCollapsed = (collapsed) => {
    sidebarCollapsed.value = collapsed
  }

  const updateWindowSize = (size) => {
    windowSize.value = size
    if (isMobile.value) {
      sidebarCollapsed.value = true
    }
  }

  return {
    sidebarCollapsed,
    windowSize,
    isMobile,
    isTablet,
    isDesktop,
    sidebarWidth,
    toggleSidebar,
    setSidebarCollapsed,
    updateWindowSize,
  }
})
