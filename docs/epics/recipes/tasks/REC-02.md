# REC-02 — Create a recipe with just a title

Tracked in beads: `vibetyna-hrx.2`. Changes: six, listed in Steps. Dependencies:
REC-01 `vibetyna-hrx.1` (closed).

## Open questions

None. The user answered REC-02/Q1 in the Inbox on 2026-11-01 (see
Decisions).

## Decisions

### REC-02/Q1 — A draft row in the list while you create a recipe

*Context:* the ingredient catalog shows a draft row at the top of its list
while the create form is open (MAC-05/Q2). The recipe list had no such row.

*Decision:* yes, like the ingredient catalog (the recommendation). Rust
sends the trimmed title in `RecipePaneView::Create { draft_row, .. }`. Swift
draws one row at the top of the list; an empty title shows "New Recipe".

### REC-02/Q2 — nutype for validated newtypes

*Context:* the review of change 2 proposed the `nutype` crate for the
hand-written `RecipeTitle::parse`.

*Decision:* yes (the recommendation). `RecipeTitle` uses nutype 0.8.0:
`sanitize(trim)`, `validate(not_empty)`. `RecipeTitle::try_new` replaces
`parse`, and the generated `RecipeTitleError::NotEmptyViolated` replaces
`RecipeTitleIssue::Empty`. Every new validated newtype uses nutype; the rule
is in the `designing-types` skill. Bead `vibetyna-nci` (tech debt) moves the
existing newtypes to nutype.

### REC-02/Q3 — `Latest<T>` in krucyfiks

*Context:* the review of change 3 found `id: CommandId` copied into the
editor's outcome variants. krucyfiks had `Scoped<T>` for scopes, but
`.latest()` returned a bare `(CommandId, T)` tuple.

*Decision:* add `Latest<T> { id, value }` to krucyfiks, the twin of
`Scoped<T>`, with `LatestCommandSlot::accept(Latest<T>) -> Option<T>`.
`.latest()` returns `Command<Latest<T>>`. The editor's outcomes hold
`Latest<T>`. A small vibetyna change ("Adapt to krucyfiks Latest<T>") keeps
the old callers compiling; bead `vibetyna-bwu` moves their outcome variants
to `Latest<T>`. The skill `krucyfiks-tag-command-outcomes` teaches the
pattern.

### Review of change 3 (2026-11-01)

- **`try_save`.** `try_save` returns `Result<Command, RecipeTitleError>` and
  uses `?`; `save` turns the error into the editor's problem.

### Review of change 2 (2026-11-01)

- **No `unwrap` or `expect` in tests.** Tests return `anyhow::Result<()>`
  and use `?`. The rule is in the `write-rust-tests` skill. Bead
  `vibetyna-yzm` fixes the older tests.

### Decided in the plan (sent with Q1, no objection)

- **Save rule:** a non-empty title, unique by title key (E-02, D-39). Rust
  checks for an empty title on **Save**. After the first Save, Rust checks
  again on every change, as the ingredient form does. The store finds
  duplicates.
- **Trimmed title.** Rust saves the trimmed title: "  Shakshuka " is saved as
  "Shakshuka". Rust keeps the spaces inside the title.
- **One transaction.** The store checks the title key and inserts the row in
  one immediate transaction. If the unique index still rejects the insert
  (two writers on one file), the store returns `DuplicateTitle`, not
  `Storage`.
- **Discard changes?** **Cancel** with a typed title asks "Discard changes?"
  (E-10). **Cancel** on an untouched editor closes it at once.
- **After Save** the list loads again (REC-01/Q2) and the pane shows "Select
  a recipe". From REC-03, the pane opens the new recipe's read view.
- **New Recipe** and ⌘N work when the pane is empty. While the editor is open,
  **New Recipe** is disabled.
- Switching to **Ingredients** and back keeps the editor and the typed
  title (REC-01).
