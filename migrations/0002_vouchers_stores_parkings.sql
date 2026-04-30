CREATE TABLE IF NOT EXISTS vouchers (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    qr_token      TEXT    NOT NULL UNIQUE,
    email         TEXT    NOT NULL,
    username      TEXT    NOT NULL,
    first_name    TEXT    NOT NULL DEFAULT '',
    last_name     TEXT    NOT NULL DEFAULT '',
    store         TEXT    NOT NULL,
    discount      INTEGER NOT NULL,
    valid_until   TEXT    NOT NULL,
    created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    redeemed      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stores (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT    NOT NULL,
    category      TEXT    NOT NULL,
    store_number  TEXT,
    level         INTEGER,
    phone         TEXT,
    website       TEXT,
    icon_path     TEXT
);

CREATE TABLE IF NOT EXISTS parkings (
    id                    TEXT PRIMARY KEY,
    name                  TEXT    NOT NULL,
    kind                  TEXT    NOT NULL,
    level                 TEXT,
    capacity              INTEGER NOT NULL,
    occupied              INTEGER NOT NULL,
    reserved_accessible   INTEGER NOT NULL,
    reserved_family       INTEGER NOT NULL,
    ev_capacity           INTEGER NOT NULL,
    ev_occupied           INTEGER NOT NULL,
    updated_at            TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS parking_charging_stations (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    parking_id    TEXT    NOT NULL REFERENCES parkings(id) ON DELETE CASCADE,
    network       TEXT    NOT NULL,
    station_type  TEXT    NOT NULL,
    power_kw      INTEGER NOT NULL,
    connectors    TEXT    NOT NULL,
    ports         INTEGER NOT NULL,
    paid          INTEGER NOT NULL,
    availability  TEXT    NOT NULL,
    notes         TEXT    NOT NULL
);
