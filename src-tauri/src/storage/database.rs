use sqlx::{Pool, Sqlite, Row};
use crate::models::event::Event;
use crate::models::calendar::Calendar;
use chrono::{DateTime, Utc};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::SqlitePool::connect(db_path).await?;
        
        // 初始化数据库表
        Self::init_tables(&pool).await?;
        
        Ok(Self { pool })
    }

    async fn init_tables(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        // 创建events表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                start DATETIME NOT NULL,
                end DATETIME NOT NULL,
                all_day BOOLEAN NOT NULL DEFAULT 0,
                reminder_minutes INTEGER NOT NULL DEFAULT 0,
                recurrence_rule TEXT,
                recurrence_id TEXT,
                sequence INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'CONFIRMED',
                location TEXT,
                organizer TEXT,
                attendees TEXT,
                url TEXT,
                categories TEXT,
                priority INTEGER,
                calendar_id TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        // 创建reminders表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS reminders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_id INTEGER NOT NULL,
                scheduled_time DATETIME NOT NULL,
                is_triggered BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE
            )
            "#
        )
        .execute(pool)
        .await?;

        // 创建calendars表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS calendars (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                color TEXT NOT NULL,
                is_primary BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        // 创建索引
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_events_start ON events(start)")
            .execute(pool)
            .await?;
            
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_events_calendar ON events(calendar_id)")
            .execute(pool)
            .await?;
            
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_reminders_scheduled ON reminders(scheduled_time, is_triggered)")
            .execute(pool)
            .await?;

        Ok(())
    }

    pub fn get_pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    // Event相关操作
    pub async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, title, description, start, end, all_day, 
                reminder_minutes, created_at, updated_at, 
                recurrence_rule, recurrence_id, sequence, 
                status, location, organizer, attendees, 
                url, categories, priority, calendar_id
            FROM events
            ORDER BY start ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let events = rows.into_iter().map(|row| Event {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            start: row.get("start"),
            end: row.get("end"),
            all_day: row.get("all_day"),
            reminder_minutes: row.get("reminder_minutes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            recurrence_rule: row.get("recurrence_rule"),
            recurrence_id: row.get("recurrence_id"),
            sequence: row.get("sequence"),
            status: row.get("status"),
            location: row.get("location"),
            organizer: row.get("organizer"),
            attendees: row.get("attendees"),
            url: row.get("url"),
            categories: row.get("categories"),
            priority: row.get("priority"),
            calendar_id: row.get("calendar_id"),
        }).collect();

        Ok(events)
    }

    pub async fn get_events_by_date(&self, date: DateTime<Utc>) -> Result<Vec<Event>, sqlx::Error> {
        let start_of_day = date.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end_of_day = date.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();

        let rows = sqlx::query(
            r#"
            SELECT 
                id, title, description, start, end, all_day, 
                reminder_minutes, created_at, updated_at, 
                recurrence_rule, recurrence_id, sequence, 
                status, location, organizer, attendees, 
                url, categories, priority, calendar_id
            FROM events
            WHERE (start BETWEEN ? AND ?) OR (end BETWEEN ? AND ?)
            ORDER BY start ASC
            "#
        )
        .bind(start_of_day)
        .bind(end_of_day)
        .bind(start_of_day)
        .bind(end_of_day)
        .fetch_all(&self.pool)
        .await?;

        let events = rows.into_iter().map(|row| Event {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            start: row.get("start"),
            end: row.get("end"),
            all_day: row.get("all_day"),
            reminder_minutes: row.get("reminder_minutes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            recurrence_rule: row.get("recurrence_rule"),
            recurrence_id: row.get("recurrence_id"),
            sequence: row.get("sequence"),
            status: row.get("status"),
            location: row.get("location"),
            organizer: row.get("organizer"),
            attendees: row.get("attendees"),
            url: row.get("url"),
            categories: row.get("categories"),
            priority: row.get("priority"),
            calendar_id: row.get("calendar_id"),
        }).collect();

        Ok(events)
    }

    pub async fn get_event_by_id(&self, id: i64) -> Result<Option<Event>, sqlx::Error> {
        if let Some(row) = sqlx::query(
            r#"
            SELECT 
                id, title, description, start, end, all_day, 
                reminder_minutes, created_at, updated_at, 
                recurrence_rule, recurrence_id, sequence, 
                status, location, organizer, attendees, 
                url, categories, priority, calendar_id
            FROM events
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await? {
            Ok(Some(Event {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                start: row.get("start"),
                end: row.get("end"),
                all_day: row.get("all_day"),
                reminder_minutes: row.get("reminder_minutes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                recurrence_rule: row.get("recurrence_rule"),
                recurrence_id: row.get("recurrence_id"),
                sequence: row.get("sequence"),
                status: row.get("status"),
                location: row.get("location"),
                organizer: row.get("organizer"),
                attendees: row.get("attendees"),
                url: row.get("url"),
                categories: row.get("categories"),
                priority: row.get("priority"),
                calendar_id: row.get("calendar_id"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_event(&self, event: &Event) -> Result<Event, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO events (
                title, description, start, end, all_day, 
                reminder_minutes, recurrence_rule, recurrence_id,
                sequence, status, location, organizer, 
                attendees, url, categories, priority, calendar_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING 
                id, title, description, start, end, all_day, 
                reminder_minutes, created_at, updated_at, 
                recurrence_rule, recurrence_id, sequence, 
                status, location, organizer, attendees, 
                url, categories, priority, calendar_id
            "#
        )
        .bind(&event.title)
        .bind(&event.description)
        .bind(event.start)
        .bind(event.end)
        .bind(event.all_day)
        .bind(event.reminder_minutes)
        .bind(&event.recurrence_rule)
        .bind(&event.recurrence_id)
        .bind(event.sequence)
        .bind(&event.status)
        .bind(&event.location)
        .bind(&event.organizer)
        .bind(&event.attendees)
        .bind(&event.url)
        .bind(&event.categories)
        .bind(event.priority)
        .bind(&event.calendar_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Event {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            start: row.get("start"),
            end: row.get("end"),
            all_day: row.get("all_day"),
            reminder_minutes: row.get("reminder_minutes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            recurrence_rule: row.get("recurrence_rule"),
            recurrence_id: row.get("recurrence_id"),
            sequence: row.get("sequence"),
            status: row.get("status"),
            location: row.get("location"),
            organizer: row.get("organizer"),
            attendees: row.get("attendees"),
            url: row.get("url"),
            categories: row.get("categories"),
            priority: row.get("priority"),
            calendar_id: row.get("calendar_id"),
        })
    }

    pub async fn update_event(&self, event: &Event) -> Result<Event, sqlx::Error> {
        let row = sqlx::query(
            r#"
            UPDATE events 
            SET 
                title = ?,
                description = ?,
                start = ?,
                end = ?,
                all_day = ?,
                reminder_minutes = ?,
                recurrence_rule = ?,
                recurrence_id = ?,
                sequence = ?,
                status = ?,
                location = ?,
                organizer = ?,
                attendees = ?,
                url = ?,
                categories = ?,
                priority = ?,
                calendar_id = ?,
                updated_at = ?
            WHERE id = ?
            RETURNING 
                id, title, description, start, end, all_day, 
                reminder_minutes, created_at, updated_at, 
                recurrence_rule, recurrence_id, sequence, 
                status, location, organizer, attendees, 
                url, categories, priority, calendar_id
            "#
        )
        .bind(&event.title)
        .bind(&event.description)
        .bind(event.start)
        .bind(event.end)
        .bind(event.all_day)
        .bind(event.reminder_minutes)
        .bind(&event.recurrence_rule)
        .bind(&event.recurrence_id)
        .bind(event.sequence)
        .bind(&event.status)
        .bind(&event.location)
        .bind(&event.organizer)
        .bind(&event.attendees)
        .bind(&event.url)
        .bind(&event.categories)
        .bind(event.priority)
        .bind(&event.calendar_id)
        .bind(Utc::now())
        .bind(event.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Event {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            start: row.get("start"),
            end: row.get("end"),
            all_day: row.get("all_day"),
            reminder_minutes: row.get("reminder_minutes"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            recurrence_rule: row.get("recurrence_rule"),
            recurrence_id: row.get("recurrence_id"),
            sequence: row.get("sequence"),
            status: row.get("status"),
            location: row.get("location"),
            organizer: row.get("organizer"),
            attendees: row.get("attendees"),
            url: row.get("url"),
            categories: row.get("categories"),
            priority: row.get("priority"),
            calendar_id: row.get("calendar_id"),
        })
    }

    pub async fn delete_event(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM events WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    // Calendar相关操作
    pub async fn get_all_calendars(&self) -> Result<Vec<Calendar>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, color, created_at, updated_at, is_primary
            FROM calendars
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let calendars = rows.into_iter().map(|row| Calendar {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            is_primary: row.get("is_primary"),
        }).collect();

        Ok(calendars)
    }

    pub async fn create_calendar(&self, calendar: &Calendar) -> Result<Calendar, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO calendars (id, name, color, is_primary)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(&calendar.id)
        .bind(&calendar.name)
        .bind(&calendar.color)
        .bind(calendar.is_primary)
        .execute(&self.pool)
        .await?;

        Ok(calendar.clone())
    }

    // Reminder相关操作
    pub async fn create_reminder(&self, event_id: i64, scheduled_time: DateTime<Utc>) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO reminders (event_id, scheduled_time)
            VALUES (?, ?)
            "#
        )
        .bind(event_id)
        .bind(scheduled_time)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_upcoming_reminders(&self) -> Result<Vec<(i64, DateTime<Utc>)>, sqlx::Error> {
        let now = Utc::now();
        
        let rows = sqlx::query(
            r#"
            SELECT event_id, scheduled_time
            FROM reminders
            WHERE scheduled_time <= ? AND is_triggered = 0
            ORDER BY scheduled_time ASC
            "#
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await?;

        let reminders = rows.into_iter().map(|row| {
            (row.get("event_id"), row.get("scheduled_time"))
        }).collect();

        Ok(reminders)
    }

    pub async fn mark_reminder_triggered(&self, event_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE reminders SET is_triggered = 1 WHERE event_id = ?"#
        )
        .bind(event_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_reminders_for_event(&self, event_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM reminders WHERE event_id = ?")
            .bind(event_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}