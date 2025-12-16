use crate::services::calendar_service::CalendarService;
use crate::services::event_service::EventService;
use crate::services::ical_service::IcalService;
use crate::services::notification_service::NotificationService;
use crate::storage::database::Database;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{Emitter, Manager};

// 导入模型
pub mod models {
    pub mod event;
    pub mod calendar;
    pub mod reminder;
}

// 导入服务
pub mod services {
    pub mod event_service;
    pub mod calendar_service;
    pub mod notification_service;
    pub mod ical_service;
}

// 导入存储
pub mod storage {
    pub mod database;
}

// 命令模块
pub mod commands {
    use super::*;
    use crate::models::event::{CreateEventDto, Event, UpdateEventDto};
    use crate::models::calendar::Calendar;
    
    #[tauri::command]
    pub async fn get_all_events(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
    ) -> Result<Vec<Event>, String> {
        state.lock().await.get_all_events().await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_events_by_date(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        date: DateTime<Utc>,
    ) -> Result<Vec<Event>, String> {
        state.lock().await.get_events_by_date(date).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_event_by_id(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        id: i64,
    ) -> Result<Option<Event>, String> {
        state.lock().await.get_event_by_id(id).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn create_event(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        event: CreateEventDto,
    ) -> Result<Event, String> {
        state.lock().await.create_event(event).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn update_event(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        event: UpdateEventDto,
    ) -> Result<Event, String> {
        state.lock().await.update_event(event).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn delete_event(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        id: i64,
    ) -> Result<(), String> {
        state.lock().await.delete_event(id).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn import_ical(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        ical_content: String,
    ) -> Result<Vec<Event>, String> {
        state.lock().await.import_ical(ical_content).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn export_ical(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        event_ids: Option<Vec<i64>>,
    ) -> Result<String, String> {
        state.lock().await.export_ical(event_ids).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_events_in_range(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Event>, String> {
        state.lock().await.get_events_in_range(start, end).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn search_events(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        query: String,
    ) -> Result<Vec<Event>, String> {
        state.lock().await.search_events(&query).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_upcoming_events(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        limit: usize,
    ) -> Result<Vec<Event>, String> {
        state.lock().await.get_upcoming_events(limit).await.map_err(|e| e.to_string())
    }
    
    // 日历管理命令
    #[tauri::command]
    pub async fn get_all_calendars(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
    ) -> Result<Vec<Calendar>, String> {
        state.lock().await.get_all_calendars().await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_calendar_by_id(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        id: String,
    ) -> Result<Option<Calendar>, String> {
        state.lock().await.get_calendar_by_id(&id).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn create_calendar(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        name: String,
        color: String,
    ) -> Result<Calendar, String> {
        state.lock().await.create_calendar(name, color).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn update_calendar(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        calendar: Calendar,
    ) -> Result<Calendar, String> {
        state.lock().await.update_calendar(calendar).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn delete_calendar(
        state: tauri::State<'_, Arc<Mutex<CalendarService>>>,
        id: String,
    ) -> Result<(), String> {
        state.lock().await.delete_calendar(&id).await.map_err(|e| e.to_string())
    }
}

// 应用初始化
pub fn init_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化数据库
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(async {
        Arc::new(Mutex::new(
            Database::new("sqlite:calendar.db").await
                .expect("Failed to initialize database")
        ))
    });

    // 初始化服务
    let event_service = Arc::new(Mutex::new(EventService::new(db.clone())));
    let ical_service = IcalService::new();
    let calendar_service = Arc::new(Mutex::new(
        CalendarService::new(event_service.clone(), ical_service)
    ));

    // 设置状态
    app.manage(calendar_service.clone());

    // 初始化通知服务
    let notification_service = NotificationService::new(event_service, app.handle().clone());
    tauri::async_runtime::spawn(async move {
        notification_service.start_notification_scheduler().await;
    });

    Ok(())
}

// 导出所有命令
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_all_events,
            commands::get_events_by_date,
            commands::get_event_by_id,
            commands::create_event,
            commands::update_event,
            commands::delete_event,
            commands::import_ical,
            commands::export_ical,
            commands::get_events_in_range,
            commands::search_events,
            commands::get_upcoming_events,
            commands::get_all_calendars,
            commands::get_calendar_by_id,
            commands::create_calendar,
            commands::update_calendar,
            commands::delete_calendar,
        ])
        .setup(|app| {
            init_app(app).map_err(|e| e.to_string().into())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}