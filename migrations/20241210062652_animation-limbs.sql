-- Add migration script here
CREATE TABLE IF NOT EXISTS animation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    description VARCHAR,
    width INT NOT NULL,
    height INT NOT NULL,
    guid VARCHAR UNIQUE
);

CREATE TABLE IF NOT EXISTS animation_limb (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    animation_id INTEGER,
    name VARCHAR NOT NULL,
    color VARCHAR,  
    parent VARCHAR UNIQUE
);

CREATE TABLE IF NOT EXISTS animation_limb_move (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    x FLOAT NOT NULL,
    y FLOAT NOT NULL,
    rot FLOAT NOT NULL,
    length FLOAT NOT NULL,
    perc FLOAT NOT NULL
);