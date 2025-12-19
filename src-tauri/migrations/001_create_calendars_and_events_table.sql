-- 创建日历表
CREATE TABLE calendars (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 创建事件表
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    start TEXT NOT NULL,
    end TEXT NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT 0,
    reminder_minutes INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    recurrence_rule TEXT,
    recurrence_id TEXT,
    sequence INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'CONFIRMED',
    location TEXT,
    organizer TEXT,
    attendees TEXT,
    url TEXT,
    categories TEXT,
    priority INTEGER,
    calendar_id TEXT,
    FOREIGN KEY (calendar_id) REFERENCES calendars (id) ON DELETE SET NULL
);

-- 创建索引以提高查询性能
CREATE INDEX idx_events_start ON events (start);
CREATE INDEX idx_events_end ON events (end);
CREATE INDEX idx_events_calendar_id ON events (calendar_id);
CREATE INDEX idx_events_recurrence_id ON events (recurrence_id);