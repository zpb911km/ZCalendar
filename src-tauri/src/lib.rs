use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Utc};
use ical::parser::ical::IcalParser;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, MySqlPool};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;
use tokio::spawn;
use tokio::time::{sleep, Duration as TokioDuration};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Event {
    id: i32,
    title: String,
    description: Option<String>,
    start: String,
    end: String,
    all_day: bool,
    reminder_minutes: i32,
    created_at: String,
    updated_at: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct Calendar {
    id: String,
    name: String,
    color: String,
    is_primary: bool,
    created_at: String,
    updated_at: String,
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
async fn get_all_events(pool: tauri::State<'_, MySqlPool>) -> Result<Vec<Event>, String> {
    let query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events ORDER BY start";
    let rows = sqlx::query_as::<_, Event>(query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
async fn get_events_by_date(
    pool: tauri::State<'_, MySqlPool>,
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
    pool: tauri::State<'_, MySqlPool>,
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
    pool: tauri::State<'_, MySqlPool>,
    event: CreateEventDto,
) -> Result<Event, String> {
    let now = Utc::now();
    let status = event.status.unwrap_or_else(|| "CONFIRMED".to_string());

    // MySQL 不支持 RETURNING，需要分两步：先插入，再查询
    let insert_query = "INSERT INTO events (title, description, start, end, all_day, reminder_minutes, created_at, updated_at, sequence, status, location, url, categories, priority, calendar_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?, ?, ?, ?)";

    let last_id;
    match sqlx::query(insert_query)
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
        .execute(&*pool)
        .await
    {
        Ok(i) => {
            last_id = i.last_insert_id();
        },
        Err(e) => {
            return Err(format!("插入事件失败: {}", e));
        }
    }

    // 查询完整记录
    let select_query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE id = ?";

    match sqlx::query_as::<_, Event>(select_query)
        .bind(last_id)
        .fetch_one(&*pool)
        .await
    {
        Ok(event) => Ok(event),
        Err(e) => Err(format!("查询事件失败 (ID: {}): {}", last_id, e)),
    }
}

#[tauri::command]
async fn update_event(
    pool: tauri::State<'_, MySqlPool>,
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

    // 执行更新
    query_builder
        .build()
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    // 查询更新后的记录
    let select_query = "SELECT id, title, description, start, end, all_day, reminder_minutes, created_at, updated_at, recurrence_rule, recurrence_id, sequence, status, location, organizer, attendees, url, categories, priority, calendar_id FROM events WHERE id = ?";
    let updated_event = sqlx::query_as::<_, Event>(select_query)
        .bind(event.id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated_event)
}

#[tauri::command]
async fn delete_event(pool: tauri::State<'_, MySqlPool>, id: i32) -> Result<(), String> {
    let query = "DELETE FROM events WHERE id = ?";
    sqlx::query(query)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn delete_all_events(pool: tauri::State<'_, MySqlPool>) -> Result<(), String> {
    sqlx::query("DELETE FROM events")
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_events_in_range(
    pool: tauri::State<'_, MySqlPool>,
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
    pool: tauri::State<'_, MySqlPool>,
    ical_content: String,
    timezone_offset: Option<i32>,
) -> Result<Vec<Event>, String> {
    let parser = IcalParser::new(ical_content.as_bytes());
    let mut events = Vec::new();
    let timezone_offset_hours = timezone_offset.unwrap_or(0);

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
            let mut reminder_minutes: i32 = 15; // 默认提醒时间 15 分钟

            // 处理事件属性
            for property in &event.properties {
                let rpl = |s: String| 
                    s.replace("\\n", "\n")
                    .replace("\\N", "\n")
                    .replace("\\;", ";")
                    .replace("\\,", ",")
                    .replace("\\\\", "\\")
                    .replace("，", ",");
                match property.name.as_str() {
                    "SUMMARY" => title = property.value.clone().unwrap_or_default().replace("\\n", "\n"),
                    "DESCRIPTION" => description = property.value.clone().map(rpl),
                    "DTSTART" => start = property.value.clone(),
                    "DTEND" => end = property.value.clone(),
                    "LOCATION" => location = property.value.clone().map(rpl),
                    "URL" => url = property.value.clone(),
                    "CATEGORIES" => categories = property.value.clone().map(rpl),
                    "PRIORITY" => {
                        if let Some(ref p) = property.value {
                            priority = p.parse::<i32>().ok();
                        }
                    }
                    "STATUS" => status = property.value.clone().map(rpl),
                    "RRULE" => recurrence_rule = property.value.clone().map(rpl),
                    _ => {
                        println!("未知属性：{} = {:?}", property.name, property.value);
                    }
                }
            }
            
            

            // 处理提醒（VALARM 组件）
            for alarm in &event.alarms {
                for property in &alarm.properties {
                    if property.name.as_str() == "TRIGGER" {
                        if let Some(ref trigger_value) = property.value {
                            // 解析 TRIGGER 值，例如 "-PT15M" 表示提前 15 分钟
                            if trigger_value.starts_with("-PT") && trigger_value.ends_with("M") {
                                let minutes_str = trigger_value.strip_prefix("-PT")
                                    .and_then(|s| s.strip_suffix("M"))
                                    .unwrap_or("15");
                                if let Ok(minutes) = minutes_str.parse::<i32>() {
                                    reminder_minutes = minutes;
                                }
                            }
                        }
                    }
                }
            }

            if let (Some(start_str), Some(end_str)) = (start, end) {
                // 使用带时区偏移的解析函数，并检测是否解析失败
                let start_dt_result = parse_ical_datetime_with_offset(&start_str, timezone_offset_hours);
                let end_dt_result = parse_ical_datetime_with_offset(&end_str, timezone_offset_hours);

                let start_dt = start_dt_result.clone().unwrap_or_else(|_| Utc::now());
                let end_dt = end_dt_result.clone().unwrap_or_else(|_| Utc::now());

                // 检查是否有日期解析失败，如果有则在标题前添加警告标志
                let final_title = if start_dt_result.is_err() || end_dt_result.is_err() {
                    if title.is_empty() {
                        "⚠️ 事件".to_string()
                    } else {
                        format!("⚠️ {}", title)
                    }
                } else {
                    title
                };

                let create_event_dto = CreateEventDto {
                    title: final_title,
                    description,
                    start: start_dt.to_rfc3339(),
                    end: end_dt.to_rfc3339(),
                    all_day: start_str.len() == 8, // ical日期格式为 YYYYMMDD 表示全天事件
                    reminder_minutes,
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
    pool: tauri::State<'_, MySqlPool>,
    event_ids: Option<Vec<i32>>,
    timezone_offset: Option<i32>,
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
                .collect()
        }
    } else {
        // 导出所有事件
        get_all_events(pool).await?
    };

    // 构建iCalendar格式
    let mut ical_content = String::from("BEGIN:VCALENDAR\nVERSION:2.0\nPRODID:-//ZCalendar//EN\n");

    // 获取时区偏移，如果没有提供则默认为0（UTC）
    let timezone_offset_hours = timezone_offset.unwrap_or(0);

    for event in events {
        ical_content.push_str("BEGIN:VEVENT\n");

        // UID
        let uid = format!("event-{}@zcalendar", event.id);
        ical_content.push_str(&format!("UID:{}\n", escape_ical_text(&uid)));

        // 创建时间
        ical_content.push_str(&format!(
            "DTSTAMP:{}\n",
            format_ical_datetime_with_offset(&event.created_at, timezone_offset_hours)
        ));

        // 开始时间
        if event.all_day {
            ical_content.push_str(&format!(
                "DTSTART;VALUE=DATE:{}\n",
                format_ical_date(&event.start)
            ));
        } else {
            ical_content.push_str(&format!(
                "DTSTART:{}\n",
                format_ical_datetime_with_offset(&event.start, timezone_offset_hours)
            ));
        }

        // 结束时间
        if event.all_day {
            ical_content.push_str(&format!(
                "DTEND;VALUE=DATE:{}\n",
                format_ical_date(&event.end)
            ));
        } else {
            ical_content.push_str(&format!(
                "DTEND:{}\n",
                format_ical_datetime_with_offset(&event.end, timezone_offset_hours)
            ));
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

        // 提醒（VALARM 组件）
        if event.reminder_minutes > 0 {
            ical_content.push_str("BEGIN:VALARM\n");
            ical_content.push_str("TRIGGER:-PT");
            ical_content.push_str(&event.reminder_minutes.to_string());
            ical_content.push_str("M\n");
            ical_content.push_str("ACTION:DISPLAY\n");
            ical_content.push_str("DESCRIPTION:Reminder\n");
            ical_content.push_str("END:VALARM\n");
        }

        ical_content.push_str("END:VEVENT\n");
    }

    ical_content.push_str("END:VCALENDAR\n");

    Ok(ical_content)
}

#[tauri::command]
async fn get_platform() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        return Ok("windows".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        return Ok("macos".to_string());
    }

    #[cfg(target_os = "linux")]
    {
        return Ok("linux".to_string());
    }

    #[cfg(target_os = "android")]
    {
        return Ok("android".to_string());
    }

    #[cfg(target_os = "ios")]
    {
        return Ok("ios".to_string());
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux", target_os = "android", target_os = "ios")))]
    {
        return Ok("unknown".to_string());
    }
}

#[tauri::command]
async fn save_file_to_downloads(app_handle: tauri::AppHandle, content: String, filename: String) -> Result<String, String> {
    use std::fs;
    use std::io::Write;
    
    // 获取下载目录
    let downloads_dir = app_handle.path().download_dir()
        .map_err(|e| format!("Failed to get download directory: {}", e))?;
    
    let file_path = downloads_dir.join(&filename);
    
    // 写入文件
    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;
    
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn search_events(
    pool: tauri::State<'_, MySqlPool>,
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
    pool: tauri::State<'_, MySqlPool>,
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
async fn get_all_calendars(pool: tauri::State<'_, MySqlPool>) -> Result<Vec<Calendar>, String> {
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
    pool: tauri::State<'_, MySqlPool>,
    calendar: Calendar,
) -> Result<Calendar, String> {
    let now = Utc::now();
    let insert_query = "INSERT INTO calendars (id, name, color, is_primary, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)";
    sqlx::query(insert_query)
        .bind(&calendar.id)
        .bind(&calendar.name)
        .bind(&calendar.color)
        .bind(calendar.is_primary)
        .bind(&now.to_rfc3339())
        .bind(&now.to_rfc3339())
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    // 查询插入的记录
    let select_query = "SELECT id, name, color, is_primary, created_at, updated_at FROM calendars WHERE id = ?";
    let new_calendar = sqlx::query_as::<_, Calendar>(select_query)
        .bind(&calendar.id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(new_calendar)
}

#[tauri::command]
async fn get_calendar_by_id(
    pool: tauri::State<'_, MySqlPool>,
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
    pool: tauri::State<'_, MySqlPool>,
    calendar: Calendar,
) -> Result<Calendar, String> {
    let now = Utc::now();
    let update_query = "UPDATE calendars SET name = ?, color = ?, is_primary = ?, updated_at = ? WHERE id = ?";
    sqlx::query(update_query)
        .bind(&calendar.name)
        .bind(&calendar.color)
        .bind(calendar.is_primary)
        .bind(&now.to_rfc3339())
        .bind(&calendar.id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    // 查询更新后的记录
    let select_query = "SELECT id, name, color, is_primary, created_at, updated_at FROM calendars WHERE id = ?";
    let updated_calendar = sqlx::query_as::<_, Calendar>(select_query)
        .bind(&calendar.id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(updated_calendar)
}

#[tauri::command]
async fn delete_calendar(pool: tauri::State<'_, MySqlPool>, id: String) -> Result<(), String> {
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
    pool: tauri::State<'_, MySqlPool>,
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

fn parse_ical_datetime_with_offset(s: &str, offset_hours: i32) -> Result<DateTime<Utc>, String> {
    // 尝试解析iCalendar日期时间格式
    if s.len() == 8 {
        // 日期格式 YYYYMMDD
        let date = chrono::NaiveDate::parse_from_str(s, "%Y%m%d").map_err(|e| e.to_string())?;
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
    } else if s.len() == 15 {
        // 日期时间格式 YYYYMMDDTHHMMSS
        let datetime_str = s.replace("T", "");
        let naive_datetime = NaiveDateTime::parse_from_str(&datetime_str, "%Y%m%d%H%M%S")
            .map_err(|e| e.to_string())?;
        // 将解析的时间视为本地时间，然后将其转换为UTC
        // 为将本地时间转换为UTC，需要减去时区偏移
        let offset_seconds = offset_hours * 3600;
        let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc)
            - chrono::Duration::seconds(offset_seconds as i64);
        Ok(dt_utc)
    } else {
        // 尝试解析ISO 8601格式
        let dt = DateTime::parse_from_rfc3339(s).map_err(|e| e.to_string())?;
        // 为将本地时间转换为UTC，需要减去时区偏移
        let offset_seconds = offset_hours * 3600;
        let dt_utc = dt.with_timezone(&Utc) - chrono::Duration::seconds(offset_seconds as i64);
        Ok(dt_utc)
    }
}

fn format_ical_datetime_with_offset(dt_str: &str, offset_hours: i32) -> String {
    // 解析字符串为 DateTime
    let dt = parse_datetime(dt_str).unwrap_or_else(|_| Utc::now());
    // 创建时区偏移
    let offset = FixedOffset::east_opt(offset_hours * 3600)
        .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    // 将UTC时间转换为指定时区
    let local_time = dt.with_timezone(&offset);
    local_time.format("%Y%m%dT%H%M%S").to_string()
}

fn format_ical_date(dt_str: &str) -> String {
    // 解析字符串为 DateTime
    let dt = parse_datetime(dt_str).unwrap_or_else(|_| Utc::now());
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
async fn start_reminder_service<R: Runtime>(app_handle: AppHandle<R>, pool: MySqlPool) {
    // 初始化Android通知渠道
    // #[cfg(target_os = "android")]
    // {
    //     // 等待应用初始化完成
    //     let app_handle_clone = app_handle.clone();
    //     tauri::async_runtime::spawn(async move {
    //         // 等待一段时间确保所有插件都已加载
    //         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    //         // 发送一个初始化通知以确保通知渠道被正确创建
    //         let _ = app_handle_clone
    //             .plugin(tauri_plugin_notification::init());
    //         let _ = app_handle_clone
    //             .notification()
    //             .builder()
    //             .title("通知系统初始化")
    //             .body("日历提醒服务已启动")
    //             .show();
    //     });
    // }

    spawn(async move {
        loop {
            check_upcoming_events(&app_handle, &pool).await;
            // 每分钟检查一次即将到来的事件
            sleep(TokioDuration::from_secs(60)).await;
        }
    });
}

async fn check_upcoming_events<R: Runtime>(app_handle: &AppHandle<R>, pool: &MySqlPool) {
    let now = Utc::now();
    // send_heartbeat_notification(app_handle).await;
    // 查询未来1分钟内需要提醒的事件
    let query =
        "SELECT * FROM events WHERE reminder_minutes > 0 AND start > ? AND status != 'CANCELLED'";
    let upcoming_events: Vec<Event> = sqlx::query_as::<_, Event>(query)
        .bind(&now.to_rfc3339())
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    // println!("find {} upcoming events at {}", upcoming_events.len(), now.to_rfc3339());
    for event in upcoming_events {
        // 解析事件开始时间
        let event_start = parse_datetime(&event.start).unwrap_or_else(|_| Utc::now());
        // 计算事件开始时间与当前时间的差值
        let reminder_minutes = event.reminder_minutes;
        let time_diff = event_start - now - Duration::minutes(reminder_minutes as i64);
        let mut minutes_diff = time_diff.num_minutes();
        if minutes_diff < 0 {
            minutes_diff = -minutes_diff; // abs
        }

        // 如果当前时间正好是提醒时间
        if minutes_diff <= 1 {
            send_reminder_notification(app_handle, &event).await;
        }
        // println!("minutes_diff: {}", minutes_diff);
        // send_reminder_notification(app_handle, &event).await;
    }
}

async fn send_reminder_notification<R: Runtime>(app_handle: &AppHandle<R>, event: &Event) {
    let title = format!("事件提醒: {}", event.title);
    let description = event.description.clone().unwrap_or_default();
    // 解析 start 时间字符串为 DateTime，然后格式化
    let start_dt = parse_datetime(&event.start).unwrap_or_else(|_| Utc::now());
    let start_time = start_dt.format("%H:%M").to_string();

    let body = if description.is_empty() {
        format!("时间: {}", start_time)
    } else {
        format!("{}\n时间: {}", description, start_time)
    };

    let _ = app_handle.plugin(tauri_plugin_notification::init());

    // 构建通知
    let notification = app_handle
        .notification()
        .builder()
        .title(&title)
        .body(&body);

    let _ = notification.show();
}

// 发送Android通知的函数
#[tauri::command]
async fn send_notification<R: Runtime>(
    app_handle: AppHandle<R>,
    title: String,
    body: String,
) -> Result<String, String> {
    let _ = app_handle.plugin(tauri_plugin_notification::init());
    let _ = app_handle
        .notification()
        .builder()
        .title(&title)
        .body(&body)
        .show();
    Ok("通知已发送".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 创建数据库连接池
            tauri::async_runtime::block_on(async {
                let app_handle = app.handle();
                // MySQL 数据库连接配置
                let db_url = "mysql://zcalendar_user:zcalendar@103.151.217.252:3306/zcalendar";

                println!("Database URL: {}", db_url);

                // 配置连接池以优化性能
                let pool = sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(10)
                    .min_connections(2)
                    .acquire_timeout(std::time::Duration::from_secs(30))
                    .idle_timeout(std::time::Duration::from_secs(600))
                    .max_lifetime(std::time::Duration::from_secs(1800))
                    .connect(db_url)
                    .await
                    .expect("Failed to connect to database");

                // 注意：不再使用自动迁移，需要在 MySQL 中手动创建表
                // 请执行 src-tauri/migrations/001_create_calendars_and_events_table.sql 中的 SQL 语句

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
                use tauri_plugin_notification::init;
                let notification_plugin = init();

                // 配置Android通知渠道
                #[cfg(target_os = "android")]
                {
                    // 初始化通知插件
                    let _ = app.handle().plugin(notification_plugin);

                    // 等待应用完全初始化后创建通知渠道
                    let _ = app.handle().clone();
                    tauri::async_runtime::spawn(async move {
                        // 等待一点时间确保插件完全加载
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                        // 由于当前Tauri版本可能不直接支持创建通知渠道，
                        // 我们通过发送一个通知来触发必要的权限和配置
                        // let _ = app_handle.plugin(tauri_plugin_notification::init());
                        // let _ = app_handle
                        //     .notification()
                        //     .builder()
                        //     .title("初始化通知")
                        //     .body("通知系统已准备就绪")
                        //     .show();
                    });
                }
                #[cfg(not(target_os = "android"))]
                {
                    app.handle().plugin(notification_plugin);
                }
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
            delete_all_events,
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
            get_calendar_events,
            send_notification,
            get_platform,
            save_file_to_downloads,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
