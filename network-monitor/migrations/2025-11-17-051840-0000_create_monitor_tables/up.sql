-- Your SQL goes here
--监控配置表
CREATE TABLE monitor_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT UNIQUE,
    target TEXT NOT NULL,
    method TEXT,
    monitor_type TEXT NOT NULL CHECK (monitor_type IN ('HTTP', 'TCP', 'ICMP', 'DNS','CPU', 'DISK', 'MEMORY', 'PROCESS')),
    interval_ms INTEGER,
    timeout_ms INTEGER NOT NULL DEFAULT 5000,
    config_json TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    tag TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 检查结果表
CREATE TABLE check_result (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    monitor_id INTEGER NOT NULL REFERENCES monitor_config(id) ON DELETE CASCADE,
    monitor_type TEXT NOT NULL,
    status INTEGER NOT NULL,
    response_time INTEGER NOT NULL,
    metadata_json TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 告警配置表
CREATE TABLE alert_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    monitor_id INTEGER NOT NULL REFERENCES monitor_config(id) ON DELETE CASCADE,
    alert_type TEXT NOT NULL, -- e.g., email, sms, webhook etc.
    config_json TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- 告警记录表
CREATE TABLE alert_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    alert_rules_id INTEGER NOT NULL REFERENCES alert_rules(id) ON DELETE CASCADE,
    monitor_id INTEGER NOT NULL REFERENCES monitor_config(id) ON DELETE CASCADE,
    alert_type TEXT NOT NULL,
    status INTEGER NOT NULL, -- e.g., sent, failed etc.
    message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 通知渠道配置表
CREATE TABLE notification_channels (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT UNIQUE,
    channel_type TEXT NOT NULL, -- e.g., email, sms, webhook etc.
    config_json TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 索引创建
CREATE INDEX idx_monitor_config_enabled ON monitor_config(enabled); -- SELECT * FROM monitor_config WHERE enabled = 1;
-- 启用/禁用筛选 + 按 id 逆序分页
CREATE INDEX IF NOT EXISTS idx_monitor_config_enabled_id ON monitor_config(enabled, id DESC); -- SELECT * FROM monitor_config WHERE enabled = 1 ORDER BY id DESC LIMIT 10 OFFSET 20;
-- 目标精确/前缀搜索
CREATE INDEX IF NOT EXISTS idx_monitor_config_target ON monitor_config(target); -- SELECT * FROM monitor_config WHERE target LIKE 'http%';
-- 标签过滤
CREATE INDEX IF NOT EXISTS idx_monitor_config_tag ON monitor_config(tag); -- SELECT * FROM monitor_config WHERE tag = 'production';
CREATE INDEX idx_check_result_monitor_id ON check_result(monitor_id); -- SELECT * FROM check_result WHERE monitor_id = 1;
CREATE INDEX idx_alert_rules_monitor_id ON alert_rules(monitor_id); -- SELECT * FROM alert_rules WHERE monitor_id = 1;
CREATE INDEX idx_alert_logs_alert_rules_id ON alert_logs(alert_rules_id); -- SELECT * FROM alert_logs WHERE alert_rules_id = 1;
CREATE INDEX idx_alert_logs_monitor_id ON alert_logs(monitor_id); -- SELECT * FROM alert_logs WHERE monitor_id = 1;
CREATE INDEX idx_notification_channels_channel_type ON notification_channels(channel_type); -- SELECT * FROM notification_channels WHERE channel_type = 'email';
CREATE INDEX idx_check_result_monitor_id_created_at ON check_result(monitor_id, created_at DESC); -- SELECT * FROM check_result WHERE monitor_id = 1 ORDER BY created_at DESC;
-- 触发器创建
CREATE TRIGGER trg_monitor_config_updated_at
AFTER UPDATE ON monitor_config
WHEN NEW.updated_at = OLD.updated_at
BEGIN
  UPDATE monitor_config SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER trg_alert_rules_updated_at
AFTER UPDATE ON alert_rules
WHEN NEW.updated_at = OLD.updated_at
BEGIN
  UPDATE alert_rules SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER trg_notification_channels_updated_at
AFTER UPDATE ON notification_channels
WHEN NEW.updated_at = OLD.updated_at
BEGIN
  UPDATE notification_channels SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER trg_check_result_updated_at
AFTER UPDATE ON check_result
WHEN NEW.updated_at = OLD.updated_at
BEGIN
  UPDATE check_result SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;