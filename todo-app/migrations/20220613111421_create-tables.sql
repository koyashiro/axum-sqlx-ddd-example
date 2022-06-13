-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(30) NOT NULL
);

CREATE TABLE user_credentials (
    user_id UUID PRIMARY KEY,
    email VARCHAR(254) NOT NULL,
    password_hash VARCHAR(79) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
