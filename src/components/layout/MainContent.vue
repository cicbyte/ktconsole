<template>
  <div class="main-content">
    <div class="content-wrapper">
      <router-view v-slot="{ Component, route }">
        <transition name="page" mode="out-in">
          <component :is="Component" :key="route.path" />
        </transition>
      </router-view>
    </div>
  </div>
</template>

<style scoped>
.main-content {
  height: 100%;
  position: relative;
  background: var(--color-background);
}

.content-wrapper {
  height: 100%;
  overflow-x: hidden;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.page-enter-active,
.page-leave-active {
  transition: all var(--transition-normal);
}

.page-enter-from {
  opacity: 0;
  transform: translateX(20px);
}

.page-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

.content-wrapper::-webkit-scrollbar {
  width: 6px;
}

.content-wrapper::-webkit-scrollbar-track {
  background: transparent;
}

.content-wrapper::-webkit-scrollbar-thumb {
  background: var(--color-border-light);
  border-radius: 3px;
}

.content-wrapper::-webkit-scrollbar-thumb:hover {
  background: var(--color-secondary);
}

@media (max-width: 768px) {
  .content-wrapper {
    padding: var(--spacing-sm);
  }
}
</style>
