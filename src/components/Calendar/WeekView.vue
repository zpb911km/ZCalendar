<template>
  <div class="week-view">
    <!-- 时间轴头部 -->
    <div class="time-header">
      <div class="time-slot-cell"></div>
      <div
        v-for="day in weekDays"
        :key="day.date.toString()"
        class="date-header-cell"
        :class="{ today: isToday(day.date) }"
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
          <!-- <div
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
          </div> -->
          <div
            v-if="getEventsForTimeSlot(day.date, timeSlot.time).length > 0"
            class="event-item"
            :style="
              getEventStyle(
                getEventsForTimeSlot(day.date, timeSlot.time)[0],
                day.date
              )
            "
            @click.stop="
              onEventClick(getEventsForTimeSlot(day.date, timeSlot.time)[0])
            "
          >
            <div class="event-content">
              <div class="event-title">
                {{ getEventsForTimeSlot(day.date, timeSlot.time)[0].title }}
              </div>
              <div
                v-if="!getEventsForTimeSlot(day.date, timeSlot.time)[0].all_day"
                class="event-time"
              >
                {{
                  formatTime(
                    ensureDate(
                      getEventsForTimeSlot(day.date, timeSlot.time)[0].start
                    )
                  )
                }}
                -
                {{
                  formatTime(
                    ensureDate(
                      getEventsForTimeSlot(day.date, timeSlot.time)[0].end
                    )
                  )
                }}
              </div>
            </div>
          </div>
          <div
            v-if="getEventsForTimeSlot(day.date, timeSlot.time).length > 1"
            class="more-events"
          >
            +
            {{ getEventsForTimeSlot(day.date, timeSlot.time).length - 1 }} more
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
import { generateHslColorB } from '@/utils/themeManager';

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
    dayOfWeek: dateUtils.getWeekdayName(date),
  }));
});

const timeSlots = computed(() => {
  const slots = [];
  for (let hour = 0; hour < 24; hour++) {
    const time = new Date();
    time.setHours(hour, 0, 0, 0);

    slots.push({
      time,
      label: dateUtils.formatLocal(time, 'HH:mm'),
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

const getEventStyle = (
  event: CalendarEvent,
  _date: Date
): Record<string, string> => {
  return {
    backgroundColor: getEventBackgroundColor(event),
    color: getEventColor(event),
    // height: "1rem",
    // width: "1rem"
    borderRadius: '4px',
    padding: '4px',
    overflow: 'hidden',
    fontSize: '11px',
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
    return generateHslColorB(event.title);
  }
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

.more-events {
  color: var(--text-secondary-color);
  font-size: 8px;
  cursor: pointer;
  padding: 2px 4px;
  border-radius: 3px;
  background-color: var(--secondary-light);
}
</style>
