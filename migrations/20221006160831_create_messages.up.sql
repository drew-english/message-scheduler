CREATE TABLE messages (
  id UUID PRIMARY KEY,
  delivery_time timestamptz NOT NULL,
  action_type smallint NOT NULL,
  version smallint NOT NULL, 
  attributes jsonb NOT NULL
);

CREATE INDEX idx_messages_on_delivery_time ON messages (delivery_time);
