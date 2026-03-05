<template>
  <div class="ai-chat-view">
    <div class="chat-header">
      <h2>AI 助手</h2>
      <div class="header-actions">
        <button @click="clearChat" class="btn btn-secondary">清空对话</button>
      </div>
    </div>

    <!-- 未配置提示 -->
    <div v-if="!aiStore.isConfigured" class="config-warning">
      <p>请先在设置中配置 AI API Key</p>
      <button @click="goToSettings" class="btn btn-primary">去设置</button>
    </div>

    <!-- 聊天区域 -->
    <div v-else class="chat-container">
      <div class="messages" ref="messagesContainer">
        <!-- 欢迎消息 -->
        <div v-if="aiStore.messages.length === 0" class="welcome-message">
          <h3>欢迎使用 AI 日历助手</h3>
          <p>我可以帮你：</p>
          <ul>
            <li>从图片中识别日程信息</li>
            <li>将日程信息转换为 iCalendar 格式</li>
            <li>自动导入生成的日历事件</li>
          </ul>
          <p>上传一张日程图片或直接描述你的日程开始吧！</p>
        </div>

        <!-- 消息列表 -->
        <div
          v-for="message in aiStore.messages"
          :key="message.id"
          :class="['message', message.role]"
        >
          <div class="message-content">
            <div v-if="message.image" class="message-image">
              <img :src="message.image" alt="上传的图片" />
            </div>
            <div class="message-text">{{ message.content }}</div>
            <div v-if="message.role === 'assistant'" class="message-actions">
              <button @click="copyMessage(message.content)" class="action-btn">
                复制
              </button>
              <button
                v-if="containsIcalFormat(message.content)"
                @click="importFromMessage(message.content)"
                class="action-btn btn-primary"
              >
                导入日历
              </button>
            </div>
          </div>
        </div>

        <!-- 加载中 -->
        <div v-if="aiStore.isLoading" class="message assistant loading">
          <div class="message-content">
            <div class="loading-dots">
              <span></span>
              <span></span>
              <span></span>
            </div>
          </div>
        </div>

        <!-- 错误消息 -->
        <div v-if="aiStore.error" class="error-message">
          {{ aiStore.error }}
        </div>
      </div>

      <!-- 输入区域 -->
      <div class="input-area">
        <!-- 图片预览 -->
        <div v-if="selectedImage" class="image-preview">
          <img :src="selectedImage" alt="预览" />
          <button @click="removeImage" class="remove-image">×</button>
        </div>

        <div class="input-row">
          <div class="input-wrapper">
            <button @click="triggerFileInput" class="icon-btn" title="上传图片">
              📷
            </button>
            <input
              ref="fileInput"
              type="file"
              accept="image/*"
              @change="handleFileChange"
              style="display: none"
            />
            <textarea
              v-model="inputText"
              placeholder="描述你的日程或直接提问..."
              @keydown="handleKeyDown"
              ref="inputTextarea"
            ></textarea>
          </div>
          <button
            @click="sendMessage"
            :disabled="!canSend"
            class="btn btn-primary send-btn"
          >
            发送
          </button>
        </div>
      </div>
    </div>

    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useAIStore } from '@/stores/aiStore';
import { invoke } from '@tauri-apps/api/core';

const router = useRouter();
const aiStore = useAIStore();

// 输入相关
const inputText = ref('');
const selectedImage = ref<string | null>(null);
const fileInput = ref<HTMLInputElement | null>(null);

// 引用
const messagesContainer = ref<HTMLElement | null>(null);
const inputTextarea = ref<HTMLTextAreaElement | null>(null);

// 是否可以发送
const canSend = computed(() => {
  return (inputText.value.trim() || selectedImage.value) && !aiStore.isLoading;
});

// 检查是否包含 iCalendar 格式
function containsIcalFormat(content: string): boolean {
  return (
    content.includes('BEGIN:VCALENDAR') &&
    content.includes('END:VCALENDAR') &&
    content.includes('BEGIN:VEVENT')
  );
}

// 处理文件选择
function triggerFileInput() {
  fileInput.value?.click();
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = e => {
      selectedImage.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
}

function removeImage() {
  selectedImage.value = null;
  if (fileInput.value) {
    fileInput.value.value = '';
  }
}

// 发送消息
async function sendMessage() {
  if (!canSend.value) return;

  const text = inputText.value.trim();
  const image = selectedImage.value;

  // 清空输入
  inputText.value = '';
  selectedImage.value = null;
  if (fileInput.value) {
    fileInput.value.value = '';
  }

  // 发送消息
  await aiStore.sendMessage(text, image || undefined);

  // 保存到本地存储
  aiStore.saveToStorage();

  // 滚动到底部
  scrollToBottom();
}

// 处理键盘事件
function handleKeyDown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  }
}

