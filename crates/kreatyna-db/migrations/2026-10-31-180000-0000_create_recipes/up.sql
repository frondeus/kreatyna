CREATE TABLE recipes (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL CHECK (length(trim(title)) > 0),
    title_key TEXT NOT NULL UNIQUE CHECK (length(title_key) > 0)
);
