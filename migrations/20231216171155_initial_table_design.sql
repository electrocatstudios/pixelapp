-- Add migration script here
CREATE TABLE IF NOT EXISTS pixelimage (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    description VARCHAR,
    owner_id INT,
    width INT NOT NULL,
    height INT NOT NULL,
    pixelwidth INT NOT NULL,
    guid VARCHAR UNIQUE
);

CREATE TABLE IF NOT EXISTS pixel (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image_id INT NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    r INT NOT NULL,
    g INT NOT NULL,
    b INT NOT NULL,
    alpha FLOAT,
    layer INT,
    frame INT
);

CREATE TABLE IF NOT EXISTS shading (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image_id INT NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    r INT NOT NULL,
    g INT NOT NULL,
    b INT NOT NULL,
    alpha FLOAT,
    frame INT
);