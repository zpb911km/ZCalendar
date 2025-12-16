<template>
  <div class="calendar-container">
    <!-- 日历导航栏 -->
    <div class="calendar-header">
      <CalendarNavigation 
        v-model:currentDate="currentDate"
        v-model:currentView="currentView"
        @today="goToToday"
        @next="nextPeriod"
        @prev="prevPeriod"
      />
    </div>
    
    <!-- 日历视图内容 -->
    <div class="calendar-content">
      <MonthView 
        v-if="currentView === 'month'"
        :events="events"
        :current-date="currentDate"
        @event-click="onEventClick"
        @date-click="onDateClick"
      />
      <WeekView 
        v-else-if="currentView === 'week'"
        :events="events"
        :current-date="currentDate"
        @event-click="onEventClick"
      />
      <DayView 
        v-else-if="currentView === 'day'"
        :events="events"
        :current-date="currentDate"
        @event-click="onEventClick"
      />
    </div>
    
    <!-- 事件详情弹窗 -->
    <teleport to="body">
      <EventDetail 
        v-if="selectedEvent"
        :event="selectedEvent"
        @close="selectedEvent = null"
        @edit="onEditEvent"
        @delete="onDeleteEvent"
      />
    </teleport>
    
    <!-- 事件编辑弹窗 -->
    <teleport to="body">
      <EventEditor 
        v-if="showEventEditor"
        :event="editingEvent || undefined"
        @save="onSaveEvent"
        @close="showEventEditor = false"
      />
    </teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useEventStore } from '@/stores/eventStore';
import { useCalendar } from '@/composables/useCalendar';
import CalendarNavigation from './CalendarNavigation.vue';
import MonthView from './MonthView.vue';
import WeekView from './WeekView.vue';
import DayView from './DayView.vue';
import EventDetail from '../Event/EventDetail.vue';
import EventEditor from '../Event/EventEditor.vue';
import { CalendarEvent } from '@/types/event';

// 使用组合式函数
const { 
  currentDate, 
  currentView, 
  nextPeriod, 
  prevPeriod, 
  goToToday 
} = useCalendar();

// 使用状态管理
const eventStore = useEventStore();

// 本地状态
const selectedEvent = ref<CalendarEvent | null>(null);
const showEventEditor = ref(false);
const editingEvent = ref<CalendarEvent | null>(null);

// 事件列表（从store获取）
const events = computed(() => eventStore.events);

// 方法
const onEventClick = (event: CalendarEvent) => {
  selectedEvent.value = event;
};

const onDateClick = (date: Date) => {
  // 在指定日期创建新事件
  editingEvent.value = {
    id: 0,
    title: '',
    description: '',
    start: date,
    end: new Date(date.getTime() + 60 * 60 * 1000), // 默认1小时后结束
    allDay: false,
    reminderMinutes: 15,
    created_at: new Date(),
    updated_at: new Date(),
    sequence: 0,
    status: 'CONFIRMED'
  } as CalendarEvent;
  showEventEditor.value = true;
};

const onEditEvent = (event: CalendarEvent) => {
  editingEvent.value = { ...event };
  showEventEditor.value = true;
  selectedEvent.value = null;
};

const onSaveEvent = async (eventData: CalendarEvent) => {
  try {
    // 确保日期字段是Date对象
    const ensureDate = (date: string | Date): Date => {
      if (typeof date === 'string') {
        return new Date(date);
      }
      return date;
    };
    
    if (eventData.id === 0) {
      // 创建新事件
      await eventStore.createEvent({
        title: eventData.title,
        description: eventData.description,
        start: ensureDate(eventData.start),
        end: ensureDate(eventData.end),
        allDay: eventData.allDay,
        reminderMinutes: eventData.reminderMinutes,
      });
    } else {
      // 更新现有事件
      await eventStore.updateEvent({
        id: eventData.id,
        title: eventData.title,
        description: eventData.description,
        start: eventData.start ? ensureDate(eventData.start) : undefined,
        end: eventData.end ? ensureDate(eventData.end) : undefined,
        allDay: eventData.allDay,
        reminderMinutes: eventData.reminderMinutes,
      });
    }
    showEventEditor.value = false;
    editingEvent.value = null;
  } catch (error) {
    console.error('保存事件失败:', error);
  }
};

const onDeleteEvent = async (eventId: number) => {
  if (confirm('确定要删除这个事件吗？')) {
    try {
      await eventStore.deleteEvent(eventId);
      selectedEvent.value = null;
    } catch (error) {
      console.error('删除事件失败:', error);
    }
  }
};

// 组件挂载时加载事件
onMounted(async () => {
  await eventStore.fetchAllEvents();
});
</script>

<style scoped>
.calendar-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  background-color: var(--calendar-bg-color);
  color: var(--calendar-text-color);
}

.calendar-header {
  flex-shrink: 0;
  border-bottom: 1px solid var(--calendar-border-color);
}

.calendar-content {
  flex: 1;
  overflow: auto;
}
</style>