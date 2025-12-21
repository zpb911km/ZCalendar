# 日历应用项目架构设计

## 原始作业要求

【作业说明】
个人作业：开发一个日历应用

【具体要求】

1. 技术实现要求：鸿蒙/Android/iOS原生开发或者跨端开发
2. 核心功能：
   参考rfc5545等文档实现一个日历App，实现基本要求的三个功能，在此基础上有兴趣的同学可以实现扩展要求的三个功能，此外如果有别的想法和功能，也可以按自己的想法实现；

基本要求：

- [x] （1）日历视图展示（月视图、周视图、日视图）；
- [x] （2）日程添加、编辑、查看和删除；
- [x] （3）日程提醒功能的实现；

扩展要求：

- [x] （1）日历事件的导入导出（支持iCalendar .ics格式，遵循RFC5545标准）；
- [ ] （2）网络订阅功能；
- [ ] （3）农历相关的实现；

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
├── App.vue               # 应用根组件
├── main.ts               # 应用入口文件
├── vite-env.d.ts         # Vite环境类型定义
├── assets/               # 静态资源
│   └── vue.svg           # Vue logo
├── components/           # 可复用UI组件
│   ├── Calendar/         # 日历核心组件
│   │   ├── CalendarNavigation.vue  # 日历导航组件
│   │   ├── MonthView.vue           # 月视图
│   │   ├── WeekView.vue            # 周视图
│   │   └── DayView.vue             # 日视图
│   └── Event/            # 事件组件
│       ├── EventCard.vue           # 事件卡片
│       ├── EventEditor.vue         # 事件编辑器
│       └── EventDetail.vue         # 事件详情
├── composables/          # Vue组合式函数
│   ├── useCalendar.ts    # 日历逻辑
│   └── useEvents.ts      # 事件操作
├── router/               # 路由配置
│   └── index.ts          # 路由定义
├── services/             # 与后端交互的服务
│   ├── calendarService.ts    # 日历服务
│   └── eventService.ts       # 事件服务
├── stores/               # 状态管理 (Pinia)
│   ├── calendarStore.ts      # 日历状态
│   └── eventStore.ts         # 事件状态
├── styles/               # 样式文件
│   ├── color.css             # 颜色主题
│   └── dark-color.css        # 暗色主题
├── types/                # TypeScript类型定义
│   ├── calendar.ts           # 日历类型
│   ├── event.ts              # 事件类型
│   └── tauri.ts              # Tauri相关类型
├── utils/                # 工具函数
│   ├── calendarUtils.ts      # 日历工具
│   ├── dateUtils.ts          # 日期处理
│   └── themeManager.ts       # 主题管理
└── views/                # 页面视图
    ├── CalendarView.vue      # 主日历页面
    ├── EventView.vue         # 事件详情页面
    └── SettingsView.vue      # 设置页面
