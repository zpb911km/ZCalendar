# 日历应用项目架构设计

## 原始作业要求

【作业说明】
开发一个android手机上的日历应用

【具体要求】
1、技术实现要求：Tauri + Vue3 + TypeScript
2、核心功能：
参考rfc5545等文档实现一个日历App，实现基本要求的三个功能
基本要求：
    （1）日历视图展示（月视图、周视图、日视图）；
    （2）日程添加、编辑、查看和删除；
    （3）日程提醒功能的实现；(android notification)

扩展要求：
    （1）日历事件的导入导出；

## 项目架构设计

### 1. 整体架构
- **前端**: Vue3 + TypeScript + Vite
- **后端**: Rust + Tauri 2.0
- **数据库**: SQLite (通过sqlx)
- **标准**: RFC5545 (iCalendar)

### 2. 技术栈
- 前端：Vue3 + TypeScript + Vite
- 后端：Rust + Tauri 2.0
- 移动端：Tauri Android
- 日历标准：遵循RFC5545 (iCalendar)

### 3. 项目模块划分

#### 前端模块 (src/)
```
src/
├── components/           # 可复用UI组件
│   ├── Calendar/         # 日历核心组件
│   │   ├── CalendarView.vue    # 日历视图主组件
│   │   ├── MonthView.vue       # 月视图
│   │   ├── WeekView.vue        # 周视图
│   │   └── DayView.vue         # 日视图
│   ├── Event/            # 事件组件
│   │   ├── EventCard.vue       # 事件卡片
│   │   ├── EventEditor.vue     # 事件编辑器
│   │   └── EventDetail.vue     # 事件详情
│   └── Notification/     # 通知组件
│       └── NotificationManager.vue
├── composables/          # Vue组合式函数
│   ├── useCalendar.ts    # 日历逻辑
│   ├── useEvents.ts      # 事件操作
│   └── useNotifications.ts # 通知管理
├── services/             # 与后端交互的服务
│   ├── calendarService.ts    # 日历服务
│   ├── eventService.ts       # 事件服务
│   └── notificationService.ts # 通知服务
├── stores/               # 状态管理 (Pinia)
│   ├── calendarStore.ts      # 日历状态
│   └── eventStore.ts         # 事件状态
├── utils/                # 工具函数
│   ├── calendarUtils.ts      # 日历工具
│   ├── dateUtils.ts          # 日期处理
│   └── icalParser.ts         # RFC5545解析器
├── types/                # TypeScript类型定义
│   ├── calendar.ts           # 日历类型
│   └── event.ts              # 事件类型
└── views/                # 页面视图
    ├── CalendarView.vue      # 主日历页面
    ├── EventView.vue         # 事件详情页面
    └── SettingsView.vue      # 设置页面
```

#### 后端模块 (src-tauri/src/)
```
src-tauri/
└── src/
    ├── lib.rs              # Tauri命令导出
    ├── main.rs             # 应用入口
    ├── models/             # 数据模型
    │   ├── event.rs        # 事件模型
    │   ├── calendar.rs     # 日历模型
    │   └── reminder.rs     # 提醒模型
    ├── services/           # 业务逻辑服务
    │   ├── calendar_service.rs   # 日历服务
    │   ├── event_service.rs      # 事件服务
    │   ├── notification_service.rs # 通知服务
    │   └── ical_service.rs       # iCalendar服务
    ├── storage/            # 数据存储
    │   ├── database.rs     # 数据库操作
    │   └── ical_storage.rs # iCalendar导入导出
    └── utils/              # 工具函数
        ├── date_utils.rs   # 日期处理
        ├── ical_utils.rs   # RFC5545处理
        └── notification.rs # 通知工具
```

### 4. 数据库设计
使用Tauri的数据库插件（如Tauri ORM或SQLx），包含以下表：
- events表：存储日历事件（标题、描述、时间、重复规则等）
- reminders表：存储提醒信息
- calendars表：存储日历信息（用于多个日历支持）

### 5. 核心功能架构

#### (1) 日历视图展示
- 月视图：网格布局显示整月日期
- 周视图：水平布局显示一周时间线
- 日视图：垂直时间轴显示当天事件

#### (2) 日程管理
- 增删改查CRUD操作
- 支持重复事件（按RFC5545 RRULE规则）
- 支持事件分类和标签

#### (3) 通知提醒
- 使用Tauri通知API
- 支持提前设定时间提醒
- Android推送通知实现

#### (4) 导入导出
- 支持iCalendar (.ics)格式
- 遵循RFC5545标准解析和生成

### 6. 前端组件结构

#### CalendarView.vue (主日历视图组件)
- 日历导航栏
- 不同日历视图（月/周/日）
- 事件展示和交互

#### CalendarNavigation.vue (日历导航组件)
- 日期导航控制
- 视图切换功能
- 当前日期显示

#### EventEditor.vue (事件编辑组件)
- 事件表单
- 时间选择
- 提醒设置
- 重复规则配置

#### 组合式函数
- useCalendar: 日历逻辑处理
- useEvents: 事件操作管理
- useNotifications: 通知管理

#### 状态管理 (Pinia Store)
- eventStore: 事件状态管理
- calendarStore: 日历状态管理

### 7. 后端服务逻辑

#### EventService (事件服务)
- 事件CRUD操作
- 重复事件处理
- 提醒管理

#### NotificationService (通知服务)
- 桌面通知
- Android推送通知
- 通知调度

#### IcalService (iCalendar服务)
- RFC5545解析
- 事件导入导出
- 重复规则处理

#### CalendarService (日历服务)
- 统一的业务逻辑入口
- 事件管理
- 日历操作

### 8. 前后端交互API

通过Tauri命令实现安全的前后端通信：

- get_all_events: 获取所有事件
- get_events_by_date: 获取指定日期事件
- create_event: 创建事件
- update_event: 更新事件
- delete_event: 删除事件
- import_ical: 导入iCalendar
- export_ical: 导出iCalendar

### 9. RFC5545标准实现

#### 完整的iCalendar属性支持
- SUMMARY: 事件标题
- DTSTART/DTEND: 开始/结束时间
- DESCRIPTION: 事件描述
- RRULE: 重复规则
- VALARM: 提醒设置
- UID: 唯一标识符
- DTSTAMP: 时间戳
- STATUS: 事件状态
- LOCATION: 位置信息
- CATEGORIES: 分类
- PRIORITY: 优先级
- URL: 相关链接

#### 重复规则解析和生成
支持FREQ、INTERVAL、COUNT、UNTIL、BYDAY等RRULE参数

#### 日期时间格式转换
- 支持DATE和DATE-TIME格式
- 正确处理UTC时间和本地时间
- 文本转义处理

### 10. 安全和性能考虑
- 使用参数化查询防止SQL注入
- 实现缓存机制提高性能
- 统一错误处理
- 类型安全的接口定义
- 遵循最小权限原则