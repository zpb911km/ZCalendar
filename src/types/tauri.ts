// Tauri命令类型定义
export interface TauriCommands {
  getAllEvents(): Promise<any[]>;
  getEventsByDate(date: { date: string }): Promise<any[]>;
  getEventById(id: { id: number }): Promise<any | null>;
  createEvent(event: { event: any }): Promise<any>;
  updateEvent(event: { event: any }): Promise<any>;
  deleteEvent(id: { id: number }): Promise<void>;
  importIcal(icalContent: { icalContent: string }): Promise<any[]>;
  exportIcal(eventIds: { eventIds: number[] | null }): Promise<string>;
  getEventsInRange(range: { start: string; end: string }): Promise<any[]>;
  searchEvents(query: { query: string }): Promise<any[]>;
  getUpcomingEvents(limit: { limit: number }): Promise<any[]>;
  getAllCalendars(): Promise<any[]>;
  getCalendarById(id: { id: string }): Promise<any | null>;
  createCalendar(calendar: { calendar: any }): Promise<any>;
  updateCalendar(calendar: { calendar: any }): Promise<any>;
  deleteCalendar(id: { id: string }): Promise<void>;
  getCalendarEvents(calendarId: { calendarId: string }): Promise<any[]>;
}