- **No migration.** REC-01 made the `recipes` table with a unique
  `title_key`.

## Outcome

You open Kreatyna on **Recipes**. The list says "No recipes yet". You click
**New Recipe** (or press ⌘N). The detail pane shows the editor: the title
"New Recipe", an empty **Title** field with the cursor in it, and empty
**Ingredients** and **Steps** areas that say "Coming soon". The toolbar has
**Cancel** and **Save**.

You type "Shakshuka" and click **Save**. The pane says "Select a recipe"
again, and the list shows one row: Shakshuka. You quit and open Kreatyna:
Shakshuka is still listed.

You click **New Recipe**, type "shakshuka ", and click **Save**. The editor
stays open and says "A recipe with this title already exists".

You click **Save** with an empty title. The editor says "Enter a title".

## Terms

- **Editor:** the `RecipeEditor` widget. In REC-02 it creates a recipe; REC-04
  adds editing an existing one.
- **Recipes section:** the `RecipeSection` widget, the coordinator of the
  Recipes sidebar item. It owns the recipe list and the detail pane, like
  `IngredientSplitCatalog` owns the ingredient list and its pane.
- **Pane scope:** each new content of the detail pane gets a new
  `CommandScopeId`. Rust drops late events and results of old content (E-19).
- **Title key:** the title folded for comparison (`ingredients::name_key`):
  trimmed and case-folded. "Shakshuka" and "shakshuka " have the same key
  (D-39). The `recipes.title_key` column is unique.
- **Immediate transaction:** a SQLite transaction that takes the write lock
  at its start (`BEGIN IMMEDIATE`). The duplicate check and the insert see
  the same database.
- **`LatestCommandSlot` and `Latest<T>`:** krucyfiks helpers. Each new
  command in the slot cancels the previous one. Its results come back as
  `Latest<T>`, and `slot.accept(latest)` returns the value only for the newest
  command (REC-02/Q3).
- **Snapshot test:** a test that renders a view off screen and compares the
  image with a recorded baseline.

## Design

The target is section 2 of the
[recipes mockups](../../../tasks/recipes/mockups.html) ("Recipe editor"):
**Cancel** and **Save** in the toolbar, the canvas, and the ingredient table.
REC-02 shows only the title field. The canvas and the ingredient table are
"Coming soon" placeholders (REC-07, REC-14). The editor copies the layout of
the ingredient form (`MacIngredientForm.swift`): a grouped `Form` with a
large title above it. No new mockup; the demo shows the result.

## Changes by layer

### db (`crates/kreatyna-db/`)

| File | Change |
|---|---|
| `src/recipe_models.rs` | `NewRecipeRow` (`Insertable`): `remote_id`, `title`, `title_key`. |
| `src/recipe_store.rs` | `create` in an immediate transaction: read the row with the same `title_key`; if one exists, fail with `DuplicateTitle`; insert; read the row back. A `UniqueViolation` from the insert also becomes `DuplicateTitle`. |
| `src/recipe_store/tests.rs` | See Tests. |

*sketch:*

```rust
fn create_in_transaction(
    connection: &mut SqliteConnection,
    recipe: NewRecipe,
) -> Result<Recipe, StoreFailure> {
    let row = NewRecipeRow::new(RecipeId::new(), &recipe.title);
    let taken = recipes::table
        .filter(recipes::title_key.eq(&row.title_key))
        .select(recipes::local_id)
        .first::<i64>(connection)
        .optional()?;
    if taken.is_some() {
        return Err(StoreFailure::DuplicateTitle);
    }
    diesel::insert_into(recipes::table).values(&row).execute(connection)?;
    Ok(Recipe { id: row.remote_id.0, title: row.title })
}
```

### rust (`crates/kreatyna/src/recipes/`)

**`model.rs`** — a title that passed the save rule, and the create request
(*sketch*):

