CREATE TABLE creation_information (
  uuid TEXT PRIMARY KEY NOT NULL,
  creator_user_uuid TEXT NOT NULL,
  creation_time TEXT NOT NULL,
  last_updated_by_user_uuid TEXT NOT NULL,
  last_updated_time TEXT NOT NULL
)