import { listen, Event } from '@tauri-apps/api/event';

export interface NotificationPayload {
  id: number;
  title: string;
  body: string;
}

export const notificationService = {
  // 监听日历提醒事件
  async listenForReminders(callback: (payload: NotificationPayload) => void) {
    try {
      const unlisten = await listen('event_reminder', (event: Event<NotificationPayload>) => {
        callback(event.payload);
      });
      return unlisten;
    } catch (error) {
      console.error('监听提醒事件失败:', error);
      throw error;
    }
  },

  // 显示本地通知
  async showLocalNotification(title: string, body: string) {
    // 在Tauri应用中，通知由后端处理，前端只负责触发
    console.log('显示通知:', { title, body });
    // 实际的通知会在后端通过Tauri API发送
  }
};