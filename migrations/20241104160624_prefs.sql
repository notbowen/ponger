CREATE TABLE IF NOT EXISTS prefs (
    server_id TEXT NOT NULL CONSTRAINT name_unique UNIQUE,
    channel_id TEXT NOT NULL,
    category_id TEXT NOT NULL
);
