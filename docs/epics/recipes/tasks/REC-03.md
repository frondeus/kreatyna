# REC-03 — Open a recipe to read it

Tracked in beads: `vibetyna-hrx.3`. Changes: five, listed in Steps.
Dependencies: REC-02 `vibetyna-hrx.2` (closed).

## Open questions

None. The user answered REC-03/Q1 and REC-03/Q2 in the Inbox on 2026-11-01
(see Decisions).

## Decisions

### REC-03/Q1 — Where the read view gets the recipe

*Context:* a recipe is only an id and a title today, and the list already
loads both. Ingredient lines arrive in REC-07 and steps in REC-14; from then
on the read view needs more than the list row.

*Decision:* **load by id now** (not the recommendation, which was to use the
list row). The store gets `get(id)`. The read view has its own states:
loading, failed with Retry, missing, and ready. REC-07 and REC-14 add lines
and steps to the same load. This adds the db layer to REC-03, and it becomes
epic decision E-08.

### REC-03/Q2 — A row click while the New Recipe editor is open

*Context:* the ingredient catalog already does this (E-10 of the Mac shell
epic).

*Decision:* the same as ingredients (the recommendation). An untouched
editor closes, and the clicked recipe opens. An editor with a typed title
asks "Discard changes?"; **Discard Changes** opens the clicked recipe, and
**Keep Editing** forgets the click. **Cancel** goes back to the recipe that
was open before **New Recipe**.

### Decided in the plan (sent with Q1 and Q2, no objection)

- The highlighted row is Rust's `selected`. Swift only reports the click.
- A click on the open recipe does nothing. ⌘-click on it clears the
  selection, and the pane shows "Select a recipe", as for ingredients.
- **Edit** shows in the read view's toolbar, but it is disabled until REC-04.
- **New Recipe** also works while a read view is open.
- iOS does not change (E-04).

## Outcome

You open Kreatyna on **Recipes**. The list shows Pasta and Shakshuka. You
click **Shakshuka**. The row is highlighted, and the detail pane shows the
read view: the title "Shakshuka", then an **Ingredients** section and a
**Steps** section. Each section says "None yet". The toolbar shows **Edit**,
disabled.

You click **Pasta**: the read view shows Pasta. You ⌘-click **Pasta**: the
pane says "Select a recipe".

You click **New Recipe**, type "Omelette", and click **Save**. The list shows
Omelette, highlighted, and the pane shows its read view.

You click **New Recipe** and type "Toast". You click **Shakshuka**. Kreatyna
asks "Discard changes?". You click **Discard Changes**: the pane shows
Shakshuka.

## Terms

- **Read view:** the detail pane for one saved recipe. Rust's widget for it is
  `RecipeDetail`, as `IngredientDetail` is for an ingredient. D-16 says that
  the read view shows steps as stages; REC-19 adds the stages.
- **Recipes section:** the `RecipeSection` widget. It owns the recipe list
  and the detail pane, like `IngredientSplitCatalog` owns the ingredient list
  and its pane.
- **Pane scope:** each new content of the detail pane gets a new
  `CommandScopeId`. Rust drops late events and results of old content (E-19).
  A click on another recipe starts a new scope, so a slow load of the
  previous recipe cannot replace the new one.
- **`LatestCommandSlot` and `Latest<T>`:** krucyfiks helpers. Each new
  command in the slot cancels the previous one, and `slot.accept(latest)`
  returns the value only for the newest command. The read view uses one for
  its load, so a Retry replaces a running load.
- **`return_to` and `clicked_row`:** two fields of the create pane. They say
  where the pane goes after **Discard Changes**. The ingredient catalog has
  the same fields and a table that explains them
  (`crates/kreatyna/src/ingredients/split_catalog.rs`, `FormPane`).
- **Snapshot test:** a test that renders a view off screen and compares the
  image with a recorded baseline.

## Design

The target is section 1 of the
[recipes mockups](../../../tasks/recipes/mockups.html) ("Main window"): the
title, then **Ingredients** and **Steps** headings, with **Edit** in the
toolbar. The **▶ Cook** button stays hidden (E-05). REC-03 shows the two
sections with "None yet". The read view copies the layout of the ingredient
detail (`MacIngredientDetail.swift`): a grouped `Form` with a large title
above it. No new mockup; the demo shows the result.

