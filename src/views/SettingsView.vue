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
          <CustomSelect
            v-model="theme"
            @change="changeTheme"
            :options="[
              { value: 'light', label: '浅色模式' },
              { value: 'dark', label: '深色模式' },
              { value: 'auto', label: '自动' },
            ]"
          />
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

      <div class="settings-section">
        <h2>数据管理</h2>
        <div class="setting-item">
          <label>数据库连接</label>
          <div class="db-config">
            <input
              v-model="dbUrl"
              type="text"
              placeholder="mysql://user:password@host:port/database"
              class="db-input"
            />
            <button type="button" class="test-btn" @click="testDbConnection">
              测试连接
            </button>
            <button type="button" class="save-btn" @click="saveDbConfig">
              保存配置
            </button>
          </div>
          <div v-if="dbTestResult" class="test-result" :class="dbTestStatus">
            {{ dbTestResult }}
          </div>
          <p class="hint">
            格式: mysql://username:password@host:port/database
          </p>
        </div>
        <div class="setting-item">
          <label>时区设置</label>
          <CustomSelect
            v-model="timezoneOffset"
            @change="updateTimezone"
            :options="[
              { value: '-12', label: 'UTC-12' },
              { value: '-11', label: 'UTC-11' },
              { value: '-10', label: 'UTC-10' },
              { value: '-9', label: 'UTC-9' },
              { value: '-8', label: 'UTC-8' },
              { value: '-7', label: 'UTC-7' },
              { value: '-6', label: 'UTC-6' },
              { value: '-5', label: 'UTC-5' },
              { value: '-4', label: 'UTC-4' },
              { value: '-3', label: 'UTC-3' },
              { value: '-2', label: 'UTC-2' },
              { value: '-1', label: 'UTC-1' },
              { value: '0', label: 'UTC+0' },
              { value: '1', label: 'UTC+1' },
              { value: '2', label: 'UTC+2' },
              { value: '3', label: 'UTC+3' },
              { value: '4', label: 'UTC+4' },
              { value: '5', label: 'UTC+5' },
              { value: '6', label: 'UTC+6' },
              { value: '7', label: 'UTC+7' },
              { value: '8', label: 'UTC+8' },
              { value: '9', label: 'UTC+9' },
              { value: '10', label: 'UTC+10' },
              { value: '11', label: 'UTC+11' },
              { value: '12', label: 'UTC+12' },
            ]"
          />
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
          <p><strong>应用版本:</strong> 0.1.0</p>
          <p><strong>构建日期:</strong> 2026-03-04</p>
          <a
            href="https://github.com/zpb911km/ZCalendar"
            target="_blank"
            rel="noopener noreferrer"
            class="github-link"
            style="
              text-decoration: underline;
              color: var(--text-color);
              padding: 5px;
              border: 1px solid var(--border-color);
            "
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="currentColor"
              style="vertical-align: middle; margin-right: 4px"
            >
              <path
                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
              />
            </svg>
            GitHub
          </a>
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
import router from '@/router';
import { useLoading } from '@/composables/useLoading';
import CustomSelect from '@/components/CustomSelect.vue';

// 设置状态
const theme = ref('auto');
const primaryColor = ref('#007bff');
const defaultReminderMinutes = ref(15);
const showWeekNumbers = ref(false);
const workdayStart = ref('09:00');
const workdayEnd = ref('18:00');
const timezoneOffset = ref('8'); // 默认UTC+8
const showNotifacationWarning = ref(true);

// 数据库配置
const dbUrl = ref('');
const dbTestResult = ref('');
const dbTestStatus = ref<'idle' | 'testing' | 'success' | 'error'>('idle');

// 日历管理状态
const calendars = ref<Calendar[]>([]);

onMounted(() => {
  // 加载保存的设置
  loadSettings();
  // 加载日历列表
  loadCalendars();
  // 加载数据库配置
  loadDbConfig();
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

// 数据库配置相关函数
const loadDbConfig = async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const config = await invoke<string | null>('get_db_config');
    dbUrl.value = config || '';
  } catch (error) {
    console.error('加载数据库配置失败:', error);
  }
};

const testDbConnection = async () => {
  if (!dbUrl.value.trim()) {
    dbTestResult.value = '请输入数据库连接字符串';
    dbTestStatus.value = 'error';
    return;
  }

  dbTestStatus.value = 'testing';
  dbTestResult.value = '正在测试连接...';

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const result = await invoke<string>('test_db_connection', { dbUrl: dbUrl.value });
    dbTestResult.value = result;
    dbTestStatus.value = 'success';
  } catch (error: any) {
    dbTestResult.value = error || '连接测试失败';
    dbTestStatus.value = 'error';
  }
};

const saveDbConfig = async () => {
  const { showLoading, hideLoading } = useLoading();
  try {
    showLoading();
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('save_db_config', { dbUrl: dbUrl.value });
    alert('数据库配置已保存，应用将重新连接数据库');
    dbTestResult.value = '';
    dbTestStatus.value = 'idle';
  } catch (error) {
    console.error('保存数据库配置失败:', error);
    alert('保存数据库配置失败');
  } finally {
    hideLoading();
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

const exportAllEvents = async () => {
  const { showLoading, hideLoading } = useLoading();
  try {
    showLoading();
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
  } finally {
    hideLoading();
  }
};

const importEvents = async () => {
  const { showLoading, hideLoading } = useLoading();
  // 打开文件选择对话框
  const input = document.createElement('input');
  input.type = 'file';
  input.onchange = async (event: any) => {
    const file = event.target.files[0];
    if (file) {
      try {
        showLoading();
        const content = await file.text();
        // 获取当前时区设置并传递给后端导入功能
        const timezoneOffsetValue =
          localStorage.getItem('timezoneOffset') || '8';
        console.log(content);
        console.log(timezoneOffsetValue);
        // return;
        const newevents: Event[] = await invoke('import_ical', {
          icalContent: content,
          timezoneOffset: parseInt(timezoneOffsetValue),
        });
        await invoke('send_notification', {
          title: '日历已导入',
          body: `日历导入成功, 共${newevents.length}条`,
        });
        // 重新加载日历列表
        loadCalendars();
        router.push('/');
      } catch (error) {
        console.error('导入事件失败:', error);
        alert('导入事件失败，请检查文件格式');
      } finally {
        hideLoading();
      }
    }
  };
  input.click();
};

const clearAllEvents = async () => {
  if (confirm('确定要清空所有事件吗？此操作不可撤销。')) {
    const { showLoading, hideLoading } = useLoading();
    try {
      showLoading();
      await invoke('delete_all_events');
      alert('所有事件已清空');
    } catch (error) {
      console.error('清空事件失败:', error);
      alert('清空事件失败，请重试');
    } finally {
      hideLoading();
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

.db-config {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.db-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--input-background-color);
  color: var(--text-color);
}

.test-btn,
.save-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  white-space: nowrap;
}

.test-btn {
  background-color: var(--secondary-color);
  color: var(--text-color);
}

.save-btn {
  background-color: var(--primary-color);
  color: white;
}

.test-result {
  margin-top: 8px;
  padding: 8px;
  border-radius: 4px;
  font-size: 14px;
}

.test-result.testing {
  background-color: rgba(255, 193, 7, 0.1);
  color: #ffc107;
}

.test-result.success {
  background-color: rgba(76, 175, 80, 0.1);
  color: #4caf50;
}

.test-result.error {
  background-color: rgba(244, 67, 54, 0.1);
  color: #f44336;
}

.hint {
  margin-top: 4px;
  font-size: 12px;
  color: var(--text-color);
  opacity: 0.6;
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
