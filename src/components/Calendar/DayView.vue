<template>
  <div class="day-view">
    <!-- 日期头部 -->
    <!-- <div class="day-header">
      <div class="date-info">
        <div class="day-of-week">{{ dayOfWeek }}</div>
        <div class="date-number">{{ dateNumber }}</div>
        <div class="full-date">{{ fullDate }}</div>
      </div>
    </div> -->

    <!-- 时间轴 -->
    <div class="time-axis">
      <div
        v-for="timeSlot in timeSlots"
        :key="timeSlot.time.getTime()"
        class="time-row"
      >
        <!-- 时间标签 -->
        <div class="time-label">
          {{ timeSlot.label }}
        </div>

        <!-- 时间槽 -->
        <div
          class="time-slot"
          :class="{ 'current-time': isCurrentTime(timeSlot.time) }"
          @click="onTimeSlotClick(timeSlot.time)"
        >
          <!-- {{ timeSlot.time.getHours() }}:{{ timeSlot.time.getMinutes() }} -->
          <!-- 事件 -->
          <div
            v-for="event in getEventsForTimeSlot(timeSlot.time)"
            :key="event.id"
            class="event-item"
            :style="getEventStyle(event)"
            @click.stop="onEventClick(event)"
          >
            <div class="event-content">
              <div class="event-title">{{ event.title }}</div>
              <div class="event-time">
                {{ formatTime(ensureDate(event.start)) }} -
                {{ formatTime(ensureDate(event.end)) }}
              </div>
              <div v-if="event.description" class="event-description">
                {{ event.description }}
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

const timeSlots = computed(() => {
  const slots = [];
  for (let hour = 0; hour < 24; hour++) {
    const time = new Date(props.currentDate);
    time.setHours(hour, 0, 0, 0);

    slots.push({
      time,
      label: dateUtils.formatLocal(time, 'HH:mm'),
    });
  }
  return slots;
});

// 方法
const getEventsForTimeSlot = (time: Date): CalendarEvent[] => {
  const timeHour = time.getHours();

  return props.events.filter(event => {
    // 全天事件不显示在时间轴上
    if (event.all_day) return false;

    const eventStart = ensureDate(event.start);
    const eventStartHour = eventStart.getHours();

    // 检查事件是否在当前时间槽内
    return (
      dateUtils.isSameDay(eventStart, props.currentDate) &&
      eventStartHour === timeHour
    );
  });
};

const getEventStyle = (event: CalendarEvent): Record<string, string> => {
  return {
    // top: `${top}px`,
    // height: `${height}px`,
    backgroundColor: getEventBackgroundColor(event),
    color: getEventColor(event),
    // position: 'absolute',
    // left: '4px',
    // right: '4px',
    borderRadius: '4px',
    padding: '4px',
    overflow: 'hidden',
    fontSize: '13px',
    zIndex: '1',
  };
};

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

const formatTime = (date: Date): string => {
  return dateUtils.formatLocal(date, 'HH:mm');
};

const isCurrentTime = (time: Date): boolean => {
  const now = new Date();
  return (
    dateUtils.isSameDay(now, props.currentDate) &&
    now.getHours() === time.getHours()
  );
};

// 事件处理
const onEventClick = (event: CalendarEvent) => {
  emit('eventClick', event);
};

const onTimeSlotClick = (time: Date) => {
  // 合并当前日期和时间创建新的事件开始时间
  const eventStart = new Date(props.currentDate);
  eventStart.setHours(
    time.getHours(),
    time.getMinutes(),
    time.getSeconds(),
    time.getMilliseconds()
  );

  emit('dateClick', eventStart);
};
</script>

<style scoped>
.day-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: auto;
  background-color: var(--background-color);
}

.day-header {
  flex-shrink: 0;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--background-color);
}

.date-info {
  text-align: center;
}

.day-of-week {
  font-size: 18px;
  font-weight: 500;
  color: var(--text-color);
}

.date-number {
  font-size: 32px;
  font-weight: 600;
  margin: 8px 0;
  color: var(--text-color);
}

.full-date {
  font-size: 14px;
  color: var(--text-secondary-color);
}

.time-axis {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  position: relative;
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
  position: sticky;
  left: 0;
  z-index: 2;
}

.time-slot {
  display: flex;
  flex: 1;
  position: relative;
  cursor: pointer;
}

.time-slot:hover {
  background-color: var(--secondary-light);
}

.time-slot.current-time {
  background-color: var(--primary-color);
}

.event-item {
  color: var(--event-text-color);
  box-sizing: border-box;
  cursor: pointer;
  transition: transform 0.2s ease;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.event-item:hover {
  transform: scale(1.02);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.event-content {
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 4px;
}

.event-title {
  font-weight: 500;
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.event-time {
  font-size: 11px;
  opacity: 0.9;
  margin-top: 2px;
}

.event-description {
  font-size: 10px;
  opacity: 0.7;
  margin-top: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 为当前时间添加指示器 */
.time-slot.current-time::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  height: 2px;
  background-color: var(--primary-color);
  z-index: 0;
}
</style>
