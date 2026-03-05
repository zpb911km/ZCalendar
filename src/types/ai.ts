// AI 对话消息类型
export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  image?: string; // base64 编码的图片
  timestamp: number;
}

// AI 配置
export interface AIConfig {
  apiKey: string;
  baseUrl: string;
  model: string;
}

// AI 响应
export interface AIResponse {
  content: string;
  usage?: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}

// 导入事件结果
export interface ImportEventsResult {
  success: boolean;
  message: string;
  eventCount?: number;
}