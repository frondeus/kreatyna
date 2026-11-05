-- The ingredient lines of a recipe (REC-07). A recipe owns its lines: they
-- go with it on delete. `ordinal` is the line's place in the list: the order
-- added, then moved by drag (REC-07/Q1).
CREATE TABLE recipe_ingredient_lines (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    recipe_local_id BIGINT NOT NULL
        REFERENCES recipes (local_id) ON DELETE CASCADE,
    ordinal BIGINT NOT NULL CHECK (ordinal >= 0),
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    UNIQUE (recipe_local_id, ordinal)
);
