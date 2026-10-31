# REC-01 — An empty Recipes section in the sidebar

Tracked in beads: `vibetyna-hrx.1`. Changes: five, listed in Steps. Dependencies: the Mac
Shell epic `vibetyna-4rn` (closed).

## Open questions

None. The user answered REC-01/Q1 in the Inbox on 2026-10-31 (see
Decisions).

## Decisions

### REC-01/Q1 — The Mac app opens on Recipes

*Context:* The Mac app opened on the ingredient catalog, with one fixed
"Ingredients" item in the sidebar. The mockup shows Recipes selected.

*Decision:* Recipes, the recommendation. Rust starts the split layout with
the Recipes section selected. The Mac UI test and demo launch helpers click
**Ingredients** first. Until REC-02 the app opens on an empty list. This is
also epic decision E-07.

### Plan review (2026-10-31, LGTM)

- **Changes under 1,000 lines.** The slice is a series of five jj changes
  (see Steps). The rule is now in `use-of-jj-vcs`.
- **Invalid states are unrepresentable.** `AppView` has one variant per
  layout, and no `Option` stands for "not on iOS". The rule is now the
  `designing-types` skill.

### Decided in the plan (sent with the question, no objection)

- Recipes exist only in the split layout (the Mac). In the stack layout
  (iOS), Rust sends one section, Ingredients, and no recipe list (E-04).
- Rust loads the recipe list at startup, next to the ingredient list.
  Switching sections keeps each section's state, including an open, unsaved
  ingredient form.
- Swift writes the section names and icons. Rust sends only which sections
  exist and which one is selected (as in MAC-07/Q1).
- **New Recipe** shows in the toolbar but is disabled until REC-02. ⌘N does
  nothing in Recipes until REC-02.
- The recipe list has no search field until REC-06.
- The off-screen snapshot draws the sidebar blank inside the split view
  (MAC-12). A snapshot renders the sidebar alone. If that is blank too, the
  demo screenshot and the UI smoke test cover the sidebar.

## Outcome

You open Kreatyna. The sidebar shows **Recipes** (book icon) above
**Ingredients** (carrot icon), and Recipes is selected. The list column says
"No recipes yet". The detail pane says "Select a recipe". The toolbar has a
grey **New Recipe** button.

You click **Ingredients**. The catalog shows Garlic and Tomato. You click
Garlic, and its detail opens. You click **Recipes**, then **Ingredients**
again: Garlic is still selected and its detail is still open.

## Terms

- **Section:** one item in the Mac sidebar. Each section has its own list
  column and detail pane. REC-01 has two: Recipes and Ingredients.
- **Split layout / stack layout:** `AppLayout::Split` is the Mac (sidebar,
  list, detail side by side). `AppLayout::Stack` is iOS (one screen at a
  time). Swift chooses the layout once, at startup (E-19).
- **Widget:** a krucyfiks state machine with `init`, `update`, and `view`.
  Events come from the user; outcomes come back from commands.
- **`ScopedChild`:** a wrapper that gives a child widget's commands a scope,
  so the parent drops outcomes from an old scope.
- **Capability:** a trait that Rust calls and the platform answers, for
  example `IngredientStore`. Tests answer it by hand through an effect
  stream.
- **Title key:** the title folded for comparison (`name_key`): trimmed,
  case-folded. "Shakshuka" and "shakshuka " have the same key (D-39).
- **Forward-only migration:** a new SQL file that changes the database
  schema. Old migrations never change, and `down.sql` stays empty.
- **Snapshot test:** a test that renders a view off screen and compares the
  image with a recorded baseline (`swift-snapshot-testing`).

## Design

The target is section 1 of the
[recipes mockups](../../../tasks/recipes/mockups.html): a sidebar with
**Recipes** and **Ingredients**, a list column, and a detail pane. REC-01
shows the empty state of that window: no rows, no ▶ Cook (E-05), and no
**Edit** (REC-03, REC-04). No new mockup. The demo shows the result.

## Changes by layer

### db (`crates/kreatyna-db/`)

