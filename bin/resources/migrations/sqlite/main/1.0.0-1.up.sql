CREATE TABLE IF NOT EXISTS scenes
(
    id TEXT PRIMARY KEY,
    "name" TEXT NOT NULL
);

CREATE INDEX scenes__name_idx ON scenes (name);
