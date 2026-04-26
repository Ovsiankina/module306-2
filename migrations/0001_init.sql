-- migrations/0001_init.sql

CREATE TABLE IF NOT EXISTS users (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    username        TEXT    NOT NULL UNIQUE,
    email           TEXT    NOT NULL UNIQUE,
    first_name      TEXT    NOT NULL DEFAULT '',
    last_name       TEXT    NOT NULL DEFAULT '',
    password_hash   TEXT    NOT NULL,
    role            TEXT    NOT NULL DEFAULT 'Editor',
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_played_at  DATE,
    daily_attempts  INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS daily_gifts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     INTEGER NOT NULL REFERENCES users(id),
    awarded_at  DATE    DEFAULT CURRENT_DATE,
    store       TEXT    NOT NULL
);
