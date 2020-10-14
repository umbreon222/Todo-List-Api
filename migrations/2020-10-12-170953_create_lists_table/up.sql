CREATE TABLE Lists (
  ID INTEGER PRIMARY KEY NOT NULL,
  UUID TEXT NOT NULL,
  Title TEXT NOT NULL,
  Description TEXT,
  ColorHex TEXT,
  TaskUUIDs TEXT,
  ParentListUUID TEXT,
  SubListUUIDs TEXT,
  SharedWithUserUUIDs TEXT,
  CreationInformationUUID TEXT NOT NULL
)