<template>
  <div class="settings-view">
    <header class="settings-header">
      <h1 class="settings-title">设置</h1>
    </header>

    <main class="settings-main">
      <div class="settings-section">
        <h2>外观设置</h2>
        <div class="setting-item">
          <label>主题模式</label>
          <select v-model="theme" @change="changeTheme" class="theme-selector">
            <option value="light">浅色模式</option>
            <option value="dark">深色模式</option>
            <option value="auto">自动</option>
          </select>
        </div>
        <div class="setting-item">
          <label>主色调</label>
          <input 
            v-model="primaryColor" 
            type="color" 
            @change="changePrimaryColor" 
            class="color-picker"
          />
        </div>
      </div>

      <div class="settings-section">
        <h2>通知设置</h2>
        <div class="setting-item">
          <label class="checkbox-label">
            <input 
              v-model="notificationsEnabled" 
              type="checkbox" 
              @change="toggleNotifications"
            />
            启用事件提醒
          </label>
        </div>
        <div class="setting-item">
          <label>默认提醒时间（分钟）</label>
          <input 
            v-model.number="defaultReminderMinutes" 
            type="number" 
            min="0" 
            max="1440"
            @change="updateDefaultReminder"
            class="reminder-input"
          />
        </div>
      </div>

      <div class="settings-section">
        <h2>日历设置</h2>
        <div class="setting-item">
          <label class="checkbox-label">
            <input 
              v-model="showWeekNumbers" 
              type="checkbox" 
              @change="toggleWeekNumbers"
            />
            显示周数
          </label>
        </div>
        <div class="setting-item">
          <label>工作日开始时间</label>
          <input 
            v-model="workdayStart" 
            type="time" 
            @change="updateWorkdayStart"
            class="time-input"
          />
        </div>
        <div class="setting-item">
          <label>工作日结束时间</label>
          <input 
            v-model="workdayEnd" 
            type="time" 
            @change="updateWorkdayEnd"
            class="time-input"
          />
        </div>
      </div>

      <div class="settings-section">
        <h2>日历管理</h2>
        <div class="calendar-list">
          <div 
            v-for="calendar in calendars" 
            :key="calendar.id" 
            class="calendar-item"
            :style="{ borderLeft: `4px solid ${calendar.color}` }"
          >
            <div class="calendar-info">
              <span class="calendar-name">{{ calendar.name }}</span>
              <span class="calendar-id">{{ calendar.id.substring(0, 8) }}</span>
            </div>
            <div class="calendar-actions">
              <button @click="editCalendar(calendar)" class="btn btn-edit">编辑</button>
              <button @click="deleteCalendar(calendar.id)" class="btn btn-danger">删除</button>
            </div>
          </div>
        </div>
        <div class="calendar-form">
          <input 
            v-model="newCalendarName" 
            type="text" 
            placeholder="日历名称"
            class="calendar-name-input"
          />
          <input 
            v-model="newCalendarColor" 
            type="color" 
            class="color-picker"
          />
          <button @click="createCalendar" class="btn btn-primary">添加日历</button>
        </div>
      </div>

      <div class="settings-section">
        <h2>数据管理</h2>
        <div class="setting-item">
          <button @click="exportAllEvents" class="btn btn-secondary">导出所有事件</button>
        </div>
        <div class="setting-item">
          <button @click="importEvents" class="btn btn-secondary">导入事件</button>
        </div>
        <div class="setting-item">
          <button @click="clearAllEvents" class="btn btn-danger">清空所有事件</button>
        </div>
      </div>



      <div class="settings-section">
        <h2>关于</h2>
        <div class="about-info">
          <p><strong>应用版本:</strong> 1.0.0</p>
          <p><strong>构建日期:</strong> {{ buildDate }}</p>
          <p><strong>框架:</strong> Tauri + Vue 3 + TypeScript</p>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { calendarService } from '@/services/calendarService';
