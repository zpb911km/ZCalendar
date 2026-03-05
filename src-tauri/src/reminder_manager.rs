use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;
use tokio::time::{sleep, Duration as TokioDuration};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Reminder {
    event_id: i32,
    reminder_time: String,
    event_title: String,
    event_description: Option<String>,
    event_start_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderData {
    reminders: Vec<Reminder>,
    last_sync: String,
    needs_update: bool,
}

impl Default for ReminderData {
    fn default() -> Self {
        Self {
            reminders: Vec::new(),
            last_sync: Utc::now().to_rfc3339(),
            needs_update: false,
        }
    }
}

pub struct ReminderManager {
    file_path: PathBuf,
}

impl ReminderManager {
    pub fn new<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Self, String> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        let file_path = app_data_dir.join("reminders.json");

        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        Ok(Self { file_path })
    }

    pub fn load(&self) -> Result<ReminderData, String> {
        if !self.file_path.exists() {
            return Ok(ReminderData::default());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read reminders file: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse reminders: {}", e))
    }

    pub fn save(&self, data: &ReminderData) -> Result<(), String> {
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize reminders: {}", e))?;

        fs::write(&self.file_path, content)
            .map_err(|e| format!("Failed to write reminders file: {}", e))?;

        Ok(())
    }

    pub fn set_update_flag(&self) -> Result<(), String> {
        let mut data = self.load()?;
        data.needs_update = true;
        self.save(&data)
    }

    pub async fn sync_reminders<R: Runtime>(
        &self,
        _app_handle: &AppHandle<R>,
        pool: &sqlx::mysql::MySqlPool,
    ) -> Result<(), String> {
        // 获取所有事件
        let query = "SELECT id, title, description, start, reminder_minutes, status FROM events";
        let events: Vec<Event> = sqlx::query_as::<_, Event>(query)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Failed to fetch events: {}", e))?;

        let now = Utc::now();
        let mut reminders = Vec::new();

        for event in events {
            // 只处理需要提醒且未取消的事件
            if event.reminder_minutes > 0 && event.status != "CANCELLED" {
                // 解析事件开始时间
                let event_start = parse_datetime(&event.start)?;

                // 计算提醒时间
                let reminder_time = event_start - Duration::minutes(event.reminder_minutes as i64);

                // 只保留未来24小时内的提醒
                if reminder_time > now && reminder_time < now + Duration::hours(24) {
                    reminders.push(Reminder {
                        event_id: event.id,
                        reminder_time: reminder_time.to_rfc3339(),
                        event_title: event.title,
                        event_description: event.description,
                        event_start_time: event.start,
                    });
                }
            }
        }

        let data = ReminderData {
            reminders,
            last_sync: now.to_rfc3339(),
            needs_update: false,
        };

        self.save(&data)
    }

    pub async fn check_and_send_reminders<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
    ) -> Result<(), String> {
        let mut data = self.load()?;
        let now = Utc::now();

        // 检查是否需要更新
        if data.needs_update {
            // 更新标志位将在外部处理
            return Ok(());
        }

        // 找到需要发送的提醒
        let mut to_send = Vec::new();
        data.reminders.retain(|reminder| {
            let reminder_time = parse_datetime(&reminder.reminder_time).unwrap_or(now);
            if reminder_time <= now {
                to_send.push(reminder.clone());
                false // 发送后移除
            } else {
                true
            }
        });

        // 发送通知
        for reminder in to_send {
            println!("Sending reminder: {}", reminder.event_title);
            send_reminder_notification(app_handle, &reminder).await?;
        }

        // 清理过期的提醒（超过1小时）
        data.reminders.retain(|reminder| {
            let reminder_time = parse_datetime(&reminder.reminder_time).unwrap_or(now);
            reminder_time > now - Duration::hours(1)
        });

        self.save(&data)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct Event {
    id: i32,
    title: String,
    description: Option<String>,
    start: String,
    reminder_minutes: i32,
    status: String,
}

fn parse_datetime(s: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
        })
        .map_err(|e| e.to_string())
}

async fn send_reminder_notification<R: Runtime>(
    app_handle: &AppHandle<R>,
    reminder: &Reminder,
) -> Result<(), String> {
    let title = format!("事件提醒: {}", reminder.event_title);

    let description = reminder.event_description.clone().unwrap_or_default();
    let start_dt = parse_datetime(&reminder.event_start_time)?;
    let start_time = start_dt.format("%H:%M").to_string();

    let body = if description.is_empty() {
        format!("时间: {}", start_time)
    } else {
        format!("{}\n时间: {}", description, start_time)
    };

    let _ = app_handle.plugin(tauri_plugin_notification::init());

    let notification = app_handle
        .notification()
        .builder()
        .title(&title)
        .body(&body);

    let _ = notification.show();

    Ok(())
}

pub async fn start_reminder_loop<R: Runtime>(
    app_handle: &AppHandle<R>,
    pool: sqlx::mysql::MySqlPool,
) {
    let manager = match ReminderManager::new(app_handle) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create reminder manager: {}", e);
            return;
        }
    };

    // 初始同步
    if let Err(e) = manager.sync_reminders(&app_handle, &pool).await {
        eprintln!("Failed to sync reminders: {}", e);
    }

    loop {
        // 检查并更新
        let data = match manager.load() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to load reminders: {}", e);
                sleep(TokioDuration::from_secs(30)).await;
                continue;
            }
        };

        if data.needs_update {
            if let Err(e) = manager.sync_reminders(&app_handle, &pool).await {
                eprintln!("Failed to sync reminders: {}", e);
            }
        }

        // 检查并发送提醒
        if let Err(e) = manager.check_and_send_reminders(&app_handle).await {
            eprintln!("Failed to check reminders: {}", e);
        }

        // 等待30秒
        sleep(TokioDuration::from_secs(10)).await;
    }
}
