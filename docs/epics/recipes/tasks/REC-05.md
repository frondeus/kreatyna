# REC-05 — Delete a recipe

Tracked in beads: `vibetyna-hrx.5`. Changes: six, listed in Steps.
Dependencies: REC-03 `vibetyna-hrx.3` (closed). REC-04 `vibetyna-hrx.4`
(closed) is not a dependency, but this plan builds on its code.

## Open questions

None. Each choice in this slice follows a rule that the user already
decided for the ingredient delete (MAC-07) or for the recipe read view
(REC-03, REC-04). The table in Decisions lists them, so you can object to one
before you accept the plan.

## Decisions

### Decided in the plan (each one follows an earlier decision)

| Rule | Follows from |
|---|---|
| **Delete** (🗑) is in the read view's toolbar, beside **Edit**. It asks "Delete Shakshuka?" with the message "This cannot be undone.", and the buttons **Delete** and **Cancel**. | MAC-07, MAC-07/Q1 (Swift writes the words) |
| **Delete** and **Edit** are enabled only when the read view shows a recipe and no delete runs. They are disabled while the recipe loads, after a failed load, for a missing recipe, and while the question or the delete runs. | MAC-07 (`isIdle`), REC-04 |
| **Cancel** closes the question. The read view stays as it was. | MAC-07 |
| While Rust deletes, the pane shows a "Deleting…" overlay. | MAC-07 |
| After the delete, the pane shows "Select a recipe", no row is highlighted, and the list loads again without the recipe. | The REC-05 card ("empties the detail pane"), E-08 (the list loads again after a write, as after Save) |
| A failed delete asks the question again. Its message shows the error, for example "The recipe could not be deleted from storage." | MAC-07 |
| The delete of a recipe that is already gone succeeds. The pane empties, and the list loads again. | MAC-07: the ingredient store deletes idempotently. |
| From the question until the delete ends, Rust ignores row clicks, ⌘-click, **New Recipe**, **Edit**, and a second **Delete**. **New Recipe** is disabled. | MAC-07 (`detail_is_locked`) |
| No ⌘⌫ shortcut. | MAC-07: ⌘⌫ deletes text in the search field (REC-06). |
| The store deletes the recipe's row. Rows that a recipe owns (lines in REC-07, steps in REC-14) will use `ON DELETE CASCADE`. `PRAGMA foreign_keys = ON` is already set in `crates/kreatyna-db/src/sqlite.rs`. Today a recipe owns no other rows, so no migration. | The REC-05 card: "Deletion cascades to everything the recipe owns at that point." |
| No snapshot of the question. The snapshot shows the "Deleting…" overlay. The demo screenshot shows the question. | MAC-07, MAC-06 difference 1: macOS draws the dialog in its own sheet window, which an off-screen snapshot does not capture. The card asks for a snapshot of the question; this rule replaces it. |
| iOS does not change. | E-04 |

## Outcome

The list has Pasta and Shakshuka. Shakshuka is open in the read view. You
click **Delete** (🗑).

- Kreatyna asks "Delete Shakshuka?" with "This cannot be undone.".
- **Cancel** closes the question. Shakshuka stays open.
- **Delete** shows "Deleting…" for a moment. Then the pane shows "Select a
  recipe", and the list shows only Pasta.
- You quit and open Kreatyna again. The list still shows only Pasta.
- You create a new recipe called "Shakshuka". The save succeeds: the title is
  free again.

## Terms

- **Read view:** the detail pane for one saved recipe, the `RecipeDetail`
  widget (REC-03).
- **Recipes section:** the `RecipeSection` widget. It owns the recipe list and
  the detail pane, and it decides what the pane shows.
- **Confirmation widget:** `crate::confirmation::Confirmation`, shared by
  every question in the app. It has two states: `AwaitingDecision`, and
  `Resolved { decision }` with `Confirmed` or `Cancelled`. It has no text;
  Swift writes the words.
- **Pane scope:** each new content of the detail pane gets a new
  `CommandScopeId`. Rust drops late events and results of old content (E-19).
- **`Latest<T>`:** a krucyfiks wrapper for the result of an operation that a
  newer one can replace. The widget accepts only the newest result.
- **Idempotent delete:** deleting a missing row is not an error. The result is
  the same: the row does not exist.
- **`ON DELETE CASCADE`:** a SQLite foreign-key rule. When the parent row is
  deleted, SQLite deletes the child rows too. It works only with
  `PRAGMA foreign_keys = ON`.

## Design

The target is the ingredient delete on the Mac (MAC-07), with the recipe's
title: see `docs/epics/mac-shell/tasks/MAC-07-demo/`. The read view keeps the
REC-03 layout and gets **Delete** in its toolbar. No new mockup; the demo
shows the result.

