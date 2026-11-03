# REC-14 — Add steps on a canvas

Tracked in beads: `vibetyna-hrx.14`. Changes: seven, listed in Steps.
Dependencies: none on the card. This plan builds on the REC-01 to REC-06 code.

## Open questions

None. You answered the two questions in the REC-14 Inbox thread. They are in
Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-14/Q1 | Where do the recipe title, the canvas, and the step inspector go? | The mockup layout. On the left: the Title field, the canvas below it, and "Ingredients: Coming soon" under the canvas. On the right: an inspector column of fixed width. With no step selected, it says "Select a step". |
| REC-14/Q2 | What happens to a step that has no title? | A. Save checks. Save refuses, selects the first step with no title, and the inspector shows "A step needs a title." under its title field. |

### Decided in the plan

You saw these rules in the questions message, with no objection.

| Rule | Follows from |
|---|---|
| A new `recipe_steps` table: id, recipe, order added, title, x, y. The steps of a recipe go with it on delete. | The card; REC-05 (`ON DELETE CASCADE`). |
| Positions are whole canvas points, with the origin at the top left. Rust stores them and saves them with the recipe. | D-19, the card. |
| Rust owns the steps, the selection, and every rule. Swift owns the pointer location and the keyboard focus. | The epic: "The graph". |
| A new step is selected at once, and the cursor goes to its title in the inspector. | The card: "Selecting a node opens the inspector". |
| A click on empty canvas clears the selection. | Node editors (D-20). |
| The read view lists the steps in the order added, until REC-19 shows stages. | The card stub. |
| The list row shows "1 step" or "3 steps" under the title, and nothing for 0 steps. | D-15, the card. |
| Adding, renaming, or (later) moving a step makes the editor "changed", so Cancel asks "Discard changes?". | E-10. |
| After the first Save, each change checks the step titles again, as for the recipe title. | REC-02. |
| **Selection type:** the editor view has one enum, `StepCanvasView::Unselected { steps }` or `Selected { selected, others }`. A selected id that names no step cannot exist. | designing-types, rule 4. |
| **Step events:** the step events are a sub-event, `RecipeEditorEvent::Canvas(StepCanvasEvent)`. The private `StepCanvas` applies them. | Your note on the plan. |
| **List type:** `RecipeStore::list` returns `RecipeSummary { id, title, step_count }`, not whole recipes. The list does not load every step. | Less data for the list. |

## Outcome

You open Shakshuka and click Edit. The canvas is empty. You right-click on
the canvas and choose **Add Step**. A node appears where you clicked, and it
is selected. The inspector on the right shows its Title field with the cursor
in it. You type "Dice onion". You add a second node, "Fry onion", and click
Save. The read view lists "Dice onion" and "Fry onion". The list row says
"Shakshuka" and "2 steps" under it.

If you click Save while a node has no title, the save does not start. That
node is selected, and the inspector says "A step needs a title."

## Terms

- **Canvas:** the area of the editor where steps are nodes. In REC-14 you can
  add a node and select it. REC-15 moves nodes, REC-16 deletes them, REC-18
  connects them.
- **Node:** a box on the canvas that shows one step's title.
- **Inspector:** the column on the right of the editor. It shows the fields of
  the selected step.
- **Context menu:** the menu that a right-click (or a Control-click) opens.
- **Canvas point:** a position on the canvas in whole points, with x to the
  right and y down from the top left corner.
- **Order added:** the order in which you added the steps. The store keeps it
  in an `ordinal` column.

## Design

The target is section 2 of the
[recipes mockups](../../../tasks/recipes/mockups.html): the canvas on the
left, the inspector on the right. REC-14 draws only a part of it:

| In the mockup | In REC-14 |
|---|---|
| Nodes with title, duration, and kind | Title only. Duration and kind come in REC-17. |
| Ports and connections | Not shown. REC-18 adds them. |
| "＋ Add step here" in the context menu | **Add Step** (Mac menu style). |
| Inspector: Title, Instructions, Duration, After, Uses | Title. Duration, Instructions, and After show as disabled placeholders. Uses comes in REC-20. |
| Ingredients table under the canvas | "Coming soon" (REC-07). |
| Recipe title in the window title only | A Title field above the canvas (REC-14/Q1). |

The read view lists the step titles in its Steps section. The list row shows
the step count under the title, as in section 1 of the mockups, without the
time (REC-21).

## Changes by layer

### db (`crates/kreatyna-db`)

A new forward-only migration, `2026-11-03-120000-0000_create_recipe_steps`
(*sketch*; `down.sql` stays empty):

