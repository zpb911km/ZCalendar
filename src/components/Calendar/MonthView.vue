<template>
  <div class="month-view">
    <div class="weekdays-header">
      <div v-for="day in weekdays" :key="day" class="weekday-cell">
        {{ day }}
      </div>
    </div>

    <div class="weeks-container">
      <div v-for="(week, weekIndex) in weeks" :key="weekIndex" class="week-row">
        <div
          v-for="(day, dayIndex) in week"
          :key="`${weekIndex}-${dayIndex}`"
          class="day-cell"
          :class="{
            'current-month': isCurrentMonth(day),
            today: isToday(day),
            'other-month': !isCurrentMonth(day),
          }"
          @click="onDateClick(day)"
        >
          <div class="day-number">
            {{ day.getDate() }}
          </div>

          <div class="day-events">
            <div
              v-for="event in getEventsForDay(day)"
              :key="event.id"
              class="event-item"
              :style="{
                color: getEventColor(event),
                backgroundColor: getEventBackgroundColor(event),
              }"
              @click.stop="onEventClick(event)"
            >
              <span class="event-title">{{ event.title }}</span>
              <span v-if="!event.all_day" class="event-time">{{
                formatTime(ensureDate(event.start))
              }}</span>
            </div>

            <div
              v-if="hasMoreEvents(day)"
              class="more-events"
              @click.stop="showMoreEvents(day)"
            >
              +{{ getMoreEventsCount(day) }}更多
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { dateUtils } from '@/utils/dateUtils';
import { CalendarEvent } from '@/types/event';

// 确保日期字段是Date对象
const ensureDate = (date: string | Date): Date => {
  if (typeof date === 'string') {
    return new Date(date);
  }
  return date;
};

// 定义props和emits
const props = defineProps<{
  events: CalendarEvent[];
  currentDate: Date;
}>();

const emit = defineEmits<{
  (e: 'eventClick', event: CalendarEvent): void;
  (e: 'dateClick', date: Date): void;
}>();

// 计算属性
const weekdays = computed(() => ['日', '一', '二', '三', '四', '五', '六']);

const weeks = computed(() => {
  const monthDates = dateUtils.getMonthDates(props.currentDate);
  const weeksArray: Date[][] = [];

  for (let i = 0; i < monthDates.length; i += 7) {
    weeksArray.push(monthDates.slice(i, i + 7));
  }

  return weeksArray;
});

const getEventColor = (event: CalendarEvent) => {
  if (event.categories?.includes('个人')) {
    return 'var(--event-personal-color)';
  } else if (event.categories?.includes('工作')) {
    return 'var(--event-work-color)';
  } else if (event.categories?.includes('重要')) {
    return 'var(--event-important-color)';
  } else if (event.categories?.includes('会议')) {
    return 'var(--event-meeting-color)';
  } else {
    return 'var(--event-default-color)';
  }
};

const getEventBackgroundColor = (event: CalendarEvent) => {
  if (event.categories?.includes('个人')) {
    return 'var(--event-personal-background-color)';
  } else if (event.categories?.includes('工作')) {
    return 'var(--event-work-background-color)';
  } else if (event.categories?.includes('重要')) {
    return 'var(--event-important-background-color)';
  } else if (event.categories?.includes('会议')) {
    return 'var(--event-meeting-background-color)';
  } else {
    return 'var(--event-default-background-color)';
  }
};

const currentMonth = computed(() => props.currentDate.getMonth());
const today = computed(() => dateUtils.startOfDay(new Date()));

// 方法
const isCurrentMonth = (date: Date): boolean => {
  return date.getMonth() === currentMonth.value;
};

const isToday = (date: Date): boolean => {
  return dateUtils.isSameDay(date, today.value);
};

const getEventsForDay = (date: Date): CalendarEvent[] => {
  const startOfDay = dateUtils.startOfDay(date);
  const endOfDay = new Date(startOfDay);
  endOfDay.setDate(endOfDay.getDate() + 1);

  return props.events
    .filter(event => {
      const eventStart = ensureDate(event.start);

      // 全天事件
      if (event.all_day) {
        const eventStartDate = dateUtils.startOfDay(eventStart);
        return dateUtils.isSameDay(eventStartDate, startOfDay);
      }
      // 非全天事件
      else {
        return eventStart >= startOfDay && eventStart < endOfDay;
      }
    })
    .slice(0, 3); // 只显示前3个事件，多余的用"更多"显示
};

