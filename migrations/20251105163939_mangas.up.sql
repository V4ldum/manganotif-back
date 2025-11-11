-- Add up migration script here
CREATE TABLE IF NOT EXISTS "manga_release"
(
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title           TEXT    NOT NULL,
    special_edition TEXT,
    volume          INTEGER,
    release_date    TEXT    NOT NULL,
    md_id           TEXT    NOT NULL,
    cover_url       TEXT
);

CREATE TABLE IF NOT EXISTS "manga_not_found"
(
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title           TEXT    NOT NULL,
    special_edition TEXT,
    volume          INTEGER,
    release_date    TEXT    NOT NULL,
    cover_url       TEXT,
    titles          TEXT
);

CREATE TABLE IF NOT EXISTS "global_manga"
(
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title           TEXT    NOT NULL,
    special_edition TEXT,
    volume          INTEGER,
    release_date    TEXT    NOT NULL,
    cover_url       TEXT
);
