<template>
  <div class="week-view">
    <!-- 时间轴头部 -->
    <div class="time-header">
      <div class="time-slot-cell"></div>
      <div 
        v-for="day in weekDays" 
        :key="day.date.toString()"
        class="date-header-cell"
        :class="{ 'today': isToday(day.date) }"
      >
        <div class="day-of-week">{{ day.dayOfWeek }}</div>
        <div class="date-number">{{ day.date.getDate() }}</div>
      </div>
    </div>

    <!-- 时间轴内容 -->
    <div class="time-grid">
      <div 
        v-for="timeSlot in timeSlots" 
        :key="timeSlot.time.getTime()"
        class="time-row"
      >
        <!-- 时间标签 -->
        <div class="time-label">
          {{ timeSlot.label }}
        </div>

        <!-- 每日时间槽 -->
        <div 
          v-for="day in weekDays" 
          :key="`${timeSlot.time}-${day.date.toString()}`"
          class="time-slot"
          @click="onTimeSlotClick(day.date, timeSlot.time)"
        >
          <!-- 事件 -->
          <div
            v-for="event in getEventsForTimeSlot(day.date, timeSlot.time)"
            :key="event.id"
            class="event-item"
            :style="getEventStyle(event, day.date)"
            @click.stop="onEventClick(event)"
          >
            <div class="event-content">
              <div class="event-title">{{ event.title }}</div>
              <div v-if="!event.all_day" class="event-time">
                {{ formatTime(ensureDate(event.start)) }} - {{ formatTime(ensureDate(event.end)) }}
              </div>
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
const weekDays = computed(() => {
  const dates = dateUtils.getWeekDates(props.currentDate);
  return dates.map(date => ({
    date,
    dayOfWeek: dateUtils.getWeekdayName(date).substring(0, 1) // 只取第一个字符，如"一"
  }));
});

const timeSlots = computed(() => {
  const slots = [];
  for (let hour = 0; hour < 24; hour++) {
    const time = new Date();
    time.setHours(hour, 0, 0, 0);
    
    slots.push({
      time,
      label: dateUtils.formatLocal(time, 'HH:mm')
    });
  }
  return slots;
});

const today = computed(() => dateUtils.startOfDay(new Date()));

// 方法
const isToday = (date: Date): boolean => {
  return dateUtils.isSameDay(date, today.value);
};

const getEventsForTimeSlot = (_date: Date, time: Date): CalendarEvent[] => {
  const timeHour = time.getHours();
  
  return props.events.filter(event => {
    // 全天事件不显示在时间轴上
    if (event.all_day) return false;
    
    const eventStart = ensureDate(event.start);
    const eventStartHour = eventStart.getHours();
    
    // 检查事件是否在当前时间槽内
    return (
      dateUtils.isSameDay(eventStart, dateUtils.startOfDay(_date)) &&
      eventStartHour === timeHour
    );
  });
};

const getEventStyle = (event: CalendarEvent, _date: Date): Record<string, string> => {
  const eventStart = ensureDate(event.start);
  const eventEnd = ensureDate(event.end);
  
  // 计算事件在时间槽中的位置和高度
  const startHour = eventStart.getHours();
  const startMinute = eventStart.getMinutes();
  const endHour = eventEnd.getHours();
  const endMinute = eventEnd.getMinutes();
  
  // 计算事件高度（基于持续时间）
  const durationInHours = (endHour + endMinute / 60) - (startHour + startMinute / 60);
  const height = Math.max(30, durationInHours * 60); // 最小高度30px，每小时60px
  
  // 计算top位置
  const top = (startHour + startMinute / 60) * 60; // 每小时60px
  
  return {
    top: `${top}px`,
    height: `${height}px`,
    backgroundColor: getEventColor(event),
    position: 'absolute',
    left: '2px',
    right: '2px',
    borderRadius: '4px',
    padding: '4px',
    overflow: 'hidden',
    fontSize: '12px',
    zIndex: '1'
  };
};

const getEventColor = (event: CalendarEvent): string => {
  // 根据事件类型返回不同的颜色
  if (event.categories?.includes('工作')) {
    return 'var(--primary-color)'; // 蓝色
  } else if (event.categories?.includes('个人')) {
    return 'var(--success-color)'; // 绿色
  } else if (event.categories?.includes('重要')) {
    return 'var(--danger-color)'; // 红色
  }
  return 'var(--secondary-color)'; // 灰色
};

const formatTime = (date: Date): string => {
  return dateUtils.formatLocal(date, 'HH:mm');
};

// 事件处理
const onEventClick = (event: CalendarEvent) => {
  emit('eventClick', event);
};

const onTimeSlotClick = (date: Date, time: Date) => {
  // 合并日期和时间创建新的事件开始时间
  const eventStart = new Date(date);
  eventStart.setHours(time.getHours(), time.getMinutes(), time.getSeconds(), time.getMilliseconds());
  
  emit('dateClick', eventStart);
};
</script>

<style scoped>
.week-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: auto;
}

.time-header {
  display: flex;
  flex-shrink: 0;
  height: 60px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--input-background-color);
}

.time-slot-cell {
  width: 60px;
  flex-shrink: 0;
}

.date-header-cell {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4px;
  text-align: center;
  border-right: 1px solid var(--border-color);
  font-size: 14px;
}

.date-header-cell.today {
  background-color: var(--primary-color);
  font-weight: 600;
}

.day-of-week {
  font-size: 12px;
  color: var(--text-color);
}

.date-number {
  font-size: 16px;
  font-weight: 500;
  margin-top: 2px;
}

.time-grid {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow-y: auto;
}

.time-row {
  display: flex;
  min-height: 60px; /* 每小时60px */
  border-bottom: 1px solid var(--border-color);
  position: relative;
}

.time-label {
  width: 60px;
  flex-shrink: 0;
  display: flex;
  align-items: flex-start;
  justify-content: flex-end;
  padding-right: 8px;
  font-size: 12px;
  color: var(--text-secondary-color);
  background-color: var(--input-background-color);
  border-right: 1px solid var(--border-color);
}

.time-slot {
  flex: 1;
  position: relative;
  border-right: 1px solid var(--border-color);
  cursor: pointer;
}

.time-slot:hover {
  background-color: var(--secondary-light);
}

.event-item {
  color: var(--event-text-color);
  box-sizing: border-box;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.event-item:hover {
  transform: scale(1.02);
}

.event-content {
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 2px;
}

.event-title {
  font-weight: 500;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.event-time {
  font-size: 10px;
  opacity: 0.8;
  margin-top: 2px;
}
</style>