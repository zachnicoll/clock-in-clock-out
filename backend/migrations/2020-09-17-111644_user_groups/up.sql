-- Your SQL goes here
CREATE TYPE USERGROUP AS ENUM ('user', 'leader', 'admin');

ALTER TABLE users
    ADD COLUMN user_group USERGROUP NOT NULL default 'user';