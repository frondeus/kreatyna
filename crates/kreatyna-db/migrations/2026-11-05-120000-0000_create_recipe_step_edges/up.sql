-- The arrows between the steps of a recipe (REC-18): "to" comes after
-- "from". An edge goes with either of its steps on delete. The table has no
-- recipe column: both ends belong to the recipe through their steps. The
-- store keeps both ends in one recipe and the edges free of loops.
CREATE TABLE recipe_step_edges (
    from_local_id BIGINT NOT NULL
        REFERENCES recipe_steps (local_id) ON DELETE CASCADE,
    to_local_id BIGINT NOT NULL
        REFERENCES recipe_steps (local_id) ON DELETE CASCADE,
    PRIMARY KEY (from_local_id, to_local_id),
    CHECK (from_local_id <> to_local_id)
);

-- The primary key finds the edges from a step; this index finds the edges
-- into a step, which the cascade needs.
CREATE INDEX recipe_step_edges_to ON recipe_step_edges (to_local_id);
