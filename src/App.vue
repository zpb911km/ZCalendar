<template>
  <div id="app">
    <nav class="main-nav">
      <router-link to="/" class="nav-link">日历</router-link>
      <router-link to="/events" class="nav-link">事件</router-link>
      <router-link to="/settings" class="nav-link">设置</router-link>
    </nav>
    
    <main class="main-content">
      <router-view />
    </main>
  
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { themeManager } from './utils/themeManager';
// import { useRouter } from 'vue-router'; // 未使用的router变量


onMounted(() => {
  // 初始化应用
  console.log('ZCalendar应用已启动');
  
  // 确保主题已加载
  themeManager.loadSettings();
});
</script>

<style scoped>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: var(--text-color);
  background-color: var(--background-color);
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-nav {
  display: flex;
  background-color: var(--input-background-color);
  border-bottom: 1px solid var(--border-color);
  padding: 0 20px;
  margin-top: 2rem;
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
}
</style>