import { ref, computed } from 'vue';
import { CalendarEvent, CreateEventDto, UpdateEventDto } from '../types/event';
import { EventServiceImpl } from '../services/eventService';
import { dateUtils } from '../utils/dateUtils';

const eventService = new EventServiceImpl();

export function useEvents() {
  const events = ref<CalendarEvent[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 加载所有事件
  const loadEvents = async () => {
    loading.value = true;
    error.value = null;
    try {
      events.value = await eventService.getAllEvents();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取事件失败';
      console.error('获取事件失败:', err);
    } finally {
      loading.value = false;
    }
  };

  // 根据日期获取事件
  const loadEventsByDate = async (date: Date) => {
    loading.value = true;
    error.value = null;
    try {
      events.value = await eventService.getEventsByDate(date);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取事件失败';
      console.error('获取日期事件失败:', err);
    } finally {
      loading.value = false;
    }
  };

  // 根据范围获取事件
  const loadEventsInRange = async (start: Date, end: Date) => {
    loading.value = true;
    error.value = null;
    try {
      events.value = await eventService.getEventsInRange(start, end);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取时间范围事件失败';
      console.error('获取时间范围事件失败:', err);
    } finally {
      loading.value = false;
    }
  };

  // 添加事件
  const addEvent = async (eventData: CreateEventDto) => {
    loading.value = true;
    error.value = null;
    try {
      const newEvent = await eventService.createEvent(eventData);
      events.value.push(newEvent);
      return newEvent;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '创建事件失败';
      console.error('创建事件失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 更新事件
  const updateEvent = async (eventData: UpdateEventDto) => {
    loading.value = true;
    error.value = null;
    try {
      const updatedEvent = await eventService.updateEvent(eventData);
      const index = events.value.findIndex(e => e.id === eventData.id);
      if (index !== -1) {
        events.value[index] = updatedEvent;
      }
      return updatedEvent;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '更新事件失败';
      console.error('更新事件失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 删除事件
  const deleteEvent = async (id: number) => {
    loading.value = true;
    error.value = null;
    try {
      await eventService.deleteEvent(id);
      events.value = events.value.filter(event => event.id !== id);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除事件失败';
      console.error('删除事件失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 搜索事件
  const searchEvents = async (query: string) => {
    loading.value = true;
    error.value = null;
    try {
      const searchResults = await eventService.searchEvents(query);
      events.value = searchResults;
      return searchResults;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '搜索事件失败';
      console.error('搜索事件失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 根据日期过滤事件
  const getEventsForDate = (date: Date) => {
    return events.value.filter(event => {
      const eventStart = dateUtils.startOfDay(new Date(event.start));
      const eventEnd = dateUtils.startOfDay(new Date(event.end));
      const checkDate = dateUtils.startOfDay(date);

      // 全天事件
      if (event.all_day) {
        return eventStart <= checkDate && eventEnd >= checkDate;
      }
      // 非全天事件
      else {
        const start = new Date(event.start);
        const end = new Date(event.end);
        return start <= date && end >= date;
      }
    });
  };

  // 获取即将到来的事件
  const upcomingEvents = computed(() => {
    const now = new Date();
    return events.value
      .filter(event => new Date(event.start) >= now)
      .sort(
        (a, b) => new Date(a.start).getTime() - new Date(b.start).getTime()
      );
  });

  // 获取今天的事件
  const todayEvents = computed(() => {
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const tomorrow = new Date(today);
    tomorrow.setDate(tomorrow.getDate() + 1);

    return events.value.filter(event => {
      const eventStart = new Date(event.start);
      return eventStart >= today && eventStart < tomorrow;
    });
  });

  return {
    events: computed(() => events.value),
    loading: computed(() => loading.value),
    error: computed(() => error.value),
    loadEvents,
    loadEventsByDate,
    loadEventsInRange,
    addEvent,
    updateEvent,
    deleteEvent,
    searchEvents,
    getEventsForDate,
    upcomingEvents,
    todayEvents,
  };
}
