# REC-04 — Edit a recipe's title

Tracked in beads: `vibetyna-hrx.4`. Changes: six, listed in Steps.
Dependencies: REC-03 `vibetyna-hrx.3` (closed).

## Open questions

None. Each choice in this slice follows a rule that the user already
decided for ingredients (MAC-06) or for the recipe editor (REC-02, REC-03).
The table in Decisions lists them, so you can object to one before you
accept the plan.

## Decisions

### REC-04/Q1 — The type that `update` writes

*Context:* the review of change 2 asked whether create and update should share
one type, and whether update could be a patch with every field optional. A
patch needs the editor to compute the changed fields and the store to merge
them, and REC-22 checks the whole recipe. Lines and steps (REC-07, REC-14) are
collections, so a real patch needs add, remove and move operations.

*Decision:* **two full types** (not the recommendation, which was one shared
type). `create` takes `NewRecipe`, and `update` takes `RecipeUpdate`. Both
carry every field of the recipe; they have the same fields for now, and they
can differ later without a rename. Update is not a patch.

### Decided in the plan (each one follows an earlier decision)

| Rule | Follows from |
|---|---|
| The edited row stays highlighted and shows the saved title until Save. No draft row appears. | MAC-06/Q1 |
| The editor's heading is the saved title, for example "Shakshuka", until Save. For a new recipe it stays "New Recipe". | MAC-06/Q1 |
| **New Recipe** is disabled while the editor is open. | REC-02: `can_create` is false while the editor is open. |
| A click on the edited row does nothing. A ⌘-click on it does nothing. | MAC-06 |
| A click on another row: an unchanged editor closes and the clicked recipe opens. A changed editor asks "Discard changes?" first. | E-10, REC-03/Q2 |
| Cancel and **Discard Changes** go back to the read view of the edited recipe. | MAC-06 |
| The editor is "changed" when the title differs from the title that it opened with. Typing the old title again makes it unchanged. | MAC-06 (`dirty` compares with the opened values) |
| Save with an unchanged title saves. The store writes the same title. | MAC-06: Save does not check `dirty`. |
| After Save, the read view loads the recipe again, and the list loads again. The row moves to its sorted place and stays highlighted. | E-08, MAC-05/Q3 |
| Esc is Cancel, and ⌘S is Save. **Edit** gets no shortcut. | MAC-06 |
| **Edit** is enabled only when the read view shows a recipe. It is disabled while the recipe loads, after a failed load, and for a missing recipe. | MAC-06 (`isIdle`) |
| A save of a recipe that no longer exists fails with "This recipe no longer exists." The editor stays open. Cancel then shows the read view, which says that the recipe is missing and loads the list again. | MAC-06: `IngredientSaveError::NotFound` keeps the form. REC-03: `Missing`. |
| No snapshot of "Discard changes?". The demo screenshot shows it. | MAC-06, difference 1: macOS draws the dialog in its own sheet window, which an off-screen snapshot does not capture. |
| iOS does not change. | E-04 |

## Outcome

Shakshuka is open in the read view. You click **Edit** (✎). The detail pane
shows the editor with the title "Shakshuka" in the field and the heading
"Shakshuka". The Shakshuka row stays highlighted.

- You change the title to "Spicy Shakshuka" and press ⌘S. The pane shows the
  read view of "Spicy Shakshuka". The row reads "Spicy Shakshuka", at its
  sorted place, highlighted.
- You change the title to "Pasta", which another recipe has, and press ⌘S.
  The editor says "A recipe with this title already exists".
- You change the title and press Esc. Kreatyna asks "Discard changes?".
  **Discard Changes** shows the read view of Shakshuka, as it was saved.
  **Keep Editing** keeps the editor.
- You change the title and click **Pasta**. The same question appears.
  **Discard Changes** opens Pasta.
- With no change, Esc or a click on Pasta leaves the editor at once.

## Terms

- **Read view:** the detail pane for one saved recipe, the `RecipeDetail`
  widget (REC-03).
- **Editor:** the `RecipeEditor` widget. In REC-02 it creates a recipe. In
  REC-04 it can also edit a saved recipe. The two uses differ in the title
  they open with and in the store call that Save makes.
