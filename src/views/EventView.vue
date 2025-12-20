<template>
  <div class="event-view">
    <header class="event-header">
      <h1 class="event-title">事件管理</h1>
      <div class="event-actions">
        <button @click="openEventEditor()" class="btn btn-primary">
          添加事件
        </button>
        <button @click="searchEvents" class="btn btn-secondary">搜索</button>
        <!-- <button @click="exportCalendar" class="btn btn-secondary">导出日历</button> -->
      </div>
    </header>

    <main class="event-main">
      <div class="event-filters">
        <select v-model="filterCategory" class="category-filter">
          <option value="">所有分类</option>
          <option
            v-for="category in categories"
            :key="category"
            :value="category"
          >
            {{ category }}
          </option>
        </select>
        <select v-model="filterDate" class="date-filter">
          <option value="">所有日期</option>
          <option value="today">今天</option>
          <option value="tomorrow">明天</option>
          <option value="thisWeek">本周</option>
          <option value="thisMonth">本月</option>
        </select>
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索事件..."
          class="search-input"
          @keyup.enter="searchEvents"
        />
      </div>

      <div class="event-list">
        <event-card
          v-for="event in filteredEvents"
          :key="event.id"
          :event="event"
          @click="openEventDetail(event)"
          @edit="editEvent"
          @delete="deleteEvent"
        />
        <div v-if="filteredEvents.length === 0" class="no-events">
          没有找到事件
        </div>
      </div>
    </main>

    <!-- 事件编辑器模态框 -->
    <div v-if="showEventEditor" class="modal-overlay" @click="closeEventEditor">
      <div class="modal-content" @click.stop>
        <event-editor
          :event="editingEvent"
          @save="saveEvent"
          @cancel="closeEventEditor"
        />
      </div>
    </div>

    <!-- 事件详情模态框 -->
    <div v-if="showEventDetail" class="modal-overlay" @click="closeEventDetail">
      <div class="modal-content" @click.stop>
        <event-detail
          :event="selectedEvent"
          @edit="editEvent(selectedEvent)"
          @delete="deleteEvent(selectedEvent.id)"
          @close="closeEventDetail"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useEventStore } from '@/stores/eventStore';
import EventCard from '@/components/Event/EventCard.vue';
import EventEditor from '@/components/Event/EventEditor.vue';
import EventDetail from '@/components/Event/EventDetail.vue';
import { CalendarEvent } from '@/types/event';

const eventStore = useEventStore();

const showEventEditor = ref(false);
const showEventDetail = ref(false);
const editingEvent = ref<CalendarEvent>({
  id: 0,
  title: '',
  description: '',
  start: new Date(),
  end: new Date(new Date().getTime() + 60 * 60 * 1000),
  all_day: false,
  reminder_minutes: 15,
  created_at: new Date(),
  updated_at: new Date(),
  sequence: 0,
  status: 'CONFIRMED',
});
const selectedEvent = ref<CalendarEvent>({
  id: 0,
  title: '',
  description: '',
  start: new Date(),
  end: new Date(new Date().getTime() + 60 * 60 * 1000),
  all_day: false,
  reminder_minutes: 15,
  created_at: new Date(),
  updated_at: new Date(),
  sequence: 0,
  status: 'CONFIRMED',
});
const searchQuery = ref('');
const filterCategory = ref('');
const filterDate = ref('');

onMounted(() => {
  eventStore.fetchAllEvents();
});

// 从所有事件中提取唯一分类
const categories = computed(() => {
  const catSet = new Set<string>();
  eventStore.events.forEach(event => {
    if (event.categories) {
      event.categories.split(',').forEach(cat => {
        const trimmedCat = cat.trim();
        if (trimmedCat) catSet.add(trimmedCat);
      });
    }
  });
  return Array.from(catSet);
});

