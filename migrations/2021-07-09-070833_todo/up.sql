CREATE TABLE todo (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  body TEXT NOT NULL,
  time_modified TIMESTAMP,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  media TEXT,
  parent_id SERIAL NOT NULL,
  parent_name TEXT NOT NULL
)