## Changes by layer

### db (`crates/kreatyna-db/`)

| File | Change |
|---|---|
| `src/recipe_store.rs` | `get(id)`: read the row with this `remote_id` in a transaction. No row gives `Ok(None)`. A row whose `title_key` does not match its title gives `Storage`, as `list` does. |
| `src/recipe_store/tests.rs` | See Tests. |

*sketch:*

```rust
fn record(connection: &mut SqliteConnection, id: RecipeId) -> Result<Option<Recipe>, StoreFailure> {
    let Some(row) = recipes::table
        .filter(recipes::remote_id.eq(RemoteRecipeId(id)))
        .select(RecipeRow::as_select())
        .first::<RecipeRow>(connection)
        .optional()?
    else {
        return Ok(None);
    };
    if row.title_key != name_key(&row.title) {
        return Err(StoreFailure::InvalidTitleKey);
    }
    Ok(Some(row.into_domain()))
}
```

No migration: `recipes.remote_id` is already unique (REC-01).

### rust (`crates/kreatyna/src/recipes/`)

**`capabilities.rs`** — one new store method (*sketch*):

```rust
pub trait RecipeStore: Send + Sync {
    async fn list(&self) -> Result<Vec<Recipe>, RecipeLoadError>;
    /// The recipe with this id, or `None` when no recipe has it.
    async fn get(&self, id: RecipeId) -> Result<Option<Recipe>, RecipeLoadError>;
    async fn create(&self, recipe: NewRecipe) -> Result<Recipe, RecipeSaveError>;
}
```

`get` reuses `RecipeLoadError` (`Storage`, `Transport`). A missing recipe is
not an error of the store: `None` says it.

**`detail.rs`** (new) — the `RecipeDetail` widget, the read view
(*sketch*):

```rust
pub enum RecipeDetailEvent {
    /// Load again after a failure (the Retry button). Ignored in other states.
    Retry,
}

pub enum RecipeDetailOutcome {
    LoadFinished(Latest<Result<Option<Recipe>, RecipeLoadError>>),
}

pub enum RecipeDetailStatus {
    Loading,
    Failed(RecipeLoadError),
    /// No recipe has this id any more, for example after a delete in another
    /// process. The section loads the list again.
    Missing,
    /// REC-07 adds the ingredient lines, REC-14 the steps.
    Ready { title: String },
}

pub struct RecipeDetailView {
    pub id: RecipeId,
    pub status: RecipeDetailStatus,
}

impl RecipeDetail {
    pub fn new(caps: RecipeCapabilities, id: RecipeId) -> Self;
}
```

The title lives inside `Ready`, so a loading or missing read view cannot show
a title (designing-types, rule 1).

| Event or outcome | State | Result |
|---|---|---|
| `init` | Loading | Render, then `store.get(id)`. |
| `LoadFinished(Ok(Some(r)))` | Loading | `Ready { title }`. Render. |
| `LoadFinished(Ok(None))` | Loading | `Missing`. Render. |
| `LoadFinished(Err(e))` | Loading | `Failed(e)`. Render. |
| `Retry` | Failed | `Loading`. Render, then `store.get(id)`. |
| `Retry` | other | Ignored. |
| An old `LoadFinished` | any | Ignored (`LatestCommandSlot`). |

**`section.rs`** — the pane gets the read view, and rows can be selected
(*sketch*):

```rust
pub enum RecipeSectionEvent {
    List(RecipeListEvent),
    /// New Recipe or ⌘N. Ignored while the editor is open.
    NewRecipe,
    /// A row click. Ignored for a row that the list does not show.
    Select(RecipeId),
    /// ⌘-click on the open recipe. The scope is the pane's.
    ClearSelection { scope: CommandScopeId },
    Detail(Scoped<RecipeDetailEvent>),
    Editor(Scoped<RecipeEditorEvent>),
}

pub enum RecipeSectionOutcome {
    List(RecipeListOutcome),
    Pane(Scoped<RecipePaneOutcome>),
}

/// An outcome of the pane's content.
pub enum RecipePaneOutcome {
    Detail(RecipeDetailOutcome),
    Editor(RecipeEditorOutcome),
}

pub enum RecipePaneView {
    /// "Select a recipe".
    Empty,
    Detail(RecipeDetailView),
    Create { editor: RecipeEditorView, draft_row: String },
}

pub struct RecipeSectionView {
    pub list: RecipeListView,
    /// The highlighted row: the open recipe. `None` while the editor is
    /// open; the draft row is highlighted then. REC-04 adds the edited recipe.
    pub selected: Option<RecipeId>,
    pub pane: Scoped<RecipePaneView>,
    pub can_create: bool,
}

// Inside the section:
enum Pane {
    Empty,
    Detail(RecipeDetail),
    Create(CreatePane),
}

struct CreatePane {
    editor: RecipeEditor,
    /// The recipe that was open when New Recipe was pressed. Cancel goes back to it.
    return_to: Option<RecipeId>,
    /// The row clicked while "Discard changes?" is open. Keep Editing clears it.
    clicked_row: Option<RecipeId>,
}
```

