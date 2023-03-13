-- This file should undo anything in `up.sql`
CREATE TABLE users (
  id VARCHAR NOT NULL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  date_of_birth DATE
)