```sql
CREATE TABLE recipe_steps (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    recipe_local_id INTEGER NOT NULL
        REFERENCES recipes (local_id) ON DELETE CASCADE,
    ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
    title TEXT NOT NULL CHECK (length(trim(title)) > 0),
    x INTEGER NOT NULL CHECK (x >= 0),
    y INTEGER NOT NULL CHECK (y >= 0),
    UNIQUE (recipe_local_id, ordinal)
);
```

| Store call | Change |
|---|---|
| `list` | Returns `RecipeSummary` rows. One query counts the steps of each recipe. |
| `get` | Also reads the recipe's steps, by `ordinal`. |
| `create` | Also inserts the steps, in the same transaction. |
| `update` | Deletes the recipe's steps and inserts the new list, in the same transaction. A step keeps its id. |
| `delete` | No change. The foreign key deletes the steps. |

`recipe_models.rs` gets `StepRow`, `NewStepRow`, and `RemoteStepId`.

### rust — model (`crates/kreatyna/src/recipes/model.rs`)

*Sketch:*

```rust
/// Stable step identity. The editor generates it when you add a step.
#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromStr, From, Into))]
pub struct StepId(Uuid);   // + `generate()`, as `RecipeId`

/// A step title that passed the save rule: trimmed and not empty (REC-14/Q2).
#[nutype(sanitize(trim), validate(not_empty), derive(Debug, Clone, PartialEq, Eq, AsRef))]
pub struct StepTitle(String);

/// A position on the canvas in whole points, from the top left corner (D-19).
pub struct CanvasPoint { pub x: u32, pub y: u32 }

/// A stored step.
pub struct Step { pub id: StepId, pub title: String, pub position: CanvasPoint }

pub struct Recipe {
    pub id: RecipeId,
    pub title: String,
    /// In the order added.
    pub steps: Vec<Step>,
}

/// A row of `RecipeStore::list`.
pub struct RecipeSummary { pub id: RecipeId, pub title: String, pub step_count: u32 }

/// A step that passed the save rule. `NewRecipe` and `RecipeUpdate` get
/// `steps: Vec<StepWrite>`, in the order added.
pub struct StepWrite { pub id: StepId, pub title: StepTitle, pub position: CanvasPoint }
```

`u32` positions: a node cannot go left of or above the canvas. Swift rounds
the click location and clamps it at 0.

### rust — list (`list.rs`)

`RecipeListRow` gets `step_count: u32`. The list stores `RecipeSummary`
entries. The search does not change.

### rust — read view (`detail.rs`)

| Change |
|---|
| The ready state keeps the whole `Recipe`. |
| `RecipeDetailStatus::Ready` gets `steps: Vec<RecipeDetailStep { id, title }>`, in the order added. |
| `OpenEdit { recipe }` carries the steps, so the editor opens with them. |

### rust — editor (`editor.rs`, new `step_canvas.rs`)

A private `StepCanvas` struct keeps the steps and the selection. It applies
the `StepCanvasEvent`s; its methods are the only way to change the steps or
the selection, so the selection always names a step. It has no effects, so
it is a plain struct, not a widget: the editor renders after each change.

```rust
// sketch
pub enum RecipeEditorEvent {
    SetTitle(String),
    /// The steps on the canvas and in the inspector.
    Canvas(StepCanvasEvent),
    Save,
    RequestClose,
    Confirmation(ConfirmationEvent),
}

pub enum StepCanvasEvent {
    /// Right-click ▸ Add Step. Adds an untitled step at `at` and selects it.
    Add { at: CanvasPoint },
    /// A click on a node. Ignored for an id that names no step.
    Select(StepId),
    /// A click on empty canvas.
    ClearSelection,
    /// The title field of the inspector, as typed. Ignored for an unknown id.
    SetTitle { id: StepId, title: String },
}

pub struct RecipeEditorView {
    pub title: String,
    pub canvas: StepCanvasView,
    pub status: RecipeEditorStatus,
}

pub enum StepCanvasView {
    Unselected { steps: Vec<StepNodeView> },
    /// The inspector shows `selected`. `others` are the other steps.
    Selected { selected: StepNodeView, others: Vec<StepNodeView> },
}

pub struct StepNodeView {
    pub id: StepId,
    /// The title as typed.
    pub title: String,
    pub position: CanvasPoint,
    /// Only after the first Save: the title breaks the rule.
    pub problem: Option<StepTitleError>,
}
```

