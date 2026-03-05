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
        @view-change="changeView"
      />
    </div>

    <!-- 日历视图内容 -->
    <div
      class="calendar-content"
      @touchstart="handleTouchStart"
      @touchmove="handleTouchMove"
      @touchend="handleTouchEnd"
    >
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
        @date-click="onDateClick"
      />
      <DayView
        v-else-if="currentView === 'day'"
        :events="events"
        :current-date="currentDate"
        @event-click="onEventClick"
        @date-click="onDateClick"
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
        @cancel="showEventEditor = false"
      />
    </teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useEventStore } from '@/stores/eventStore';
import { useCalendar } from '@/composables/useCalendar';
import CalendarNavigation from '../components/Calendar/CalendarNavigation.vue';
import MonthView from '../components/Calendar/MonthView.vue';
import WeekView from '../components/Calendar/WeekView.vue';
import DayView from '../components/Calendar/DayView.vue';
import EventDetail from '../components/Event/EventDetail.vue';
import EventEditor from '../components/Event/EventEditor.vue';
import { CalendarEvent } from '@/types/event';

// 使用组合式函数
const {
  currentDate,
  currentView,
  nextPeriod,
  prevPeriod,
  goToToday,
  loadCalendars,
  changeView,
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
  isSliding.value = false;
  selectedEvent.value = event;
};

const defaultReminderMinutes =
  Number(localStorage.getItem('defaultReminderMinutes')) || 15;
console.log(`default reminder minutes: ${defaultReminderMinutes}`);

const onDateClick = (date: Date) => {
  // 在指定日期创建新事件
  console.log(`create event at ${date}`);
  isSliding.value = false;
  editingEvent.value = {
    id: 0,
    title: '',
    description: '',
    start: date,
    end: new Date(date.getTime() + 60 * 60 * 1000), // 默认1小时后结束
    all_day: false,
    reminder_minutes: defaultReminderMinutes,
    created_at: new Date(),
    updated_at: new Date(),
    sequence: 0,
    status: 'CONFIRMED',
  } as CalendarEvent;
  showEventEditor.value = true;
};

// const openEventEditor = (event?: CalendarEvent) => {
//   if (event) {
//     editingEvent.value = { ...event };
//   } else {
//     editingEvent.value = {
//       id: 0,
//       title: '',
//       description: '',
//       start: new Date(),
//       end: new Date(new Date().getTime() + 60 * 60 * 1000),
//       allDay: false,
//       reminderMinutes: 15,
//       created_at: new Date(),
//       updated_at: new Date(),
//       sequence: 0,
//       status: 'CONFIRMED'
//     } as CalendarEvent;
//   }
//   showEventEditor.value = true;
// };

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
    console.log(`save event: ${JSON.stringify(eventData, null, 2)}`);

    if (eventData.id === 0) {
      // 创建新事件
      await eventStore.createEvent({
        ...eventData,
        start: ensureDate(eventData.start),
        end: ensureDate(eventData.end),
      });
    } else {
      // 更新现有事件
      await eventStore.updateEvent({
        ...eventData,
        start: eventData.start ? ensureDate(eventData.start) : undefined,
        end: eventData.end ? ensureDate(eventData.end) : undefined,
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

// 触摸事件处理相关变量
const touchStartX = ref(0);
const touchEndX = ref(0);
const isSliding = ref(false);

// 触摸事件处理方法
const handleTouchStart = (event: TouchEvent) => {
  touchStartX.value = event.touches[0].clientX;
};

const handleTouchMove = (event: TouchEvent) => {
  touchEndX.value = event.touches[0].clientX;
  isSliding.value = true;
};

const handleTouchEnd = () => {
  if (!isSliding.value) {
    return;
  }
  if (touchStartX.value - touchEndX.value > 100) {
    // 从左向右滑动，触发 nextPeriod
    nextPeriod();
  } else if (touchEndX.value - touchStartX.value > 100) {
    // 从右向左滑动，触发 prevPeriod
    prevPeriod();
  }
  isSliding.value = false;
};

// 组件挂载时加载事件和日历
onMounted(async () => {
  await eventStore.fetchAllEvents();
  await loadCalendars();
});
</script>

<style scoped>
.calendar-container {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 56px - var(--android-status-bar-height));
  width: 100%;
  background-color: var(--background-color);
  color: var(--text-color);
}

.calendar-header {
  flex-shrink: 0;
  /* display: flex; */
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--background-color);
}

.calendar-actions {
  display: flex;
  gap: 10px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary {
  background-color: var(--primary-color);
  color: white;
}

.btn-secondary {
  background-color: var(--secondary-color);
  color: var(--text-color);
}

.calendar-content {
  flex: 1;
  overflow: auto;
}
</style>
