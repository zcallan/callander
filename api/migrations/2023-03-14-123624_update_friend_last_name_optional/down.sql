-- This file should undo anything in `up.sql`
ALTER TABLE friends ALTER COLUMN last_name SET NOT NULL;
