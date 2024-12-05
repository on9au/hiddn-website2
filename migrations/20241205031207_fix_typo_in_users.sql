-- Add migration script here
ALTER TABLE users CHANGE COLUMN marzbann_username marzban_username VARCHAR(255) NOT NULL UNIQUE;