-- A line's amount, known unit and note (REC-08). The amount is exact decimal
-- text, as in the catalog. A unit needs an amount: "2 eggs" has no unit, and
-- "g flour" with no amount cannot be stored. Lines from REC-07 keep no
-- amount, no unit and no note.
ALTER TABLE recipe_ingredient_lines ADD COLUMN amount TEXT;
ALTER TABLE recipe_ingredient_lines ADD COLUMN unit TEXT
    CHECK (unit IS NULL OR (amount IS NOT NULL
        AND unit IN ('g', 'kg', 'ml', 'l', 'tsp', 'tbsp', 'cup', 'piece')));
ALTER TABLE recipe_ingredient_lines ADD COLUMN note TEXT
    CHECK (note IS NULL OR length(trim(note)) > 0);
