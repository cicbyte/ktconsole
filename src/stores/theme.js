import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  // 固定深色主题
  const currentTheme = ref('dark')
  const isDark = computed(() => true)
  const isLight = computed(() => false)

  const applyTheme = () => {
    document.documentElement.setAttribute('data-theme', 'dark')
  }

  // 初始化
  applyTheme()

  return {
    currentTheme,
    isDark,
    isLight,
    applyTheme,
  }
})
