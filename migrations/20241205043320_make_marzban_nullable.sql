-- Add migration script here
ALTER TABLE users
MODIFY marzban_username VARCHAR(255) NULL; -- Marzban can be null, since we dont want users to be able to connect without a plan.