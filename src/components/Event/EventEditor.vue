<template>
  <div class="modal-overlay" @click="onOverlayClick">
    <div class="event-editor" @click.stop @keyup.esc="cancel">
      <div class="editor-header">
        <h3>{{ props.event?.id ? '编辑事件' : '新建事件' }}</h3>
        <button class="close-btn" @click="cancel" aria-label="关闭">
          ✕
        </button>
      </div>
      
      <form class="editor-form" @submit.prevent="save">
        <div class="form-group">
          <label for="title">标题 *</label>
          <input
            id="title"
            v-model="formData.title"
            type="text"
            required
            placeholder="输入事件标题"
          />
        </div>
        
        <div class="form-group">
          <label for="description">描述</label>
          <textarea
            id="description"
            v-model="formData.description"
            placeholder="输入事件描述"
            rows="3"
          ></textarea>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="status">状态</label>
            <select id="status" v-model="formData.status">
              <option value="CONFIRMED">已确认</option>
              <option value="TENTATIVE">待定</option>
              <option value="CANCELLED">已取消</option>
              <option value="DRAFT">草稿</option>
              <option value="FINAL">最终</option>
              <option value="NEEDS-ACTION">需要行动</option>
              <option value="COMPLETED">已完成</option>
              <option value="IN-PROCESS">正在处理</option>
            </select>
          </div>
        </div>
        
        <div class="form-row">
          <div class="form-group">
            <label for="start">开始时间</label>
            <input
              id="start"
              v-model="formData.start"
              type="datetime-local"
            />
          </div>
          
          <div class="form-group">
            <label for="end">结束时间</label>
            <input
              id="end"
              v-model="formData.end"
              type="datetime-local"
            />
          </div>
        </div>
        
        <div class="form-group checkbox-group">
          <label>
            <input
              v-model="formData.all_day"
              type="checkbox"
            />
            全天事件
          </label>
        </div>
        
        <div class="form-group">
          <label for="location">位置</label>
          <input
            id="location"
            v-model="formData.location"
            type="text"
            placeholder="输入事件位置"
          />
        </div>
        
        <div class="form-group">
          <label for="reminder">提醒</label>
          <select id="reminder" v-model="formData.reminder_minutes">
            <option value="0">无提醒</option>
            <option value="5">5分钟前</option>
            <option value="15">15分钟前</option>
            <option value="30">30分钟前</option>
            <option value="60">1小时前</option>
            <option value="1440">1天前</option>
          </select>
        </div>
        
        <div class="form-group">
          <label for="category">分类</label>
          <select id="category" v-model="formData.categories">
            <option value="">无分类</option>
            <option value="工作">工作</option>
            <option value="个人">个人</option>
            <option value="重要">重要</option>
            <option value="会议">会议</option>
          </select>
        </div>
        
        <div class="form-actions">
          <button type="button" class="btn-cancel" @click="cancel">
            取消
          </button>
          <button type="submit" class="btn-save">
            保存
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
import { CalendarEvent } from '@/types/event';

// 定义props和emits
const props = defineProps<{
  event?: CalendarEvent;
}>();

const emit = defineEmits<{
  (e: 'save', event: CalendarEvent): void;
  (e: 'cancel'): void;
}>();

// 表单数据
const formData = reactive(props.event || {
  title: '',
  description: '',
  start: new Date(),
  end: new Date(Date.now() + 60 * 60 * 1000), // 默认1小时后
  all_day: false,
  location: '',
  reminder_minutes: 15,
  categories: '',
  status: 'CONFIRMED'
});

// 监听props.event的变化，初始化表单数据
watch(
  () => props.event,
  (newEvent) => {
    if (newEvent) {
      // 确保日期字段是Date对象
      const ensureDate = (date: string | Date | undefined): Date => {
        if (typeof date === 'string') {
          return new Date(date);
        }
        if (date instanceof Date) {
          return date;
        }
        return new Date();
      };
      
      formData.title = newEvent.title || '';
      formData.description = newEvent.description || '';
      formData.start = ensureDate(newEvent.start);
      formData.end = ensureDate(newEvent.end);
      formData.all_day = newEvent.all_day || false;
      formData.location = newEvent.location || '';
      formData.reminder_minutes = newEvent.reminder_minutes || 15;
      formData.categories = newEvent.categories || '';
      formData.status = newEvent.status || 'CONFIRMED';
    } else {
      // 如果没有传入事件，重置为默认值
      resetForm();
    }
  },
  { immediate: true }
);

// 方法
const resetForm = () => {
  formData.title = '';
  formData.description = '';
  formData.start = new Date();
  formData.end = new Date(Date.now() + 60 * 60 * 1000);
  formData.all_day = false;
  formData.location = '';
  formData.reminder_minutes = 15;
  formData.categories = '';
  formData.status = 'CONFIRMED';
};

const save = () => {
  const eventToSave: CalendarEvent = {
    id: props.event?.id || 0, // 0表示新建
    title: formData.title,
    description: formData.description,
    start: new Date(formData.start),
    end: new Date(formData.end),
    all_day: formData.all_day,
    reminder_minutes: Number(formData.reminder_minutes),
    created_at: props.event?.created_at || new Date(),
    updated_at: new Date(),
    sequence: props.event?.sequence || 0,
    status: formData.status? formData.status : 'CONFIRMED',
    location: formData.location || undefined,
    categories: formData.categories || undefined
  } as CalendarEvent;
  console.log(`emit save event: ${JSON.stringify(eventToSave, null, 2)}`)
  emit('save', eventToSave);
};

const cancel = () => {
  resetForm();
  emit('cancel');
};

const onOverlayClick = () => {
  cancel();
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--background-color);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.event-editor {
  background-color: var(--background-color);
  border-radius: 8px;
  width: 100%;
  max-width: 500px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
  max-height: 90vh;
  overflow-y: auto;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.editor-header h3 {
  margin: 0;
  color: var(--text-color);
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
}

.close-btn:hover {
  background-color: var(--secondary-light);
}

.editor-form {
  padding: 20px;
  flex: 1;
  overflow-y: auto;
}

.form-group {
  margin-bottom: 16px;
}

.form-row {
  display: flex;
  gap: 16px;
}

.form-row .form-group {
  flex: 1;
}

.form-group label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
  color: var(--text-color);
  font-size: 14px;
}

.form-group input,
.form-group textarea,
.form-group select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
  font-size: 14px;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group textarea:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--primary-light);
  box-shadow: 0 0 0 2px rgba(0,123,255,0.25);
}

.checkbox-group {
  display: flex;
  align-items: center;
}

.checkbox-group input {
  width: auto;
  margin-right: 8px;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}

.btn-cancel,
.btn-save {
  padding: 8px 20px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.btn-cancel {
  background-color: var(--secondary-color);
  color: var(--text-color);
}

.btn-cancel:hover {
  background-color: var(--secondary-light);
}

.btn-save {
  background-color: var(--primary-color);
  color: white;
}

.btn-save:hover {
  background-color: var(--primary-dark);
}
</style>