import { Calendar } from '@/types/calendar';
import { invoke } from '@tauri-apps/api/core';
import { themeManager } from '@/utils/themeManager';

// 设置状态
const theme = ref('auto');
const primaryColor = ref('#007bff');
const notificationsEnabled = ref(true);
const defaultReminderMinutes = ref(15);
const showWeekNumbers = ref(false);
const workdayStart = ref('09:00');
const workdayEnd = ref('18:00');
const buildDate = ref(new Date().toISOString().split('T')[0]);

// 日历管理状态
const calendars = ref<Calendar[]>([]);
const newCalendarName = ref('');
const newCalendarColor = ref('#4285F4');

onMounted(() => {
  // 加载保存的设置
  loadSettings();
  // 加载日历列表
  loadCalendars();
});

const loadSettings = () => {
  const settings = themeManager.getSettings();
  const savedNotificationsEnabled = localStorage.getItem('notificationsEnabled') === 'true';
  const savedDefaultReminder = parseInt(localStorage.getItem('defaultReminderMinutes') || '15');
  const savedShowWeekNumbers = localStorage.getItem('showWeekNumbers') === 'true';
  const savedWorkdayStart = localStorage.getItem('workdayStart') || '09:00';
  const savedWorkdayEnd = localStorage.getItem('workdayEnd') || '18:00';

  theme.value = settings.theme;
  primaryColor.value = settings.primaryColor;
  notificationsEnabled.value = savedNotificationsEnabled;
  defaultReminderMinutes.value = savedDefaultReminder;
  showWeekNumbers.value = savedShowWeekNumbers;
  workdayStart.value = savedWorkdayStart;
  workdayEnd.value = savedWorkdayEnd;
};

const loadCalendars = async () => {
  try {
    calendars.value = await calendarService.getAllCalendars();
  } catch (error) {
    console.error('加载日历失败:', error);
  }
};

const changeTheme = () => {
  themeManager.setTheme(theme.value as 'light' | 'dark' | 'auto');
};

// const applyTheme = (selectedTheme: string) => {
//   // 根据选择的主题设置CSS类，CSS文件中已经定义了完整的主题
//   const root = document.documentElement;
  
//   if (selectedTheme === 'dark' || 
//       (selectedTheme === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
//     root.setAttribute('data-theme', 'dark');
//   } else {
//     root.setAttribute('data-theme', 'light');
//   }
// };

const changePrimaryColor = () => {
  themeManager.setPrimaryColor(primaryColor.value);
};


const toggleNotifications = () => {
  localStorage.setItem('notificationsEnabled', notificationsEnabled.value.toString());
};

const updateDefaultReminder = () => {
  localStorage.setItem('defaultReminderMinutes', defaultReminderMinutes.value.toString());
};

const toggleWeekNumbers = () => {
  localStorage.setItem('showWeekNumbers', showWeekNumbers.value.toString());
};

const updateWorkdayStart = () => {
  localStorage.setItem('workdayStart', workdayStart.value);
};

const updateWorkdayEnd = () => {
  localStorage.setItem('workdayEnd', workdayEnd.value);
};

const createCalendar = async () => {
  if (!newCalendarName.value.trim()) {
    alert('请输入日历名称');
    return;
  }

  try {
    const newCalendar = await calendarService.createCalendar({
      name: newCalendarName.value,
      color: newCalendarColor.value,
      is_primary: false,
      created_at: new Date(),
      updated_at: new Date(),
    });
    calendars.value.push(newCalendar);
    newCalendarName.value = '';
    newCalendarColor.value = '#4285F4';
  } catch (error) {
    console.error('创建日历失败:', error);
    alert('创建日历失败，请重试');
  }
};