- **Recipes section:** the `RecipeSection` widget. It owns the recipe list and
  the detail pane, and it decides what the pane shows.
- **Pane scope:** each new content of the detail pane gets a new
  `CommandScopeId`. Rust drops late events and results of old content (E-19).
- **`return_to` and `clicked_row`:** two fields of the editor pane. They say
  where the pane goes after **Discard Changes**. For the edit editor,
  `return_to` is the edited recipe. The ingredient catalog has the same
  fields and a table that explains them
  (`crates/kreatyna/src/ingredients/split_catalog.rs`, `FormPane`).
- **Immediate transaction:** a SQLite transaction that takes the write lock
  at its start. Two saves at the same time then run one after the other, so
  the title check and the write see the same rows.

## Design

The target is section 2 of the
[recipes mockups](../../../tasks/recipes/mockups.html) (the editor), reduced
to the title as in REC-02. The edit editor is the REC-02 editor with the
saved title in the field and as the heading. It copies the ingredient edit
form (`MacIngredientForm.swift`, the `title` parameter). No new mockup; the
demo shows the result.

## Changes by layer

### db (`crates/kreatyna-db/`)

| File | Change |
|---|---|
| `src/recipe_store.rs` | `update(id, data)` in an immediate transaction. No row with this `remote_id` gives `NotFound`, and nothing is inserted. A title key that another recipe has gives `DuplicateTitle`. The recipe's own key is not a conflict, so "Shakshuka" can become "shakshuka". The unique index on `title_key` still guards the write. |
| `src/recipe_models.rs` | No change: `NewRecipeRow::new(id, &title)` builds the row for both writes. No migration. |
| `src/recipe_store/tests.rs` | See Tests. |

*sketch:*

```rust
fn update_in_transaction(
    connection: &mut SqliteConnection,
    id: RecipeId,
    data: RecipeUpdate,
) -> Result<Recipe, StoreFailure> {
    let row = NewRecipeRow::new(id, &data.title);
    let Some(local_id) = local_id(connection, id)? else {
        return Err(StoreFailure::NotFound);
    };
    let taken = recipes::table
        .filter(recipes::title_key.eq(&row.title_key))
        .filter(recipes::local_id.ne(local_id))
        .select(recipes::local_id)
        .first::<i64>(connection)
        .optional()?;
    if taken.is_some() {
        return Err(StoreFailure::DuplicateTitle);
    }
    write_title(connection, local_id, &row)?; // UniqueViolation → DuplicateTitle
    Ok(row.into_domain())
}
```

### rust (`crates/kreatyna/src/recipes/`)

**`model.rs`** — a new type for update, beside `NewRecipe` (REC-04/Q1,
*real*):

```rust
/// What `RecipeStore::update` writes: every field of the recipe, not a
/// patch. It has the same fields as `NewRecipe` for now, as its own type so
/// the two can differ later (REC-04/Q1).
pub struct RecipeUpdate {
    pub title: RecipeTitle,
}
```

**`capabilities.rs`** — one new store method and one new error (*sketch*):

```rust
pub enum RecipeSaveError {
    DuplicateTitle,
    /// Only `update`: no recipe has this id any more.
    NotFound,
    Storage,
    Transport,
}

pub trait RecipeStore: Send + Sync {
    // list, get and create as today.
    /// Saves new data for the recipe with this id. Fails with `NotFound` when
    /// no recipe has the id (never an insert), and with `DuplicateTitle` when
    /// another recipe has the same title key.
    async fn update(&self, id: RecipeId, recipe: RecipeUpdate) -> Result<Recipe, RecipeSaveError>;
}
```

**`detail.rs`** — the read view asks for the editor (*sketch*):

```rust
pub enum RecipeDetailEvent {
    Retry,
    /// The Edit button. Ignored unless the recipe is shown.
    Edit,
}

pub enum RecipeDetailOutcome {
    LoadFinished(Latest<Result<Option<Recipe>, RecipeLoadError>>),
    /// The section opens the editor for this recipe.
    OpenEdit { recipe: Recipe },
}
```

