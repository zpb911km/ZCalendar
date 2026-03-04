<template>
  <div
    class="custom-select"
    :class="{ open: isOpen, disabled: disabled }"
    ref="selectRef"
  >
    <div class="select-trigger" @click="toggleDropdown">
      <span class="selected-value">{{ displayValue }}</span>
      <span class="arrow">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path
            d="M2 4L6 8L10 4"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </span>
    </div>

    <div v-if="isOpen" class="dropdown-menu" :style="{ maxHeight: maxHeight }">
      <div
        v-for="option in options"
        :key="option.value"
        class="dropdown-item"
        :class="{ selected: modelValue === option.value }"
        @click="selectOption(option.value)"
      >
        <span class="item-text">{{ option.label }}</span>
        <span v-if="modelValue === option.value" class="check-icon">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path
              d="M4 8L7 11L12 5"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

interface Option {
  value: any;
  label: string;
}

interface Props {
  modelValue: any;
  options: Option[];
  disabled?: boolean;
  maxHeight?: string;
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  maxHeight: '300px',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: any): void;
  (e: 'change', value: any): void;
}>();

const isOpen = ref(false);
const selectRef = ref<HTMLElement | null>(null);

const displayValue = computed(() => {
  const selected = props.options.find(opt => opt.value === props.modelValue);
  return selected?.label || '请选择';
});

const toggleDropdown = () => {
  if (!props.disabled) {
    isOpen.value = !isOpen.value;
  }
};

const selectOption = (value: any) => {
  emit('update:modelValue', value);
  emit('change', value);
  isOpen.value = false;
};

const closeDropdown = (event: MouseEvent) => {
  if (selectRef.value && !selectRef.value.contains(event.target as Node)) {
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
</script>

<style scoped>
.custom-select {
  position: relative;
  user-select: none;
  display: inline-block;
  border: 1px solid var(--border-color);
  border-radius: 4px;
}

.select-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--input-background-color);
  cursor: pointer;
  padding: 0 12px;
  transition: all 0.2s ease;
  min-height: 40px;
  min-width: 100%;
  box-sizing: border-box;
}

.select-trigger:hover {
  border-color: var(--primary-color);
}

.custom-select.open .select-trigger {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(var(--primary-color-rgb), 0.1);
}

.custom-select.disabled .select-trigger {
  opacity: 0.6;
  cursor: not-allowed;
  background-color: var(--background-color);
}

.selected-value {
  flex: 1;
  font-size: 14px;
  color: var(--text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 0;
}

.arrow {
  display: flex;
  align-items: center;
  margin-left: 8px;
  color: var(--text-color);
  opacity: 0.6;
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.custom-select.open .arrow {
  transform: rotate(180deg);
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
  overflow-y: auto;
  max-height: v-bind(maxHeight);
  min-width: 100%;
  white-space: nowrap;
}

.dropdown-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  min-width: 100%;
  box-sizing: border-box;
}

.dropdown-item:hover {
  background-color: var(--hover-background-color);
}

.dropdown-item.selected {
  background-color: rgba(var(--primary-color-rgb), 0.1);
}

.item-text {
  font-size: 14px;
  color: var(--text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  min-width: 0;
}

.check-icon {
  display: flex;
  align-items: center;
  color: var(--primary-color);
  flex-shrink: 0;
  margin-left: 8px;
}

/* 滚动条样式 */
.dropdown-menu::-webkit-scrollbar {
  width: 6px;
}

.dropdown-menu::-webkit-scrollbar-track {
  background: transparent;
}

.dropdown-menu::-webkit-scrollbar-thumb {
  background-color: var(--border-color);
  border-radius: 3px;
}

.dropdown-menu::-webkit-scrollbar-thumb:hover {
  background-color: var(--text-color);
  opacity: 0.5;
}
</style>