| File | Change |
|---|---|
| `migrations/2026-10-31-180000-0000_create_recipes/up.sql` (new) | The `recipes` table. `down.sql` stays empty. |
| `src/schema.rs` | The `recipes` table, from `diesel print-schema`. |
| `src/recipe_models.rs` (new) | `RecipeRow` (`Queryable`, `Selectable`) and `RemoteRecipeId` (UUID text), with `into_domain`. |
| `src/recipe_store.rs` (new) | `impl RecipeStoreMut for SqliteDatabase`: `list` in one transaction. A wrong title key or a bad UUID fails the whole list with `RecipeLoadError::Storage`. |
| `src/sqlite.rs` | `DatabaseEffect::Recipe(Envelope<RecipeStoreRequest>)`, served by the worker. |
| `diesel.toml` | `print_schema.file` names `src/schema.rs`. Today it names `src/database/schema.rs`, which does not exist. |

*sketch:*

```sql
CREATE TABLE recipes (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL CHECK (length(trim(title)) > 0),
    title_key TEXT NOT NULL UNIQUE CHECK (length(title_key) > 0)
);
```

The columns follow `ingredients`: a local integer key for joins (REC-07
adds `recipe_lines`), a UUID for identity, and a unique key for the title
rule (REC-02).

### rust (`crates/kreatyna/`)

**New module `src/recipes/`**, next to `ingredients`:

| File | Contents |
|---|---|
| `mod.rs` | Re-exports. |
| `model.rs` | `RecipeId` (UUID newtype, like `IngredientId`) and `Recipe { id, title }`. |
| `capabilities.rs` | `RecipeStore::list`, `RecipeLoadError { Storage, Transport }`, `RecipeCapabilities { render, store }`, `RecipeEffect`. |
| `list.rs` | The `RecipeList` widget. |

*sketch:*

```rust
#[krucyfiks::capability]
pub trait RecipeStore: Send + Sync {
    async fn list(&self) -> Result<Vec<Recipe>, RecipeLoadError>;
}

pub enum RecipeListEvent {
    /// Load again after a failure (the Retry button).
    Refresh,
}

pub enum RecipeListOutcome {
    LoadFinished { id: CommandId, result: Result<Vec<Recipe>, RecipeLoadError> },
}

pub enum RecipeListStatus {
    Loading,
    Failed(RecipeLoadError),
    /// Rows sorted by title key, then by id. Empty means "No recipes yet".
    Ready { rows: Vec<RecipeListRow> },
}

pub struct RecipeListRow {
    pub id: RecipeId,
    pub title: String,
}

pub struct RecipeListView {
    pub status: RecipeListStatus,
}
```

`RecipeList` copies the load part of `IngredientList`: `init` renders, then
calls `store.list()` through a `LatestCommandSlot`, so a second **Refresh**
drops the first result. It has no search (REC-06) and no selection
(REC-03). The title key reuses `ingredients::name_key` (D-39).

The old `IngredientsView`, `IngredientsOutcome`, and the private `Ingredients`
enum go away: `AppView` and `Layout` replace them. `AppEvent::Ingredients`
becomes `AppEvent::Stack`, and `AppEvent::SplitIngredients` becomes
`AppEvent::Split(SplitAppEvent::Ingredients(..))`. About 90 lines in the
tests, the FFI, and Swift change with the rename.

**Sidebar order.** The sidebar lists Recipes above Ingredients. Swift writes
that order, as it writes the names and icons: `SplitAppView` always has both
sections, so no list of sections can disagree with the selection.

**`src/app.rs`** — Rust owns the sections. Each layout is one variant with
exactly its own data, so the types cannot hold recipes on iOS or a Mac
without a recipe list ([designing-types](../../../../.agents/skills/designing-types/SKILL.md),
REC-01 plan review):

```rust
// sketch
pub enum AppSection {
    Recipes,
    Ingredients,
}

pub enum AppView {
    /// iOS: the ingredient catalog on a stack. No sections (E-04).
    Stack(IngredientCatalogView),
    /// Mac: a sidebar with the Recipes and Ingredients sections.
    Split(SplitAppView),
}

pub struct SplitAppView {
    /// The selected section. Both sections always exist.
    pub section: AppSection,
    pub recipes: RecipeListView,
    pub ingredients: IngredientSplitCatalogView,
}

/// Events follow the same shape: each layout has its own.
pub enum AppEvent {
    Stack(IngredientCatalogEvent),
    Split(SplitAppEvent),
}

pub enum SplitAppEvent {
    SelectSection(AppSection),
    Recipes(RecipeListEvent),
    Ingredients(IngredientSplitCatalogEvent),
}

/// Inside `AppWidget`: the same two variants hold the child widgets.
enum Layout {
    Stack(ScopedChild<IngredientCatalog>),
    Split {
        section: AppSection,
        recipes: ScopedChild<RecipeList>,
        ingredients: ScopedChild<IngredientSplitCatalog>,
    },
}

pub enum AppOutcome {
    Stack(Scoped<IngredientCatalogOutcome>),
    SplitRecipes(Scoped<RecipeListOutcome>),
    SplitIngredients(Scoped<IngredientSplitCatalogOutcome>),
}

pub struct AppCapabilities {
    pub ingredients: IngredientCapabilities,
    pub recipes: RecipeCapabilities,
}

pub enum AppEffect {
    Ingredients(IngredientEffect),
    Recipes(RecipeEffect),
}
```

