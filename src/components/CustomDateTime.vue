<template>
  <div
    class="custom-datetime"
    :class="{ open: isOpen, disabled: disabled }"
    ref="datetimeRef"
  >
    <div class="datetime-trigger" @click="toggleDropdown">
      <span class="datetime-value">{{ displayValue }}</span>
      <span class="calendar-icon">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <rect
            x="2"
            y="4"
            width="12"
            height="10"
            rx="1"
            stroke="currentColor"
            stroke-width="1.5"
          />
          <path
            d="M5 2V4"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
          <path
            d="M11 2V4"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
          <path
            d="M2 7H14"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
        </svg>
      </span>
    </div>

    <div v-if="isOpen" class="dropdown-menu">
      <div class="datetime-tabs">
        <button
          type="button"
          class="tab-btn"
          :class="{ active: activeTab === 'date' }"
          @click="activeTab = 'date'"
        >
          日期
        </button>
        <button
          type="button"
          class="tab-btn"
          :class="{ active: activeTab === 'time' }"
          @click="activeTab = 'time'"
        >
          时间
        </button>
      </div>

      <!-- 日期选择 -->
      <div v-if="activeTab === 'date'" class="date-picker">
        <div class="date-header">
          <button type="button" class="nav-btn" @click="prevMonth">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <path
                d="M10 12L6 8L10 4"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
          <span class="month-year">{{ currentMonthYear }}</span>
          <button type="button" class="nav-btn" @click="nextMonth">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <path
                d="M6 12L10 8L6 4"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
        </div>

        <div class="calendar-grid">
          <div class="weekdays">
            <span v-for="day in weekDays" :key="day">{{ day }}</span>
          </div>
          <div class="days">
            <span
              v-for="day in calendarDays"
              :key="day.key"
              class="day-cell"
              :class="{
                'other-month': day.otherMonth,
                selected: day.selected,
                today: day.today,
              }"
              @click="selectDate(day.date)"
            >
              {{ day.day }}
            </span>
          </div>
        </div>
      </div>

      <!-- 时间选择 -->
      <div v-if="activeTab === 'time'" class="time-picker">
        <div class="time-inputs">
          <div class="time-group">
            <label>时</label>
            <input
              v-model.number="hours"
              min="0"
              max="23"
              @input="updateTime"
            />
          </div>
          <span class="time-separator">:</span>
          <div class="time-group">
            <label>分</label>
            <input
              v-model.number="minutes"
              min="0"
              max="59"
              @input="updateTime"
            />
          </div>
          <span class="time-separator">:</span>
          <div class="time-group">
            <label>秒</label>
            <input
              v-model.number="seconds"
              min="0"
              max="59"
              @input="updateTime"
            />
          </div>
        </div>

        <!-- <div class="quick-times">
          <button
            type="button"
            v-for="time in quickTimes"
            :key="time.label"
            class="quick-time-btn"
            @click="selectQuickTime(time)"
          >
            {{ time.label }}
          </button>
        </div> -->
      </div>

      <div class="datetime-actions">
        <button type="button" class="btn-cancel" @click="cancel">取消</button>
        <button type="button" class="btn-confirm" @click="confirm">确认</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';

interface Props {
  modelValue: string;
  disabled?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
}>();

const isOpen = ref(false);
const activeTab = ref<'date' | 'time'>('date');
const datetimeRef = ref<HTMLElement | null>(null);
const tempDate = ref<Date>(new Date());
const currentMonth = ref<number>(0);
const currentYear = ref<number>(0);
const hours = ref<number>(0);
const minutes = ref<number>(0);
const seconds = ref<number>(0);

const weekDays = ['日', '一', '二', '三', '四', '五', '六'];

const displayValue = computed(() => {
  if (!props.modelValue) return '请选择日期时间';
  const date = new Date(props.modelValue);
  if (isNaN(date.getTime())) return '请选择日期时间';
  return formatDateTime(date);
});

const currentMonthYear = computed(() => {
  return `${currentYear.value}年 ${currentMonth.value + 1}月`;
});

