CREATE TABLE IF NOT EXISTS chat_messages
(
    id     UUID PRIMARY KEY,
    sender TEXT NOT NULL,
    sender_id UUID NOT NULL,
    message_text TEXT NOT NULL,
    created_date TIMESTAMP NOT NULL
);