| Event | Result |
|---|---|
| `Canvas(Add { at })` | A new step with a generated id and an empty title. It is selected. |
| `Canvas(Select(id))` | Selects the step. |
| `Canvas(ClearSelection)` | No step is selected. |
| `Canvas(SetTitle { id, title })` | Keeps the raw text. After the first Save, the step's problem is checked again. |
| `Save` | Checks the recipe title and every step title. On a bad step title, it selects the first such step (order added) and does not save. Otherwise it sends `NewRecipe` or `RecipeUpdate` with `steps`. |

`Canvas` events apply only in the `Editing` status, like `SetTitle`.

`is_dirty` compares the title and the step list (id, title, position) with
the ones that the editor opened with.

`RecipeEditorProblem::Invalid` keeps its meaning: the recipe title. A step's
problem lives on its node.

**`section.rs`** — no change in its logic. Its tests get recipes with steps.

### ffi (`crates/kreatyna-ffi/src/app_types/recipes.rs`)

| Change |
|---|
| `StepId` as a UniFFI custom type, as `RecipeId`. |
| `CanvasPoint` record. |
| `RecipeListRow.step_count`. |
| `RecipeDetailStatus::Ready.steps` with `RecipeDetailStep`. |
| `RecipeEditorView.canvas`, `StepCanvasView`, `StepNodeView`, `StepTitleError`. |
| `RecipeEditorEvent::Canvas { event: StepCanvasEvent }` and the four `StepCanvasEvent` cases. |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacRecipeList.swift` | "1 step" or "N steps" under the title, nothing for 0. |
| `MacRecipeDetail.swift` | The Steps section lists the titles, or "None yet". |
| `MacRecipeEditor.swift` | The REC-14/Q1 layout: Title, canvas, "Ingredients: Coming soon" on the left; the inspector on the right. |
| `MacStepCanvas.swift` (new) | A scroll view with the nodes at their positions. A node click sends `.canvas(.select)`; an empty-canvas click sends `.canvas(.clearSelection)`. The context menu has **Add Step**, which sends `.canvas(.add(at:))` with the last pointer location. The selected node has the accent border, a node with a problem has a red border. An untitled node shows "Untitled step" in secondary text. |
| `MacStepInspector.swift` (new) | "Select a step", or the selected step's Title field, its problem, and disabled Duration, Instructions, and After placeholders. A new step moves the focus to its Title. |

Risk: SwiftUI's `.contextMenu` does not give the click location. The canvas
keeps the last pointer location from `onContinuousHover` and uses it for
**Add Step**. If that is not reliable, an AppKit view that handles
`rightMouseDown` gives the location.

### tests

| Layer | File | Tests |
|---|---|---|
| db | `recipe_store/tests.rs` | Create and get keep the steps, their order, and positions. Update replaces the steps and keeps their ids. Delete deletes the steps. List counts the steps. A migration test: the old recipes stay, with 0 steps. |
| Rust widget | `tests/recipe_editor.rs` | Add selects the new step. Select and clear. Rename. Unknown ids are ignored. Save with an untitled step selects it and shows its problem; a typed title clears it. Save sends the steps. Edit opens with the saved steps. Adding a step makes Cancel ask. |
| Rust widget | `tests/recipe_detail.rs`, `tests/recipe_list.rs` | The read view lists the steps in order; Edit carries them. The row has the step count. |
| ffi | `app_types/tests.rs` | The new events, views, and the step count convert. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New: `recipeCanvas` (3 nodes, one selected, the inspector) and `untitledStep` (the problem), light and dark. Changed: the editor images (new layout), and `recipeReadView` with steps and step counts. I delete the changed images, record them again, and check each by eye. |
| UI smoke (Mac) | `MacRecipeUITests.swift` | `testAddStepAndSave`: create a recipe, right-click the canvas, Add Step, type "Dice onion", Save. The read view lists it, and the row says "1 step". |
| Demo (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureREC14Demo`: `01-context-menu`, `02-inspector`, `03-read-view`. |

## Steps

The slice is about 2,500 lines of diff without images. It is seven jj
changes, each under 1,000 lines. You review each change with Rust before the
next one starts. A change with no `.rs` file gets a report and no review.

### Change 1 — REC-14: plan (no review)

This plan, the REC-14 card on `epic.html`, and the beads.

### Change 2 — REC-14: steps in the store (about 650 lines, review)

1. Add the model types: `StepId`, `StepTitle`, `CanvasPoint`, `Step`, `StepWrite`, `RecipeSummary`.
2. Add `steps` to `Recipe`, `NewRecipe`, and `RecipeUpdate`. Change `list` to return `RecipeSummary`.
3. Add the migration and the Diesel schema.
4. Write and read the steps in the store.
5. Write the store tests and the migration test.
6. Fix the core call sites with empty step lists, so the change builds alone.
7. Run checks 1 to 3 and check 7. Send the guided review.

