-- Add migration script here
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    marzbann_username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    password_salt VARCHAR(255) NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT users_marzbann_username_unique UNIQUE (marzbann_username),
    CONSTRAINT users_email_unique UNIQUE (email)
);

CREATE TABLE plans (
    id INT AUTO_INCREMENT PRIMARY KEY,
    enabled BOOLEAN DEFAULT TRUE NOT NULL,  -- if false, the plan is not available for purchase
                                            -- more preferable so that information is not lost
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL, -- AUD
    data_limit BIGINT NOT NULL, -- in GB
    duration_days INT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE transactions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    plan_id INT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    status ENUM('unpaid', 'pending', 'completed', 'failed', 'cancelled') DEFAULT 'unpaid', -- for stripe
    stripe_payment_intent_id VARCHAR(255), -- for stripe
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (plan_id) REFERENCES plans(id)
);