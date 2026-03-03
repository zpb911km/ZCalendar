-- 创建日历表
CREATE TABLE calendars (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    color VARCHAR(255) NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 创建事件表
CREATE TABLE events (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start VARCHAR(255) NOT NULL,
    end VARCHAR(255) NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT 0,
    reminder_minutes INT NOT NULL DEFAULT 0,
    created_at VARCHAR(255) NOT NULL,
    updated_at VARCHAR(255) NOT NULL,
    recurrence_rule TEXT,
    recurrence_id VARCHAR(255),
    sequence INT NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'CONFIRMED',
    location VARCHAR(255),
    organizer VARCHAR(255),
    attendees TEXT,
    url VARCHAR(255),
    categories VARCHAR(255),
    priority INT,
    calendar_id VARCHAR(255),
    FOREIGN KEY (calendar_id) REFERENCES calendars (id) ON DELETE SET NULL
);

-- 创建索引以提高查询性能
CREATE INDEX idx_events_start ON events (start);
CREATE INDEX idx_events_end ON events (end);
CREATE INDEX idx_events_calendar_id ON events (calendar_id);
CREATE INDEX idx_events_recurrence_id ON events (recurrence_id);