## Changes by layer

### db (`crates/kreatyna-db/`)

| File | Change |
|---|---|
| `src/recipe_store.rs` | `delete(id)` in an immediate transaction: one `DELETE` by `remote_id`. Zero deleted rows is not an error. A failure gives `RecipeDeleteError::Storage`. |
| `src/recipe_store/tests.rs` | See Tests. |

No migration and no model change.

### rust (`crates/kreatyna/src/recipes/`)

**`capabilities.rs`** — one new store method and one new error (*sketch*):

```rust
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecipeDeleteError {
    #[error("Recipe storage failed")]
    Storage,
    #[error("Recipe store transport failed")]
    Transport,
}

pub trait RecipeStore: Send + Sync {
    // list, get, create and update as today.
    /// Deletes the recipe with this id and every row that it owns. Deleting
    /// a missing recipe succeeds (idempotent).
    async fn delete(&self, id: RecipeId) -> Result<(), RecipeDeleteError>;
}
```

**`detail.rs`** — the read view asks, deletes, and reports. The delete state
lives only inside `Ready`, because only a shown recipe can be deleted
(*sketch*):

```rust
pub enum RecipeDetailEvent {
    Retry,
    Edit,
    /// The Delete button. Ignored unless the recipe is shown and no delete runs.
    RequestDelete,
    /// Delete or Cancel in the question.
    Confirmation(ConfirmationEvent),
}

pub enum RecipeDetailOutcome {
    LoadFinished(Latest<Result<Option<Recipe>, RecipeLoadError>>),
    OpenEdit { recipe: Recipe },
    ConfirmationDecided(Latest<ConfirmationOutcome>),
    DeleteFinished(Latest<Result<(), RecipeDeleteError>>),
    /// The store deleted the recipe. The section empties the pane.
    Deleted { id: RecipeId },
}

pub enum RecipeDetailStatus {
    Loading,
    Failed(RecipeLoadError),
    Missing,
    Ready { title: String, deletion: RecipeDeletion },
}

/// The delete of the shown recipe (MAC-07).
pub enum RecipeDeletion {
    Idle,
    /// The question is open. `error` is the last failed delete, if any.
    Confirming { confirmation: ConfirmationView, error: Option<RecipeDeleteError> },
    Deleting,
    /// The store deleted it; the section replaces the pane next.
    Deleted,
}
```

| Event or outcome | Status | Result |
|---|---|---|
| `RequestDelete` | Ready, Idle | Confirming, no error. Render. |
| `RequestDelete` | other | Ignored. |
| `Edit` | Ready, not Idle | Ignored (new: REC-04 accepted any Ready). |
| `Confirmation(Confirm)` | Confirming | Deleting. Render, then `store.delete(id)`. |
| `Confirmation(Cancel)` | Confirming | Idle. Render. |
| `DeleteFinished(Ok)` | Deleting | Deleted. Render, then `Deleted { id }`. |
| `DeleteFinished(Err(e))` | Deleting | Confirming with `error: Some(e)`. Render. |
| `Retry` | Ready | Ignored, as today. |

**`section.rs`** — the section empties the pane after a delete, and locks the
pane while a delete is asked or runs (*sketch*):

```rust
impl RecipeSection {
    /// The question is open, or a delete runs or has finished.
    fn detail_is_locked(detail: &RecipeDetail) -> bool;
}
```

| Event or outcome | Pane | Result |
|---|---|---|
| `Deleted { id }` | Detail of `id`, deletion Deleted | Empty in a new scope. The list loads again. |
| `Deleted` | other, or an old scope | Ignored. |
| `Select(id)`, `ClearSelection`, `NewRecipe` | Detail, locked | Ignored. |
| `OpenEdit` | Detail, locked | Ignored. |

| Pane | `can_create` | `selected` |
|---|---|---|
| Detail, locked | false | the shown recipe's id |

The other rows of the REC-03 and REC-04 tables do not change.

**`mod.rs`** — re-exports `RecipeDeleteError` and `RecipeDeletion`.
**`crates/kreatyna/src/app.rs`** does not change.

References to update:

- `rg -l 'RecipeStoreRequest::'` (the new `Delete` request in test drivers):
  `crates/kreatyna/tests/{recipe_editor,recipe_section,app}.rs`.
- `rg -l 'RecipeDetailStatus::Ready'` (the new `deletion` field):
  `crates/kreatyna/src/recipes/{detail,section}.rs`,
  `crates/kreatyna/tests/{recipe_detail,recipe_section,app}.rs`,
  `crates/kreatyna-ffi/src/{app_types/recipes,app_types/tests,app_runtime_tests}.rs`.
