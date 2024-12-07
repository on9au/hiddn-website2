-- Add migration script here
ALTER TABLE transactions
MODIFY status ENUM('unpaid', 'pending', 'completed', 'failed', 'cancelled') DEFAULT 'unpaid' NOT NULL;