| Rule | Behavior |
|---|---|
| Start | Split: `section = Recipes` (REC-01/Q1). Stack has no sections. |
| `init` | Stack: the catalog load. Split: the ingredient load and the recipe load, with `Command::select_all`. |
| `SelectSection(s)` | Rust selects `s` and renders. Neither child changes. |
| Section events | Ingredient events reach the catalog and recipe events reach the recipe list, whichever section is selected. |
| Wrong layout | Rust ignores a `Stack` event in the split layout and a `Split` event in the stack layout, as today. |
| Shutdown | Unchanged: only the ingredient catalog can delay shutdown. A recipe load has nothing to save. |

### ffi (`crates/kreatyna-ffi/`)

| File | Change |
|---|---|
| `src/app_types/recipes.rs` (new) | `RecipeLoadError`, `RecipeListEvent`, `RecipeListRow` (id as `String`), `RecipeListStatus`, `RecipeListView`, and their conversions. |
| `src/app_types/app.rs` | `AppSection`. `AppEvent` becomes `Stack { event }` and `Split { event: SplitAppEvent }`. `RunningAppView` becomes an enum: `Stack { catalog }` and `Split { app: SplitAppView }`, the same shape as Rust. |
| `src/app_shell.rs` | `prepare_app` creates a `RecipeStoreClient`. The database worker serves both inboxes (`select`) and closes both. |
| `src/app_types/tests.rs` | Conversion tests for the new types. |
| `src/app_runtime_tests.rs` | The real runtime on a new SQLite file starts on Recipes with an empty, ready recipe list. |

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `AppEventComposition.swift` | `RunningAppView.stack` and `.split` replace `IngredientsView.stack` and `.split`. `SplitAppView.selectSection(_:)`, `.recipes(_:)`, and `.ingredients(_:)` build the events. |
| `ErrorText.swift` | `RecipeLoadError.message`: "Recipes could not be read from storage." / "Recipes are temporarily unavailable." |