```rust
/// A recipe title that passed the save rule: trimmed and not empty (E-02).
/// Uniqueness is the store's rule (D-39). nutype generates `try_new` and
/// `RecipeTitleError::NotEmptyViolated` (REC-02/Q2).
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct RecipeTitle(String);

/// What `RecipeStore::create` writes. REC-07 and REC-14 add lines and steps.
pub struct NewRecipe {
    pub title: RecipeTitle,
}
```

**`capabilities.rs`** (*sketch*):

```rust
pub enum RecipeSaveError {
    DuplicateTitle,
    Storage,
    Transport,
}

pub trait RecipeStore: Send + Sync {
    async fn list(&self) -> Result<Vec<Recipe>, RecipeLoadError>;
    /// Gives the recipe a new id. Fails with `DuplicateTitle` when the
    /// title key is taken.
    async fn create(&self, recipe: NewRecipe) -> Result<Recipe, RecipeSaveError>;
}
```

**`editor.rs`** (new) — the `RecipeEditor` widget. It copies the create part
of `IngredientForm`, with one field (*sketch*):

```rust
pub enum RecipeEditorEvent {
    SetTitle(String),
    Save,
    /// Cancel. A changed title asks "Discard changes?" first (E-10).
    RequestClose,
    Confirmation(ConfirmationEvent),
}

pub enum RecipeEditorOutcome {
    ConfirmationDecided(Latest<ConfirmationOutcome>),
    SaveFinished(Latest<Result<Recipe, RecipeSaveError>>),
    Saved { recipe: Recipe },
    Discarded,
}

/// Why the last Save did not save. Only an editable editor shows it.
pub enum RecipeEditorProblem {
    Invalid(RecipeTitleError),
    SaveFailed(RecipeSaveError),
}

pub enum RecipeEditorStatus {
    Editing { problem: Option<RecipeEditorProblem> },
    ConfirmingDiscard(ConfirmationView),
    Saving,
    Finished,
}

pub struct RecipeEditorView {
    pub title: String,
    pub status: RecipeEditorStatus,
}
```

`problem` lives inside `Editing`, so a saving editor cannot show an error
(designing-types, rule 1). One field replaces the ingredient form's
`issues` and `save_error`, which could both be set.

| Event or outcome | State | Result |
|---|---|---|
| `SetTitle(t)` | Editing | Store `t` raw. Clear a save error. After the first Save, check the title again. Render. |
| `Save`, empty title | Editing | `problem = Invalid(NotEmptyViolated)`. Render. |
| `Save`, valid title | Editing | `Saving`. Render, then `store.create`. |
| `SaveFinished(Ok)` | Saving | `Finished`. Render, then `Saved { recipe }`. |
| `SaveFinished(Err(e))` | Saving | `Editing { problem: SaveFailed(e) }`. Render. |
| `RequestClose`, title unchanged | Editing | `Finished`. Render, then `Discarded`. |
| `RequestClose`, title changed | Editing | `ConfirmingDiscard`. Render. |
| `Confirmation(Confirm)` → decided | ConfirmingDiscard | `Finished`, then `Discarded`. |
| `Confirmation(Cancel)` → decided | ConfirmingDiscard | `Editing`, with the previous problem. |
| Any other pair | any | Ignored. A stale `SaveFinished` (old id) is ignored. |

**`section.rs`** (new) — the `RecipeSection` coordinator. It copies the
create part of `IngredientSplitCatalog` (*sketch*):

```rust
pub enum RecipeSectionEvent {
    List(RecipeListEvent),
    /// New Recipe or ⌘N. Ignored while the pane is not empty.
    NewRecipe,
    Editor(Scoped<RecipeEditorEvent>),
}

pub enum RecipeSectionOutcome {
    List(RecipeListOutcome),
    Pane(Scoped<RecipeEditorOutcome>),
}

pub enum RecipePaneView {
    /// "Select a recipe". REC-03 adds the read view.
    Empty,
    Create {
        editor: RecipeEditorView,
        /// The typed title, trimmed, for the draft row at the top of the
        /// list. Empty shows "New Recipe" (REC-02/Q1).
        draft_row: String,
    },
}

pub struct RecipeSectionView {
    pub list: RecipeListView,
    /// The scope goes with editor events.
    pub pane: Scoped<RecipePaneView>,
    /// Whether New Recipe works now. Rust decides: REC-03 and REC-05 add
    /// panes where it does and does not.
    pub can_create: bool,
}
```

