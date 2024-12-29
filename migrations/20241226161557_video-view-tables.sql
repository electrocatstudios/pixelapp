CREATE TABLE IF NOT EXISTS video_view (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guid VARCHAR,
    video_guid VARCHAR,
    name VARCHAR
);

CREATE TABLE IF NOT EXISTS video_view_frame (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    video_view_id INTEGER,
    frame INTEGER,
    x INTEGER,
    y INTEGER,
    width INTEGER,
    height INTEGER,
    img BLOB
);

ALTER TABLE animation ADD COLUMN view_id INT DEFAULT NULL;
