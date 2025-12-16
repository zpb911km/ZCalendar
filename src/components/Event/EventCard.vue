<template>
  <div 
    class="event-card"
    :class="{ 'all-day': event.allDay }"
    :style="{ backgroundColor: getEventColor }"
    @click="onEventClick"
  >
    <div class="event-content">
      <div class="event-title">{{ event.title }}</div>
      <div v-if="!event.allDay" class="event-time">
        {{ formatTime(ensureDate(event.start)) }} - {{ formatTime(ensureDate(event.end)) }}
      </div>
      <div v-if="event.location" class="event-location">
        📍 {{ event.location }}
      </div>
      <div v-if="event.description" class="event-description">
        {{ event.description }}
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

// 格式化时间
const formatTime = (date: Date): string => {
  return dateUtils.formatLocal(date, 'HH:mm');
};

// 定义props和emits
const props = defineProps<{
  event: CalendarEvent;
}>();

const emit = defineEmits<{
  (e: 'click', event: CalendarEvent): void;
}>();

// 计算属性
const getEventColor = computed(() => {
  // 根据事件类型返回不同的颜色
  if (props.event.categories?.includes('工作')) {
    return 'var(--primary-color)';
  } else if (props.event.categories?.includes('个人')) {
    return 'var(--success-color)';
  } else if (props.event.categories?.includes('重要')) {
    return 'var(--danger-color)';
  }
  return 'var(--secondary-color)';
});

const onEventClick = () => {
  emit('click', props.event);
};
</script>

<style scoped>
.event-card {
  border-radius: 6px;
  padding: 6px 8px;
  margin: 2px 0;
  color: var(--event-text-color);
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  border: 1px solid rgba(255, 255, 255, 0.2);
  overflow: hidden;
  display: flex;
  align-items: center;
}

.event-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
}

.event-content {
  flex: 1;
  min-width: 0;
}

.event-title {
  font-weight: 500;
  font-size: 13px;
  margin-bottom: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.event-time {
  font-size: 11px;
  opacity: 0.9;
  margin-bottom: 2px;
}

.event-location {
  font-size: 10px;
  opacity: 0.8;
  margin-bottom: 2px;
}

.event-description {
  font-size: 10px;
  opacity: 0.7;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.all-day {
  border-left: 3px solid var(--primary-color);
}
</style>