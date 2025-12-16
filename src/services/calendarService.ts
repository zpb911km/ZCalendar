import { invoke } from '@tauri-apps/api/core';
import { Calendar } from '../types/calendar';

// 内部辅助函数
const convertCalendarToBackend = (calendar: any): any => {
  return {
    ...calendar,
    createdAt: calendar.createdAt instanceof Date ? calendar.createdAt.toISOString() : calendar.createdAt,
    updatedAt: calendar.updatedAt instanceof Date ? calendar.updatedAt.toISOString() : calendar.updatedAt,
  };
};

const convertCalendarFromBackend = (backendCalendar: any): Calendar => {
  return {
    ...backendCalendar,
    createdAt: typeof backendCalendar.createdAt === 'string' ? new Date(backendCalendar.createdAt) : backendCalendar.createdAt,
    updatedAt: typeof backendCalendar.updatedAt === 'string' ? new Date(backendCalendar.updatedAt) : backendCalendar.updatedAt,
  } as Calendar;
};

export const calendarService = {
  // 获取所有日历
  async getAllCalendars(): Promise<Calendar[]> {
    const result = await invoke<any[]>('get_all_calendars');
    return result.map(convertCalendarFromBackend);
  },

  // 创建日历
  async createCalendar(calendar: Omit<Calendar, 'id' | 'createdAt' | 'updatedAt'>): Promise<Calendar> {
    // 确保isPrimary有默认值
    const calendarWithDefaults = {
      ...calendar,
      isPrimary: calendar.isPrimary ?? false
    };
    const backendCalendar = convertCalendarToBackend(calendarWithDefaults);
    const result = await invoke<any>('create_calendar', { calendar: backendCalendar });
    return convertCalendarFromBackend(result);
  },
  
  // 获取日历详情
  async getCalendarById(id: string): Promise<Calendar | null> {
    const result = await invoke<any | null>('get_calendar_by_id', { id });
    return result ? convertCalendarFromBackend(result) : null;
  },
  
  // 更新日历
  async updateCalendar(calendar: Calendar): Promise<Calendar> {
    const backendCalendar = convertCalendarToBackend(calendar);
    const result = await invoke<any>('update_calendar', { calendar: backendCalendar });
    return convertCalendarFromBackend(result);
  },
  
  // 删除日历
  async deleteCalendar(id: string): Promise<void> {
    await invoke('delete_calendar', { id });
  },
  
  // 获取日历事件
  async getCalendarEvents(calendarId: string): Promise<any[]> {
    return await invoke('get_calendar_events', { calendarId });
  }
};