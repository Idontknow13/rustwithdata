-- init.sql
CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    age INTEGER NOT NULL
);