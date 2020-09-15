-- Your SQL goes here
CREATE TABLE users (
    id UUID UNIQUE PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL
);