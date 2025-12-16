export interface CalendarEvent {
  id: number;
  title: string;
  description?: string;
  start: string | Date;  // 后端返回字符串，前端处理为Date
  end: string | Date;
  allDay: boolean;
  reminderMinutes: number;
  created_at: string | Date;
  updated_at: string | Date;
  recurrenceRule?: string;
  recurrenceId?: string;
  sequence: number;
  status: string;
  location?: string;
  organizer?: string;
  attendees?: string;
  url?: string;
  categories?: string;
  priority?: number;
  calendarId?: string;
}

export interface CreateEventDto {
  title: string;
  description?: string;
  start: Date;
  end: Date;
  allDay: boolean;
  reminderMinutes: number;
  recurrenceRule?: string;
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
  allDay?: boolean;
  reminderMinutes?: number;
  recurrenceRule?: string;
  location?: string;
  url?: string;
  categories?: string;
  priority?: number;
  status?: string;
}