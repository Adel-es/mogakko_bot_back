CREATE TABLE schedules (
  id SERIAL PRIMARY KEY,
  start timestamp NOT NULL,
  "end" timestamp NOT NULL,
  user_id TEXT NOT NULL,
  title TEXT NOT NULL,
  content TEXT NOT NULL
)