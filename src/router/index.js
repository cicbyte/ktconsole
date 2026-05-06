import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    redirect: '/dashboard'
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('@/views/dashboard/index.vue'),
    meta: { title: '首页' }
  },
  {
    path: '/tunnels',
    name: 'Tunnels',
    component: () => import('@/views/tunnels/index.vue'),
    meta: { title: '隧道管理' }
  },
  {
    path: '/logs',
    name: 'Logs',
    component: () => import('@/views/logs/index.vue'),
    meta: { title: '实时日志' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/settings/index.vue'),
    meta: { title: '系统设置' }
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