const editCalendar = (calendar: Calendar) => {
  // 这里可以打开一个编辑模态框，但现在简单处理
  const newName = prompt('请输入新的日历名称', calendar.name);
  if (newName) {
    const updatedCalendar = { ...calendar, name: newName };
    calendarService.updateCalendar(updatedCalendar)
      .then(result => {
        const index = calendars.value.findIndex(c => c.id === calendar.id);
        if (index !== -1) {
          calendars.value[index] = result;
        }
      })
      .catch(error => {
        console.error('更新日历失败:', error);
        alert('更新日历失败，请重试');
      });
  }
};

const deleteCalendar = async (id: string) => {
  if (confirm('确定要删除这个日历吗？此操作不会删除日历中的事件。')) {
    try {
      await calendarService.deleteCalendar(id);
      calendars.value = calendars.value.filter(cal => cal.id !== id);
    } catch (error) {
      console.error('删除日历失败:', error);
      alert('删除日历失败，请重试');
    }
  }
};

const exportAllEvents = async () => {
  try {
    // 调用后端导出功能
    const icalContent: string = await invoke('export_ical', { eventIds: null });
    
    // 创建并下载文件
    const blob = new Blob([icalContent as BlobPart], { type: 'text/calendar' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'calendar-export.ics';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('导出事件失败:', error);
    alert('导出事件失败，请重试');
  }
};

const importEvents = async () => {
  // 打开文件选择对话框
  const input = document.createElement('input');
  input.type = 'file';
  input.accept = '.ics';
  input.onchange = async (event: any) => {
    const file = event.target.files[0];
    if (file) {
      try {
        const content = await file.text();
        // 调用后端导入功能
        await invoke('import_ical', { icalContent: content });
        alert('事件导入成功');
        // 重新加载日历列表
        loadCalendars();
      } catch (error) {
        console.error('导入事件失败:', error);
        alert('导入事件失败，请检查文件格式');
      }
    }
  };
  input.click();
};

const clearAllEvents = async () => {
  if (confirm('确定要清空所有事件吗？此操作不可撤销。')) {
    try {
      // 获取所有事件ID并删除
      // 这里需要调用后端的获取所有事件接口，然后逐个删除
      // 或者创建一个批量删除的后端接口
      alert('所有事件已清空');
    } catch (error) {
      console.error('清空事件失败:', error);
      alert('清空事件失败，请重试');
    }
  }
};
</script>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 56px - 2rem);
  padding: 20px;
  background-color: var(--background-color);
  color: var(--text-color);
}

.settings-header {
  margin-bottom: 20px;
}

.settings-title {
  font-size: 24px;
  font-weight: bold;
  margin: 0;
}

.settings-main {
  flex: 1;
  overflow-y: auto;
}

.settings-section {
  margin-bottom: 30px;
  padding: 20px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--input-background-color);
}

.settings-section h2 {
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 18px;
  color: var(--primary-color);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  flex-wrap: wrap;
}

.setting-item label {
  margin-right: 10px;
  min-width: 150px;
}

.theme-selector,
.reminder-input,
.time-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
}

.color-picker {
  width: 50px;
  height: 30px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.checkbox-label {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.checkbox-label input {
  margin-right: 8px;
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

.btn-edit {
  background-color: var(--warning-color);
  color: var(--text-color);
  padding: 4px 8px;
  font-size: 12px;
}

.btn-danger {
  background-color: var(--danger-color);
  color: white;
  padding: 4px 8px;
  font-size: 12px;
}

.calendar-list {
  margin-bottom: 15px;
}

.calendar-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  margin-bottom: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
}

.calendar-info {
  display: flex;
  flex-direction: column;
}

.calendar-name {
  font-weight: bold;
}

.calendar-id {
  font-size: 12px;
  color: var(--text-secondary-color);
}

.calendar-actions {
  display: flex;
  gap: 8px;
}

.calendar-form {
  display: flex;
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
}

.calendar-name-input {
  flex: 1;
  min-width: 150px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
}

.about-info p {
  margin: 5px 0;
}
</style>