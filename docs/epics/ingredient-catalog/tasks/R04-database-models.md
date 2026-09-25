# R04 — Ingredient migration and typed Diesel models

Status: awaiting review. Change: `zltwqxro`. R03 is approved.

Add a forward-only ingredient migration after the counter migration. Leave its
`down.sql` empty, following the existing counter migration. Parent rows
hold a private auto-increment local ID, a unique UUID text ID, name/key, nullable
notes and nutrition text, and a checked basis. Reference-unit rows use the parent
local ID and a checked position as a composite key; enforce foreign-key cascade,
per-parent unit-name uniqueness, and at least one measurement. SQLite never casts
decimals to floating point. Preserve the counter table and its data.

Add typed Diesel row models and wrappers for local/remote IDs, basis, and exact
quantity text. Convert domain quantities with `From`/`Into`, use checked
`TryFrom` for fallible mapping, reject malformed and negative stored values, and
retain decimal scale and optional unknown values. Validate positive reference
measurements in Rust. Generate name keys through the shared `name_key` function.
Keep six-decimal user-input validation in R02; storage supports full Decimal
precision.

Use real SQLite tests for empty and counter-only migration, reruns,
schema constraints, exact typed value round trips, invalid stored data, cascade,
ordering, and reopening a database. Run FFI and core tests, Clippy, and formatting.
R05 implements the store service; this change contains only schema, models, and
their direct tests.

Validation: 23 FFI tests and the core tests pass. Clippy with warnings denied
and changed-file formatting pass. Workspace formatting still reports existing
import-order differences in unrelated counter and FFI files.
