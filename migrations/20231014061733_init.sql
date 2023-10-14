CREATE TABLE events_table
(
  uid       VARCHAR(20)   PRIMARY KEY NOT NULL,
  name      VARCHAR(20)   NOT NULL,
  body      TEXT          NOT NULL,
  timestamp TIMESTAMPTZ
);
