CREATE TABLE schedules (
  id SERIAL PRIMARY KEY,
  start timestamp NOT NULL,
  "end" timestamp NOT NULL,
  user_id integer NOT NULL REFERENCES users (id),
  title TEXT NOT NULL,
  content TEXT NOT NULL
)