| Event | Status | Result |
|---|---|---|
| `Edit` | Ready | `OpenEdit { recipe }`, with the shown id and title. No render. |
| `Edit` | Loading, Failed, Missing | Ignored. |

**`editor.rs`** — the editor knows what it edits (*sketch*):

```rust
/// What Save writes to. Private: `new` and `edit` choose it.
enum EditorTarget {
    New,
    Saved { id: RecipeId },
}

pub struct RecipeEditor {
    caps: RecipeCapabilities,
    target: EditorTarget,
    /// The title the editor opened with: empty for a new recipe.
    opened_with: String,
    title: String,
    // save_attempted, state and the two slots as today.
}

impl RecipeEditor {
    /// An editor for a new recipe, with an empty title.
    pub fn new(caps: RecipeCapabilities) -> Self;
    /// An editor for a saved recipe, with its title.
    pub fn edit(caps: RecipeCapabilities, recipe: &Recipe) -> Self;
    /// Whether the title differs from the one the editor opened with.
    pub(crate) fn is_dirty(&self) -> bool {
        self.title != self.opened_with
    }
}
```

Save calls `store.create(NewRecipe { title })` for `New` and
`store.update(id, RecipeUpdate { title })` for
`Saved`. Everything else in the editor stays: the title rule, the problems,
"Discard changes?" and the outcomes. `RecipeEditorView` does not change.

**`section.rs`** — the pane gets the edit editor (*sketch*):

```rust
pub enum RecipePaneView {
    Empty,
    Detail(RecipeDetailView),
    Create { editor: RecipeEditorView, draft_row: String },
    /// `saved_title` is the title as stored: the heading shows it until Save
    /// (MAC-06/Q1).
    Edit { editor: RecipeEditorView, saved_title: String },
}

// Inside the section. `CreatePane` becomes `EditorPane`, for both editors.
enum Pane {
    Empty,
    Detail(RecipeDetail),
    Create(EditorPane),
    /// `saved` is the edited recipe as stored. The row and the heading show it.
    Edit { saved: Recipe, pane: EditorPane },
}
```

| Event or outcome | Pane | Result |
|---|---|---|
| `OpenEdit { recipe }` | Detail of `recipe.id` | Edit in a new scope; `return_to` is `recipe.id`. |
| `OpenEdit` | other, or an old scope | Ignored. |
| `Select(id)`, `id` is the edited recipe | Edit | Ignored. |
| `Select(id)` | Edit, unchanged | Detail of `id` in a new scope. |
| `Select(id)` | Edit, changed | `clicked_row = id`, and the editor asks "Discard changes?". |
| `Select(id)` | Edit, saving or asking | Ignored. |
| `ClearSelection` | Edit | Ignored. |
| `NewRecipe` | Edit | Ignored. |
| `Saved { recipe }` | Edit | The list loads again; Detail of `recipe.id` in a new scope. |
| `Discarded` | Edit | Detail of `clicked_row`, else of `return_to`. |
| The editor goes back to Editing (Keep Editing, a failed save) | Edit | `clicked_row = None`. |

| Pane | `can_create` | `selected` |
|---|---|---|
| Edit | false | the edited recipe's id |

The Create rows of the REC-03 table do not change.

**`mod.rs`** — re-exports. **`crates/kreatyna/src/app.rs`** does not change.

References to update:

- `RecipeUpdate` (new): `crates/kreatyna/src/recipes/{model,mod,capabilities,editor}.rs`,
  `crates/kreatyna-db/src/{recipe_store,recipe_store/tests}.rs`.
- `rg -l 'RecipeSaveError'` (the new `NotFound`):
  `crates/kreatyna/src/recipes/{capabilities,editor}.rs`,
  `crates/kreatyna-db/src/{recipe_store,recipe_store/tests}.rs`,
  `crates/kreatyna-ffi/src/{app_types/recipes,app_runtime_tests}.rs`,
  `crates/kreatyna/tests/{recipe_section,recipe_editor}.rs`, `crates/kreatyna-ffi/src/app_types/tests.rs`,
  `apple/Kreatyna/Shared/ErrorText.swift`.
