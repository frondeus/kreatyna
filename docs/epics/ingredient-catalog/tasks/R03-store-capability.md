# R03 — Ingredient store capability

Status: awaiting review. Change: `yttqpszu`. Dependency R02 is approved.

Add a separate `ingredients/capabilities.rs` module using the existing
`krucyfiks::capability` pattern. `IngredientStore` lists complete records,
saves a validated `IngredientWrite`, and deletes by `IngredientId`. The store
contract assigns a fresh UUID for creation, preserves update identity, enforces
name-key uniqueness, returns a missing-update error, and treats deletion of an
absent row as success. SQLite behavior is implemented and verified in R05.

Use operation-specific typed errors. Load and delete can report storage or
transport failures. Save can additionally report a duplicate name or a missing
update. Transport errors do not trigger automatic write retries.

Expose generated request/response and resolver types, an effect enum combining
render and store envelopes, and a testing fixture with render and store inboxes.
Reuse the existing render capability and the established `(32, 1)` capacities.

Check the contract by compiling the core and FFI crates, running core tests,
Clippy with denied warnings, and formatting checks. The generated client and
resolver plumbing is already covered by the existing krucyfiks capability
tests. Exercise ingredient behavior through the real store in R05 and through
widget effects in R06–R08. Report existing unrelated workspace formatting
failures separately.

Validation: core tests, Clippy with warnings denied, FFI compilation, and
changed-file formatting pass. Workspace formatting reports the existing import
ordering differences in counter tests and FFI files.

Update the epic and R02 plan to record R02 approval. Mark R03 awaiting review
once checks pass. No database, widget, runtime, navigation, or Swift code belongs
in this change.