| Event or outcome | Pane | Result |
|---|---|---|
| `Select(id)`, `id` not in the list | any | Ignored. |
| `Select(id)` | Empty, or Detail of another recipe | Detail of `id` in a new scope. It starts loading. |
| `Select(id)` | Detail of `id` | Ignored. |
| `Select(id)` | Create, editor untouched | Detail of `id` in a new scope. |
| `Select(id)` | Create, title typed | `clicked_row = id`, and the editor asks "Discard changes?". |
| `Select(id)` | Create, saving or asking | Ignored. |
| `ClearSelection { scope }` | Detail in `scope` | Empty in a new scope. |
| `ClearSelection` | other, or an old scope | Ignored. |
| `NewRecipe` | Empty or Detail | Create in a new scope; `return_to` is the open recipe. |
| `NewRecipe` | Create | Ignored. |
| `Saved { recipe }` | Create | The list loads again; Detail of `recipe.id` in a new scope. |
| `Discarded` | Create | Detail of `clicked_row`, else of `return_to`, else Empty. |
| The editor goes back to Editing (Keep Editing, a failed save) | Create | `clicked_row = None`. |
| `Detail(e)` in the pane scope | Detail | To the read view. Another scope: ignored. |
| `LoadFinished` makes the read view `Missing` | Detail | The list loads again. The pane stays `Missing`. |

| Pane | `can_create` | `selected` |
|---|---|---|
| Empty | true | `None` |
| Detail | true | the recipe's id |
| Create | false | `None` |

**`list.rs`** — `RecipeList::contains(id) -> bool`: true when the list is
ready and has a row with this id.

**`mod.rs`** — re-exports. **`crates/kreatyna/src/app.rs`** does not change:
it already routes `RecipeSectionEvent` and `Scoped<RecipeSectionOutcome>`.

References to update (`rg 'RecipeSectionOutcome::Pane|RecipePaneView|RecipeSectionEvent'`):
`crates/kreatyna/src/recipes/section.rs`, `crates/kreatyna/tests/recipe_section.rs`,
`crates/kreatyna/tests/app.rs`, `crates/kreatyna-ffi/src/app_types/{app,recipes,support,tests}.rs`,
`crates/kreatyna-ffi/src/app_runtime_tests.rs`,
`apple/Kreatyna/Shared/AppEventComposition.swift`,
`apple/Kreatyna/KreatynaMac/{MacMainView,MacRecipeList,MacRecipePane}.swift`,
`apple/Kreatyna/SharedTests/{Fixtures,AdapterPlumbingTests}.swift`.

### ffi (`crates/kreatyna-ffi/src/`)

