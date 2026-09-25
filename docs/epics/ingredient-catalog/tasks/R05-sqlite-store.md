# R05 — SQLite ingredient store

Status: approved. Change: `uzrmtwut`. R04 is approved.

Implement `IngredientStoreMut` on the existing SQLite database using the R03
contract and R04 typed rows. List complete records in one read transaction,
loading units in position order. Treat invalid stored data as a load failure.

Save in an immediate transaction: check normalized name uniqueness, insert with a
fresh UUID or update the specified existing UUID, replace units in order, and
return the complete saved record. A failed parent or unit write rolls back the
whole operation. Delete by UUID with foreign-key cascade; deleting an absent ID
succeeds. Keep database and blocking failures as operation-specific storage
errors. Counter persistence and startup behavior remain intact.

Test real SQLite CRUD, both nutrition bases, exact decimal scale, unknown and
zero values, optional-field clearing, Unicode duplicate names, concurrent
conflicts, missing updates, rollback after an injected unit failure, cascade,
reopen persistence, counter preservation, and corrupt rows. Run FFI and core
tests, Clippy with warnings denied, and changed-file formatting checks.

R13 will wire the store to the app runtime; this change has no runtime, widget,
binding, Swift, or migration implementation.

Validation: 36 FFI tests and all core tests pass. Clippy with warnings denied
and changed-file formatting pass. Workspace formatting still reports existing
import-order differences in unrelated counter and FFI files.
