-- Your SQL goes here
CREATE TABLE user (
    id SERIAL PRIMARY KEY,
    email VARCHAR(80) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
)