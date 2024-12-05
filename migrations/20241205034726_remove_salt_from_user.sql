-- Add migration script here
ALTER TABLE users
DROP COLUMN password_salt;