| File | Change |
|---|---|
| `app_types/recipes.rs` | `RecipeDetailEvent`, `RecipeDetailStatus`, `RecipeDetailView` (the id as a `String`), `RecipePaneView::Detail`, `RecipeSectionView::selected: Option<String>`, and `RecipeSectionEvent::{Select { id }, ClearSelection { scope }, Detail { scope, event }}`. `RecipeSectionEvent` becomes `TryFrom`, because an id string can be invalid. |
| `app_types/support.rs` | `AppTypeConversionError::InvalidRecipeId { value }` and `recipe_id(String)`, like `ingredient_id`. |
| `app_types/app.rs` | `SplitAppEvent::Recipes` converts with `try_into()?`. |
| `app_types/tests.rs` | Conversion tests: the new types, and an invalid recipe id gives `InvalidRecipeId`. |
| `app_runtime_tests.rs` | The real runtime on a new SQLite file: create "Shakshuka"; its read view opens with the title. |

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `AppEventComposition.swift` | `RecipeSectionView.select(_:)`, `.clearSelection`, `.detail(_:)`, which build the events with the pane scope. `draftRow` covers the new `.detail` case. |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacRecipeDetail.swift` (new) | `.loading`: a spinner (`recipeDetailLoading`). `.failed`: "Recipe unavailable", the error text, and **Retry** (`retryRecipeDetail`). `.missing`: "This recipe no longer exists." (`recipeDetailMissing`). `.ready`: the title (`recipeTitle`) above a grouped `Form` with **Ingredients** and **Steps** sections that say "None yet". Toolbar: **Edit** (`editRecipe`), disabled. |
| `MacRecipePane.swift` | `.detail`: `MacRecipeDetail`, with `.id(scope)`. |
| `MacRecipeList.swift` | The list selection binding: Rust's `selected`, or the draft tag while the editor is open. A click sends `select(id)`; ⌘-click on the selected row sends `clearSelection`. Arrow keys work after a click, as in `MacIngredientList`. |
| `MacMainView.swift` | Passes `selected` and `select` to `MacRecipeList`. |

### tests

| Layer | File | Tests |
|---|---|---|
| db | `src/recipe_store/tests.rs` | `get` returns a created recipe. An unknown id gives `None`. A row written with SQL with a wrong `title_key` gives `Storage`. |
| Rust widget | `tests/recipe_detail.rs` (new) | The load shows `Ready` with the title. `None` shows `Missing`. A failure shows `Failed`; Retry loads again and shows `Ready`. Retry outside `Failed` is ignored. A result of a replaced load is ignored. |
| Rust widget | `tests/recipe_section.rs` | Select opens the read view and sets `selected`. Select of the open recipe is ignored. Select of an id that the list does not show is ignored. Selecting another recipe gives a new scope, and the old load result is ignored. ClearSelection empties the pane; with an old scope it is ignored. Save opens the saved recipe's read view and loads the list again (this replaces `saving_empties_the_pane_and_loads_the_list_again`). New Recipe from a read view, then Cancel: the read view of that recipe again. Select while the editor is untouched: the clicked recipe. Select while a title is typed: "Discard changes?"; Discard: the clicked recipe; Keep Editing, then Cancel and Discard: the recipe from before New Recipe. A missing recipe loads the list again. |
| Rust app | `tests/app.rs` | A recipe row click routes through the app. Switching to Ingredients and back keeps the open read view. |
| ffi | `app_types/tests.rs`, `app_runtime_tests.rs` | See the ffi table. |
| Swift adapter | `SharedTests/AdapterPlumbingTests.swift` | `select`, `clearSelection` and `detail(.retry)` carry the pane scope and the id. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | `recipeReadView` (light and dark): 2 new images. |
| UI smoke (Mac) | `MacRecipeUITests.swift` | `testOpenRecipeShowsItsReadView`: create "Shakshuka"; its read view opens. Create "Pasta"; click **Shakshuka**; the read view shows "Shakshuka". `testRowClickAsksToDiscardTypedTitle`: create "Shakshuka", click **New Recipe**, type "Toast", click **Shakshuka**, confirm; the read view shows "Shakshuka". `testCreatedRecipeSurvivesRelaunch` checks the read view after Save. |
| Demo (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureREC03Demo`: `01-read-view`, `02-after-save`, `03-discard-on-click`. |

### docs

| File | Change |
|---|---|
| `docs/epics/recipes/epic.html` | REC-03 card: `plan: "tasks/REC-03.md"`, and `db` in its layers. A new decision E-08 (REC-03/Q1). |
| `docs/epics/recipes/tasks/REC-03-demo/` (new) | The 3 demo screenshots. |

## Steps

The slice is about 2,300 lines of diff, so it is five jj changes, each under
1,000 lines. Each change builds and passes every check alone. You review
each change before the next one starts. A change with no `.rs` file gets a
report and no review.

### Change 1 — REC-03: plan (about 400 lines, no review)

This plan, the REC-03 card and E-08 on `epic.html`, and the beads.

### Change 2 — REC-03: the recipe read view widget (about 450 lines, review)

1. Add `RecipeStore::get`.
2. Add `get` to the SQLite store, with its tests.
3. Add `detail.rs`.
4. Write `tests/recipe_detail.rs`.
5. Run checks 1 to 3 and check 7.
6. Send the guided review.