### Change 3 — REC-14: the list and the read view show steps (about 300 lines, review)

1. Add `step_count` to the list row.
2. Keep the whole recipe in the read view; add `steps` to its view; carry them in `OpenEdit`.
3. Write the widget tests.
4. Map the new fields in the FFI with fixed values, so the change builds alone. Change 5 adds them.
5. Run checks 1 to 3 and check 7. Send the guided review.

### Change 4 — REC-14: the editor adds and names steps (about 750 lines, review)

1. Add `StepCanvas`, `StepCanvasEvent`, `RecipeEditorEvent::Canvas`, and the canvas view.
2. Extend the save rule and `is_dirty`.
3. Write the editor tests, and the section tests that change.
4. Keep the FFI building, as in change 3.
5. Run checks 1 to 3 and check 7. Send the guided review.

### Change 5 — REC-14: steps in the FFI (about 400 lines, review)

1. Add the FFI types, events, and their tests.
2. Run `mac-bindings debug`.
3. Update the Swift call sites and fixtures, so the Mac build stays green.
4. Run checks 1 to 5 and check 7. Send the guided review.

### Change 6 — REC-14: the read view and the list row show steps on the Mac (about 150 lines, report)

1. Show the steps in `MacRecipeDetail` and the count in `MacRecipeList`.
2. Record `recipeReadView` again with steps, and check it by eye.
3. Run checks 4, 5 and 7. Send the report with the images.

### Change 7 — REC-14: the canvas and the inspector on the Mac (about 600 lines, report)

1. Write `MacStepCanvas` and `MacStepInspector`, and the new editor layout.
2. Add the snapshot tests. Delete the editor images, record all images, and check each by eye.
3. Add the UI test and the demo method.
4. Run checks 4, 5 and 7.
5. Capture the demo (check 6).
6. Fill in Validation in this plan.
7. Send the demo and the report.

## Checks

Test scope: Rust in full for each change. On the Mac, all unit and snapshot
tests, and the UI tests of the Recipes section only (`--ui MacRecipeUITests`).

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. New and changed images checked by eye. No ingredient image changes. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh mac …/testCaptureREC14Demo docs/epics/recipes/tasks/REC-14-demo` | `01-context-menu.png`, `02-inspector.png`, `03-read-view.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub | Replaced by |
|---|---|
| Nodes cannot move. | REC-15 |
| Nodes cannot be deleted. | REC-16 |
| The inspector shows Duration, Instructions, and After as disabled placeholders. | REC-17, REC-18 |
| No ports and no connections on the canvas. | REC-18 |
| The read view shows a flat list in the order added. | REC-19 |
| The list row shows no time. | REC-21 |
| A recipe can save with no steps. | REC-22 (E-02) |

- No step letters (A, B, C) from the mockup. No slice asks for them yet.
- No zoom on the canvas.
- No keyboard shortcut for Add Step.
- No recipe screens on iOS (E-04).

## Validation

| # | Check | Result |
|---|---|---|
| 1 | `cargo test --workspace` | 723 pass. New: 9 store tests (change 2), 2 read-view and list tests (change 3), 19 editor tests (change 4), 9 FFI conversion tests (change 5). |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | TEST SUCCEEDED. All unit and snapshot tests, and 9 recipe UI tests with the new `testAddStepAndSave`. New images: `recipeCanvas` and `untitledStep`. Recorded again: `recipeReadView` (change 6) and the three editor images (change 7). I checked each image by eye in light and dark. No ingredient image changed. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh mac …/testCaptureREC14Demo docs/epics/recipes/tasks/REC-14-demo` | `01-context-menu.png`, `02-inspector.png`, `03-read-view.png`. |
| 7 | `jj diff --stat` per change | Change 1: 364. Change 2: 752. Change 3: 206. Change 4: 714. Change 5: 370. Change 6: 51. Change 7: 476 (images count 0). |

### Differences from the plan

- **Change 4, after review:** `StepCanvas::apply` and `select` return `CanvasChange { Changed, Unchanged }`, not `bool`. `EditorTarget::opened_with` returns the struct `OpenedWith { title, steps }`, not a tuple.
- **Change 5:** the FFI crate gets `uuid` as a dev-dependency, for fixed step ids in its tests.
- **Add Step location:** `onContinuousHover` is reliable, so the AppKit fallback was not needed. The UI test and the demo right-click at chosen points, and the new node appears there.
- **Editor heading:** the heading moved from the top inset into the left column, so the inspector runs the full height of the pane.
- **Demo `01-context-menu`:** a window screenshot leaves out the open menu. The demo takes a screen screenshot and cuts it to the window.
