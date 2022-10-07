CREATE TABLE messages (
  id UUID PRIMARY KEY,
  delivery_time timestamptz NOT NULL,
  payload text NOT NULL,
  action jsonb NOT NULL
);
