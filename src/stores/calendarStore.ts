import { defineStore } from 'pinia';
import { ref } from 'vue';
import { Calendar } from '../types/calendar';
import { calendarService } from '../services/calendarService';

export const useCalendarStore = defineStore('calendar', {
  state: () => ({
    calendars: [] as Calendar[],
    currentCalendarId: ref<string | null>(null),
    loading: false,
    error: null as string | null,
  }),

  getters: {
    // 获取当前日历
    currentCalendar: state => {
      if (!state.currentCalendarId) {
        return (
          state.calendars.find(cal => cal.is_primary) || state.calendars[0]
        );
      }
      return state.calendars.find(cal => cal.id === state.currentCalendarId);
    },

    // 获取默认日历
    defaultCalendar: state => {
      return state.calendars.find(cal => cal.is_primary) || state.calendars[0];
    },
  },

  actions: {
    async fetchCalendars() {
      this.loading = true;
      this.error = null;
      try {
        this.calendars = await calendarService.getAllCalendars();
      } catch (error) {
        this.error = error instanceof Error ? error.message : '获取日历失败';
        console.error('获取日历失败:', error);
      } finally {
        this.loading = false;
      }
    },

    async createCalendar(
      calendarData: Omit<Calendar, 'id' | 'createdAt' | 'updatedAt'>
    ) {
      this.loading = true;
      this.error = null;
      try {
        const newCalendar = await calendarService.createCalendar({
          ...calendarData,
          id: '',
        });
        this.calendars.push(newCalendar);
        if (newCalendar.is_primary) {
          this.currentCalendarId = newCalendar.id;
        }
        return newCalendar;
      } catch (error) {
        this.error = error instanceof Error ? error.message : '创建日历失败';
        console.error('创建日历失败:', error);
        throw error;
      } finally {
        this.loading = false;
      }
    },

    setCurrentCalendar(calendarId: string) {
      this.currentCalendarId = calendarId;
    },
  },
});