```

### 4. 数据库设计 (SQLite)

使用SQLx库进行数据库操作，包含以下表：

#### events表

存储日历事件的详细信息，字段包括：

- id: INTEGER PRIMARY KEY AUTOINCREMENT - 事件唯一标识符
- title: TEXT NOT NULL - 事件标题
- description: TEXT - 事件描述
- start: TEXT NOT NULL - 事件开始时间 (ISO 8601格式)
- end: TEXT NOT NULL - 事件结束时间 (ISO 8601格式)
- all_day: BOOLEAN NOT NULL DEFAULT 0 - 是否为全天事件
- reminder_minutes: INTEGER NOT NULL DEFAULT 0 - 提醒提前分钟数
- created_at: TEXT NOT NULL - 创建时间戳
- updated_at: TEXT NOT NULL - 更新时间戳
- recurrence_rule: TEXT - 重复规则 (RRULE)
- recurrence_id: TEXT - 重复事件标识符
- sequence: INTEGER NOT NULL DEFAULT 0 - 事件序列号
- status: TEXT NOT NULL DEFAULT 'CONFIRMED' - 事件状态 (CONFIRMED, CANCELLED等)
- location: TEXT - 事件位置
- organizer: TEXT - 事件组织者
- attendees: TEXT - 参与者列表
- url: TEXT - 相关链接
- categories: TEXT - 事件分类
- priority: INTEGER - 事件优先级
- calendar_id: TEXT - 所属日历ID (外键关联calendars表)

#### calendars表

存储日历信息，支持多个日历，字段包括：

- id: TEXT PRIMARY KEY NOT NULL - 日历唯一标识符
- name: TEXT NOT NULL - 日历名称
- color: TEXT NOT NULL - 日历显示颜色
- is_primary: BOOLEAN NOT NULL DEFAULT 0 - 是否为主日历
- created_at: TEXT NOT NULL - 创建时间戳
- updated_at: TEXT NOT NULL - 更新时间戳

#### 数据库索引

为提高查询性能，创建了以下索引：

- idx_events_start: 基于事件开始时间的索引
- idx_events_end: 基于事件结束时间的索引
- idx_events_calendar_id: 基于日历ID的索引
- idx_events_recurrence_id: 基于重复事件ID的索引

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

#### Calendar/ 目录组件

- **CalendarNavigation.vue**: 日历导航组件
  - 日期导航控制
  - 视图切换功能（月/周/日）
  - 当前日期显示

- **MonthView.vue**: 月视图组件
  - 以网格形式展示整月的日历视图
  - 显示每日事件概览
  - 支持月份切换

- **WeekView.vue**: 周视图组件
  - 水平布局展示一周的时间线
  - 按小时显示事件
  - 支持周切换

- **DayView.vue**: 日视图组件
  - 垂直时间轴显示当天事件
  - 按小时详细展示事件
  - 支持日切换

#### Event/ 目录组件

- **EventCard.vue**: 事件卡片组件
  - 以卡片形式展示单个事件
  - 显示事件标题、时间、颜色标识
  - 支持快速查看和编辑

- **EventDetail.vue**: 事件详情组件
  - 显示事件的完整信息
  - 包括标题、描述、时间、位置、分类等
  - 提供编辑和删除功能

- **EventEditor.vue**: 事件编辑组件
  - 事件表单界面
  - 时间选择器
  - 提醒设置（提前分钟数）
  - 重复规则配置（RRULE）
  - 位置、分类、优先级等高级选项

#### 组合式函数 (composables/)

- **useCalendar.ts**: 日历逻辑处理
  - 日期计算和格式化
  - 视图切换逻辑
  - 日历数据显示处理

- **useEvents.ts**: 事件操作管理
  - 事件CRUD操作
  - 事件搜索和过滤
  - 事件提醒管理

#### 状态管理 (stores/)

- **eventStore.ts**: 事件状态管理
  - 管理事件列表状态
  - 事件缓存和同步
  - 事件过滤条件

- **calendarStore.ts**: 日历状态管理
  - 管理日历视图状态
  - 当前日期和视图类型
  - 日历设置和配置

### 7. 后端服务逻辑 (Rust/Tauri)

#### 核心数据模型

- **Event结构体**: 定义事件数据模型，包含标题、描述、时间、提醒、重复规则、状态等完整信息
- **Calendar结构体**: 定义日历数据模型，包含名称、颜色、主日历标识等信息
- **DTOs**: 定义用于API传输的数据对象，如CreateEventDto、UpdateEventDto等

#### 事件管理服务

通过Tauri命令实现的事件管理功能：

- **get_all_events**: 获取所有事件，按开始时间排序
- **get_events_by_date**: 获取指定日期的事件
- **get_event_by_id**: 根据ID获取单个事件详情
- **create_event**: 创建新事件
- **update_event**: 更新事件信息（支持部分更新）
- **delete_event**: 删除指定事件
- **delete_all_events**: 删除所有事件
- **get_events_in_range**: 获取时间范围内的事件
- **search_events**: 搜索事件（按标题、描述、位置等字段）
- **get_upcoming_events**: 获取即将到来的事件

#### 日历管理服务

- **get_all_calendars**: 获取所有日历列表
- **create_calendar**: 创建新日历
- **get_calendar_by_id**: 根据ID获取日历详情
- **update_calendar**: 更新日历信息
- **delete_calendar**: 删除日历
- **get_calendar_events**: 获取指定日历的事件

#### iCalendar (ICS) 导入导出服务

- **import_ical**: 解析iCalendar格式内容并导入为本地事件
  - 支持多时区偏移处理
  - 解析VEVENT组件的所有标准属性
  - 处理重复事件规则
- **export_ical**: 将本地事件导出为iCalendar格式
  - 支持导出所有事件或指定事件ID列表
  - 生成符合RFC5545标准的格式
  - 支持时区偏移转换

#### 智能提醒服务

- **自动提醒检查**: 每50秒检查一次即将到来的事件
- **通知发送**: 通过Tauri通知插件发送桌面通知
- **跨平台支持**: 支持桌面和移动平台的通知功能
- **时间计算**: 准确计算提醒时间，考虑用户设置的提前分钟数

#### 数据库服务 (SQLx)

- **连接池管理**: 使用SQLite连接池提高性能
- **参数化查询**: 防止SQL注入攻击
- **数据迁移**: 自动执行数据库迁移脚本
- **事务支持**: 确保数据操作的原子性

### 8. 前后端交互API (Tauri Commands)

通过Tauri命令实现安全的前后端通信，所有API均使用异步调用：

#### 事件管理API

- **get_all_events**: 获取所有事件
  - 参数: 无
  - 返回: Event[]数组，按开始时间排序

- **get_events_by_date**: 获取指定日期的事件
  - 参数: date (string, ISO 8601格式)
  - 返回: Event[]数组

- **get_event_by_id**: 根据ID获取单个事件
  - 参数: id (i32)
  - 返回: Event对象或null

- **create_event**: 创建新事件
  - 参数: CreateEventDto对象
  - 返回: 新创建的Event对象

- **update_event**: 更新事件信息
  - 参数: UpdateEventDto对象（支持部分更新）
  - 返回: 更新后的Event对象

- **delete_event**: 删除事件
  - 参数: id (i32)
  - 返回: 无

- **delete_all_events**: 删除所有事件
  - 参数: 无
  - 返回: 无

- **get_events_in_range**: 获取时间范围内的事件
  - 参数: start (string), end (string)
  - 返回: Event[]数组

- **search_events**: 搜索事件
  - 参数: query (string)
  - 返回: Event[]数组，按标题、描述、位置搜索

- **get_upcoming_events**: 获取即将到来的事件
  - 参数: limit (i32)
  - 返回: Event[]数组

#### iCalendar导入导出API

- **import_ical**: 导入iCalendar格式数据
  - 参数: ical_content (string), timezone_offset (Option<i32>)
  - 返回: 导入的Event[]数组

- **export_ical**: 导出事件为iCalendar格式
  - 参数: event_ids (Option<Vec<i32>>), timezone_offset (Option<i32>)
  - 返回: iCalendar格式字符串

#### 日历管理API

- **get_all_calendars**: 获取所有日历
  - 参数: 无
  - 返回: Calendar[]数组

- **create_calendar**: 创建新日历
  - 参数: Calendar对象
  - 返回: 新创建的Calendar对象

- **get_calendar_by_id**: 根据ID获取日历
  - 参数: id (string)
  - 返回: Calendar对象或null

- **update_calendar**: 更新日历信息
  - 参数: Calendar对象
  - 返回: 更新后的Calendar对象

- **delete_calendar**: 删除日历
  - 参数: id (string)
  - 返回: 无

- **get_calendar_events**: 获取指定日历的事件
  - 参数: calendar_id (string)
  - 返回: Event[]数组

#### 通知API

- **send_notification**: 发送通知
  - 参数: title (string), body (string)
  - 返回: 结果字符串

### 9. RFC5545标准实现

#### iCalendar格式导入导出

项目完整实现了RFC5545标准，支持iCalendar (.ics)格式的导入和导出功能：

- 使用`ical` crate解析和生成iCalendar数据
- 支持完整的VCALENDAR和VEVENT组件
- 正确处理时区偏移转换

#### 完整的iCalendar属性支持

- VCALENDAR组件：
  - VERSION: 日历版本 (2.0)
  - PRODID: 产品标识符
- VEVENT组件：
  - UID: 事件唯一标识符
  - DTSTAMP: 事件时间戳
  - DTSTART: 事件开始时间 (支持DATE和DATE-TIME格式)
  - DTEND: 事件结束时间 (支持DATE和DATE-TIME格式)
  - SUMMARY: 事件标题
  - DESCRIPTION: 事件描述
  - LOCATION: 事件位置
  - STATUS: 事件状态 (CONFIRMED, CANCELLED等)
  - PRIORITY: 事件优先级
  - CATEGORIES: 事件分类
  - URL: 相关链接
  - RRULE: 重复规则 (遵循RFC5545重复规则语法)

#### 重复规则(RRULE)解析和生成

支持以下RRULE参数：

- FREQ: 重复频率 (DAILY, WEEKLY, MONTHLY, YEARLY)
- INTERVAL: 重复间隔
- COUNT: 重复次数限制
- UNTIL: 重复结束时间
- BYDAY: 指定星期几重复
- BYMONTH: 指定月份重复
- BYMONTHDAY: 指定月中的日期重复

#### 日期时间格式转换

- 支持DATE格式 (YYYYMMDD) 表示全天事件
- 支持DATE-TIME格式 (YYYYMMDDTHHMMSS) 表示具体时间事件
- 正确处理UTC时间和本地时间转换
- 实现了完整的文本转义处理 (\\, \; , \, , \n)
- 处理时区偏移转换功能

### 10. 安全和性能考虑

#### 安全措施

- **SQL注入防护**: 使用SQLx的参数化查询和类型安全的数据库操作，有效防止SQL注入攻击
- **类型安全**: 通过TypeScript和Rust的强类型系统，确保数据类型安全，减少运行时错误
- **Tauri安全模型**: 利用Tauri的安全沙箱机制，限制前端对系统资源的直接访问
- **数据验证**: 在前后端接口处对输入数据进行验证，防止恶意数据注入

#### 性能优化

- **数据库连接池**: 使用SQLx连接池管理数据库连接，提高数据库操作性能
- **数据库索引**: 在events表的关键字段（start, end, calendar_id, recurrence_id）上创建索引，优化查询性能
- **异步处理**: 使用Tokio异步运行时处理I/O密集型操作，提高应用响应性
- **提醒服务优化**: 智能提醒服务每60秒检查一次即将到来的事件，平衡了响应性和性能
- **数据缓存**: 在前端使用Pinia状态管理，缓存常用数据减少后端请求

#### 数据管理

- **自动数据迁移**: 应用启动时自动执行数据库迁移，确保数据库结构与代码版本同步
- **数据持久化**: 使用SQLite本地数据库，确保数据在应用重启后不丢失
- **数据备份**: 支持iCalendar格式的导入导出，便于用户备份和迁移数据

#### 跨平台兼容性

- **统一代码库**: 使用Tauri框架实现一套代码同时支持桌面和移动平台
- **响应式设计**: 前端UI采用响应式设计，适配不同屏幕尺寸
- **平台特性适配**: 针对不同平台特性（如Android通知系统）进行专门适配