const calendarDays = computed(() => {
  const days: Array<{
    key: string;
    day: number;
    date: Date;
    otherMonth: boolean;
    selected: boolean;
    today: boolean;
  }> = [];

  const firstDay = new Date(currentYear.value, currentMonth.value, 1);
  const lastDay = new Date(currentYear.value, currentMonth.value + 1, 0);
  const startDayOfWeek = firstDay.getDay();

  // 上个月的日期
  const prevMonth = new Date(currentYear.value, currentMonth.value, 0);
  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    const date = new Date(
      currentYear.value,
      currentMonth.value - 1,
      prevMonth.getDate() - i
    );
    days.push({
      key: `prev-${i}`,
      day: date.getDate(),
      date,
      otherMonth: true,
      selected: isSameDate(date, tempDate.value),
      today: isSameDate(date, new Date()),
    });
  }

  // 当前月的日期
  for (let i = 1; i <= lastDay.getDate(); i++) {
    const date = new Date(currentYear.value, currentMonth.value, i);
    days.push({
      key: `current-${i}`,
      day: i,
      date,
      otherMonth: false,
      selected: isSameDate(date, tempDate.value),
      today: isSameDate(date, new Date()),
    });
  }

  // 下个月的日期
  const remainingDays = 42 - days.length;
  for (let i = 1; i <= remainingDays; i++) {
    const date = new Date(currentYear.value, currentMonth.value + 1, i);
    days.push({
      key: `next-${i}`,
      day: i,
      date,
      otherMonth: true,
      selected: isSameDate(date, tempDate.value),
      today: isSameDate(date, new Date()),
    });
  }

  return days;
});

const formatDateTime = (date: Date): string => {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  const seconds = String(date.getSeconds()).padStart(2, '0');
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
};

const isSameDate = (date1: Date, date2: Date): boolean => {
  return (
    date1.getFullYear() === date2.getFullYear() &&
    date1.getMonth() === date2.getMonth() &&
    date1.getDate() === date2.getDate()
  );
};

const toggleDropdown = () => {
  if (!props.disabled) {
    isOpen.value = !isOpen.value;
    if (isOpen.value) {
      if (props.modelValue) {
        tempDate.value = new Date(props.modelValue);
      } else {
        tempDate.value = new Date();
      }
      currentMonth.value = tempDate.value.getMonth();
      currentYear.value = tempDate.value.getFullYear();
      hours.value = tempDate.value.getHours();
      minutes.value = tempDate.value.getMinutes();
      seconds.value = tempDate.value.getSeconds();
    }
  }
};

const selectDate = (date: Date) => {
  tempDate.value = date;
  hours.value = date.getHours();
  minutes.value = date.getMinutes();
  seconds.value = date.getSeconds();
  currentMonth.value = date.getMonth();
  currentYear.value = date.getFullYear();
};

const updateTime = () => {
  tempDate.value.setHours(hours.value || 0);
  tempDate.value.setMinutes(minutes.value || 0);
  tempDate.value.setSeconds(seconds.value || 0);
};

const prevMonth = () => {
  currentMonth.value--;
  if (currentMonth.value < 0) {
    currentMonth.value = 11;
    currentYear.value--;
  }
};

const nextMonth = () => {
  currentMonth.value++;
  if (currentMonth.value > 11) {
    currentMonth.value = 0;
    currentYear.value++;
  }
};

const confirm = () => {
  tempDate.value.setHours(hours.value || 0);
  tempDate.value.setMinutes(minutes.value || 0);
  tempDate.value.setSeconds(seconds.value || 0);

  const value = formatDateTime(tempDate.value);
  emit('update:modelValue', value);
  emit('change', value);
  isOpen.value = false;
};

const cancel = () => {
  isOpen.value = false;
};

const closeDropdown = (event: MouseEvent) => {
  if (datetimeRef.value && !datetimeRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
};

const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    isOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', closeDropdown);
  document.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  document.removeEventListener('click', closeDropdown);
  document.removeEventListener('keydown', handleKeyDown);
});

