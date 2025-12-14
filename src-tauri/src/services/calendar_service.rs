use crate::models::event::{Event, CreateEventDto, UpdateEventDto};
use crate::services::ical_service::IcalService;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalendarServiceError {
    #[error("事件服务错误: {0}")]
    EventService(#[from] crate::services::event_service::EventServiceError),
    #[error("iCalendar服务错误: {0}")]
    IcalService(#[from] crate::services::ical_service::IcalServiceError),
    #[error("导入选项错误: {0}")]
    ImportOptionError(String),
}

pub struct CalendarService {
    event_service: Arc<Mutex<crate::services::event_service::EventService>>,
    ical_service: IcalService,
}

impl CalendarService {
    pub fn new(
        event_service: Arc<Mutex<crate::services::event_service::EventService>>, 
        ical_service: IcalService
    ) -> Self {
        Self {
            event_service,
            ical_service,
        }
    }

    // 事件管理功能
    pub async fn get_all_events(&self) -> Result<Vec<Event>, CalendarServiceError> {
        Ok(self.event_service.lock().await.get_all_events().await?)
    }

    pub async fn get_events_by_date(&self, date: DateTime<Utc>) -> Result<Vec<Event>, CalendarServiceError> {
        Ok(self.event_service.lock().await.get_events_by_date(date).await?)
    }

    pub async fn get_event_by_id(&self, id: i64) -> Result<Option<Event>, CalendarServiceError> {
        Ok(self.event_service.lock().await.get_event_by_id(id).await?)
    }

    pub async fn create_event(&self, dto: CreateEventDto) -> Result<Event, CalendarServiceError> {
        Ok(self.event_service.lock().await.create_event(dto).await?)
    }

    pub async fn update_event(&self, dto: UpdateEventDto) -> Result<Event, CalendarServiceError> {
        Ok(self.event_service.lock().await.update_event(dto).await?)
    }

    pub async fn delete_event(&self, id: i64) -> Result<(), CalendarServiceError> {
        self.event_service.lock().await.delete_event(id).await?;
        Ok(())
    }

    pub async fn get_events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Event>, CalendarServiceError> {
        Ok(self.event_service.lock().await.get_events_in_range(start, end).await?)
    }

    // iCalendar导入导出功能
    pub async fn import_ical(&self, ical_content: String) -> Result<Vec<Event>, CalendarServiceError> {
        // 解析iCalendar内容
        let events_dto = self.ical_service.parse_ical_content(&ical_content)?;
        let mut imported_events = Vec::new();

        // 逐个创建事件
        for dto in events_dto {
            let event = self.event_service.lock().await.create_event(dto).await?;
            imported_events.push(event);
        }

        Ok(imported_events)
    }

    pub async fn export_ical(&self, event_ids: Option<Vec<i64>>) -> Result<String, CalendarServiceError> {
        let events = match event_ids {
            Some(ids) => {
                let mut selected_events = Vec::new();
                for id in ids {
                    if let Some(event) = self.event_service.lock().await.get_event_by_id(id).await? {
                        selected_events.push(event);
                    }
                }
                selected_events
            }
            None => self.event_service.lock().await.get_all_events().await?,
        };

        Ok(self.ical_service.generate_ical_content(&events)?)
    }

    // 高级功能
    pub async fn import_ical_with_options(&self, ical_content: String, options: ImportOptions) -> Result<Vec<Event>, CalendarServiceError> {
        let events_dto = self.ical_service.parse_ical_content(&ical_content)?;
        let mut imported_events = Vec::new();

        for dto in events_dto {
            // 根据选项处理事件
            if options.convert_timezone {
                // 时区转换逻辑
                // 这里可以添加时区转换功能
            }
            
            if options.assign_to_calendar {
                // 分配到指定日历
                // 在实际实现中，可以在CreateEventDto中添加calendar_id
            }
            
            if options.skip_duplicates {
                // 检查是否已存在相同的事件
                // 这里可以基于UID或其他唯一标识符检查重复
            }
            
            let event = self.event_service.lock().await.create_event(dto).await?;
            imported_events.push(event);
        }

        Ok(imported_events)
    }

    pub async fn search_events(&self, query: &str) -> Result<Vec<Event>, CalendarServiceError> {
        let all_events = self.event_service.lock().await.get_all_events().await?;
        
        let filtered_events = all_events.into_iter()
            .filter(|event| {
                event.title.to_lowercase().contains(&query.to_lowercase()) ||
                event.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query.to_lowercase())) ||
                event.location.as_ref().map_or(false, |loc| loc.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect();
        
        Ok(filtered_events)
    }

    pub async fn get_events_by_category(&self, category: &str) -> Result<Vec<Event>, CalendarServiceError> {
        let all_events = self.event_service.lock().await.get_all_events().await?;
        
        let filtered_events = all_events.into_iter()
            .filter(|event| {
                event.categories.as_ref().map_or(false, |cats| {
                    cats.split(',').any(|c| c.trim().to_lowercase() == category.to_lowercase())
                })
            })
            .collect();
        
        Ok(filtered_events)
    }

    pub async fn get_upcoming_events(&self, limit: usize) -> Result<Vec<Event>, CalendarServiceError> {
        let all_events = self.event_service.lock().await.get_all_events().await?;
        let now = Utc::now();
        
        let mut upcoming_events: Vec<Event> = all_events.into_iter()
            .filter(|event| event.start >= now)
            .collect();
        
        // 按开始时间排序
        upcoming_events.sort_by(|a, b| a.start.cmp(&b.start));
        
        // 限制返回数量
        upcoming_events.truncate(limit);
        
        Ok(upcoming_events)
    }
}

#[derive(Debug)]
pub struct ImportOptions {
    pub convert_timezone: bool,
    pub assign_to_calendar: bool,
    pub skip_duplicates: bool,
    pub default_calendar_id: Option<String>,
}