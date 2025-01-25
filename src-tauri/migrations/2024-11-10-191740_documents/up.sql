-- Documents
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    kind TEXT NOT NULL,
    body TEXT NOT NULL
);
-- Devices
CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    unique_id TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    certificate BLOB NOT NULL,
    is_trusted BOOLEAN NOT NULL DEFAULT FALSE
);
-- Private keys
CREATE TABLE IF NOT EXISTS private_keys (private_key BLOB UNIQUE);
-- Vector Clock
CREATE TABLE IF NOT EXISTS vector_clock (
    device_id INTEGER NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (device_id) REFERENCES devices(id),
    PRIMARY KEY (device_id)
);
-- Events
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    creator_id INTEGER NOT NULL,
    creator_counter INTEGER NOT NULL DEFAULT 0,
    vector_clock TEXT NOT NULL,
    document_id INTEGER NOT NULL,
    kind TEXT NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    data TEXT NOT NULL,
    FOREIGN KEY (creator_id) REFERENCES devices(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);
CREATE INDEX IF NOT EXISTS idx_events_path_vector ON events (document_id, vector_clock);

CREATE TABLE IF NOT EXISTS order_of_events (
    "order" INTEGER NOT NULL,
    document_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);
