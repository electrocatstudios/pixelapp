-- Add migration script here
CREATE TABLE IF NOT EXISTS collection (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    creator VARCHAR
);

ALTER TABLE pixelimage ADD COLUMN collection_id INT DEFAULT NULL;

CREATE TABLE IF NOT EXISTS palette (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    description VARCHAR
);

CREATE TABLE palettecolor (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    palette_id INT NOT NULL,
    r INT NOT NULL,
    g INT NOT NULL,
    b INT NOT NULL,
    alpha FLOAT,
    name VARCHAR
);