const hasMoreEvents = (date: Date): boolean => {
  const startOfDay = dateUtils.startOfDay(date);
  const endOfDay = new Date(startOfDay);
  endOfDay.setDate(endOfDay.getDate() + 1);

  const allDayEvents = props.events.filter(event => {
    if (event.all_day) {
      const eventStartDate = dateUtils.startOfDay(ensureDate(event.start));
      return dateUtils.isSameDay(eventStartDate, startOfDay);
    }
    return false;
  });

  const timedEvents = props.events.filter(event => {
    if (!event.all_day) {
      const eventStart = ensureDate(event.start);
      return eventStart >= startOfDay && eventStart < endOfDay;
    }
    return false;
  });

  const totalEvents = allDayEvents.length + timedEvents.length;
  return totalEvents > 3;
};

const getMoreEventsCount = (date: Date): number => {
  const startOfDay = dateUtils.startOfDay(date);
  const endOfDay = new Date(startOfDay);
  endOfDay.setDate(endOfDay.getDate() + 1);

  const allDayEvents = props.events.filter(event => {
    if (event.all_day) {
      const eventStartDate = dateUtils.startOfDay(ensureDate(event.start));
      return dateUtils.isSameDay(eventStartDate, startOfDay);
    }
    return false;
  });

  const timedEvents = props.events.filter(event => {
    if (!event.all_day) {
      const eventStart = ensureDate(event.start);
      return eventStart >= startOfDay && eventStart < endOfDay;
    }
    return false;
  });

  const totalEvents = allDayEvents.length + timedEvents.length;
  return Math.max(0, totalEvents - 3);
};
const formatTime = (date: Date): string => {
  return dateUtils.formatLocal(date, 'HH:mm');
};

// 事件处理
const onEventClick = (event: CalendarEvent) => {
  emit('eventClick', event);
};

const onDateClick = (date: Date) => {
  emit('dateClick', date);
};

const showMoreEvents = (date: Date) => {
  // 这里可以弹出一个显示该日所有事件的面板
  console.log('显示更多事件', date);
};
</script>

<style scoped>
.month-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.weekdays-header {
  display: flex;
  height: 40px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--input-background-color);
}

.weekday-cell {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 500;
  color: var(--text-color);
  font-size: 14px;
}

.weeks-container {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.week-row {
  display: flex;
  flex: 1;
}

.day-cell {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border-color);
  border-bottom: 1px solid var(--border-color);
  padding: 4px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  /* min-height: 120px; */
  position: relative;
}

.day-cell:hover {
  background-color: var(--secondary-light);
}

.day-cell.current-month {
  background-color: var(--input-background-color);
}

.day-cell.today {
  background-color: var(--primary-color);
  position: relative;
}

.day-cell.today::before {
  content: '';
  position: absolute;
  top: 4px;
  right: 4px;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: var(--primary-color);
}

.day-cell.other-month {
  background-color: var(--secondary-color);
  color: var(--text-secondary-color);
}

.day-number {
  align-self: flex-end;
  font-size: 14px;
  font-weight: 500;
  padding: 2px 4px;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.day-cell.today .day-number {
  background-color: var(--primary-color);
  color: white;
}

.day-events {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow-y: auto;
}

.event-item {
  display: flex;
  flex-direction: column;
  padding: 2px 4px;
  border-radius: 3px;
  color: var(--event-text-color);
  font-size: 12px;
  cursor: pointer;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.event-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.event-time {
  font-size: 10px;
  opacity: 0.8;
}

.more-events {
  color: var(--text-secondary-color);
  font-size: 12px;
  cursor: pointer;
  padding: 2px 4px;
  border-radius: 3px;
  background-color: var(--secondary-light);
}

.more-events:hover {
  background-color: var(--secondary-color);
}
</style>
