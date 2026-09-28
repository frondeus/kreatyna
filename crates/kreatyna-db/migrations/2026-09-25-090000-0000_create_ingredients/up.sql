CREATE TABLE ingredients (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    name_key TEXT NOT NULL UNIQUE CHECK (length(name_key) > 0),
    notes TEXT,
    basis INTEGER NOT NULL CHECK (basis IN (0, 1)),
    calories TEXT,
    protein TEXT,
    carbohydrates TEXT,
    fat TEXT
);

CREATE TABLE ingredient_units (
    ingredient_local_id BIGINT NOT NULL REFERENCES ingredients(local_id) ON DELETE CASCADE,
    position BIGINT NOT NULL CHECK (position >= 0),
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    name_key TEXT NOT NULL CHECK (length(name_key) > 0),
    grams TEXT,
    millilitres TEXT,
    PRIMARY KEY (ingredient_local_id, position),
    UNIQUE (ingredient_local_id, name_key),
    CHECK (grams IS NOT NULL OR millilitres IS NOT NULL)
);
