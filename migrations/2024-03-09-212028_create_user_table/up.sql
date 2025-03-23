CREATE TABLE users (
    id VARCHAR(24) PRIMARY KEY,
    username VARCHAR(255),
    email VARCHAR(255) NOT NULL,
    is_email_activate BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
    );