The draft row lives in the `Create` variant, so it cannot exist without the
editor (designing-types, rule 2).

| Event or outcome | Result |
|---|---|
| `init` | The list load. |
| `NewRecipe`, pane empty | The pane becomes `Create` in a new scope. Render. |
| `NewRecipe`, editor open | Ignored. |
| `Editor(e)` in the pane scope | To the editor. Another scope: ignored. |
| `Saved { recipe }` | The pane becomes `Empty` in a new scope. The list loads again. |
| `Discarded` | The pane becomes `Empty` in a new scope. Render. |
| `List(e)` | To the list. |

**`list.rs`** — the list loads again after a save. Today **Refresh** works
only after a failure, so one load runs at a time, and `LoadFinished` has no
command id (REC-01, deviation 2). A save can finish while the first load
still runs, so REC-02 adds a `LatestCommandSlot`: the newest load wins.

```rust
// sketch
pub enum RecipeListOutcome {
    LoadFinished(Latest<Result<Vec<Recipe>, RecipeLoadError>>),
}

impl RecipeList {
    /// Loads again after a save. `Ready` keeps its rows until the result,
    /// so the list does not flash "Loading". A failure shows `Failed`.
    pub(crate) fn reload(&mut self) -> Command<RecipeListOutcome>;
}
```

**`mod.rs`** — re-exports. **`crates/kreatyna/src/app.rs`** — the split
layout holds a `RecipeSection` in place of the `RecipeList`:

| Before | After |
|---|---|
| `SplitAppEvent::Recipes(RecipeListEvent)` | `SplitAppEvent::Recipes(RecipeSectionEvent)` |
| `AppOutcome::SplitRecipes(Scoped<RecipeListOutcome>)` | `AppOutcome::SplitRecipes(Scoped<RecipeSectionOutcome>)` |
| `SplitAppView::recipes: RecipeListView` | `SplitAppView::recipes: RecipeSectionView` |
| `SplitLayout::recipes: ScopedChild<RecipeList>` | `SplitLayout::recipes: ScopedChild<RecipeSection>` |

Shutdown does not change: a running save finishes, as the ingredient save
does (`shutdown_hook_preserves_an_accepted_ingredient_save_and_follow_up`).

References to update (`rg 'RecipeListView|RecipeListEvent|SplitRecipes|RecipeListOutcome'`):
`crates/kreatyna/src/app.rs`, `crates/kreatyna/tests/app.rs`,
`crates/kreatyna/tests/recipe_list.rs`, `crates/kreatyna-ffi/src/app_types/{app,recipes,tests}.rs`,
`crates/kreatyna-ffi/src/app_runtime_tests.rs`,
`apple/Kreatyna/Shared/AppEventComposition.swift`,
`apple/Kreatyna/KreatynaMac/{MacMainView,MacRecipeList}.swift`,
`apple/Kreatyna/SharedTests/{Fixtures,AdapterPlumbingTests}.swift`.

### ffi (`crates/kreatyna-ffi/src/`)

