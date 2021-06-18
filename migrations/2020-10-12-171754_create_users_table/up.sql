CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  uuid TEXT NOT NULL,
  username TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  nickname TEXT NOT NULL
)