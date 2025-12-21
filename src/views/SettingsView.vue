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
        <div v-if="showNotifacationWarning" class="setting-item">
          <div class="note">
            <p>请在系统设置处开启通知权限</p>
            <p>并自定义通知的提醒方式。</p>
            <!-- Android通知配置说明：
                1. 通知样式、提示音和悬浮效果通过Android原生通知渠道实现
                2. 权限配置在AndroidManifest.xml中自动处理
                3. 高优先级通知可实现悬浮效果
                4. 提示音和震动通过Android通知渠道配置实现 -->
            <button
              style="
                background-color: transparent;
                border: 1px solid var(--border-color);
                color: var(--text-color);
              "
              @click="changeShowNotifacationWarning"
            >
              我知道了
            </button>
          </div>
        </div>
      </div>

      <!-- <div class="settings-section">
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
      </div> -->

      <!-- TODO -->
      <!-- 日历目前无意义,其他地方也没有用到,有时间再优化 -->
      <!-- <div class="settings-section">
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
        
      </div> -->

      <div class="settings-section">
        <h2>数据管理</h2>
        <div class="setting-item">
          <label>时区设置</label>
          <select
            v-model="timezoneOffset"
            @change="updateTimezone"
            class="timezone-selector"
          >
            <option value="-12">UTC-12</option>
            <option value="-11">UTC-11</option>
            <option value="-10">UTC-10</option>
            <option value="-9">UTC-9</option>
            <option value="-8">UTC-8</option>
            <option value="-7">UTC-7</option>
            <option value="-6">UTC-6</option>
            <option value="-5">UTC-5</option>
            <option value="-4">UTC-4</option>
            <option value="-3">UTC-3</option>
            <option value="-2">UTC-2</option>
            <option value="-1">UTC-1</option>
            <option value="0">UTC+0</option>
            <option value="1">UTC+1</option>
            <option value="2">UTC+2</option>
            <option value="3">UTC+3</option>
            <option value="4">UTC+4</option>
            <option value="5">UTC+5</option>
            <option value="6">UTC+6</option>
            <option value="7">UTC+7</option>
            <option value="8" selected>UTC+8</option>
            <option value="9">UTC+9</option>
            <option value="10">UTC+10</option>
            <option value="11">UTC+11</option>
            <option value="12">UTC+12</option>
          </select>
        </div>
        <div class="setting-item">
          <button @click="exportAllEvents" class="btn btn-secondary">
            导出所有事件
          </button>
        </div>
        <div class="setting-item">
          <button @click="importEvents" class="btn btn-secondary">
            导入事件
          </button>
        </div>
        <div class="setting-item">
          <button @click="clearAllEvents" class="btn btn-danger">
            清空所有事件
          </button>
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
const defaultReminderMinutes = ref(15);
const showWeekNumbers = ref(false);
const workdayStart = ref('09:00');
const workdayEnd = ref('18:00');
const timezoneOffset = ref('8'); // 默认UTC+8
const buildDate = ref(new Date().toISOString().split('T')[0]);
const showNotifacationWarning = ref(true);

// 日历管理状态
const calendars = ref<Calendar[]>([]);
// const newCalendarName = ref('');
// const newCalendarColor = ref('#4285F4');

onMounted(() => {
  // 加载保存的设置
  loadSettings();
  // 加载日历列表
  loadCalendars();
});

const loadSettings = () => {
  const settings = themeManager.getSettings();
  const savedDefaultReminder = parseInt(
    localStorage.getItem('defaultReminderMinutes') || '15'
  );
  const savedShowWeekNumbers =
    localStorage.getItem('showWeekNumbers') === 'true';
  const savedWorkdayStart = localStorage.getItem('workdayStart') || '09:00';
  const savedWorkdayEnd = localStorage.getItem('workdayEnd') || '18:00';
  const savedTimezoneOffset = localStorage.getItem('timezoneOffset') || '8';
  const showNotifacationWarningValue =
    localStorage.getItem('showNotifacationWarning') !== 'false';

  showNotifacationWarning.value = showNotifacationWarningValue;
  theme.value = settings.theme;
  primaryColor.value = settings.primaryColor;
  defaultReminderMinutes.value = savedDefaultReminder;
  showWeekNumbers.value = savedShowWeekNumbers;
  workdayStart.value = savedWorkdayStart;
  workdayEnd.value = savedWorkdayEnd;
  timezoneOffset.value = savedTimezoneOffset;
};

