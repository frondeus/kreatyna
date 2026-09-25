# R09 — Typed ingredient navigation

Status: awaiting review. Change: `skqrovro`. R08 is approved.

## Model

Add a pure `IngredientNavigation` with List (the default), Detail { id }, Create,
and Edit { id, origin }. `IngredientEditOrigin` is List or Detail; one edit ID
also identifies its underlying detail. No arbitrary stack or mismatched detail
and edit identities can be represented. Public navigation types derive Debug,
Clone, Copy, PartialEq, and Eq.

`transition(&self, action)` computes a new value or a typed error:

| Action | Allowed source | Destination |
| --- | --- | --- |
| OpenCreate | List | Create |
| OpenDetail { id } | List or Detail | Detail with supplied ID; replaces previous detail |
| OpenEdit { id } | List or matching Detail | Edit retaining its origin |
| CloseDetail | Detail | List |
| FormDiscarded | Create or Edit | List for creation/direct editing; matching Detail otherwise |
| FormSaved { id } | Create or matching Edit | Same destination as discard |
| IngredientDeleted { id } | Matching Detail | List |

Unsupported source/action pairs return InvalidTransition. Otherwise valid
transitions with the wrong ingredient ID return IngredientMismatch. The source
value remains unchanged. `back_destination()` returns None from List, List from
Detail/Create/direct Edit, and matching Detail from detail-origin Edit. It only
queries a destination and does not authorize closing a form.

## Integration boundary

R10 derives navigation from its active child state and uses these rules to
change children, without a second independently mutable route. Back on forms
first sends RequestClose; only an accepted Discarded outcome applies
FormDiscarded. Save/delete transitions likewise follow accepted child outcomes.
Dirty-form, pending-operation, and replaced-child guards remain with R10.

No widgets, commands, effects, ingredient records, or generated identities are
owned by this model. Database, runtime, bindings, and Swift remain in later tasks.

## Tests and delivery

Use named test_case scenarios for every allowed transition, all unsupported
source/action combinations, mismatched IDs, both edit origins, detail replacement
(including the same ID), back destinations, default state, and unchanged source
values. Direct model tests need no effect driver or transport coverage.

Create one R09 jj change containing code, tests, this plan, and R08 approval/epic
status updates. Run core tests, Clippy with warnings denied, FFI compilation, and
formatting checks. Report unrelated formatting failures separately. Mark awaiting
review only after validation.

## Validation

- Core tests passed, including 46 navigation scenarios and doc tests.
- Clippy passed for core and FFI, all targets, with warnings denied.
- FFI compilation passed.
- Formatting passed for all changed Rust files. Workspace formatting still reports
  unrelated import ordering in `tests/background.rs`, `tests/composition.rs`
  (core), and `kreatyna-ffi/src/lib.rs`; these files are unchanged by R09.
