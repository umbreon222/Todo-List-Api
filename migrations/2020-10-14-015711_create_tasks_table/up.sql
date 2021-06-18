CREATE TABLE tasks (
  id INTEGER PRIMARY KEY NOT NULL,
  uuid TEXT NOT NULL,
  content TEXT NOT NULL,
  priority INTEGER NOT NULL DEFAULT 0,
  tag_uuids TEXT,
  is_complete BOOLEAN NOT NULL DEFAULT 0,
  creation_information_uuid TEXT NOT NULL
)