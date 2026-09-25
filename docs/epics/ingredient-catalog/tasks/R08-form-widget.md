# R08 — Ingredient create/edit form widget

Status: awaiting review. Change: `uwkxwylq`. R07 is approved.

## Contract

Add `IngredientForm`, its typed field/event/outcome/status/view types, and a
fixture using `IngredientCapabilities::testing()`. Construct from a supplied
`IngredientDraft` for creation or editing. Retain that draft and an exact initial
snapshot; no widget-generated identity. The private draft is exposed only as a
cloned view. Events edit text through `IngredientDraftField`, change basis or ordered
units, save, request close, or route a `ConfirmationEvent`. Draft mutation
methods preserve raw text and report whether an edit changed anything; the
form handles validation, rendering, and save state. Invalid unit indices and unchanged edits do nothing.
Dirty means the entire raw draft differs from the initial snapshot; reverting
changes clears it. Basis edits never convert nutrition values.

## Validation and save

Initially hide validation issues. The first Save validates with
`IngredientWrite::try_from(&draft)`; every subsequent effective edit recomputes
issues. Invalid saves render all issues without a store effect or draft rewrite.
Valid saves set Saving synchronously, render, then call the store with the
validated write. A `LatestCommandSlot` tags completions. While saving, all
interaction events are ignored, so a write is never restarted implicitly.

Failures restore Editing, retain draft and dirty state, and expose the exact
`IngredientSaveError`; the user may correct and explicitly retry. Effective edits
clear the previous store error. Success enters Finished, renders, and emits the
complete store-returned `Ingredient`. Stale completions and parent outcomes are
ignored. Dropping the widget cancels its pending command, without a promise to
roll back an already submitted write.

## Closing and confirmation

Clean close renders Finished and emits Discarded. Dirty close creates a
`Confirmation` child and renders; only its decisions are accepted while open.
Track the child command through another `LatestCommandSlot` and require both an
accepted `CommandId` and the child's matching resolved view. Cancel restores
Editing with draft, issues, and store error intact. Confirm renders Finished and
emits Discarded. Old or repeated decisions are ignored.

## Tests and delivery

Use named `test_case` scenarios and the effect driver for initial create/edit
state, exact text and scale, dirty/revert behavior, unit add/edit/remove/move,
validation visibility and indexed issues, create/update writes, render-before-save,
all store errors, explicit retry, pending action suppression, stale results,
clean/dirty close, confirmation cancellation/discard, and drop cancellation.
Keep the local fixture below tests. Do not duplicate R02's parsing matrix or
generated transport tests.

One R08 jj change contains implementation, tests, this plan, and epic approval
updates. Run core tests, Clippy with warnings denied, FFI compilation, and
formatting checks. Report unrelated format failures separately. No database,
framework, navigation, coordinator, runtime, binding, or Swift changes.

Validation: all core tests pass, including 14 form scenarios. Clippy with
warnings denied, FFI compilation, and changed-file formatting pass. Full
workspace formatting reports existing import-order differences in
`crates/kreatyna/tests/background.rs`, `crates/kreatyna/tests/composition.rs`,
and `crates/kreatyna-ffi/src/lib.rs`; these files are unchanged.
