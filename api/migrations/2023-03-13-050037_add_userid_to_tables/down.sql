-- This file should undo anything in `up.sql`
ALTER TABLE friends DROP COLUMN user_id;
ALTER TABLE friends_ideas DROP COLUMN user_id;
ALTER TABLE friends_events DROP COLUMN user_id;
ALTER TABLE posts DROP COLUMN user_id;
