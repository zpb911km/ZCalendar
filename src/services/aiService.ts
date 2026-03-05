import { AIConfig, AIResponse, ChatMessage } from '@/types/ai';

class AIService {
  private config: AIConfig = {
    apiKey: '',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    model: 'qwen3.5-plus', // 默认使用通义千问视觉大模型
  };

  constructor() {
    this.loadConfig();
  }

  // 加载配置
  private loadConfig(): void {
    const savedConfig = localStorage.getItem('aiConfig');
    if (savedConfig) {
      try {
        this.config = { ...this.config, ...JSON.parse(savedConfig) };
      } catch (e) {
        console.error('Failed to load AI config:', e);
      }
    }
  }

  // 保存配置
  saveConfig(config: Partial<AIConfig>): void {
    this.config = { ...this.config, ...config };
    localStorage.setItem('aiConfig', JSON.stringify(this.config));
  }

  // 获取配置
  getConfig(): AIConfig {
    return { ...this.config };
  }

  // 检查配置是否完整
  isConfigured(): boolean {
    return !!this.config.apiKey;
  }

  // 获取系统提示词
  private getSystemPrompt(): string {
    return `# Role
你是一个课程表信息提取助手，输出标准 ICS (iCalendar) 格式。

# Task
从课程表图片中提取所有课程和练习信息，生成 ICS 日历文件。

# 重要规则

## 1. 事件分类
- **课程事件**：正常的上课时间
- **练习事件**：图片中标注的练习题、作业，作为独立事件

## 2. SUMMARY 字段（标题）
只能使用以下 2 字标题：
- "数学" - 考研数学课程
- "政治" - 考研政治课程  
- "英语" - 考研英语课程
- "408" - 计算机 408 课程
- "练习" - 所有科目的练习题
- "xx" - 所有其他课程,可以就是"其他",也可以用两字简称

## 3. DESCRIPTION 字段（详情）
格式严格为 3 行，**使用 "\\n" 表示换行**（写出反斜杠和 n 两个字符）：

"""
DESCRIPTION:课程全名\\n教师：姓名\\n地点：xxx
"""

示例：
"""
DESCRIPTION:线代 21-二次型的标准化与规范化\\n教师：付尧\\n地点：线上直播
"""

⚠️ 注意："\\n" 必须写在同一行，不要真正换行！

## 4. CATEGORIES 字段
格式："科目，类型"
- 课程："数学，课程" "政治，课程"
- 练习："数学，练习" "政治，练习"

## 5. 时间处理
- 使用 24 小时制
- 日期格式：YYYYMMDDTHHMMSS（例如：20260304T090000）
- 根据图片中的星期推算具体日期（2026 年 3 月）

## 6. 提醒设置
- 课程：提前 5 分钟提醒（TRIGGER:-PT5M）
- 练习：准时提醒（TRIGGER:PT0S）

## 7. 必填字段
整个 ICS 必须包含以下字段：
- VERSION
- PRODID
- BEGIN:VCALENDAR
- END:VCALENDAR
每个 VEVENT 必须包含：
- SUMMARY
- DESCRIPTION
- DTSTART
- DTEND
- CATEGORIES
- PRIORITY
- VALARM

# ICS 模板

BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//ZCalendar//EN
BEGIN:VEVENT
SUMMARY:数学
DESCRIPTION:线代 21-二次型的标准化与规范化\\n教师：付尧\\n地点：线上直播
DTSTART:20260304T090000
DTEND:20260304T113000
CATEGORIES:数学，课程
PRIORITY:1
BEGIN:VALARM
ACTION:DISPLAY
DESCRIPTION:提醒
TRIGGER:-PT5M
END:VALARM
END:VEVENT
BEGIN:VEVENT
SUMMARY:练习
DESCRIPTION:线代数第五章 17 题\\n教师：-\\n地点：自习
DTSTART:20260310T200000
DTEND:20260310T210000
CATEGORIES:数学，练习
PRIORITY:5
BEGIN:VALARM
ACTION:DISPLAY
DESCRIPTION:提醒
TRIGGER:-PT5M
END:VALARM
END:VEVENT
END:VCALENDAR

# 输出要求
0. 对于信息不足以生成日历的情况, 要明确拒绝, 并询问缺失的信息
1. 先以你喜欢的格式简单列出全部的课程,然后再在代码块中写ics文件
2. 每个课程和练习都是独立的 VEVENT
3. 按时间顺序排列
4. 确保日期推算正确
5. 练习如果没有明确时间，安排在当天晚上 20:00-21:00
6. DESCRIPTION 中的 "\\n" 要显式写出，不要真正换行

当前时间: ${new Date().toLocaleString()}

现在请等待用户输入
`;
  }

  // 发送消息到 AI
  async sendMessage(messages: ChatMessage[]): Promise<AIResponse> {
    if (!this.isConfigured()) {
      throw new Error('AI 配置不完整，请先设置 API Key');
    }

    // 添加系统提示词
    const systemMessage: ChatMessage = {
      id: 'system',
      role: 'system',
      content: this.getSystemPrompt(),
      timestamp: Date.now(),
    };

    const allMessages = [systemMessage, ...messages];

    // 构建请求体
    const requestBody = {
      model: this.config.model,
      messages: allMessages.map(msg => {
        const message: any = {
          role: msg.role,
          content: msg.content,
        };

        // 如果有图片，添加到 content 中（多模态）
        if (msg.image) {
          message.content = [
            {
              type: 'text',
              text: msg.content,
            },
            {
              type: 'image_url',
              image_url: {
                url: msg.image,
              },
            },
          ];
        }

        return message;
      }),
    };

    try {
      console.log('Sending message to AI:', requestBody);
      const response = await fetch(`${this.config.baseUrl}/chat/completions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${this.config.apiKey}`,
        },
        body: JSON.stringify(requestBody),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`AI API 请求失败: ${response.status} - ${errorText}`);
      }

      const data = await response.json();

      if (!data.choices || !data.choices[0] || !data.choices[0].message) {
        throw new Error('AI API 返回格式错误');
      }

      return {
        content: data.choices[0].message.content,
        usage: data.usage
          ? {
              promptTokens: data.usage.prompt_tokens,
              completionTokens: data.usage.completion_tokens,
              totalTokens: data.usage.total_tokens,
            }
          : undefined,
      };
    } catch (error) {
      console.error('AI Service Error:', error);
      throw error;
    }
  }
}

// 导出单例
export const aiService = new AIService();