- `rg -l 'RecipePaneView|RecipeDetailEvent|RecipeDetailOutcome'` (the new
  cases): `crates/kreatyna/src/recipes/{detail,section,mod}.rs`,
  `crates/kreatyna/tests/{recipe_detail,recipe_section,app}.rs`,
  `crates/kreatyna-ffi/src/{app_types/recipes,app_types/tests,app_runtime_tests}.rs`,
  `apple/Kreatyna/Shared/AppEventComposition.swift`,
  `apple/Kreatyna/KreatynaMac/MacRecipeDetail.swift`,
  `apple/Kreatyna/SharedTests/Fixtures.swift`,
  `apple/Kreatyna/KreatynaMacTests/Snapshots/MacScreenSnapshotTests.swift`.

### ffi (`crates/kreatyna-ffi/src/`)

| File | Change |
|---|---|
| `app_types/recipes.rs` | `RecipeDetailEvent::Edit`, `RecipePaneView::Edit { editor, saved_title }`, `RecipeSaveError::NotFound`. |
| `app_types/tests.rs` | Conversion tests for the three new cases. |
| `app_runtime_tests.rs` | The real runtime on a new SQLite file: create "Shakshuka", open it, Edit, set "Spicy Shakshuka", Save. The read view and the list show "Spicy Shakshuka". |

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `ErrorText.swift` | `RecipeSaveError.notFound`: "This recipe no longer exists." |
| `AppEventComposition.swift` | The `.edit` pane case where the code switches on the pane. `draftRow` stays `nil` for it. |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacRecipeDetail.swift` | **Edit** sends `.edit`. It is enabled only for `.ready`. |
| `MacRecipeEditor.swift` | A `heading` parameter: "New Recipe", or the saved title. |
| `MacRecipePane.swift` | `.edit(editor, savedTitle)`: `MacRecipeEditor` with `heading: savedTitle`, and `.id(scope)`. |

`MacRecipeList` and `MacMainView` do not change: the list already highlights
Rust's `selected`.

### tests

| Layer | File | Tests |
|---|---|---|
| db | `src/recipe_store/tests.rs` | Update renames the recipe; `get` and `list` show the new title, in its sorted place. Update to the recipe's own title in another case succeeds. Update to another recipe's title gives `DuplicateTitle`, and both titles stay. Update of an unknown id gives `NotFound`, and the list does not change. |
| Rust widget | `tests/recipe_detail.rs` | Edit in Ready gives `OpenEdit` with the id and title. Edit while loading, failed or missing is ignored. |
| Rust widget | `tests/recipe_editor.rs` | An edit editor opens with the saved title and is unchanged. Save calls `update` with the id, then reports `Saved`. `NotFound` and `DuplicateTitle` show as problems. Cancel when unchanged gives `Discarded` at once. Cancel when changed asks "Discard changes?". Typing the saved title again makes it unchanged. |
| Rust widget | `tests/recipe_section.rs` | Edit opens the editor in a new scope, keeps `selected`, and sets `can_create` false. Save loads the list again and shows the read view of the saved recipe. Cancel, unchanged: the read view of the recipe. Cancel, changed, Discard: the read view. A click on another row when unchanged opens it; when changed it asks, and Discard opens it; Keep Editing, then Cancel and Discard: the edited recipe. A click on the edited row, ⌘-click, New Recipe and an `OpenEdit` from an old scope are ignored. |
| Rust app | `tests/app.rs` | Edit and Save route through the app. |
| ffi | `app_types/tests.rs`, `app_runtime_tests.rs` | See the ffi table. |
| Swift adapter | `SharedTests/AdapterPlumbingTests.swift` | `detail(.edit)` carries the pane scope. `editor(_:)` on the edit pane carries its scope. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | `editRecipeEditor` (light and dark): 2 new images. `recipeReadView` (light and dark) changes: **Edit** is enabled, not grey. I check each image by eye. |
| UI smoke (Mac) | `MacRecipeUITests.swift` | `testRenameRecipe`: create "Shakshuka", Edit, type "Spicy Shakshuka", ⌘S; the read view and the row show "Spicy Shakshuka". `testCancelEditAsksToDiscard`: create "Shakshuka", Edit, type "Toast", Esc, **Discard Changes**; the read view shows "Shakshuka". |
| Demo (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureREC04Demo`: `01-edit-recipe`, `02-discard-question`, `03-after-rename`. |

