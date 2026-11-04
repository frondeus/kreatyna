-- The duration, kind, and instructions of a step (REC-17). Steps from
-- before REC-17 become active steps with no duration and no instructions.
-- `duration_seconds` is 1 second to 365 days (REC-17/Q3, Q4); BIGINT makes
-- Diesel read it as i64, so a `u32` always fits.
ALTER TABLE recipe_steps ADD COLUMN duration_seconds BIGINT
    CHECK (duration_seconds BETWEEN 1 AND 31536000);
ALTER TABLE recipe_steps ADD COLUMN kind TEXT NOT NULL DEFAULT 'active'
    CHECK (kind IN ('active', 'waiting'));
ALTER TABLE recipe_steps ADD COLUMN instructions TEXT
    CHECK (instructions IS NULL OR length(trim(instructions)) > 0);
