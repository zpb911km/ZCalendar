<template>
  <div class="modal-overlay" @click="onOverlayClick">
    <div class="event-detail" @click.stop @keyup.esc="close">
      <div class="detail-header">
        <h3>{{ event.title }}</h3>
        <button class="close-btn" @click="close" aria-label="关闭">✕</button>
      </div>

      <div class="detail-content">
        <div class="event-info">
          <div class="info-item">
            <span class="info-label">时间</span>
            <span class="info-value">
              {{ formatDateTime(event.start) }}
              <span v-if="!event.all_day">
                - {{ formatDateTime(event.end) }}</span
              >
              <span v-else> (全天)</span>
            </span>
          </div>

          <div v-if="event.description" class="info-item">
            <span class="info-label">描述</span>
            <span class="info-value">{{ event.description }}</span>
          </div>

          <div v-if="event.location" class="info-item">
            <span class="info-label">位置</span>
            <span class="info-value">📍 {{ event.location }}</span>
          </div>

          <div v-if="event.categories" class="info-item">
            <span class="info-label">分类</span>
            <span class="info-value">{{ event.categories }}</span>
          </div>

          <div class="info-item">
            <span class="info-label">提醒</span>
            <span class="info-value">
              {{
                event.reminder_minutes > 0
                  ? `${event.reminder_minutes}分钟前`
                  : '无提醒'
              }}
            </span>
          </div>

          <div class="info-item">
            <span class="info-label">状态</span>
            <span class="info-value">{{ event.status }}</span>
          </div>

          <div class="info-item">
            <span class="info-label">创建时间</span>
            <span class="info-value">{{
              formatDateTime(event.created_at)
            }}</span>
          </div>

          <div class="info-item">
            <span class="info-label">更新时间</span>
            <span class="info-value">{{
              formatDateTime(event.updated_at)
            }}</span>
          </div>
        </div>
      </div>

      <div class="detail-actions">
        <button class="btn-edit" @click="edit">编辑</button>
        <button class="btn-delete" @click="deleteEvent">删除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { dateUtils } from '@/utils/dateUtils';
import { CalendarEvent } from '@/types/event';

// 确保日期字段是Date对象
const ensureDate = (date: string | Date): Date => {
  if (typeof date === 'string') {
    return new Date(date);
  }
  return date;
};

// 格式化日期时间
const formatDateTime = (date: string | Date): string => {
  const dateObj = ensureDate(date);
  return dateUtils.formatLocal(dateObj, 'yyyy年MM月dd日 HH:mm');
};

// 定义props和emits
const props = defineProps<{
  event: CalendarEvent;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'edit', event: CalendarEvent): void;
  (e: 'delete', id: number): void;
}>();

const close = () => {
  emit('close');
};

const edit = () => {
  emit('edit', props.event);
};

const deleteEvent = () => {
  if (confirm('确定要删除这个事件吗？')) {
    emit('delete', props.event.id);
  }
};

const onOverlayClick = () => {
  close();
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.event-detail {
  background-color: var(--modal-background-color);
  border-radius: 8px;
  width: 100%;
  max-width: 500px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
  max-height: 90vh;
  overflow-y: auto;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.detail-header h3 {
  margin: 0;
  color: var(--text-color);
  flex: 1;
  word-break: break-word;
}

.close-btn {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: var(--text-color);
  padding: 4px;
  border-radius: 4px;
  transition: background-color 0.2s ease;
  flex-shrink: 0;
  margin-left: 16px;
}

.close-btn:hover {
  background-color: var(--secondary-light);
}

.detail-content {
  padding: 20px;
  flex: 1;
  overflow-y: auto;
}

.event-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-label {
  font-size: 12px;
  color: var(--text-secondary-color);
  font-weight: 500;
}

.info-value {
  color: var(--text-color);
  font-size: 14px;
  word-break: break-word;
}

.detail-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 20px;
  border-top: 1px solid var(--border-color);
}

.btn-edit,
.btn-delete {
  padding: 8px 20px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.btn-edit {
  background-color: var(--secondary-color);
  color: var(--text-color);
}

.btn-edit:hover {
  background-color: var(--secondary-light);
}

.btn-delete {
  background-color: var(--danger-color);
  color: white;
}

.btn-delete:hover {
  background-color: #c82333;
}
</style>