| File | Change |
|---|---|
| `app_types/recipes.rs` | `RecipeSaveError`, `RecipeTitleError`, `RecipeEditorProblem`, `RecipeEditorStatus`, `RecipeEditorView`, `RecipeEditorEvent`, `RecipePaneView`, `RecipeSectionView`, `RecipeSectionEvent`, with conversions. Same enum shapes as Rust. Scoped values carry the scope as a `u64`, like the ingredient pane. |
| `app_types/app.rs` | `SplitAppEvent::Recipes { event: RecipeSectionEvent }`; `SplitAppView::recipes: RecipeSectionView`. |
| `app_types/tests.rs` | Conversion tests for the new types. |
| `app_runtime_tests.rs` | The real runtime on a new SQLite file: create "Shakshuka"; the list shows it; a second "shakshuka" gets `DuplicateTitle`. |

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `AppEventComposition.swift` | `SplitAppView.recipes(_: RecipeSectionEvent)`; `RecipeSectionView.newRecipe`, `.list(_:)`, `.editor(_:)` build the events with the pane scope. |
| `ErrorText.swift` | `RecipeSaveError.message`: "A recipe with this title already exists" / "The recipe could not be saved." / "Recipes are temporarily unavailable." `RecipeTitleError.message`: "Enter a title". |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacRecipeEditor.swift` (new) | Title "New Recipe". A grouped `Form`: the **Title** field (`recipeTitleField`) with its problem under it (`recipeTitleProblem`); **Ingredients** and **Steps** sections with "Coming soon". Toolbar **Cancel** (`cancelRecipeEditor`, Esc) and **Save** (`saveRecipe`, ⌘S), a spinner while saving. The "Discard changes?" dialog. The cursor starts in Title. |
| `MacRecipePane.swift` (new) | `.empty`: "Select a recipe" (`recipeDetailPlaceholder`, moved from `MacMainView`). `.create`: `MacRecipeEditor`. |
| `MacRecipeList.swift` | **New Recipe** enabled by `canCreate`, with ⌘N. The draft row (`recipeDraftRow`) at the top while the pane is `.create`. "No recipes yet" hides while the draft row shows. |
| `MacMainView.swift` | The Recipes detail shows `MacRecipePane`. |

### tests

| Layer | File | Tests |
|---|---|---|
| db | `src/recipe_store/tests.rs` | Create returns the trimmed title and a new id, and the list shows it. Two titles in title-key order. Create "shakshuka " after "Shakshuka": `DuplicateTitle`, and the list still has one row. A row written with SQL with the same key: `DuplicateTitle`. |
| Rust model | `tests/recipe_domain.rs` (new) | `RecipeTitle::try_new`: trims and keeps inner spaces; `""`, `"   "` give `NotEmptyViolated` (`test_case`). |
| Rust widget | `tests/recipe_editor.rs` (new) | Save sends the trimmed title, then `Saved`. Empty title: `Invalid(NotEmptyViolated)`, no store call; the issue clears when you type. Duplicate and storage errors show; typing clears them. Cancel untouched: `Discarded`. Cancel dirty: confirm → `Discarded`; keep editing → `Editing` with the problem. A stale save result is ignored. |
| Rust widget | `tests/recipe_section.rs` (new) | New Recipe opens the editor with an empty draft row; a second New Recipe is ignored. The draft row trims. Save: pane empty, list reloads with the row. Discard: pane empty. An editor event with an old scope is ignored. A save that finishes during the first list load: the newest load wins. |
| Rust widget | `tests/recipe_list.rs` | `LoadFinished` takes an id. Reload keeps `Ready` rows until the result. An old load result is ignored. |
| Rust app | `tests/app.rs` | Recipe section events route through the app. Switching to Ingredients and back keeps an open editor and its title. Shutdown keeps an accepted recipe save. |
| ffi | `app_types/tests.rs`, `app_runtime_tests.rs` | See the ffi table. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | `emptyRecipeEditor` and `duplicateRecipeTitle`, light and dark: 4 new images. |
| UI smoke (Mac) | `KreatynaMacUITests.swift` | `testCreatedRecipeSurvivesRelaunch`: New Recipe, type "Shakshuka", Save; the row shows; quit and launch; the row still shows. `testDuplicateRecipeTitleIsRefused`: create "Shakshuka", then "shakshuka"; the message shows. |
| Demo (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureREC02Demo`: `01-empty-editor`, `02-saved`, `03-duplicate-title`. |

