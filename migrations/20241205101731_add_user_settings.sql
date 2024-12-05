-- Add migration script here
ALTER TABLE users
ADD COLUMN email_data_reminder BOOLEAN NOT NULL DEFAULT TRUE,
ADD COLUMN email_expiration_reminder BOOLEAN NOT NULL DEFAULT TRUE;