### Change 3 — REC-03: open a recipe in the Recipes section (about 650 lines, review)

1. Add `RecipeList::contains`.
2. Add `Select`, `ClearSelection`, the read view pane, and `CreatePane` to `section.rs`.
3. Update and extend `tests/recipe_section.rs`.
4. Run checks 1 to 3 and check 7.
5. Send the guided review.

### Change 4 — REC-03: the read view in the app and the FFI (about 400 lines, review)

1. Update `tests/app.rs`.
2. Add the FFI types and their tests.
3. Run checks 1 to 3.
4. Run `mac-bindings debug` and `ios-bindings debug`.
5. Update the Swift call sites so the build stays green. The Mac shows the
   empty pane for a read view until change 5.
6. Run checks 4, 5, and 7.
7. Send the guided review.

### Change 5 — REC-03: read a recipe on the Mac (about 450 lines, report)

1. Add `MacRecipeDetail`. Update `MacRecipePane`, `MacRecipeList` and
   `MacMainView`.
2. Add the fixtures, the adapter tests, and the snapshot test. Record the 2
   new images and look at each one.
3. Add the UI tests and the demo method.
4. Run checks 4, 5, and 7.
5. Capture the demo (check 6).
6. Fill in Validation in this plan.
7. Send the demo and the report to the REC-03 Inbox thread.

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
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. 2 new images, each checked by eye. No existing image changes. |
| 5 | iOS `xcodebuild build-for-testing` (generic iOS Simulator) | Builds. iOS is parked, so only the build runs. |
| 6 | `capture-demo.sh mac …/testCaptureREC03Demo docs/epics/recipes/tasks/REC-03-demo` | `01-read-view.png`, `02-after-save.png`, `03-discard-on-click.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| **Edit** in the read view is disabled. | REC-04 |
| **Ingredients** and **Steps** in the read view say "None yet". | REC-07, REC-14, REC-19 (stages) |
| Rows show the title only. No time, no step count. | REC-14, REC-21 |
| No **Delete** in the read view. | REC-05 |
| No search. | REC-06 |

- The read view loads the recipe again each time it opens, also right after
  Save. The load is one SQLite read by a unique key.
- Quitting with an open, changed editor does not ask "Discard changes?".
  `vibetyna-3rv` tracks that for both sections.
- No recipe screens on iOS (E-04).
- No recipe seed for tests. The UI tests create recipes through the UI.

## Validation

Run on 2026-11-01, on the last change of the slice.

| # | Check | Result |
|---|---|---|
| 1 | `cargo test --workspace` | 577 passed, 0 failed. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings (run on changes 2 to 4; change 5 has no Rust). |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | 80 passed, 0 failed: every Mac unit and snapshot test, and the 4 recipe UI tests. The 2 new images (`recipeReadView`, light and dark) were checked by eye. No existing image changed. |
| 5 | iOS `xcodebuild build-for-testing` | Builds. |
| 6 | `capture-demo.sh … testCaptureREC03Demo` | `REC-03-demo/01-read-view.png`, `02-after-save.png`, `03-discard-on-click.png`. |
| 7 | Change sizes (insertions + deletions) | Plan 425, read view widget 447, section 884, app and FFI 261, Mac 273. All under 1,000. |

### Differences from the plan

- **`RecipeId` is a UniFFI custom type** (review of change 4). The FFI
  carries recipe ids as `RecipeId`, lowered as its UUID text, in place of
  `String`. Invalid text from Swift fails `dispatch` with
  `AppFfiError::Conversion { InvalidRecipeId }`, so `RecipeSectionEvent`
  converts with `From`, not `TryFrom`. Swift sees `typealias RecipeId =
  String`. Bead `vibetyna-v56` (tech debt) moves `IngredientId` to the same
  type and gives both ids a Swift struct (the user's answer: later).
- **Change 3 had a temporary FFI arm** `Detail => Empty`, so it built alone;
  change 4 replaced it.
- **The app test moved to change 3** in part: Save now loads the saved
  recipe, so the app test's effect driver answers `get` from change 3 on.
- **The read view's identifier sits on its form.** On the whole view, SwiftUI
  gave `recipeDetail` to the title too, and the UI tests could not find
  `recipeTitle`.