iOS (`apple/Kreatyna/Kreatyna/`): `ContentView` reads `app.stack` in place
of `app.ingredients.stack`. Nothing visible changes.

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacMainView.swift` (new, replaces `MacCatalogView.swift`) | Takes a `SplitAppView`. The `NavigationSplitView`. Sidebar: `MacSidebar`. Content and detail: switch on `app.section`. |
| `MacSidebar.swift` (new) | A `List` with Recipes, then Ingredients. The selection binding reads `app.section`; its setter sends `.selectSection`. Identifiers `sidebarRecipes`, `sidebarIngredients`. |
| `MacSectionText.swift` (new) | `AppSection.title` ("Recipes", "Ingredients") and `AppSection.systemImage` ("book", "carrot"). |
| `MacRecipeList.swift` (new) | Loading, failed (with **Retry**), "No recipes yet" (`recipeListEmpty`), and title rows. Title "Recipes". Toolbar: **New Recipe** (`newRecipe`), disabled. |
| `MacIngredientPane.swift` (new) | The ingredient detail pane, moved unchanged out of `MacCatalogView`. |
| `MacRootView.swift` | `.running(let app)` shows `MacMainView` for `app.split`. |

The Recipes detail pane is a `ContentUnavailableView` "Select a recipe"
(`recipeDetailPlaceholder`).

*sketch:*

```swift
List(selection: Binding(get: { app.section }, set: { if let s = $0 { send(app.selectSection(s)) } })) {
    ForEach([AppSection.recipes, .ingredients], id: \.self) { section in
        Label(section.title, systemImage: section.systemImage)
            .tag(section)
            .accessibilityIdentifier(section.accessibilityID)
    }
}
```

### tests

| Layer | File | Tests |
|---|---|---|
| Rust widget | `crates/kreatyna/tests/recipe_list.rs` (new) | Init renders, then loads; rows sorted by title key, then id (empty, populated, equal folded titles). Load failure shows `Failed`. **Refresh** after a failure shows `Loading`, then the rows. A stale load result is ignored. |
| Rust app | `crates/kreatyna/tests/app.rs` | Split starts on Recipes. Init loads both lists. Switching to Ingredients and back keeps the selected Garlic, an open dirty form, and the recipe rows. Stack loads only the catalog and ignores `Split` events. Recipe outcomes route through the app. Existing tests use the renamed events. |
| db | `src/recipe_store/tests.rs` (new) | Empty list. Rows written with SQL are listed. A wrong title key or a bad UUID fails the list. |
| db | `src/ingredient_models/tests.rs` | The migration counts change: 3 → 4 on an empty database, 1 → 2 for the upgrade from before MAC-03. Existing ingredients survive the new migration. |
| ffi | `app_types/tests.rs`, `app_runtime_tests.rs` | See the ffi table. |
| Swift fixtures | `SharedTests/Fixtures.swift` | `splitApp(section:recipes:ingredients:)` builds a `SplitAppView`. `recipeList(_ status:)`. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New `emptyRecipeList` and `sidebar`, light and dark: 4 new images. The 22 existing images render through `MacMainView` with the Ingredients section and must not change. |
| UI smoke (Mac) | `KreatynaMacUITests.swift` | `launch` clicks **Ingredients** unless a test asks for no section. New `testSidebarSwitchesSections`: seed Garlic and Tomato; "No recipes yet" shows; click **Ingredients**, open Garlic; click **Recipes**, then **Ingredients**; Garlic's detail is still open. |
| Demo capture (Mac) | `MacDemoCaptureUITests.swift` | `launch` clicks **Ingredients** for the old demos. New `testCaptureREC01Demo`: `01-recipes-empty`, then `02-ingredients` after the switch. |

### docs

| File | Change |
|---|---|
| `docs/epics/recipes/epic.html` | REC-01 card: `plan: "tasks/REC-01.md"`. New E-07: the Mac app opens on Recipes (REC-01/Q1). |
| `docs/epics/recipes/tasks/REC-01-demo/` (new) | The 2 demo screenshots. |

## Steps

The whole slice is about 2,000 lines of diff, as MAC-04 (2,300) and MAC-05
(2,600) were. A change must stay under 1,000 lines, so the slice is five jj
changes. Each change builds and passes every check alone. The user reviews
each change before the next one starts. A change with no `.rs` file gets a
report and no review.

**Size** means the insertions plus the deletions in
`jj diff -r <change> --stat '~.beads/issues.jsonl'`. Images count as 0.
Before each report, measure the change. If it is over 1,000 lines, split it
with `jj split` at a layer or file boundary.

### Change 1 — REC-01: plan (`smqxqllm`, about 330 lines, no review)

This plan, the REC-01 card and E-07 on `epic.html`, and the beads. You
reviewed the plan already, so the change needs no other review.

### Change 2 — REC-01: recipes table and store (about 500 lines, review)

1. Add the `create_recipes` migration.
2. Fix `print_schema.file` in `diesel.toml`.
3. Run the migrations on a new database and run `diesel print-schema` to update `schema.rs`.
4. Add `recipes/model.rs`, `recipes/capabilities.rs`, and `recipes/mod.rs`.
5. Add `recipe_models.rs`, `recipe_store.rs`, and the worker route.
6. Write the store tests and update the migration counts.
7. Run checks 1 to 3 and check 7.
8. Send the guided review of the change.

### Change 3 — REC-01: the recipe list widget (about 350 lines, review)

1. Add the `RecipeList` widget in `recipes/list.rs`.
2. Write `tests/recipe_list.rs`.
3. Run checks 1 to 3 and check 7.
4. Send the guided review of the change.

The widget is public and tested, but nothing uses it until change 4.

### Change 4 — REC-01: app sections in Rust and the FFI (about 800 lines, review)

The new `AppView` changes the FFI, and the FFI changes the Swift types. One
change holds all three, so every build stays green.

1. Replace `IngredientsView` with the `AppView` and `Layout` enums in `app.rs`.
2. Rename the events in `tests/app.rs` and add the section tests.
3. Add the FFI types, the shell wiring, and their tests.
4. Run checks 1 to 3.
5. Run `mac-bindings debug` and `ios-bindings debug`.
6. Update the Swift places that read the old types: `AppEventComposition.swift`, `ContentView.swift`, `MacRootView.swift`, and the shared tests. The Mac still shows only the catalog.
7. Run checks 4, 5, and 7.
8. Send the guided review of the change.

### Change 5 — REC-01: the Recipes section on the Mac (about 650 lines, report)

This change edits no `.rs` file, so it gets a report and no review.

1. Add the recipe error text.
2. Add `MacMainView`, `MacSidebar`, `MacRecipeList`, and `MacIngredientPane`. Delete `MacCatalogView.swift`.
3. Update the fixtures and the snapshot tests.
4. Record the 4 new images and look at each one.
5. Update the UI test launch helpers and add the new UI test and demo method.
6. Run checks 4, 5, and 7.
7. Capture the demo into `REC-01-demo/` (check 6).
8. Fill in Validation in this plan.
9. Send the demo and the report to the REC-01 Inbox thread.

If a change is over 1,000 lines, it becomes two changes at a file boundary.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test` | All pass. 4 new images, each checked by eye. No existing image changes. |
| 5 | iOS `xcodebuild build-for-testing` (generic iOS Simulator) | Builds. iOS is parked, so only the build runs. |
| 6 | `capture-demo.sh mac …/testCaptureREC01Demo docs/epics/recipes/tasks/REC-01-demo` | `01-recipes-empty.png`, `02-ingredients.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| **New Recipe** is disabled; ⌘N does nothing in Recipes. | REC-02 |
| The detail pane says "Select a recipe"; rows cannot be selected. | REC-03 |
| Rows show the title only. No time, no step count. | REC-14, REC-21 |
| No search field. | REC-06 |
| No ▶ Cook. | Not in the MVP (E-05) |

- The app does not remember the last section. It always opens on Recipes.
- No keyboard shortcut to switch sections.
- No recipe screens on iOS (E-04).
- No way to add a recipe in the UI or in the test seed. The store tests
  write rows with SQL.

## Validation

Run on 2026-10-31, macOS 27, Xcode 27.0, on the last change.

### Checks

| # | Check | Result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | Clean. |
| 3 | `cargo fmt --check` | Clean. |
| 4 | `mac-test` | First run recorded the 4 new images and failed, as expected. Second run: `TEST SUCCEEDED`, 78 passed, 0 failed, 7 capture tests skipped. The 22 existing images did not change. `emptyRecipeList` and `sidebar` were checked by eye in light and dark; the sidebar renders when it is captured alone. |
| 5 | iOS `xcodebuild build-for-testing` | `TEST BUILD SUCCEEDED`. iOS is parked, so only the build ran. |
| 6 | `capture-demo.sh mac …/testCaptureREC01Demo …/REC-01-demo` | `01-recipes-empty.png`, `02-ingredients.png`. |
| 7 | Size of each change | Every REC-01 change is under 1,000 lines. |

UI test `testSidebarSwitchesSections` passed: the app opens on "No recipes
yet"; **New Recipe** is disabled; Garlic opened in Ingredients is still open
after a switch to Recipes and back.

### Not run

- iOS tests (iOS is parked; only the build ran).
- A failed recipe load in the UI. The UI cannot cause one; the Rust tests
  cover it.

### Deviations from the plan

1. **Change 2: tests.** The store tests follow the new skill
   `write-rust-tests` (green path, validation, red cases, support last) and
   use `#[tokio::test]` (review of change 2).
