<template>
  <div id="app_content">
    <!-- Splash 页面 -->
    <div v-if="showSplash" class="splash-screen">
      <div class="splash-content">
        <div class="logo">
          <svg width="80" height="80" viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="10" y="10" width="25" height="25" rx="4" fill="var(--primary-color)" opacity="0.8"/>
            <rect x="45" y="10" width="25" height="25" rx="4" fill="var(--primary-color)" opacity="0.6"/>
            <rect x="10" y="45" width="25" height="25" rx="4" fill="var(--primary-color)" opacity="0.6"/>
            <rect x="45" y="45" width="25" height="25" rx="4" fill="var(--primary-color)" opacity="0.4"/>
          </svg>
        </div>
        <h1 class="app-title">ZCalendar</h1>
        <p v-if="connectionStatus === 'connecting'" class="status-text">正在连接数据库...</p>
        <p v-else-if="connectionStatus === 'error'" class="status-text error">无法连接到服务器</p>
        <p v-else class="status-text">加载中...</p>
        <button v-if="connectionStatus === 'error'" @click="retryConnection" class="retry-btn">重试</button>
      </div>
    </div>

    <!-- 主应用 -->
    <template v-else>
      <LoadingBar ref="loadingBarRef" />
      <nav class="main-nav">
        <router-link to="/" class="nav-link">日历</router-link>
        <router-link to="/events" class="nav-link">事件</router-link>
        <router-link to="/settings" class="nav-link">设置</router-link>
      </nav>

      <main class="main-content">
        <router-view />
      </main>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { themeManager } from './utils/themeManager';
import LoadingBar from './components/LoadingBar.vue';
import { setLoadingBarInstance } from './composables/useLoading';

const loadingBarRef = ref<InstanceType<typeof LoadingBar> | null>(null);
const showSplash = ref(true);
const connectionStatus = ref<'connecting' | 'success' | 'error'>('connecting');
let connectionTimeout: number | null = null;

const checkDatabaseConnection = async (): Promise<boolean> => {
  try {
    // 尝试获取所有事件来测试数据库连接
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('get_all_events');
    return true;
  } catch (error) {
    console.error('数据库连接失败:', error);
    return false;
  }
};

const initApp = async () => {
  // 初始化主题
  themeManager.loadSettings();

  // 注册加载条实例
  if (loadingBarRef.value) {
    setLoadingBarInstance(loadingBarRef.value);
  }

  // 设置30秒超时
  connectionTimeout = window.setTimeout(() => {
    if (connectionStatus.value === 'connecting') {
      connectionStatus.value = 'error';
    }
  }, 30000);

  // 等待数据库连接，使用重试机制
  const maxRetries = 60; // 30秒内每0.5秒重试一次
  let retries = 0;
  let connected = false;

  while (retries < maxRetries && connectionStatus.value === 'connecting') {
    // 每500ms重试一次
    await new Promise(resolve => setTimeout(resolve, 500));
    retries++;

    connected = await checkDatabaseConnection();
    if (connected) {
      break;
    }
  }

  // 清除超时
  if (connectionTimeout) {
    clearTimeout(connectionTimeout);
    connectionTimeout = null;
  }

  if (connected) {
    connectionStatus.value = 'success';
    // 延迟隐藏 Splash 页面，让用户看到连接成功
    setTimeout(() => {
      showSplash.value = false;
    }, 500);
  } else if (connectionStatus.value === 'connecting') {
    // 只有在30秒后还没连接成功才显示错误
    connectionStatus.value = 'error';
  }
};

const retryConnection = () => {
  connectionStatus.value = 'connecting';
  initApp();
};

onMounted(() => {
  console.log('ZCalendar应用已启动');
  initApp();
});
</script>

<style scoped>
#app_content {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: var(--text-color);
  background-color: var(--background-color);
  height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Splash 页面样式 */
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, var(--background-color) 0%, var(--input-background-color) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}

.splash-content {
  text-align: center;
  animation: fadeIn 0.5s ease-in;
}

.logo {
  margin-bottom: 24px;
  animation: bounce 2s ease-in-out infinite;
}

.app-title {
  font-size: 32px;
  font-weight: 700;
  color: var(--primary-color);
  margin: 0 0 16px 0;
}

.status-text {
  font-size: 16px;
  color: var(--text-color);
  margin: 0 0 24px 0;
  opacity: 0.8;
}

.status-text.error {
  color: var(--danger-color);
  font-weight: 500;
}

.retry-btn {
  padding: 12px 32px;
  font-size: 14px;
  font-weight: 500;
  color: white;
  background-color: var(--primary-color);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.retry-btn:hover {
  background-color: var(--primary-color);
  opacity: 0.9;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.retry-btn:active {
  transform: translateY(0);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

/* 主应用样式 */
.main-nav {
  display: flex;
  background-color: var(--input-background-color);
  border-bottom: 1px solid var(--border-color);
  padding: 0 20px;
  padding-top: 2rem;
}

.nav-link {
  padding: 15px 20px;
  text-decoration: none;
  color: var(--text-color);
  font-weight: 500;
  border-bottom: 3px solid transparent;
  transition: all 0.3s ease;
}

.nav-link.router-link-exact-active {
  border-bottom-color: var(--primary-color);
  color: var(--primary-color);
  /* background-color: var(--secondary-light); */
}

.main-content {
  /* flex: 1; */
  overflow: hidden;
  height: calc(100vh - 56px - 2rem);
}
</style>
