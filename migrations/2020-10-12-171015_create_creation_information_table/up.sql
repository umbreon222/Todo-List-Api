CREATE TABLE creation_information (
  id INTEGER PRIMARY KEY NOT NULL,
  uuid TEXT NOT NULL,
  creator_user_uuid TEXT NOT NULL,
  creation_time TEXT NOT NULL,
  last_updated_by_user_uuid TEXT NOT NULL,
  last_updated_time TEXT NOT NULL
)