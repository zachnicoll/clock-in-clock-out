-- This file should undo anything in `up.sql`
ALTER TABLE users 
DROP COLUMN IF EXISTS user_group;

DROP TYPE IF EXISTS USERGROUP;