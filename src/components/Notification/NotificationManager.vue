<template>
  <div class="notification-manager">
    <transition-group name="notification">
      <div 
        v-for="notification in notifications" 
        :key="notification.id" 
        class="notification"
        :class="getNotificationType(notification)"
      >
        <div class="notification-content">
          <h4>{{ notification.title }}</h4>
          <p>{{ notification.body }}</p>
          <small>{{ formatDate(new Date()) }}</small>
        </div>
        <button @click="dismissNotification(notification.id)" class="dismiss-btn">×</button>
      </div>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useNotifications } from '@/composables/useNotifications';

const { startListening, notifications } = useNotifications();
let unlistenFn: (() => void) | null = null;

// 获取通知类型
const getNotificationType = (_notification: any) => {
  // 可以根据notification的某些属性来确定类型，目前默认为warning
  return 'warning';
};

// 监听事件提醒 - 使用全局通知API
onMounted(async () => {
  try {
    // 开始监听通知
    const unlisten = await startListening();
    if (unlisten) {
      unlistenFn = unlisten;
    }
  } catch (error) {
    console.error('Failed to listen for event reminders:', error);
  }
});

const dismissNotification = (_id: number) => {
  // 通知通过全局系统管理，这里无需做任何操作
};

const formatDate = (date: Date) => {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
};

// 清理函数
onUnmounted(() => {
  if (unlistenFn) {
    unlistenFn();
  }
});
</script>

<style scoped>
.notification-manager {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 10000;
  max-width: 350px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.notification {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 15px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-height: 80px;
  background-color: var(--modal-background-color);
  border-left: 4px solid var(--primary-color);
  color: var(--text-color);
}

.notification.info {
  border-left-color: var(--info-color);
}

.notification.success {
  border-left-color: var(--success-color);
}

.notification.warning {
  border-left-color: var(--warning-color);
}

.notification.error {
  border-left-color: var(--danger-color);
}

.notification-content {
  flex: 1;
}

.notification h4 {
  margin: 0 0 5px 0;
  font-size: 16px;
}

.notification p {
  margin: 0 0 5px 0;
  font-size: 14px;
}

.notification small {
  opacity: 0.7;
  font-size: 12px;
}

.dismiss-btn {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: inherit;
  padding: 0;
  margin-left: 10px;
  line-height: 1;
}

/* 过渡动画 */
.notification-enter-active, .notification-leave-active {
  transition: all 0.3s ease;
}
.notification-enter-from {
  opacity: 0;
  transform: translateX(100%);
}
.notification-leave-to {
  opacity: 0;
  transform: translateX(100%);
  max-height: 0;
  padding-top: 0;
  padding-bottom: 0;
  margin-top: 0;
  margin-bottom: 0;
}
</style>