- `rg -l 'RecipeDetailEvent|RecipeDetailOutcome'` (the new cases):
  `crates/kreatyna/src/recipes/{detail,section,mod}.rs`,
  `crates/kreatyna/tests/{recipe_detail,recipe_section}.rs`,
  `crates/kreatyna-ffi/src/app_types/{recipes,tests}.rs`,
  `apple/Kreatyna/KreatynaMac/MacRecipeDetail.swift`,
  `apple/Kreatyna/SharedTests/AdapterPlumbingTests.swift`.
- Swift `.ready(title:)` (the new `deletion` field):
  `apple/Kreatyna/KreatynaMac/MacRecipeDetail.swift`,
  `apple/Kreatyna/SharedTests/AdapterPlumbingTests.swift`,
  `apple/Kreatyna/KreatynaMacTests/Snapshots/MacScreenSnapshotTests.swift`.

### ffi (`crates/kreatyna-ffi/src/`)

| File | Change |
|---|---|
| `app_types/recipes.rs` | `RecipeDeleteError`, `RecipeDeletion` (the four states; `Confirming { confirmation, error }`), `RecipeDetailStatus::Ready { title, deletion }`, `RecipeDetailEvent::RequestDelete` and `Confirmation { event }`. |
| `app_types/tests.rs` | Conversion tests for the new cases. |
| `app_runtime_tests.rs` | The real runtime on a new SQLite file: create "Shakshuka", Delete, Confirm. The pane is Empty, and the list is empty. |

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `ErrorText.swift` | `RecipeDeleteError.message`: "The recipe could not be deleted from storage." and "Recipes are temporarily unavailable." |
| `IngredientText.swift` | `DeleteText.message` gets an overload for `RecipeDeleteError?`. |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacRecipeDetail.swift` | **Delete** (🗑, destructive) sends `.requestDelete`. **Edit** and **Delete** are enabled only for `.ready` with `.idle`. The question uses `DeleteText` and sends `.confirmation(.confirm)` or `.confirmation(.cancel)`. A "Deleting…" overlay for `.deleting` and `.deleted`. |

`MacRecipePane`, `MacRecipeList` and `MacMainView` do not change.

### tests

| Layer | File | Tests |
|---|---|---|
| db | `src/recipe_store/tests.rs` | Delete removes the recipe from `list` and `get`, and keeps the other recipes. Delete of an unknown id succeeds and changes nothing. After a delete, a new recipe can take the deleted title. |
| Rust widget | `tests/recipe_detail.rs` | RequestDelete opens the question. Cancel goes back to Idle. Confirm deletes the shown id, then reports `Deleted`. A failed delete asks again with the error. RequestDelete while loading, failed, missing, asking or deleting is ignored. Edit while asking or deleting is ignored. A late `DeleteFinished` from a replaced delete is dropped. |
| Rust widget | `tests/recipe_section.rs` | A confirmed delete empties the pane in a new scope, clears `selected`, and loads the list again. A failed delete keeps the read view. While the question is open: a row click, ⌘-click, New Recipe and Edit are ignored, and `can_create` is false. A `Deleted` from an old scope is ignored. |
| Rust app | `tests/app.rs` | A delete routes through the app. |
| ffi | `app_types/tests.rs`, `app_runtime_tests.rs` | See the ffi table. |
| Swift adapter | `SharedTests/AdapterPlumbingTests.swift` | `detail(.requestDelete)` and `detail(.confirmation(event: .confirm))` carry the pane scope. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | `deletingRecipe` (light and dark): 2 new images, the overlay with **Edit** and **Delete** off. `recipeReadView` (light and dark) changes: **Delete** appears. I check each image by eye. |
| UI smoke (Mac) | `MacRecipeUITests.swift` | `testDeletedRecipeStaysGoneAfterRelaunch`: create "Shakshuka", Delete, Cancel (it stays), Delete, Delete; the pane shows "Select a recipe" and the list is empty; relaunch; the list is still empty. |
| Demo (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureREC05Demo`: `01-delete-question`, `02-after-delete`. |

### docs

| File | Change |
|---|---|
| `docs/epics/recipes/epic.html` | REC-05 card: `plan: "tasks/REC-05.md"`. |
| `docs/epics/recipes/tasks/REC-05-demo/` (new) | The 2 demo screenshots. |

## Steps

The slice is about 1,800 lines of diff, so it is six jj changes, each under
1,000 lines. Each change builds and passes every check alone. You review
each change before the next one starts. A change with no `.rs` file gets a
report and no review.

### Change 1 — REC-05: plan (about 350 lines, no review)

