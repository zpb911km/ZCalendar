<template>
  <div class="loading-bar-container" v-if="loading">
    <div class="loading-bar" :class="{ active: active }"></div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const loading = ref(false);
const active = ref(false);

const show = () => {
  loading.value = true;
  setTimeout(() => {
    active.value = true;
  }, 10);
};

const hide = () => {
  active.value = false;
  setTimeout(() => {
    loading.value = false;
  }, 300);
};

defineExpose({
  show,
  hide,
});
</script>

<style scoped>
.loading-bar-container {
  position: fixed;
  top: var(--android-status-bar-height);
  left: 0;
  right: 0;
  z-index: 9999;
  background: rgba(0, 0, 0, 0);
  padding: 8px 0;
}

.loading-bar {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 4px;
  background: linear-gradient(
    90deg,
    transparent,
    var(--primary-color),
    transparent
  );
  background-size: 200% 100%;
  animation: none;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.loading-bar.active {
  animation: loading 1.5s linear infinite;
  opacity: 1;
}

.loading-text {
  color: white;
  font-size: 14px;
  margin-top: 8px;
  animation: blink 1s ease-in-out infinite;
}

@keyframes loading {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}

@keyframes blink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
