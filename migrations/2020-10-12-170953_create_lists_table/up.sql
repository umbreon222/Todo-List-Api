CREATE TABLE lists (
  id INTEGER PRIMARY KEY NOT NULL,
  uuid TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  color_hex TEXT,
  task_uuids TEXT,
  parent_list_uuid TEXT,
  sub_list_uuids TEXT,
  shared_with_user_uuids TEXT,
  creation_information_uuid TEXT NOT NULL
)