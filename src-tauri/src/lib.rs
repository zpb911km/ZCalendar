use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ical::parser::ical::IcalParser;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;
use tokio::spawn;
use tokio::time::{sleep, Duration as TokioDuration};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Event {
    id: i32,
    title: String,
    description: Option<String>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    all_day: bool,
    reminder_minutes: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    recurrence_rule: Option<String>,
    recurrence_id: Option<String>,
    sequence: i32,
    status: String,
    location: Option<String>,
    organizer: Option<String>,
    attendees: Option<String>,
    url: Option<String>,
    categories: Option<String>,
    priority: Option<i32>,
    calendar_id: Option<String>,
}

// 提醒相关的结构体和常量
#[derive(Debug, Clone, Serialize)]
struct ReminderPayload {
    id: i32,
    title: String,
    body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Calendar {
    id: String,
    name: String,
    color: String,
    is_primary: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreateEventDto {
    title: String,
    description: Option<String>,
    start: String, // ISO 8601 formatted string
    end: String,   // ISO 8601 formatted string
    all_day: bool,
    reminder_minutes: i32,
    recurrence_rule: Option<String>,
    location: Option<String>,
    url: Option<String>,
    categories: Option<String>,
    priority: Option<i32>,
    status: Option<String>,
    calendar_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct UpdateEventDto {
    id: i32,
    title: Option<String>,
    description: Option<String>,
    start: Option<String>,
    end: Option<String>,
    all_day: Option<bool>,
    reminder_minutes: Option<i32>,
    recurrence_rule: Option<String>,
    location: Option<String>,
    url: Option<String>,
    categories: Option<String>,
    priority: Option<i32>,
    status: Option<String>,
    calendar_id: Option<String>,
}

#[tauri::command]
async fn get_all_events(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<Event>, String> {
    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events ORDER BY start";
    let rows = sqlx::query_as::<_, Event>(query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn get_events_by_date(
    pool: tauri::State<'_, SqlitePool>,
    date: String,
) -> Result<Vec<Event>, String> {
    let date_start = format!("{} 00:00:00", date.split('T').next().unwrap_or(&date));
    let date_end = format!("{} 23:59:59", date.split('T').next().unwrap_or(&date));

    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE start BETWEEN ? AND ? ORDER BY start";
    let rows = sqlx::query_as::<_, Event>(query)
        .bind(&date_start)
        .bind(&date_end)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn get_event_by_id(
    pool: tauri::State<'_, SqlitePool>,
    id: i32,
) -> Result<Option<Event>, String> {
    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE id = ?";
    let row = sqlx::query_as::<_, Event>(query)
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row)
}

#[tauri::command]
async fn create_event(
    pool: tauri::State<'_, SqlitePool>,
    event: CreateEventDto,
) -> Result<Event, String> {
    let now = Utc::now();
    let status = event.status.unwrap_or_else(|| "CONFIRMED".to_string());

    let query = "INSERT INTO events (title, description, start, end, all_day, reminder_minutes, created_at, updated_at, sequence, status, location, url, categories, priority, calendar_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?, ?, ?, ?) RETURNING id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id";
    let new_event = sqlx::query_as::<_, Event>(query)
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.start)
        .bind(&event.end)
        .bind(event.all_day)
        .bind(event.reminder_minutes)
        .bind(&now.to_rfc3339())
        .bind(&now.to_rfc3339())
        .bind(&status)
        .bind(&event.location)
        .bind(&event.url)
        .bind(&event.categories)
        .bind(event.priority)
        .bind(&event.calendar_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(new_event)
}

#[tauri::command]
async fn update_event(
    pool: tauri::State<'_, SqlitePool>,
    event: UpdateEventDto,
) -> Result<Event, String> {
    let mut query_builder = sqlx::QueryBuilder::new("UPDATE events SET updated_at = ");
    query_builder.push_bind(Utc::now().to_rfc3339());

    if let Some(ref title) = event.title {
        query_builder.push(", title = ");
        query_builder.push_bind(title);
    }
    if let Some(ref desc) = event.description {
        query_builder.push(", description = ");
        query_builder.push_bind(desc);
    }
    if let Some(ref start) = event.start {
        query_builder.push(", start = ");
        query_builder.push_bind(start);
    }
    if let Some(ref end) = event.end {
        query_builder.push(", end = ");
        query_builder.push_bind(end);
    }
    if let Some(all_day) = event.all_day {
        query_builder.push(", all_day = ");
        query_builder.push_bind(all_day);
    }
    if let Some(reminder_minutes) = event.reminder_minutes {
        query_builder.push(", reminder_minutes = ");
        query_builder.push_bind(reminder_minutes);
    }
    if let Some(ref rule) = event.recurrence_rule {
        query_builder.push(", recurrence_rule = ");
        query_builder.push_bind(rule);
    }
    if let Some(ref location) = event.location {
        query_builder.push(", location = ");
        query_builder.push_bind(location);
    }
    if let Some(ref url) = event.url {
        query_builder.push(", url = ");
        query_builder.push_bind(url);
    }
    if let Some(ref categories) = event.categories {
        query_builder.push(", categories = ");
        query_builder.push_bind(categories);
    }
    if let Some(priority) = event.priority {
        query_builder.push(", priority = ");
        query_builder.push_bind(priority);
    }
    if let Some(ref status) = event.status {
        query_builder.push(", status = ");
        query_builder.push_bind(status);
    }
    if let Some(ref calendar_id) = event.calendar_id {
        query_builder.push(", calendar_id = ");
        query_builder.push_bind(calendar_id);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(event.id);
    query_builder.push(" RETURNING id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id");

    let updated_event = query_builder
        .build_query_as::<Event>()
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated_event)
}

#[tauri::command]
async fn delete_event(pool: tauri::State<'_, SqlitePool>, id: i32) -> Result<(), String> {
    let query = "DELETE FROM events WHERE id = ?";
    sqlx::query(query)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_events_in_range(
    pool: tauri::State<'_, SqlitePool>,
    start: String,
    end: String,
) -> Result<Vec<Event>, String> {
    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE start BETWEEN ? AND ? ORDER BY start";
    let rows = sqlx::query_as::<_, Event>(query)
        .bind(&start)
        .bind(&end)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn import_ical(
    pool: tauri::State<'_, SqlitePool>,
    ical_content: String,
) -> Result<Vec<Event>, String> {
    let parser = IcalParser::new(ical_content.as_bytes());
    let mut events = Vec::new();

    for calendar_result in parser {
        let calendar = calendar_result.map_err(|e| format!("Failed to parse calendar: {:?}", e))?;

        // 遍历日历中的事件
        for event in calendar.events {
            let mut title = String::new();
            let mut description: Option<String> = None;
            let mut start: Option<String> = None;
            let mut end: Option<String> = None;
            let mut location: Option<String> = None;
            let mut url: Option<String> = None;
            let mut categories: Option<String> = None;
            let mut priority: Option<i32> = None;
            let mut status: Option<String> = None;
            let mut recurrence_rule: Option<String> = None;

            // 处理事件属性
            for property in &event.properties {
                match property.name.as_str() {
                    "SUMMARY" => title = property.value.clone().unwrap_or_default(),
                    "DESCRIPTION" => description = property.value.clone(),
                    "DTSTART" => start = property.value.clone(),
                    "DTEND" => end = property.value.clone(),
                    "LOCATION" => location = property.value.clone(),
                    "URL" => url = property.value.clone(),
                    "CATEGORIES" => categories = property.value.clone(),
                    "PRIORITY" => {
                        if let Some(ref p) = property.value {
                            priority = p.parse::<i32>().ok();
                        }
                    }
                    "STATUS" => status = property.value.clone(),
                    "RRULE" => recurrence_rule = property.value.clone(),
                    _ => {}
                }
            }

            if let (Some(start_str), Some(end_str)) = (start, end) {
                let start_dt = parse_ical_datetime(&start_str).unwrap_or_else(|_| Utc::now());
                let end_dt = parse_ical_datetime(&end_str).unwrap_or_else(|_| Utc::now());

                let create_event_dto = CreateEventDto {
                    title,
                    description,
                    start: start_dt.to_rfc3339(),
                    end: end_dt.to_rfc3339(),
                    all_day: start_str.len() == 8, // ical日期格式为 YYYYMMDD 表示全天事件
                    reminder_minutes: 15,          // 默认提醒时间
                    recurrence_rule,
                    location,
                    url,
                    categories,
                    priority,
                    status: status.or(Some("CONFIRMED".to_string())),
                    calendar_id: None, // 默认没有日历ID
                };

                let new_event = create_event(pool.clone(), create_event_dto).await?;
                events.push(new_event);
            }
        }
    }

    Ok(events)
}

#[tauri::command]
async fn export_ical(
    pool: tauri::State<'_, SqlitePool>,
    event_ids: Option<Vec<i32>>,
) -> Result<String, String> {
    let events = if let Some(ids) = event_ids {
        // 导出指定ID的事件
        if ids.is_empty() {
            vec![]
        } else {
            let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
            let query = format!("SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE id IN ({})", placeholders.join(","));

            let mut query_builder = sqlx::query(&query);
            for id in &ids {
                query_builder = query_builder.bind(id);
            }

            let rows = query_builder
                .fetch_all(&*pool)
                .await
                .map_err(|e| e.to_string())?;

            rows.iter()
                .map(|row| Event {
                    id: row.get("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    start: parse_datetime(&row.get::<String, &str>("start"))
                        .unwrap_or_else(|_| Utc::now()),
                    end: parse_datetime(&row.get::<String, &str>("end"))
                        .unwrap_or_else(|_| Utc::now()),
                    all_day: row.get("all_day"),
                    reminder_minutes: row.get("reminder_minutes"),
                    created_at: parse_datetime(&row.get::<String, &str>("created_at"))
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: parse_datetime(&row.get::<String, &str>("updated_at"))
                        .unwrap_or_else(|_| Utc::now()),
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
                .collect()
        }
    } else {
        // 导出所有事件
        get_all_events(pool).await?
    };

    // 构建iCalendar格式
    let mut ical_content = String::from("BEGIN:VCALENDAR\nVERSION:2.0\nPRODID:-//ZCalendar//EN\n");

    for event in events {
        ical_content.push_str("BEGIN:VEVENT\n");

        // UID
        let uid = format!("event-{}@zcalendar", event.id);
        ical_content.push_str(&format!("UID:{}\n", escape_ical_text(&uid)));

        // 创建时间
        ical_content.push_str(&format!(
            "DTSTAMP:{}\n",
            format_ical_datetime(&event.created_at)
        ));

        // 开始时间
        if event.all_day {
            ical_content.push_str(&format!(
                "DTSTART;VALUE=DATE:{}\n",
                format_ical_date(&event.start)
            ));
        } else {
            ical_content.push_str(&format!("DTSTART:{}\n", format_ical_datetime(&event.start)));
        }

        // 结束时间
        if event.all_day {
            ical_content.push_str(&format!(
                "DTEND;VALUE=DATE:{}\n",
                format_ical_date(&event.end)
            ));
        } else {
            ical_content.push_str(&format!("DTEND:{}\n", format_ical_datetime(&event.end)));
        }

        // 标题
        ical_content.push_str(&format!("SUMMARY:{}\n", escape_ical_text(&event.title)));

        // 描述
        if let Some(ref desc) = event.description {
            ical_content.push_str(&format!("DESCRIPTION:{}\n", escape_ical_text(desc)));
        }

        // 位置
        if let Some(ref loc) = event.location {
            ical_content.push_str(&format!("LOCATION:{}\n", escape_ical_text(loc)));
        }

        // 状态
        ical_content.push_str(&format!("STATUS:{}\n", escape_ical_text(&event.status)));

        // 优先级
        if let Some(priority) = event.priority {
            ical_content.push_str(&format!("PRIORITY:{}\n", priority));
        }

        // 分类
        if let Some(ref cats) = event.categories {
            ical_content.push_str(&format!("CATEGORIES:{}\n", escape_ical_text(cats)));
        }

        // URL
        if let Some(ref url) = event.url {
            ical_content.push_str(&format!("URL:{}\n", escape_ical_text(url)));
        }

        // 重复规则
        if let Some(ref rrule) = event.recurrence_rule {
            ical_content.push_str(&format!("RRULE:{}\n", escape_ical_text(rrule)));
        }

        ical_content.push_str("END:VEVENT\n");
    }

    ical_content.push_str("END:VCALENDAR\n");

    Ok(ical_content)
}

#[tauri::command]
async fn search_events(
    pool: tauri::State<'_, SqlitePool>,
    query: String,
) -> Result<Vec<Event>, String> {
    let search_query = format!("%{}%", query);
    let db_query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE title LIKE ? OR description LIKE ? OR location LIKE ? ORDER BY start";

    let rows = sqlx::query_as::<_, Event>(db_query)
        .bind(&search_query)
        .bind(&search_query)
        .bind(&search_query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn get_upcoming_events(
    pool: tauri::State<'_, SqlitePool>,
    limit: i32,
) -> Result<Vec<Event>, String> {
    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE start >= ? ORDER BY start LIMIT ?";
    let now = Utc::now().to_rfc3339();

    let rows = sqlx::query_as::<_, Event>(query)
        .bind(&now)
        .bind(limit)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn get_all_calendars(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<Calendar>, String> {
    let query =
        "SELECT id, name, color, is_primary, created_at, updated_at FROM calendars ORDER BY name";
    let rows = sqlx::query_as::<_, Calendar>(query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn create_calendar(
    pool: tauri::State<'_, SqlitePool>,
    calendar: Calendar,
) -> Result<Calendar, String> {
    let now = Utc::now();
    let query = "INSERT INTO calendars (id, name, color, is_primary, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?) RETURNING id, name, color, is_primary, created_at, updated_at";
    let new_calendar = sqlx::query_as::<_, Calendar>(query)
        .bind(&calendar.id)
        .bind(&calendar.name)
        .bind(&calendar.color)
        .bind(calendar.is_primary)
        .bind(&now.to_rfc3339())
        .bind(&now.to_rfc3339())
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(new_calendar)
}

#[tauri::command]
async fn get_calendar_by_id(
    pool: tauri::State<'_, SqlitePool>,
    id: String,
) -> Result<Option<Calendar>, String> {
    let query =
        "SELECT id, name, color, is_primary, created_at, updated_at FROM calendars WHERE id = ?";
    let row = sqlx::query_as::<_, Calendar>(query)
        .bind(&id)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row)
}

#[tauri::command]
async fn update_calendar(
    pool: tauri::State<'_, SqlitePool>,
    calendar: Calendar,
) -> Result<Calendar, String> {
    let now = Utc::now();
    let query = "UPDATE calendars SET name = ?, color = ?, is_primary = ?, updated_at = ? WHERE id = ? RETURNING id, name, color, is_primary, created_at, updated_at";

    let updated_calendar = sqlx::query_as::<_, Calendar>(query)
        .bind(&calendar.name)
        .bind(&calendar.color)
        .bind(calendar.is_primary)
        .bind(&now.to_rfc3339())
        .bind(&calendar.id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated_calendar)
}

#[tauri::command]
async fn delete_calendar(pool: tauri::State<'_, SqlitePool>, id: String) -> Result<(), String> {
    let query = "DELETE FROM calendars WHERE id = ?";
    sqlx::query(query)
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_calendar_events(
    pool: tauri::State<'_, SqlitePool>,
    calendar_id: String,
) -> Result<Vec<Event>, String> {
    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE calendar_id = ? ORDER BY start";
    let rows = sqlx::query_as::<_, Event>(query)
        .bind(&calendar_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

// 辅助函数
fn parse_datetime(s: &str) -> Result<DateTime<Utc>, String> {
    // 尝试解析RFC3339格式
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            // 尝试解析其他格式
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
        })
        .map_err(|e| e.to_string())
}

fn parse_ical_datetime(s: &str) -> Result<DateTime<Utc>, String> {
    // 尝试解析iCalendar日期时间格式
    if s.len() == 8 {
        // 日期格式 YYYYMMDD
        let date = chrono::NaiveDate::parse_from_str(s, "%Y%m%d").map_err(|e| e.to_string())?;
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    } else if s.len() == 16 {
        // 日期时间格式 YYYYMMDDTHHMMSS
        let datetime_str = s.replace("T", "");
        let datetime = NaiveDateTime::parse_from_str(&datetime_str, "%Y%m%d%H%M%S")
            .map_err(|e| e.to_string())?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    } else {
        // 尝试解析ISO 8601格式
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| e.to_string())
    }
}

fn format_ical_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y%m%dT%H%M%S").to_string()
}

fn format_ical_date(dt: &DateTime<Utc>) -> String {
    dt.date_naive().format("%Y%m%d").to_string()
}

fn escape_ical_text(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
        .replace('\r', "")
}

// 提醒服务相关函数
async fn start_reminder_service<R: Runtime>(app_handle: AppHandle<R>, pool: SqlitePool) {
    spawn(async move {
        loop {
            println!("Checking for upcoming events...");
            check_upcoming_events(&app_handle, &pool).await;
            // 每分钟检查一次即将到来的事件
            sleep(TokioDuration::from_secs(30)).await;
        }
    });
}

async fn check_upcoming_events<R: Runtime>(app_handle: &AppHandle<R>, pool: &SqlitePool) {
    let now = Utc::now();
    send_heartbeat_notification(app_handle).await;
    // 查询未来1分钟内需要提醒的事件
    let query = "SELECT * FROM events WHERE reminder_minutes > 0 AND start > ? AND status != 'CANCELLED'";
    let upcoming_events: Vec<Event> = sqlx::query_as::<_, Event>(query)
        .bind(&now.to_rfc3339())
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    println!("find {} upcoming events at {}", upcoming_events.len(), now.to_rfc3339());
    for event in upcoming_events {
        // 计算事件开始时间与当前时间的差值
        let reminder_minutes = event.reminder_minutes;
        let time_diff = event.start - now - Duration::minutes(reminder_minutes as i64);
        let mut minutes_diff = time_diff.num_minutes();
        if minutes_diff < 0 {
            minutes_diff = -minutes_diff;  // abs
        }

        // 如果当前时间正好是提醒时间
        if minutes_diff <= 1 {
            send_reminder_notification(app_handle, &event).await;
        }
        // println!("minutes_diff: {}", minutes_diff);
        // send_reminder_notification(app_handle, &event).await;
    }
}

async fn send_heartbeat_notification<R: Runtime>(app_handle: &AppHandle<R>) {
    let _ = app_handle.plugin(tauri_plugin_notification::init());
    let _ = app_handle
        .notification()
        .builder()
        .title("Heartbeat")
        .body(Utc::now().to_rfc3339())
        .show();
}

async fn send_reminder_notification<R: Runtime>(app_handle: &AppHandle<R>, event: &Event) {
    let title = format!("事件提醒: {}", event.title);
    let description = event.description.clone().unwrap_or_default();
    let start_time = event.start.format("%H:%M").to_string();

    let body = if description.is_empty() {
        format!("时间: {}", start_time)
    } else {
        format!("{}\n时间: {}", description, start_time)
    };

    // 发送通知到前端
    let payload = ReminderPayload {
        id: event.id,
        title,
        body,
    };
    let _ = app_handle.plugin(tauri_plugin_notification::init());
    let _ = app_handle
        .notification()
        .builder()
        .title(payload.title)
        .body(payload.body)
        .show();
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建数据库连接池
            tauri::async_runtime::block_on(async {
                let app_handle = app.handle();
                // 获取应用数据目录并构建数据库路径
                let app_dir = match app_handle.path().app_data_dir() {
                    Ok(dir) => dir,
                    Err(_) => {
                        // 如果无法获取app_data_dir，则使用当前工作目录
                        std::env::current_dir().unwrap().join("app_data")
                    }
                };
                let db_dir = app_dir.join("database");
                let db_path = db_dir.join("zcalendar.db");

                // 确保数据库目录存在
                std::fs::create_dir_all(&db_dir).expect("Failed to create database directory");

                // 验证数据库路径是否可写入
                let db_path_str = db_path.to_string_lossy().to_string();
                println!("Database path: {}", db_path_str);
                if std::fs::File::open(&db_path).is_err() {
                    // 如果文件不存在，尝试创建一个空文件
                    std::fs::File::create(&db_path).expect("Failed to create database file");
                }

                let db_url = format!("sqlite:{}", db_path_str);

                let pool = SqlitePool::connect(&db_url)
                    .await
                    .expect("Failed to connect to database");

                // 创建数据表
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run migrations");

                // 启动提醒服务
                spawn(start_reminder_service(app_handle.clone(), pool.clone()));

                // 将数据库连接池存储在应用状态中
                app.manage(pool);
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 为移动平台添加通知插件
            #[cfg(mobile)]
            {
                app.handle().plugin(tauri_plugin_notification::init())?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_all_events,
            get_events_by_date,
            get_event_by_id,
            create_event,
            update_event,
            delete_event,
            get_events_in_range,
            import_ical,
            export_ical,
            search_events,
            get_upcoming_events,
            get_all_calendars,
            create_calendar,
            get_calendar_by_id,
            update_calendar,
            delete_calendar,
            get_calendar_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