// 根据过滤条件筛选事件
const filteredEvents = computed(() => {
  let result = eventStore.events;

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(
      event =>
        event.title.toLowerCase().includes(query) ||
        (event.description &&
          event.description.toLowerCase().includes(query)) ||
        (event.location && event.location.toLowerCase().includes(query))
    );
  }

  // 分类过滤
  if (filterCategory.value) {
    result = result.filter(
      event =>
        event.categories &&
        event.categories
          .split(',')
          .map(cat => cat.trim())
          .includes(filterCategory.value)
    );
  }

  // 日期过滤
  if (filterDate.value) {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const tomorrow = new Date(today);
    tomorrow.setDate(tomorrow.getDate() + 1);
    const startOfWeek = new Date(
      now.getFullYear(),
      now.getMonth(),
      now.getDate() - now.getDay()
    );
    const endOfWeek = new Date();
    endOfWeek.setDate(startOfWeek.getDate() + 6);
    const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0);

    result = result.filter(event => {
      const eventDate = new Date(event.start);
      switch (filterDate.value) {
        case 'today':
          return eventDate.toDateString() === today.toDateString();
        case 'tomorrow':
          return eventDate.toDateString() === tomorrow.toDateString();
        case 'thisWeek':
          return eventDate >= startOfWeek && eventDate <= endOfWeek;
        case 'thisMonth':
          return eventDate >= startOfMonth && eventDate <= endOfMonth;
        default:
          return true;
      }
    });
  }

  // 按开始时间排序
  result.sort(
    (a, b) => new Date(a.start).getTime() - new Date(b.start).getTime()
  );

  return result;
});

const openEventEditor = (event?: CalendarEvent) => {
  editingEvent.value = event || {
    id: 0,
    title: '',
    description: '',
    start: new Date(),
    end: new Date(new Date().getTime() + 60 * 60 * 1000),
    all_day: false,
    reminder_minutes: 15,
    created_at: new Date(),
    updated_at: new Date(),
    sequence: 0,
    status: 'CONFIRMED',
  };
  showEventEditor.value = true;
};

const closeEventEditor = () => {
  showEventEditor.value = false;
  editingEvent.value = {
    id: 0,
    title: '',
    description: '',
    start: new Date(),
    end: new Date(new Date().getTime() + 60 * 60 * 1000),
    all_day: false,
    reminder_minutes: 15,
    created_at: new Date(),
    updated_at: new Date(),
    sequence: 0,
    status: 'CONFIRMED',
  };
};

const openEventDetail = (event: CalendarEvent) => {
  selectedEvent.value = { ...event };
  showEventDetail.value = true;
};

const closeEventDetail = () => {
  showEventDetail.value = false;
};

const saveEvent = async (eventData: CalendarEvent) => {
  try {
    if (eventData.id) {
      await eventStore.updateEvent({
        ...eventData,
        start: eventData.start as Date,
        end: eventData.end as Date,
      });
    } else {
      await eventStore.createEvent({
        ...eventData,
        start: eventData.start as Date,
        end: eventData.end as Date,
      });
    }
    closeEventEditor();
  } catch (error) {
    console.error('保存事件失败:', error);
  }
};

const editEvent = (event: CalendarEvent) => {
  openEventEditor(event);
  closeEventDetail();
};

const deleteEvent = async (id: number) => {
  if (confirm('确定要删除这个事件吗？')) {
    try {
      await eventStore.deleteEvent(id);
      closeEventDetail();
    } catch (error) {
      console.error('删除事件失败:', error);
    }
  }
};

const searchEvents = async () => {
  // 如果有搜索查询，则调用后端搜索功能
  if (searchQuery.value) {
    try {
      await eventStore.searchEvents(searchQuery.value);
    } catch (error) {
      console.error('搜索事件失败:', error);
    }
  } else {
    // 如果没有搜索查询，则获取所有事件
    await eventStore.fetchAllEvents();
  }
};
</script>

<style scoped>
.event-view {
  display: flex;
  flex-direction: column;
  padding: 20px;
  background-color: var(--background-color);
  color: var(--text-color);
  height: calc(100% - 40px);
}

.event-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--border-color);
}

.event-title {
  font-size: 24px;
  font-weight: bold;
  margin: 0;
}

.event-actions {
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

.event-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.event-filters {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 200px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
}

.category-filter,
.date-filter {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
}

.event-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.no-events {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
  font-size: 18px;
  color: var(--text-secondary-color);
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--modal-background-color);
  border-radius: 8px;
  max-width: 90%;
  max-height: 90%;
  overflow-y: auto;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
</style>
