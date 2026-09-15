CREATE TABLE counter (
    id INTEGER NOT NULL PRIMARY KEY CHECK (id = 1),
    value BIGINT NOT NULL CHECK (value >= 0)
);
