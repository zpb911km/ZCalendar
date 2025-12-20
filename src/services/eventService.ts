import { invoke } from '@tauri-apps/api/core';
import { CalendarEvent, CreateEventDto, UpdateEventDto } from '../types/event';

// 内部辅助函数
const convertIdToBackend = (id: number): number => {
  return id;
};

const convertIdToFrontend = (id: number): number => {
  return id;
};

const convertDateToBackend = (date: Date): string => {
  return date.toISOString();
};

const convertDateToFrontend = (dateStr: string): Date => {
  return new Date(dateStr);
};

const convertEventToBackend = (event: any): any => {
  return {
    ...event,
    start:
      event.start instanceof Date
        ? convertDateToBackend(event.start)
        : event.start,
    end:
      event.end instanceof Date ? convertDateToBackend(event.end) : event.end,
    created_at:
      event.created_at instanceof Date
        ? convertDateToBackend(event.created_at)
        : event.created_at,
    updated_at:
      event.updated_at instanceof Date
        ? convertDateToBackend(event.updated_at)
        : event.updated_at,
    id: event.id ? convertIdToBackend(event.id) : undefined,
  };
};

const convertEventFromBackend = (backendEvent: any): CalendarEvent => {
  return {
    ...backendEvent,
    id: convertIdToFrontend(backendEvent.id),
    start:
      typeof backendEvent.start === 'string'
        ? convertDateToFrontend(backendEvent.start)
        : backendEvent.start,
    end:
      typeof backendEvent.end === 'string'
        ? convertDateToFrontend(backendEvent.end)
        : backendEvent.end,
    created_at:
      typeof backendEvent.created_at === 'string'
        ? convertDateToFrontend(backendEvent.created_at)
        : backendEvent.created_at,
    updated_at:
      typeof backendEvent.updated_at === 'string'
        ? convertDateToFrontend(backendEvent.updated_at)
        : backendEvent.updated_at,
  } as CalendarEvent;
};

export interface EventService {
  getAllEvents(): Promise<CalendarEvent[]>;
  getEventsByDate(date: Date): Promise<CalendarEvent[]>;
  getEventById(id: number): Promise<CalendarEvent | null>;
  createEvent(event: CreateEventDto): Promise<CalendarEvent>;
  updateEvent(event: UpdateEventDto): Promise<CalendarEvent>;
  deleteEvent(id: number): Promise<void>;
  getEventsInRange(start: Date, end: Date): Promise<CalendarEvent[]>;
  importIcal(icalContent: string): Promise<CalendarEvent[]>;
  exportIcal(eventIds?: number[]): Promise<string>;
  searchEvents(query: string): Promise<CalendarEvent[]>;
  getUpcomingEvents(limit: number): Promise<CalendarEvent[]>;
}

export class EventServiceImpl implements EventService {
  async getAllEvents(): Promise<CalendarEvent[]> {
    const result = await invoke<any[]>('get_all_events');
    // 将后端返回的数据转换为前端类型
    return result.map(convertEventFromBackend);
  }

  async getEventsByDate(date: Date): Promise<CalendarEvent[]> {
    const result = await invoke<any[]>('get_events_by_date', {
      date: convertDateToBackend(date),
    });
    return result.map(convertEventFromBackend);
  }

  async getEventById(id: number): Promise<CalendarEvent | null> {
    const result = await invoke<any | null>('get_event_by_id', {
      id: convertIdToBackend(id),
    });
    return result ? convertEventFromBackend(result) : null;
  }

  async createEvent(event: CreateEventDto): Promise<CalendarEvent> {
    const backendEvent = convertEventToBackend(event);
    const result = await invoke<any>('create_event', { event: backendEvent });
    return convertEventFromBackend(result);
  }

  async updateEvent(event: UpdateEventDto): Promise<CalendarEvent> {
    const backendEvent = convertEventToBackend(event);
    const result = await invoke<any>('update_event', { event: backendEvent });
    return convertEventFromBackend(result);
  }

  async deleteEvent(id: number): Promise<void> {
    await invoke('delete_event', { id: convertIdToBackend(id) });
  }

  async getEventsInRange(start: Date, end: Date): Promise<CalendarEvent[]> {
    const result = await invoke<any[]>('get_events_in_range', {
      start: convertDateToBackend(start),
      end: convertDateToBackend(end),
    });
    return result.map(convertEventFromBackend);
  }

  async importIcal(icalContent: string): Promise<CalendarEvent[]> {
    const result = await invoke<any[]>('import_ical', { icalContent });
    return result.map(convertEventFromBackend);
  }

  async exportIcal(eventIds?: number[]): Promise<string> {
    return await invoke('export_ical', {
      eventIds: eventIds?.map(id => convertIdToBackend(id)) || null,
    });
  }

  async searchEvents(query: string): Promise<CalendarEvent[]> {
    const result = await invoke<any[]>('search_events', { query });
    return result.map(convertEventFromBackend);
  }

  async getUpcomingEvents(limit: number): Promise<CalendarEvent[]> {
    const result = await invoke<any[]>('get_upcoming_events', { limit });
    return result.map(convertEventFromBackend);
  }
}
