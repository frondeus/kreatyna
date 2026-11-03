-- The steps of a recipe (REC-14). A recipe owns its steps: they go with it
-- on delete. `ordinal` is the order added; `x` and `y` are the node's place
-- on the canvas in whole points (D-19). BIGINT makes Diesel read them as i64,
-- like `local_id`, so a `u32` position always fits.
CREATE TABLE recipe_steps (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    recipe_local_id BIGINT NOT NULL
        REFERENCES recipes (local_id) ON DELETE CASCADE,
    ordinal BIGINT NOT NULL CHECK (ordinal >= 0),
    title TEXT NOT NULL CHECK (length(trim(title)) > 0),
    x BIGINT NOT NULL CHECK (x >= 0),
    y BIGINT NOT NULL CHECK (y >= 0),
    UNIQUE (recipe_local_id, ordinal)
);
