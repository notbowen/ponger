CREATE TABLE polls (
    id SERIAL PRIMARY KEY,
    message_id TEXT NOT NULL,
    end_time BIGINT NOT NULL
);