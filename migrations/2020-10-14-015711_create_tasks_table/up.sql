CREATE TABLE tasks (
  uuid TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  priority INTEGER NOT NULL DEFAULT 0,
  tags TEXT,
  is_complete BOOLEAN NOT NULL DEFAULT 0,
  parent_list_uuid TEXT NOT NULL,
  creation_information_uuid TEXT NOT NULL
)