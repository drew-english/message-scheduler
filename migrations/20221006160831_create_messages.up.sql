CREATE TABLE messages (
  id UUID,
  delivery_time timestamp,
  payload text,
  action jsonb
);
