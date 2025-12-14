use crate::models::event::Event;
use crate::services::event_service::EventService;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::AppHandle;
use tauri::Emitter;

pub struct NotificationService {
    event_service: Arc<Mutex<EventService>>,
    app_handle: AppHandle,
}

impl NotificationService {
    pub fn new(event_service: Arc<Mutex<EventService>>, app_handle: AppHandle) -> Self {
        Self {
            event_service,
            app_handle,
        }
    }

    pub async fn check_and_send_notifications(&self) {
        match self.event_service.lock().await.get_upcoming_reminders().await {
            Ok(reminders) => {
                for (event, _scheduled_time) in reminders {
                    self.send_notification(&event).await;
                    
                    // 标记提醒已触发
                    if let Err(e) = self.event_service.lock().await.mark_reminder_triggered(event.id).await {
                        eprintln!("标记提醒已触发失败: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("检查提醒时出错: {}", e);
            }
        }
    }

    async fn send_notification(&self, event: &Event) {
        // 发送通知到所有窗口
        if let Err(e) = self.app_handle.emit("calendar-reminder", serde_json::json!({
            "id": event.id,
            "title": &event.title,
            "body": self.format_notification_body(event),
        })) {
            eprintln!("发送通知事件失败: {}", e);
        }
    }

    fn format_notification_body(&self, event: &Event) -> String {
        let mut body = format!("事件提醒: {}", event.title);

        if let Some(ref location) = event.location {
            body.push_str(&format!("\n地点: {}", location));
        }

        // 添加时间信息
        let start_time = event.start.format("%Y-%m-%d %H:%M").to_string();
        body.push_str(&format!("\n时间: {}", start_time));

        body
    }

    async fn send_android_notification(&self, event: &Event) {
        // Android特定的推送通知实现
        // 使用Tauri事件系统发送通知
        let notification_payload = serde_json::json!({
            "id": event.id,
            "title": &event.title,
            "body": self.format_notification_body(event),
        });

        if let Err(e) = self.app_handle.emit("calendar-reminder", &notification_payload) {
            eprintln!("发送通知事件失败: {}", e);
        }
    }

    pub async fn start_notification_scheduler(&self) {
        let service = self.clone();
        tokio::spawn(async move {
            loop {
                service.check_and_send_notifications().await;
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; // 每分钟检查一次
            }
        });
    }
}

// 为NotificationService实现Clone，以便在线程间传递
impl Clone for NotificationService {
    fn clone(&self) -> Self {
        Self {
            event_service: self.event_service.clone(),
            app_handle: self.app_handle.clone(),
        }
    }
}