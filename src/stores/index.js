import { createPinia } from 'pinia'

const pinia = createPinia()

export default pinia

export { useLayoutStore } from './layout'
export { useThemeStore } from './theme'