This plan, the REC-05 card on `epic.html`, and the beads.

### Change 2 — REC-05: delete a recipe in the store (about 250 lines, review)

1. Add `RecipeDeleteError` and `RecipeStore::delete`.
2. Add `delete` to the SQLite store, with its tests.
3. Add a `Delete` arm to the test drivers that match on `RecipeStoreRequest`.
4. Run checks 1 to 3 and check 7.
5. Send the guided review.

### Change 3 — REC-05: the read view deletes its recipe (about 450 lines, review)

1. Add `RecipeDeletion`, the `deletion` field of `Ready`, and the new events and outcomes.
2. Add the rows of the detail table above.
3. Extend `tests/recipe_detail.rs`, and update the `Ready` values in the other tests.
4. Add the FFI field with a fixed `Idle`, so the change builds alone. Change 5 replaces it.
5. Run checks 1 to 3 and check 7.
6. Send the guided review.

### Change 4 — REC-05: the section empties the pane after a delete (about 350 lines, review)

1. Add `detail_is_locked`, and the section rows of the tables above.
2. Extend `tests/recipe_section.rs` and `tests/app.rs`.
3. Run checks 1 to 3 and check 7.
4. Send the guided review.

### Change 5 — REC-05: delete in the FFI (about 250 lines, review)

1. Add the FFI types and their tests.
2. Run `mac-bindings debug`.
3. Update the Swift call sites so the build stays green. **Delete** does not
   show until change 6.
4. Run checks 1 to 5 and check 7.
5. Send the guided review.

### Change 6 — REC-05: delete a recipe on the Mac (about 300 lines, report)

1. Add **Delete**, the question and the overlay to `MacRecipeDetail`.
2. Add the error text, the adapter tests and the snapshot test. Record the 2
   new images. Look at each new and changed image.
3. Add the UI test and the demo method.
4. Run checks 4, 5 and 7.
5. Capture the demo (check 6).
6. Fill in Validation in this plan.
7. Send the demo and the report to the REC-05 Inbox thread.

If a change is over 1,000 lines, it becomes two changes at a file boundary.

## Checks

Test scope: Rust in full for each change. On the Mac, all unit and snapshot
tests, and the UI tests of the Recipes section only (`--ui MacRecipeUITests`).
The full suite runs once per cycle, in the cycle's full-suite step.

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. 2 new images and 2 changed images, each checked by eye. No other image changes. |
| 5 | `devenv shell -- ios-app-build` | Builds. iOS is parked, so only the build runs. |
| 6 | `capture-demo.sh mac …/testCaptureREC05Demo docs/epics/recipes/tasks/REC-05-demo` | `01-delete-question.png`, `02-after-delete.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| A recipe owns no other rows, so the delete removes one row. | REC-07 (lines) and REC-14 (steps) add their tables with `ON DELETE CASCADE`, and a store test that the delete removes them. |
| No search. | REC-06 |

- No undo. The question says "This cannot be undone."
- No delete from the list (a context menu or a swipe). The card asks only
  for the read view.
- No recipe screens on iOS (E-04).

## Validation

Run on 2026-11-03, on the last change of the slice.

| # | Check | Result |
|---|---|---|
| 1 | `cargo test --workspace` | 662 passed, 0 failed. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings (run on changes 2 to 5; change 6 has no Rust). |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | Passed: 83 Swift Testing cases (every Mac unit and snapshot test) and the 7 recipe UI tests. The 2 new images (`deletingRecipe`, light and dark) and 4 changed images were checked by eye: `recipeReadView` gains an enabled **Delete**, and `recipeReadViewFailed` gains a disabled **Delete**. No other image changed. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh … testCaptureREC05Demo` | `REC-05-demo/01-delete-question.png`, `02-after-delete.png`. |
| 7 | Change sizes (insertions + deletions) | Plan 334, store 90, read view 556, section 293, FFI 208, Mac 189 (without images). All under 1,000. |

### Differences from the plan

- **Change 3 dropped the new field in the FFI conversion** (`Ready { title, .. }`)
  instead of adding an FFI field with a fixed `Idle`. The FFI type then
  changed once, in change 5.
- **`recipeReadViewFailed` changed too** (light and dark). The toolbar shows
  **Delete** in every read view state, disabled unless the recipe is shown,
  the same as **Edit**. The plan expected only `recipeReadView` to change.
- **The transport error text** is "The recipe could not be deleted right
  now.", the same wording as the ingredient delete, not "Recipes are
  temporarily unavailable.".
- **The Swift fixture's `canCreate`** now follows Rust: **New Recipe** is off
  while a delete is asked or runs.
