import { ref, computed } from 'vue';
import { NotificationPayload, notificationService } from '../services/notificationService';

export function useNotifications() {
  const notifications = ref<NotificationPayload[]>([]);
  const isListening = ref(false);

  // 开始监听通知
  const startListening = async () => {
    if (isListening.value) return;
    
    try {
      const unlisten = await notificationService.listenForReminders((payload) => {
        // 添加到通知列表
        notifications.value.push(payload);
        
        // 显示本地通知
        notificationService.showLocalNotification(payload.title, payload.body);
      });
      
      isListening.value = true;
      
      // 返回取消监听的函数
      return () => {
        unlisten();
        isListening.value = false;
      };
    } catch (error) {
      console.error('开始监听通知失败:', error);
    }
  };

  // 获取通知
  const getNotifications = () => {
    return [...notifications.value];
  };

  // 清除通知
  const clearNotifications = () => {
    notifications.value = [];
  };

  // 清除特定通知
  const clearNotification = (id: number) => {
    notifications.value = notifications.value.filter(notification => notification.id !== id);
  };

  return {
    notifications: computed(() => notifications.value),
    startListening,
    getNotifications,
    clearNotifications,
    clearNotification
  };
}