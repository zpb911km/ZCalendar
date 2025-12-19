import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from './router'
import "./styles/color.css";
import "./styles/dark-color.css";
import { themeManager } from './utils/themeManager';

// 创建Pinia实例
const pinia = createPinia();

// 初始化主题管理器
themeManager.loadSettings();
themeManager.watchSystemThemeChange();

createApp(App).use(pinia).use(router).mount("#app");
