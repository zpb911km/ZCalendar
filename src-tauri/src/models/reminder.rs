use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: i64,
    pub event_id: i64,
    pub scheduled_time: DateTime<Utc>,
    pub is_triggered: bool,
    pub created_at: DateTime<Utc>,
}

impl Reminder {
    pub fn new(event_id: i64, scheduled_time: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // 由数据库自动生成
            event_id,
            scheduled_time,
            is_triggered: false,
            created_at: now,
        }
    }
}