const changeShowNotifacationWarning = () => {
  showNotifacationWarning.value = false;
  localStorage.setItem('showNotifacationWarning', 'false');
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

const changePrimaryColor = () => {
  themeManager.setPrimaryColor(primaryColor.value);
};

const updateDefaultReminder = () => {
  localStorage.setItem(
    'defaultReminderMinutes',
    defaultReminderMinutes.value.toString()
  );
};

const updateTimezone = () => {
  localStorage.setItem('timezoneOffset', timezoneOffset.value);
};

// const createCalendar = async () => {
//   if (!newCalendarName.value.trim()) {
//     alert('请输入日历名称');
//     return;
//   }

//   try {
//     const newCalendar = await calendarService.createCalendar({
//       id: '',
//       name: newCalendarName.value,
//       color: newCalendarColor.value,
//       is_primary: false,
//       created_at: new Date(),
//       updated_at: new Date()
//     });
//     calendars.value.push(newCalendar);
//     newCalendarName.value = '';
//     newCalendarColor.value = '#4285F4';
//   } catch (error) {
//     console.error('创建日历失败:', error);
//     alert('创建日历失败，请重试');
//   }
// };

// const editCalendar = (calendar: Calendar) => {
//   // 这里可以打开一个编辑模态框，但现在简单处理
//   const newName = prompt('请输入新的日历名称', calendar.name);
//   if (newName) {
//     const updatedCalendar = { ...calendar, name: newName };
//     calendarService.updateCalendar(updatedCalendar)
//       .then(result => {
//         const index = calendars.value.findIndex(c => c.id === calendar.id);
//         if (index !== -1) {
//           calendars.value[index] = result;
//         }
//       })
//       .catch(error => {
//         console.error('更新日历失败:', error);
//         alert('更新日历失败，请重试');
//       });
//   }
// };

// const deleteCalendar = async (id: string) => {
//   if (confirm('确定要删除这个日历吗？此操作不会删除日历中的事件。')) {
//     try {
//       await calendarService.deleteCalendar(id);
//       calendars.value = calendars.value.filter(cal => cal.id !== id);
//     } catch (error) {
//       console.error('删除日历失败:', error);
//       alert('删除日历失败，请重试');
//     }
//   }
// };

const exportAllEvents = async () => {
  try {
    // 获取当前时区设置
    const timezoneOffsetValue = localStorage.getItem('timezoneOffset') || '8';
    // 调用后端导出功能，传递时区信息
    const icalContent: string = await invoke('export_ical', {
      eventIds: null,
      timezoneOffset: parseInt(timezoneOffsetValue),
    });

    // 检测是否在Android平台上
    try {
      // 尝试使用app插件检测平台
      const platform = await invoke('get_platform');
      console.log(platform);
      if (platform === 'android') {
        // 在Android上使用Tauri命令保存文件到下载目录
        await invoke('save_file_to_downloads', {
          content: icalContent,
          filename: 'calendar-export.ics',
        });
        await invoke('send_notification', {
          title: '日历已导出',
          body: '日历导出成功,已保存到下载文件夹',
        });
      } else {
        // 桌面端保持原来的下载方式
        const blob = new Blob([icalContent as BlobPart], {
          type: 'text/calendar',
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'calendar-export.ics';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        await invoke('send_notification', {
          title: '日历已导出',
          body: '日历导出成功,请查看下载文件夹',
        });
      }
    } catch {
      // 如果无法获取平台信息，使用桌面端的下载方式
      const blob = new Blob([icalContent as BlobPart], {
        type: 'text/calendar',
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'calendar-export.ics';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      await invoke('send_notification', {
        title: '日历已导出',
        body: '日历导出成功,请查看下载文件夹',
      });
    }
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
        // 获取当前时区设置并传递给后端导入功能
        const timezoneOffsetValue =
          localStorage.getItem('timezoneOffset') || '8';
        console.log(content);
        console.log(timezoneOffsetValue);
        // return;
        await invoke('import_ical', {
          icalContent: content,
          timezoneOffset: parseInt(timezoneOffsetValue),
        });
        await invoke('send_notification', {
          title: '日历已导入',
          body: '日历导入成功',
        });
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
      invoke('delete_all_events');
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
  height: calc(100vh - 56px - 2rem - 40px);
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
.timezone-selector,
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

.note {
  color: var(--text-color);
  background-color: var(--warning-color);
  padding: 2px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
}
</style>
