CREATE TABLE events_table
(
  event_id      serial        PRIMARY KEY,
  uid           VARCHAR(20)   NOT NULL,
  event_name    VARCHAR(50)   NOT NULL,
  event_detail  TEXT          NOT NULL,
  timestamp     TIMESTAMPTZ
);
