use crate::models::event::{Event, CreateEventDto, UpdateEventDto};
use crate::storage::database::Database;
use chrono::{DateTime, Utc, Duration};
use sqlx::Error as SqlxError;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventServiceError {
    #[error("数据库错误: {0}")]
    Database(#[from] SqlxError),
    #[error("事件不存在: {0}")]
    EventNotFound(i64),
    #[error("无效的日期时间: {0}")]
    InvalidDateTime(String),
    #[error("重复规则解析错误: {0}")]
    RecurrenceParseError(String),
}

pub struct EventService {
    db: Arc<Mutex<Database>>,
}

impl EventService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    pub async fn get_all_events(&self) -> Result<Vec<Event>, EventServiceError> {
        let db = self.db.lock().await;
        let events = db.get_all_events().await?;
        Ok(events)
    }

    pub async fn get_events_by_date(&self, date: DateTime<Utc>) -> Result<Vec<Event>, EventServiceError> {
        let db = self.db.lock().await;
        let events = db.get_events_by_date(date).await?;
        Ok(events)
    }

    pub async fn get_event_by_id(&self, id: i64) -> Result<Option<Event>, EventServiceError> {
        let db = self.db.lock().await;
        let event = db.get_event_by_id(id).await?;
        Ok(event)
    }

    pub async fn create_event(&self, dto: CreateEventDto) -> Result<Event, EventServiceError> {
        let event = Event {
            id: 0, // 由数据库自动生成
            title: dto.title,
            description: dto.description,
            start: dto.start,
            end: dto.end,
            all_day: dto.all_day,
            reminder_minutes: dto.reminder_minutes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            recurrence_rule: dto.recurrence_rule,
            recurrence_id: None,
            sequence: 0,
            status: dto.status.unwrap_or_else(|| "CONFIRMED".to_string()),
            location: dto.location,
            organizer: None,
            attendees: None,
            url: dto.url,
            categories: dto.categories,
            priority: dto.priority,
            calendar_id: None,
        };

        let db = self.db.lock().await;
        let created_event = db.create_event(&event).await?;

        // 创建提醒
        if dto.reminder_minutes > 0 {
            let reminder_time = created_event.start - Duration::minutes(dto.reminder_minutes as i64);
            db.create_reminder(created_event.id, reminder_time).await?;
        }

        Ok(created_event)
    }

    pub async fn update_event(&self, dto: UpdateEventDto) -> Result<Event, EventServiceError> {
        // 获取现有事件
        let existing_event = self.get_event_by_id(dto.id).await?
            .ok_or(EventServiceError::EventNotFound(dto.id))?;

        // 更新事件字段
        let updated_event = Event {
            id: dto.id,
            title: dto.title.unwrap_or(existing_event.title),
            description: dto.description.unwrap_or(existing_event.description),
            start: dto.start.unwrap_or(existing_event.start),
            end: dto.end.unwrap_or(existing_event.end),
            all_day: dto.all_day.unwrap_or(existing_event.all_day),
            reminder_minutes: dto.reminder_minutes.unwrap_or(existing_event.reminder_minutes),
            recurrence_rule: dto.recurrence_rule.unwrap_or(existing_event.recurrence_rule),
            location: dto.location.unwrap_or(existing_event.location),
            url: dto.url.unwrap_or(existing_event.url),
            categories: dto.categories.unwrap_or(existing_event.categories),
            priority: match dto.priority {
                Some(p) => p,
                None => existing_event.priority,
            },
            status: dto.status.unwrap_or(existing_event.status),
            updated_at: Utc::now(),
            ..existing_event // 保持其他字段不变
        };

        let db = self.db.lock().await;
        
        // 删除旧的提醒
        db.delete_reminders_for_event(dto.id).await?;
        
        // 创建新的提醒（如果需要）
        if updated_event.reminder_minutes > 0 {
            let reminder_time = updated_event.start - Duration::minutes(updated_event.reminder_minutes as i64);
            db.create_reminder(updated_event.id, reminder_time).await?;
        }

        let result = db.update_event(&updated_event).await?;
        Ok(result)
    }

    pub async fn delete_event(&self, id: i64) -> Result<(), EventServiceError> {
        let db = self.db.lock().await;
        
        // 删除相关的提醒
        db.delete_reminders_for_event(id).await?;
        
        // 删除事件本身
        db.delete_event(id).await?;
        
        Ok(())
    }

    pub async fn get_upcoming_reminders(&self) -> Result<Vec<(Event, DateTime<Utc>)>, EventServiceError> {
        let db = self.db.lock().await;
        let reminder_pairs = db.get_upcoming_reminders().await?;

        let mut results = Vec::new();
        for (event_id, scheduled_time) in reminder_pairs {
            if let Some(event) = db.get_event_by_id(event_id).await? {
                results.push((event, scheduled_time));
            }
        }

        Ok(results)
    }

    pub async fn mark_reminder_triggered(&self, event_id: i64) -> Result<(), EventServiceError> {
        let db = self.db.lock().await;
        db.mark_reminder_triggered(event_id).await?;
        Ok(())
    }

    pub async fn get_events_by_calendar(&self, calendar_id: &str) -> Result<Vec<Event>, EventServiceError> {
        let db = self.db.lock().await;
        let events = db.get_all_events().await?;
        let filtered_events = events.into_iter()
            .filter(|event| event.calendar_id.as_deref() == Some(calendar_id))
            .collect();
        Ok(filtered_events)
    }

    pub async fn get_events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Event>, EventServiceError> {
        let db = self.db.lock().await;
        let all_events = db.get_all_events().await?;
        
        let filtered_events = all_events.into_iter()
            .filter(|event| {
                // 检查事件是否在指定范围内
                (event.start >= start && event.start <= end) ||  // 事件开始时间在范围内
                (event.end >= start && event.end <= end) ||      // 事件结束时间在范围内
                (event.start <= start && event.end >= end)       // 事件完全包含范围
            })
            .collect();
        
        Ok(filtered_events)
    }
}