import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { ChatMessage } from '@/types/ai';
import { aiService } from '@/services/aiService';

export const useAIStore = defineStore('ai', () => {
  // 对话历史
  const messages = ref<ChatMessage[]>([]);
  
  // 加载状态
  const isLoading = ref(false);
  
  // 错误信息
  const error = ref<string | null>(null);

  // 是否已配置
  const isConfigured = computed(() => aiService.isConfigured());

  // 添加用户消息
  function addUserMessage(content: string, image?: string): void {
    const message: ChatMessage = {
      id: `msg-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      role: 'user',
      content,
      image,
      timestamp: Date.now(),
    };
    messages.value.push(message);
  }

  // 添加助手消息
  function addAssistantMessage(content: string): void {
    const message: ChatMessage = {
      id: `msg-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      role: 'assistant',
      content,
      timestamp: Date.now(),
    };
    messages.value.push(message);
  }

  // 发送消息
  async function sendMessage(content: string, image?: string): Promise<void> {
    if (!content.trim() && !image) {
      return;
    }

    error.value = null;
    isLoading.value = true;

    try {
      // 添加用户消息
      addUserMessage(content, image);

      // 调用 AI 服务
      const response = await aiService.sendMessage(messages.value);

      // 添加助手回复
      addAssistantMessage(response.content);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '发送消息失败';
      console.error('Send message error:', err);
    } finally {
      isLoading.value = false;
    }
  }

  // 清空对话
  function clearMessages(): void {
    messages.value = [];
    error.value = null;
  }

  // 删除最后一条消息
  function removeLastMessage(): void {
    if (messages.value.length > 0) {
      messages.value.pop();
    }
  }

  // 重新生成最后一条助手消息
  async function regenerateLastMessage(): Promise<void> {
    // 移除最后一条助手消息
    const lastMessage = messages.value[messages.value.length - 1];
    if (lastMessage && lastMessage.role === 'assistant') {
      messages.value.pop();
    }

    // 获取除最后一条助手消息外的所有消息
    const messagesToSend = messages.value.filter(msg => msg.role !== 'assistant');

    if (messagesToSend.length === 0) {
      return;
    }

    error.value = null;
    isLoading.value = true;

    try {
      // 调用 AI 服务
      const response = await aiService.sendMessage(messagesToSend);

      // 添加助手回复
      addAssistantMessage(response.content);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重新生成失败';
      console.error('Regenerate message error:', err);
    } finally {
      isLoading.value = false;
    }
  }

  // 从本地存储加载对话历史
  function loadFromStorage(): void {
    const saved = localStorage.getItem('aiMessages');
    if (saved) {
      try {
        messages.value = JSON.parse(saved);
      } catch (e) {
        console.error('Failed to load AI messages:', e);
      }
    }
  }

  // 保存对话历史到本地存储
  function saveToStorage(): void {
    localStorage.setItem('aiMessages', JSON.stringify(messages.value));
  }

  // 监听消息变化，自动保存
  messages.value = messages.value || [];
  loadFromStorage();

  return {
    messages,
    isLoading,
    error,
    isConfigured,
    sendMessage,
    clearMessages,
    removeLastMessage,
    regenerateLastMessage,
    saveToStorage,
  };
});