// 滚动到底部
function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

// 复制消息
async function copyMessage(content: string) {
  try {
    await navigator.clipboard.writeText(content);
    // 这里可以添加一个提示
  } catch (e) {
    console.error('Copy failed:', e);
  }
}

// 从消息导入
function importFromMessage(content: string) {
  // 预处理模型回复: 挑出被```ics ```包裹的内容
  let importContent = content;
  const match = content.match(/```ics([\s\S]*?)```/);
  if (match) {
    importContent = match[1].trim();
  }
  // 保存到 localStorage，供设置页面使用
  localStorage.setItem('pendingImportText', importContent);
  // 跳转到设置页面
  router.push('/settings');
}

// 清空对话
function clearChat() {
  if (confirm('确定要清空所有对话吗？')) {
    aiStore.clearMessages();
    aiStore.saveToStorage();
  }
}

// 跳转到设置
function goToSettings() {
  router.push('/settings');
}

// 监听消息变化
watch(
  () => aiStore.messages,
  () => {
    scrollToBottom();
  },
  { deep: true }
);

// 自动调整输入框高度
watch(inputText, () => {
  if (inputTextarea.value) {
    inputTextarea.value.style.height = 'auto';
    inputTextarea.value.style.height = `${inputTextarea.value.scrollHeight}px`;
  }
});

// 初始化
onMounted(() => {
  scrollToBottom();
});
</script>

<style scoped>
.ai-chat-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--background-color);
  color: var(--text-color);
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid var(--border-color);
  background-color: var(--card-background-color);
}

.chat-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 12px;
}

.config-warning {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px;
  gap: 16px;
}

.chat-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.messages {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.welcome-message {
  text-align: center;
  padding: 32px;
  background-color: var(--card-background-color);
  border-radius: 12px;
  border: 1px solid var(--border-color);
}

.welcome-message h3 {
  margin: 0 0 16px 0;
  color: var(--primary-color);
}

.welcome-message ul {
  text-align: left;
  margin: 16px 0;
  padding-left: 24px;
}

.welcome-message li {
  margin: 8px 0;
}

.message {
  display: flex;
  gap: 12px;
  max-width: 80%;
}

.message.user {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.message.assistant {
  align-self: flex-start;
}

.message-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 12px;
  background-color: var(--card-background-color);
  border: 1px solid var(--border-color);
}

.message.user .message-content {
  background-color: var(--primary-color);
  color: white;
  border: none;
}

.message-image img {
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
}

.message-text {
  white-space: pre-wrap;
  word-wrap: break-word;
  line-height: 1.6;
}

.message-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 4px 12px;
  font-size: 12px;
  border: 1px solid var(--border-color);
  background-color: var(--input-background-color);
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-color);
}

.action-btn:hover {
  background-color: var(--hover-color);
}

.action-btn.btn-primary {
  background-color: var(--primary-color);
  color: white;
  border: none;
}

.message.loading .message-content {
  padding: 16px 24px;
}

.loading-dots {
  display: flex;
  gap: 4px;
}

.loading-dots span {
  width: 8px;
  height: 8px;
  background-color: var(--text-color);
  border-radius: 50%;
  animation: bounce 1.4s infinite ease-in-out;
}

.loading-dots span:nth-child(1) {
  animation-delay: -0.32s;
}

.loading-dots span:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%,
  80%,
  100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

.error-message {
  padding: 12px 16px;
  background-color: rgba(244, 67, 54, 0.1);
  color: #f44336;
  border-radius: 8px;
  text-align: center;
}

.input-area {
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
  background-color: var(--card-background-color);
}

.image-preview {
  display: inline-block;
  position: relative;
  margin-bottom: 12px;
}

.image-preview img {
  max-height: 120px;
  border-radius: 8px;
}

.remove-image {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: none;
  background-color: #f44336;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
}

.input-row {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}

.input-wrapper {
  flex: 1;
  display: flex;
  gap: 8px;
  align-items: flex-end;
}

.icon-btn {
  width: 40px;
  height: 40px;
  border: 1px solid var(--border-color);
  background-color: var(--input-background-color);
  border-radius: 8px;
  cursor: pointer;
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-color);
}

.icon-btn:hover {
  background-color: var(--hover-color);
}

textarea {
  flex: 1;
  min-height: 40px;
  max-height: 120px;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--input-background-color);
  color: var(--text-color);
  resize: none;
  font-family: inherit;
}

textarea:focus {
  outline: none;
  border-color: var(--primary-color);
}

.send-btn {
  height: 40px;
  padding: 0 20px;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
