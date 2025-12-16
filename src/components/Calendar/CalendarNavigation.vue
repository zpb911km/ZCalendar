<template>
  <div class="calendar-navigation">
    <div class="nav-controls">
      <button class="nav-btn" @click="prev" :aria-label="`上一个${viewLabel}`">
        &lt;
      </button>
      <button class="nav-btn today-btn" @click="goToToday">
        今天
      </button>
      <button class="nav-btn" @click="next" :aria-label="`下一个${viewLabel}`">
        &gt;
      </button>
    </div>
    
    <div class="view-selector">
      <button
        v-for="view in viewTypes"
        :key="view.value"
        class="view-btn"
        :class="{ active: currentView === view.value }"
        @click="changeView(view.value)"
      >
        {{ view.label }}
      </button>
    </div>
    
    <div class="current-date">
      {{ formattedCurrentDate }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useCalendar } from '@/composables/useCalendar';

// 定义props和emits
const props = defineProps<{
  currentDate: Date;
  currentView: 'month' | 'week' | 'day';
}>();

const emit = defineEmits<{
  (e: 'update:currentDate', date: Date): void;
  (e: 'update:currentView', view: 'month' | 'week' | 'day'): void;
  (e: 'today'): void;
  (e: 'next'): void;
  (e: 'prev'): void;
  (e: 'date-change', date: Date): void;
  (e: 'view-change', view: 'month' | 'week' | 'day'): void;
}>();

// 使用组合式函数
const { 
  currentDate,
  currentViewTitle, 
  nextPeriod, 
  prevPeriod, 
  goToToday: goToTodayFromHook,
  changeView: changeViewFromHook,
  viewTypes
} = useCalendar(props.currentDate);

// 计算属性
const formattedCurrentDate = computed(() => currentViewTitle.value);

const viewLabel = computed(() => {
  switch (props.currentView) {
    case 'month': return '月';
    case 'week': return '周';
    case 'day': return '日';
    default: return '月';
  }
});

// 方法
const next = () => {
  nextPeriod();
  emit('next');
  emit('date-change', currentDate.value);
};

const prev = () => {
  prevPeriod();
  emit('prev');
  emit('date-change', currentDate.value);
};

const goToToday = () => {
  goToTodayFromHook();
  emit('today');
  emit('date-change', currentDate.value);
};

const changeView = (view: 'month' | 'week' | 'day') => {
  changeViewFromHook(view);
  emit('update:currentView', view);
  emit('view-change', view);
};
</script>

<style scoped>
.calendar-navigation {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: var(--calendar-nav-bg-color);
  border-bottom: 1px solid var(--calendar-border-color);
}

.nav-controls {
  display: flex;
  gap: 8px;
}

.nav-btn {
  width: 32px;
  height: 32px;
  border: 1px solid var(--calendar-border-color);
  background-color: var(--calendar-button-bg-color);
  color: var(--calendar-button-text-color);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  transition: all 0.2s ease;
}

.nav-btn:hover {
  background-color: var(--calendar-button-hover-bg-color);
}

.nav-btn.today-btn {
  min-width: 40px;
  padding: 4px 8px;
}

.view-selector {
  display: flex;
  gap: 4px;
}

.view-btn {
  padding: 6px 12px;
  border: 1px solid var(--calendar-border-color);
  background-color: var(--calendar-button-bg-color);
  color: var(--calendar-button-text-color);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.view-btn:hover {
  background-color: var(--calendar-button-hover-bg-color);
}

.view-btn.active {
  background-color: var(--calendar-active-button-bg-color);
  color: var(--calendar-active-button-text-color);
}

.current-date {
  font-size: 16px;
  font-weight: 500;
  color: var(--calendar-text-color);
}
</style>