CREATE TABLE messages (
  id UUID PRIMARY KEY,
  delivery_time timestamptz NOT NULL,
  payload text NOT NULL,
  action jsonb NOT NULL
);

CREATE INDEX idx_messages_on_delivery_time ON messages (delivery_time);
