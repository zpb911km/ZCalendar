use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i64,
    pub title: String,          // SUMMARY属性
    pub description: Option<String>, // DESCRIPTION属性
    pub start: DateTime<Utc>,   // DTSTART属性
    pub end: DateTime<Utc>,     // DTEND属性
    pub all_day: bool,          // 通过DTSTART/DTEND格式体现
    pub reminder_minutes: i32,  // VALARM TRIGGER属性
    pub created_at: DateTime<Utc>,  // CREATED属性
    pub updated_at: DateTime<Utc>,  // LAST-MODIFIED属性
    pub recurrence_rule: Option<String>, // RRULE属性
    pub recurrence_id: Option<String>,   // RECURRENCE-ID属性
    pub sequence: i32,          // SEQUENCE属性
    pub status: String,         // STATUS属性 (TENTATIVE/CONFIRMED/CANCELLED)
    pub location: Option<String>, // LOCATION属性
    pub organizer: Option<String>, // ORGANIZER属性
    pub attendees: Option<String>, // ATTENDEE属性 (存储JSON格式)
    pub url: Option<String>,    // URL属性
    pub categories: Option<String>, // CATEGORIES属性
    pub priority: Option<i32>,  // PRIORITY属性
    pub calendar_id: Option<String>, // 扩展属性
}

impl Event {
    pub fn new(
        title: String,
        description: Option<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        all_day: bool,
        reminder_minutes: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // 由数据库自动生成
            title,
            description,
            start,
            end,
            all_day,
            reminder_minutes,
            created_at: now,
            updated_at: now,
            recurrence_rule: None,
            recurrence_id: None,
            sequence: 0,
            status: "CONFIRMED".to_string(),
            location: None,
            organizer: None,
            attendees: None,
            url: None,
            categories: None,
            priority: Some(0),
            calendar_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventDto {
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub all_day: bool,
    pub reminder_minutes: i32,
    pub recurrence_rule: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub categories: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEventDto {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub all_day: Option<bool>,
    pub reminder_minutes: Option<i32>,
    pub recurrence_rule: Option<Option<String>>,
    pub location: Option<Option<String>>,
    pub url: Option<Option<String>>,
    pub categories: Option<Option<String>>,
    pub priority: Option<Option<i32>>,
    pub status: Option<String>,
}