2. **Change 3: one load at a time.** **Refresh** works only after a failure
   (the Retry button); while the list loads or is ready, Rust ignores it. So
   only one load runs at a time. `RecipeListOutcome::LoadFinished` has no
   command id, the widget needs no `LatestCommandSlot`, and the planned
   "stale load result" test became two tests: Refresh is ignored while the
   first load runs and when the list is ready.
3. **REC-01/Q2: the database sorts.** The user chose it in the review of
   change 3. `RecipeStore::list` returns recipes sorted by title key
   (`ORDER BY title_key`, change 2), and `RecipeList` keeps that order. Title
   keys are unique, so there is no id tie-break. After a save, REC-02 loads
   the list again instead of inserting the recipe in Rust.
4. **Change 4 became two changes.** It measured 1,031 lines, so it was split:
   "REC-01: recipe store and FFI recipe types" (the recipe capabilities in
   `AppCapabilities`, the database task serving both stores, and the FFI
   recipe types) and "REC-01: app sections in Rust and the FFI" (the layout
   enums, their FFI shape, and the Swift call sites). REC-01 has six changes.
5. **Change 4 Swift keeps the old Mac screen.** `MacRootView` shows
   `split.ingredients` until the Recipes section UI lands in the last change,
   so the Mac UI tests run unchanged in between.