watch(
  () => props.modelValue,
  newVal => {
    if (newVal && !isOpen.value) {
      const date = new Date(newVal);
      if (!isNaN(date.getTime())) {
        tempDate.value = date;
        currentMonth.value = date.getMonth();
        currentYear.value = date.getFullYear();
        hours.value = date.getHours();
        minutes.value = date.getMinutes();
        seconds.value = date.getSeconds();
      }
    }
  }
);
</script>

<style scoped>
.custom-datetime {
  position: relative;
  display: inline-block;
  min-width: 200px;
  user-select: none;
}

.datetime-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background-color: var(--input-background-color);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  min-height: 40px;
  box-sizing: border-box;
}

.datetime-trigger:hover {
  border-color: var(--primary-color);
}

.custom-datetime.open .datetime-trigger {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(var(--primary-color-rgb), 0.1);
}

.custom-datetime.disabled .datetime-trigger {
  opacity: 0.6;
  cursor: not-allowed;
  background-color: var(--background-color);
}

.datetime-value {
  flex: 1;
  font-size: 14px;
  color: var(--text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.calendar-icon {
  display: flex;
  align-items: center;
  margin-left: 8px;
  color: var(--text-color);
  opacity: 0.6;
  flex-shrink: 0;
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  background-color: var(--input-background-color);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  min-width: 300px;
  max-width: 400px;
}

.datetime-tabs {
  display: flex;
  border-bottom: 1px solid var(--border-color);
}

.tab-btn {
  flex: 1;
  padding: 10px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 14px;
  color: var(--text-color);
  transition: all 0.2s ease;
  border-bottom: 2px solid transparent;
}

.tab-btn:hover {
  background-color: var(--hover-background-color);
}

.tab-btn.active {
  color: var(--primary-color);
  border-bottom-color: var(--primary-color);
}

.date-picker,
.time-picker {
  padding: 16px;
}

.date-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.nav-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-color);
  transition: all 0.2s ease;
}

.nav-btn:hover {
  background-color: var(--hover-background-color);
  border-color: var(--primary-color);
}

.month-year {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color);
}

.calendar-grid {
  margin-top: 12px;
}

.weekdays {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
  margin-bottom: 8px;
}

.weekdays span {
  text-align: center;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-color);
  opacity: 0.6;
}

.days {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.day-cell {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 32px;
  font-size: 13px;
  color: var(--text-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.day-cell:hover:not(.other-month) {
  background-color: var(--hover-background-color);
}

.day-cell.other-month {
  opacity: 0.4;
}

.day-cell.selected {
  background-color: var(--primary-color);
  color: white;
}

.day-cell.today {
  border: 1px solid var(--primary-color);
}

.time-inputs {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  margin-bottom: 16px;
}

.time-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.time-group label {
  font-size: 12px;
  color: var(--text-color);
  opacity: 0.6;
}

.time-group input {
  width: 60px;
  padding: 6px 8px;
  text-align: center;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
  font-size: 14px;
  box-sizing: border-box;
}

.time-group input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px rgba(var(--primary-color-rgb), 0.1);
}

.time-separator {
  font-size: 20px;
  font-weight: 500;
  color: var(--text-color);
  margin-top: 20px;
}

.quick-times {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: center;
}

.quick-time-btn {
  padding: 6px 12px;
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  color: var(--text-color);
  transition: all 0.2s ease;
}

.quick-time-btn:hover {
  background-color: var(--hover-background-color);
  border-color: var(--primary-color);
}

.datetime-actions {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  border-top: 1px solid var(--border-color);
}

.btn-cancel,
.btn-confirm {
  flex: 1;
  padding: 8px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.btn-cancel {
  background-color: var(--background-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
}

.btn-cancel:hover {
  background-color: var(--hover-background-color);
}

.btn-confirm {
  background-color: var(--primary-color);
  color: white;
}

.btn-confirm:hover {
  opacity: 0.9;
}
</style>
