# R10 — Ingredient catalog coordinator

Tracked in beads: `vibetyna-flx.12`. Change: `rllwnqlk`. R09a is approved.

## Composition contract

Add `IngredientCatalog`, which retains one list widget and owns exactly one active
List, Detail, Create, direct Edit, or detail-origin Edit screen. Its public event,
outcome, and view types nest the child contracts. The view contains the retained
list, navigation derived from the active child state, the active command-scope ID,
and a typed screen view. No second mutable route is stored.

Each activation receives a fresh `CommandScope`. Child initialization and update
commands are scoped before their outcomes are mapped into the catalog. Events and
outcomes must carry the current scope ID and target the active branch. Replacing a
screen cancels pending work and rejects queued results from the old activation,
including a prior activation of the same ingredient.

## Navigation and retained state

Use R09 transitions to check every accepted screen change. List intents open
creation, detail, or direct edit; detail edit retains its detail destination.
Form Back goes through RequestClose and only Discarded navigates. Detail Back
returns to the list while idle, dismisses an open delete confirmation, and is
ignored during deletion. Accepted saves return to their origin, update a retained
detail when needed, and update the list cache. Accepted deletion removes the row.

The list preserves search and cached records while hidden. Leaving it cancels an
active load; returning restarts an interrupted load. Accepted writes patch a
complete cache and clear refresh errors without fetching. A successful early
creation reloads because a single saved record is not a complete catalog.

Add crate-private list operations only for suspend/resume, explicit reload,
visible-record verification, and accepted save/delete cache updates. Parsing,
validation, persistence, child operation errors, sorting, and filtering remain in
their existing owners.

## Validation and delivery

Use effect-driver tests for initial loading, nested routing, create/detail/edit
journeys, both edit origins, Back and confirmation behavior, save/delete
propagation, cache and search retention, interrupted and early loads, pending
operation guards, failures, inactive branches, fabricated terminal outcomes,
stale scopes, repeated outcomes, and cancellation on catalog drop.

Run core tests, Clippy with warnings denied, FFI compilation, and formatting
checks. Report existing unrelated formatting failures separately. The R10 change
contains this plan, implementation, tests, R09a approval, and epic updates. Tabs,
runtime wiring, database changes, bindings, and Swift remain later tasks.

Implementation validation completed on 2026-09-26:

- `cargo test -p kreatyna --offline`
- `cargo test -p kreatyna-ffi --offline`
- `cargo clippy -p kreatyna -p kreatyna-ffi --all-targets --offline -- -D warnings`
- targeted `rustfmt --check` for every R10 Rust file

The workspace-wide formatting check remains blocked only by the pre-existing
import-order differences in `tests/background.rs`, `tests/composition.rs`, and
`kreatyna-ffi/src/lib.rs`; R10 does not modify those files.
