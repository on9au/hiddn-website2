-- Add migration script here
CREATE TABLE
    verification_codes (
        id INT AUTO_INCREMENT PRIMARY KEY, -- Unique ID for each verification request
        email VARCHAR(255) NOT NULL, -- The email address associated with the verification
        code VARCHAR(10) NOT NULL, -- The generated verification code
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL, -- Time when the code was generated
        expires_at TIMESTAMP NOT NULL, -- Expiration time for the code
        is_used BOOLEAN DEFAULT FALSE NOT NULL, -- Flag to mark whether the code has been used
        UNIQUE KEY (email, code) -- Ensure no duplicate codes for the same email
    );