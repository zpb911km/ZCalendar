import { defineStore } from 'pinia';
import { CalendarEvent, CreateEventDto, UpdateEventDto } from '../types/event';
import { EventServiceImpl } from '../services/eventService';

const eventService = new EventServiceImpl();

export const useEventStore = defineStore('event', {
  state: () => ({
    events: [] as CalendarEvent[],
    loading: false,
    error: null as string | null,
  }),

  getters: {
    // 按日期分组的事件
    eventsByDate: (state) => {
      const grouped = new Map<string, CalendarEvent[]>();
      state.events.forEach(event => {
        const dateKey = new Date(event.start).toDateString();
        if (!grouped.has(dateKey)) {
          grouped.set(dateKey, []);
        }
        grouped.get(dateKey)!.push(event);
      });
      return grouped;
    },
    
    // 未来的事件
    upcomingEvents: (state) => {
      const now = new Date();
      return state.events
        .filter(event => new Date(event.start) > now)
        .sort((a, b) => new Date(a.start).getTime() - new Date(b.start).getTime());
    },
    
    // 今天的事件
    todayEvents: (state) => {
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      const tomorrow = new Date(today);
      tomorrow.setDate(tomorrow.getDate() + 1);
      
      return state.events.filter(event => {
        const eventStart = new Date(event.start);
        return eventStart >= today && eventStart < tomorrow;
      });
    },
    
    // 根据ID获取事件
    getEventById: (state) => {
      return (id: number) => state.events.find(event => event.id === id);
    }
  },

  actions: {
    async fetchAllEvents() {
      this.loading = true;
      this.error = null;
      try {
        this.events = await eventService.getAllEvents();
      } catch (error) {
        this.error = error instanceof Error ? error.message : '获取事件失败';
        console.error('获取事件失败:', error);
      } finally {
        this.loading = false;
      }
    },

    async fetchEventsByDate(date: Date) {
      this.loading = true;
      this.error = null;
      try {
        this.events = await eventService.getEventsByDate(date);
      } catch (error) {
        this.error = error instanceof Error ? error.message : '获取日期事件失败';
        console.error('获取日期事件失败:', error);
      } finally {
        this.loading = false;
      }
    },

    async createEvent(eventData: CreateEventDto) {
      this.loading = true;
      this.error = null;
      try {
        const newEvent = await eventService.createEvent(eventData);
        this.events.push(newEvent);
        return newEvent;
      } catch (error) {
        this.error = error instanceof Error ? error.message : '创建事件失败';
        console.error('创建事件失败:', error);
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async updateEvent(eventData: UpdateEventDto) {
      this.loading = true;
      this.error = null;
      try {
        const updatedEvent = await eventService.updateEvent(eventData);
        const index = this.events.findIndex(e => e.id === eventData.id);
        if (index !== -1) {
          this.events[index] = updatedEvent;
        }
        return updatedEvent;
      } catch (error) {
        this.error = error instanceof Error ? error.message : '更新事件失败';
        console.error('更新事件失败:', error);
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async deleteEvent(id: number) {
      this.loading = true;
      this.error = null;
      try {
        await eventService.deleteEvent(id);
        this.events = this.events.filter(event => event.id !== id);
      } catch (error) {
        this.error = error instanceof Error ? error.message : '删除事件失败';
        console.error('删除事件失败:', error);
        throw error;
      } finally {
        this.loading = false;
      }
    },
    
    async searchEvents(query: string) {
      this.loading = true;
      this.error = null;
      try {
        const results = await eventService.searchEvents(query);
        this.events = results;
        return results;
      } catch (error) {
        this.error = error instanceof Error ? error.message : '搜索事件失败';
        console.error('搜索事件失败:', error);
        throw error;
      } finally {
        this.loading = false;
      }
    }
  },
});