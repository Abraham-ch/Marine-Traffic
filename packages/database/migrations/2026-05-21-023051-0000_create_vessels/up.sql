-- Your SQL goes here
CREATE TABLE vessels (
    mmsi BIGINT PRIMARY KEY,
    ship_name TEXT NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lng DOUBLE PRECISION NOT NULL,
    speed REAL,
    heading REAL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);