### docs

| File | Change |
|---|---|
| `docs/epics/recipes/epic.html` | REC-04 card: `plan: "tasks/REC-04.md"`. |
| `docs/epics/recipes/tasks/REC-04-demo/` (new) | The 3 demo screenshots. |

## Steps

The slice is about 2,000 lines of diff, so it is six jj changes, each under
1,000 lines. Each change builds and passes every check alone. You review
each change before the next one starts. A change with no `.rs` file gets a
report and no review.

### Change 1 — REC-04: plan (about 400 lines, no review)

This plan, the REC-04 card on `epic.html`, and the beads.

### Change 2 — REC-04: update a recipe in the store (about 400 lines, review)

1. Add `RecipeUpdate` (REC-04/Q1).
2. Add `RecipeSaveError::NotFound` and `RecipeStore::update`.
3. Add `update` to the SQLite store, with its tests.
4. Map `NotFound` in the FFI error and in `ErrorText.swift`, so the build stays green.
5. Run checks 1 to 3 and check 7.
6. Send the guided review.

### Change 3 — REC-04: the editor edits a saved recipe (about 450 lines, review)

1. Add `RecipeDetailEvent::Edit` and `RecipeDetailOutcome::OpenEdit`.
2. Add `EditorTarget`, `opened_with` and `RecipeEditor::edit`.
3. Extend `tests/recipe_detail.rs` and `tests/recipe_editor.rs`.
4. Run checks 1 to 3 and check 7.
5. Send the guided review.

### Change 4 — REC-04: edit from the read view in the Recipes section (about 600 lines, review)

1. Rename `CreatePane` to `EditorPane`, and add `Pane::Edit` and `RecipePaneView::Edit`.
2. Handle `OpenEdit`, and the Edit rows of the table above.
3. Extend `tests/recipe_section.rs` and `tests/app.rs`.
4. Add a temporary FFI arm `Edit => Empty`, so the change builds alone. Change 5 replaces it.
5. Run checks 1 to 3 and check 7.
6. Send the guided review.

### Change 5 — REC-04: edit in the FFI (about 250 lines, review)

1. Add the FFI types and their tests.
2. Run `mac-bindings debug` and `ios-bindings debug`.
3. Update the Swift call sites so the build stays green. The Mac shows the
   empty pane for the edit editor until change 6.
4. Run checks 1 to 5 and check 7.
5. Send the guided review.

### Change 6 — REC-04: edit a recipe on the Mac (about 350 lines, report)

1. Enable **Edit** in `MacRecipeDetail`. Add `heading` to `MacRecipeEditor`
   and the `.edit` case to `MacRecipePane`.
2. Add the fixtures, the adapter tests and the snapshot test. Record the 2
   new images. Look at each new and changed image.
3. Add the UI tests and the demo method.
4. Run checks 4, 5 and 7.
5. Capture the demo (check 6).
6. Fill in Validation in this plan.
7. Send the demo and the report to the REC-04 Inbox thread.

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
| 6 | `capture-demo.sh mac …/testCaptureREC04Demo docs/epics/recipes/tasks/REC-04-demo` | `01-edit-recipe.png`, `02-discard-question.png`, `03-after-rename.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| The editor shows only the title. **Ingredients** and **Steps** say "Coming soon". | REC-07, REC-14 |
| No **Delete** in the read view. | REC-05 |
| No search. | REC-06 |

- Quitting with a changed editor does not ask "Discard changes?".
  `vibetyna-3rv` tracks that for both sections.
- No recipe screens on iOS (E-04).
- `Recipe.title` stays a `String`. The editor checks the title with
  `RecipeTitle` on Save, as today.

## Validation

Not run yet.