### docs

| File | Change |
|---|---|
| `docs/epics/recipes/epic.html` | REC-02 card: `plan: "tasks/REC-02.md"`. |
| `docs/epics/recipes/tasks/REC-02-demo/` (new) | The 3 demo screenshots. |

## Steps

The slice is about 3,000 lines of diff, so it is six jj changes, each under
1,000 lines. Each change builds and passes every check alone. You review
each change before the next one starts. A change with no `.rs` file gets a
report and no review.

### Change 1 — REC-02: plan (about 450 lines, no review)

This plan, the REC-02 card on `epic.html`, and the beads.

### Change 2 — REC-02: create a recipe in the store (about 350 lines, review)

1. Add `RecipeTitle` (nutype) and `NewRecipe` to `model.rs`, with tests.
2. Add `RecipeSaveError` and `RecipeStore::create`.
3. Add `NewRecipeRow` and `create` in the store.
4. Write the store tests.
5. Run checks 1 to 3 and check 7.
6. Send the guided review.

### Change 3 — REC-02: the recipe editor widget (about 550 lines, review)

1. Add `editor.rs`.
2. Write `tests/recipe_editor.rs`.
3. Run checks 1 to 3 and check 7.
4. Send the guided review.

### Change 4 — REC-02: the Recipes section widget (about 700 lines, review)

1. Add the command id and `reload` to `RecipeList`. Update `tests/recipe_list.rs`.
2. Add `section.rs`.
3. Write `tests/recipe_section.rs`.
4. Run checks 1 to 3 and check 7.
5. Send the guided review.

### Change 5 — REC-02: the Recipes section in the app and the FFI (about 600 lines, review)

1. Put `RecipeSection` in the split layout. Update `tests/app.rs`.
2. Add the FFI types and their tests.
3. Run checks 1 to 3.
4. Run `mac-bindings debug` and `ios-bindings debug`.
5. Update the Swift call sites so the build stays green. The Mac shows the list as before.
6. Run checks 4, 5, and 7.
7. Send the guided review.

### Change 6 — REC-02: create a recipe on the Mac (about 600 lines, report)

1. Add the error text.
2. Add `MacRecipeEditor` and `MacRecipePane`. Update `MacRecipeList` and `MacMainView`.
3. Add the fixtures and the snapshot tests. Record the 4 new images and look at each one.
4. Add the UI tests and the demo method.
5. Run checks 4, 5, and 7.
6. Capture the demo (check 6).
7. Fill in Validation in this plan.
8. Send the demo and the report to the REC-02 Inbox thread.

If a change is over 1,000 lines, it becomes two changes at a file boundary.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test` | All pass. 4 new images, each checked by eye. No existing image changes. |
| 5 | iOS `xcodebuild build-for-testing` (generic iOS Simulator) | Builds. iOS is parked, so only the build runs. |
| 6 | `capture-demo.sh mac …/testCaptureREC02Demo docs/epics/recipes/tasks/REC-02-demo` | `01-empty-editor.png`, `02-saved.png`, `03-duplicate-title.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| The editor's **Ingredients** and **Steps** areas say "Coming soon". | REC-07, REC-14 |
| Rows show the title only. No time, no step count. | REC-14, REC-21 |
| After Save, the pane says "Select a recipe"; rows cannot be selected. | REC-03 |
| The editor creates only; it cannot edit a saved recipe. | REC-04 |
| A recipe needs only a title. | REC-22 (D-37) |

- Quitting with an open, changed editor does not ask "Discard changes?".
  `vibetyna-3rv` tracks that for both sections.
- No title length limit, as for ingredient names.
- No recipe screens on iOS (E-04).
- No recipe seed for tests. The UI tests create recipes through the UI.

## Validation

Not run yet.
