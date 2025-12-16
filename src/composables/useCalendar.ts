import { ref, computed } from 'vue';
import { dateUtils } from '../utils/dateUtils';
import { Calendar, CalendarViewType } from '../types/calendar';
import { calendarService } from '../services/calendarService';

export function useCalendar(initialDate: Date = new Date()) {
  const currentDate = ref<Date>(new Date(initialDate));
  const currentView = ref<'month' | 'week' | 'day'>('month');
  const currentCalendar = ref<Calendar | null>(null);
  const calendars = ref<Calendar[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 月视图相关计算
  const monthDates = computed(() => {
    return dateUtils.getMonthDates(currentDate.value);
  });

  // 周视图相关计算
  const weekDates = computed(() => {
    return dateUtils.getWeekDates(currentDate.value);
  });

  // 视图类型
  const viewTypes = ref<CalendarViewType[]>([
    { value: 'month', label: '月' },
    { value: 'week', label: '周' },
    { value: 'day', label: '日' }
  ]);

  // 当前视图的标题
  const currentViewTitle = computed(() => {
    switch (currentView.value) {
      case 'month':
        return dateUtils.getMonthName(currentDate.value);
      case 'week':
        const startDate = weekDates.value[0];
        const endDate = weekDates.value[6];
        return `${dateUtils.formatLocal(startDate, 'yyyy年MM月dd日')} - ${dateUtils.formatLocal(endDate, 'MM月dd日')}`;
      case 'day':
        return dateUtils.formatLocal(currentDate.value, 'yyyy年MM月dd日');
      default:
        return dateUtils.getMonthName(currentDate.value);
    }
  });

  // 日期导航
  const nextPeriod = () => {
    switch (currentView.value) {
      case 'month':
        currentDate.value = dateUtils.addMonths(currentDate.value, 1);
        break;
      case 'week':
        currentDate.value = dateUtils.addWeeks(currentDate.value, 1);
        break;
      case 'day':
        currentDate.value = dateUtils.addDays(currentDate.value, 1);
        break;
    }
  };

  const prevPeriod = () => {
    switch (currentView.value) {
      case 'month':
        currentDate.value = dateUtils.subMonths(currentDate.value, 1);
        break;
      case 'week':
        currentDate.value = dateUtils.subWeeks(currentDate.value, 1);
        break;
      case 'day':
        currentDate.value = dateUtils.addDays(currentDate.value, -1);
        break;
    }
  };

  const goToToday = () => {
    currentDate.value = new Date();
  };

  const changeView = (view: 'month' | 'week' | 'day') => {
    currentView.value = view;
  };

  // 获取指定日期的周
  const getWeekForDate = (date: Date) => {
    return dateUtils.getWeekDates(date);
  };

  // 获取指定日期的月
  const getMonthForDate = (date: Date) => {
    return dateUtils.getMonthDates(date);
  };

  // 日历管理功能
  const loadCalendars = async () => {
    loading.value = true;
    error.value = null;
    try {
      calendars.value = await calendarService.getAllCalendars();
      
      // 设置默认日历
      if (calendars.value.length > 0 && !currentCalendar.value) {
        const primaryCal = calendars.value.find(cal => cal.isPrimary);
        currentCalendar.value = primaryCal || calendars.value[0];
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取日历失败';
      console.error('获取日历失败:', err);
    } finally {
      loading.value = false;
    }
  };

  const selectCalendar = (calendar: Calendar) => {
    currentCalendar.value = calendar;
  };

  const createCalendar = async (name: string, color: string) => {
    loading.value = true;
    error.value = null;
    try {
      const newCalendar = await calendarService.createCalendar({ 
        name, 
        color,
        isPrimary: false 
      });
      calendars.value.push(newCalendar);
      
      // 如果这是第一个日历，选择它
      if (calendars.value.length === 1) {
        currentCalendar.value = newCalendar;
      }
      
      return newCalendar;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '创建日历失败';
      console.error('创建日历失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const updateCalendar = async (calendar: Calendar) => {
    loading.value = true;
    error.value = null;
    try {
      const updatedCalendar = await calendarService.updateCalendar(calendar);
      const index = calendars.value.findIndex(cal => cal.id === calendar.id);
      if (index !== -1) {
        calendars.value[index] = updatedCalendar;
        
        // 如果更新的是当前选中的日历，也更新当前日历
        if (currentCalendar.value?.id === calendar.id) {
          currentCalendar.value = updatedCalendar;
        }
      }
      return updatedCalendar;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '更新日历失败';
      console.error('更新日历失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  const deleteCalendar = async (id: string) => {
    loading.value = true;
    error.value = null;
    try {
      await calendarService.deleteCalendar(id);
      calendars.value = calendars.value.filter(cal => cal.id !== id);
      
      // 如果删除的是当前选中的日历，选择第一个日历
      if (currentCalendar.value?.id === id) {
        currentCalendar.value = calendars.value.length > 0 ? calendars.value[0] : null;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除日历失败';
      console.error('删除日历失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  return {
    currentDate: computed(() => currentDate.value),
    currentView: computed(() => currentView.value),
    currentCalendar: computed(() => currentCalendar.value),
    calendars: computed(() => calendars.value),
    loading: computed(() => loading.value),
    error: computed(() => error.value),
    monthDates,
    weekDates,
    viewTypes: computed(() => viewTypes.value),
    currentViewTitle,
    nextPeriod,
    prevPeriod,
    goToToday,
    changeView,
    getWeekForDate,
    getMonthForDate,
    // 日历管理方法
    loadCalendars,
    selectCalendar,
    createCalendar,
    updateCalendar,
    deleteCalendar
  };
}