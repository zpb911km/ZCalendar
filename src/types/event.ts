export interface CalendarEvent {
  id: number;
  title: string;
  description?: string;
  start: string | Date;  // 后端返回字符串，前端处理为Date
  end: string | Date;
  all_day: boolean;
  reminder_minutes: number;
  created_at: string | Date;
  updated_at: string | Date;
  recurrence_rule?: string;
  recurrence_id?: string;
  sequence: number;
  status: string;
  location?: string;
  organizer?: string;
  attendees?: string;
  url?: string;
  categories?: string;
  priority?: number;
  calendar_id?: string;
}

export interface CreateEventDto {
  title: string;
  description?: string;
  start: Date;
  end: Date;
  all_day: boolean;
  reminder_minutes: number;
  recurrence_rule?: string;
  location?: string;
  url?: string;
  categories?: string;
  priority?: number;
  status?: string;
}

export interface UpdateEventDto {
  id: number;
  title?: string;
  description?: string;
  start?: Date;
  end?: Date;
  all_day?: boolean;
  reminder_minutes?: number;
  recurrence_rule?: string;
  location?: string;
  url?: string;
  categories?: string;
  priority?: number;
  status?: string;
}