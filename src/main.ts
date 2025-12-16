import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from './router'
import "./styles/color.css";
import "./styles/dark-color.css";

// 创建Pinia实例
const pinia = createPinia();

// 初始化主题
function initializeTheme() {
  const savedTheme = localStorage.getItem('theme') || 'auto';
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  
  // 应用主题
  if (savedTheme === 'dark' || (savedTheme === 'auto' && prefersDark)) {
    document.documentElement.setAttribute('data-theme', 'dark');
  } else {
    document.documentElement.setAttribute('data-theme', 'light');
  }
}

// 监听系统主题变化
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
  if (localStorage.getItem('theme') === 'auto') {
    if (e.matches) {
      document.documentElement.setAttribute('data-theme', 'dark');
    } else {
      document.documentElement.setAttribute('data-theme', 'light');
    }
  }
});

// 初始化主题
initializeTheme();

createApp(App).use(pinia).use(router).mount("#app");
