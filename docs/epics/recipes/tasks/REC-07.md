# REC-07 — Ingredient lines with a name

Tracked in beads: `vibetyna-hrx.7`. Changes: five, listed in Steps.
Dependencies: REC-04 (`vibetyna-hrx.4`), closed. REC-14 to REC-18 are
closed too; this plan builds on their editor, store and transaction.

## Open questions

None. You answered one question in the REC-07 Inbox thread. It is in
Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-07/Q1 (epic Q-R1) | In what order do ingredient lines appear, and can you reorder them? | The order you added them, and you drag a line to reorder it, in this slice. The epic page records this as E-09. |

## Decided in the plan

| Rule | Follows from |
|---|---|
| **＋ Line** adds an empty line at the bottom. Swift puts the cursor in its name field. | Q1; the cursor is a presentation detail (AGENTS.md). |
| A line's name is trimmed and must not be empty. A line with an empty name blocks Save and shows "Name required" on that line, from the first Save on. | The epic card; the step title rule (REC-14/Q2). |
| Two lines may have the same name. | "salt" can appear twice, for example for the dough and for the topping. REC-10 links lines; it does not merge them. |
| The remove button (−) removes the line at once, with no question. Cancel still asks "Discard changes?" for the whole edit (E-10). | A step deletes the same way (REC-16). |
| You drag a line by its handle (≡) onto another line. The dragged line takes that line's place, and the lines between move one place. So it ends up before that line when it moves up, and after it when it moves down: with penne, oil, basil, basil onto penne gives basil, penne, oil, and penne onto basil gives oil, basil, penne. | Q1; the name `onto` stays (change 3 review). |
| If the drag cannot work on macOS, each line gets **Move Up** and **Move Down** in its context menu instead, and Validation says so. | MAC-05/Q4: a drag inside a grouped `Form` did not reorder, and MAC-05 used this fallback. |
| Save writes the lines with the title, the steps and the arrows, in the same transaction. A line keeps its id through every save. | The epic card; D-40. |
| The read view lists the lines by name, in their order, above the steps. A recipe with no lines shows "None yet", as today. | The mockup, section 1. |
| A line change marks the editor dirty: Cancel then asks "Discard changes?". | E-10. |

## Outcome

You open "Roasted tomato pasta" and click Edit. Under the canvas,
**Ingredients** has a **＋ Line** button. You click it, type "penne pasta",
click it again, and type "olive oil". You add "basil", then drag "basil"
onto "penne pasta", so the order is basil, penne pasta, olive oil. You remove
"olive oil" with its − button and click Save. The read view shows:

```
Ingredients
  basil
  penne pasta
Steps
  ...
```

If you click ＋ Line and then Save with the new line empty, the recipe does not
save, and that line shows "Name required" under its field.

## Terms

| Term | Meaning |
|---|---|
| Ingredient line | One row of a recipe's ingredient list, such as "penne pasta". It is not a catalog ingredient. REC-10 can link it to one (D-11). |
| Ordinal | The position of a row in its list, stored as a number from 0. The steps already use one (`StepOrdinal`). |
| `nutype` | The crate that makes a validated newtype from a field, such as `StepTitle`. `try_new` returns the value or a generated error. |
| `DragGesture` | A SwiftUI gesture that reports the pointer while you hold the mouse button and move. The canvas nodes move with it (REC-15). |
| jj | The version control tool of this repository. Each change is one reviewed unit. |

## Design

[REC-07-design.html](REC-07-design.html) shows the editor with three lines,
one with "Name required", and the read view, in light and dark. The layout
follows the mockup's editor (section 2): the Ingredients list is under the
canvas, with **＋ Line** in its header. Only the name is shown; amount, unit
and note come in REC-08.

The list shows up to 6 lines and then scrolls, so the canvas keeps most of the
height.

## Changes by layer

### db (`crates/kreatyna-db`)

A new forward-only migration, `2026-11-06-120000-0000_create_recipe_ingredient_lines`
(*sketch*):

```sql
-- The ingredient lines of a recipe (REC-07). A recipe owns its lines: they
-- go with it on delete. `ordinal` is the line's place in the list (REC-07/Q1).
CREATE TABLE recipe_ingredient_lines (
    local_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    remote_id TEXT NOT NULL UNIQUE,
    recipe_local_id BIGINT NOT NULL
        REFERENCES recipes (local_id) ON DELETE CASCADE,
    ordinal BIGINT NOT NULL CHECK (ordinal >= 0),
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    UNIQUE (recipe_local_id, ordinal)
);
```

`down.sql` drops the table. The diesel `schema.rs` gets the table and its
`joinable!`.

`recipe_models.rs`:

- `StepOrdinal` becomes `DbOrdinal`, used by steps and lines. It is the same
  type for the same column rule (an existing helper, not a second one).
- `RemoteLineId(IngredientLineId)` and `DbLineName(IngredientLineName)`. Reading
  a blank name fails, as `DbInstructions` does.
- `LineRow` and `NewLineRow`, as `StepRow` and `NewStepRow`.

`recipe_store.rs`:

- `record` loads the lines in ordinal order (`line_rows`), next to the steps.
- `create_in_transaction` and `update_in_transaction` call `insert_lines`
  after `insert_steps`. The update deletes the old line rows first, as it does
  for steps. A line keeps its `remote_id`.

### rust (`crates/kreatyna/src/recipes`)

`model.rs` (*sketch*):

```rust
/// Stable ingredient line identity. The editor generates it when you add a
/// line (REC-07), and the line keeps it through every save.
#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr, From, Into))]
pub struct IngredientLineId(Uuid);

/// A line's name: trimmed and not empty (REC-07). `try_new` fails with
/// `IngredientLineNameError::NotEmptyViolated`.
#[nutype(sanitize(trim), validate(not_empty), derive(Debug, Clone, PartialEq, Eq, AsRef))]
pub struct IngredientLineName(String);

/// A line of a recipe's ingredient list, as stored and as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientLine {
    pub id: IngredientLineId,
    pub name: IngredientLineName,
}

pub struct Recipe {
    pub id: RecipeId,
    pub title: String,
    /// In list order (REC-07/Q1).
    pub lines: Vec<IngredientLine>,
    pub steps: Vec<Step>,
    pub edges: StepEdges,
}
// NewRecipe and RecipeUpdate get the same `lines` field.
```

The stored line holds the validated name, so a stored line cannot be blank.
One type serves the store and the editor's save; a step needs two
(`Step` and `StepWrite`) only because its stored title is a `String`.

A new module `ingredient_lines.rs` holds the editor's lines, as
`step_canvas.rs` holds its steps (*sketch*):

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientLinesEvent {
    /// ＋ Line: an empty line at the bottom.
    Add,
    /// The raw text of a name field. Rust trims it only on Save.
    SetName { id: IngredientLineId, name: String },
    /// The − button.
    Remove { id: IngredientLineId },
    /// A drop of `id` onto `onto`: `id` takes `onto`'s place (REC-07/Q1),
    /// so it ends up before `onto` when it moves up and after `onto` when
    /// it moves down. With penne, oil, basil:
    ///
    /// | Drag              | Result            | The dragged line ends up |
    /// |-------------------|-------------------|--------------------------|
    /// | basil onto penne  | basil, penne, oil | before penne             |
    /// | penne onto basil  | oil, basil, penne | after basil              |
    ///
    /// Ignored when either id names no line, or when they are the same.
    Move { id: IngredientLineId, onto: IngredientLineId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientLineView {
    pub id: IngredientLineId,
    /// The name as typed.
    pub name: String,
    /// `None` until the first Save; then the name's problem, if any.
    pub problem: Option<IngredientLineNameError>,
}

pub(crate) struct DraftLines { /* Vec<DraftLine { id, name: String }> */ }

impl DraftLines {
    pub(crate) fn new(lines: &[IngredientLine]) -> Self;
    pub(crate) fn apply(&mut self, event: IngredientLinesEvent) -> DraftChange;
    pub(crate) fn view(&self, problems: ProblemDisplay) -> Vec<IngredientLineView>;
    /// Every line as written, or `Invalid` when a name is empty.
    pub(crate) fn writes(&self) -> Result<Vec<IngredientLine>, InvalidLines>;
    pub(crate) fn differs_from(&self, saved: &[IngredientLine]) -> bool;
}
```

Move by ids, not indices, so that a drop names the lines that you see. A
context-menu Move Up sends `onto` = the line above.

`CanvasChange` becomes `DraftChange` in `crate::draft_change`, because
the lines need the same "did it change" result. The canvas uses it too
(33 places in `step_canvas.rs`, a rename only). `vibetyna-rkb7` can use it
for the ingredient draft later.

`editor.rs`:

- `RecipeEditorEvent::Lines(IngredientLinesEvent)`.
- `RecipeEditorView.lines: Vec<IngredientLineView>`.
- `OpenedWith` gets `lines`, so `is_dirty` compares them.
- `save` checks the title, the steps and the lines. Any problem blocks Save.

`detail.rs`: `RecipeDetailStatus::Ready` gets
`lines: Vec<RecipeDetailLine>`, with `RecipeDetailLine { id, name: String }`.

### ffi (`crates/kreatyna-ffi`)

- `IngredientLineId` crosses as UUID text, like `StepId`: a
  `custom_type!` in `support.rs`, and an entry in `uniffi.toml` that maps it
  to a Swift struct in `apple/Kreatyna/Shared/Ids.swift`.
- `recipes.rs` mirrors `IngredientLinesEvent`, `IngredientLineView`,
  `IngredientLineNameError` (one variant, `NotEmptyViolated`, as
  `StepTitleError`), `RecipeDetailLine`, the
  new `RecipeEditorEvent::Lines` and the new view fields.

### swift

- New `KreatynaMac/MacIngredientLines.swift`: the Ingredients list under the
  canvas. The header has "Ingredients" and **＋ Line**
  (`addIngredientLine`). Each row has a handle ≡ (`ingredientLineHandle.<n>`),
  a `SyncedTextField` with the prompt "Name" (`ingredientLineName.<n>`),
  "Name required" under it when Rust says so (`ingredientLineProblem.<n>`),
  and a − button (`removeIngredientLine.<n>`). The handle has a
  `DragGesture`: Swift finds the row under the pointer and shows it, and the
  drop sends `Move` with that row's line as `onto`. The rows are in a
  `ScrollView` of at most 6 rows, not in a `List` or `Form` (MAC-05/Q4).
  A system drag (`.draggable` and `.dropDestination`) did not start in the
  UI test, so the handle uses the same gesture as the canvas nodes.
- `MacRecipeEditor.swift`: the "Coming soon" strip becomes
  `MacIngredientLines`. When a new line appears, its field gets the cursor.
- `MacRecipeDetail.swift`: a "Ingredients" section that lists the names;
  "None yet" when there are none.
- `SharedTests/Fixtures.swift`: lines in the recipe fixtures.
- Snapshots (`MacScreenSnapshotTests`): the editor with three lines, the
  editor with "Name required", and the read view with lines, in light and dark.
- UI test (`MacRecipeUITests`): add two lines, add a third, drag it to the
  top, remove one, save; the read view shows the two names in the new order.

### docs

- The epic page: Q-R1 moves into the decisions as E-09, and the REC-07 card
  says that you can drag a line.
- This plan, its design page, and the demo.

## Steps

### Change 1 — REC-07: plan (this change, no Rust)

1. Write this plan and `REC-07-design.html`. Look at the design in light and
   dark.
2. Add E-09 to the epic page and update the REC-07 card.
3. Run `bd export -o .beads/issues.jsonl`.

### Change 2 — REC-07: recipes store ingredient lines (about 450 lines, review)

1. Add the migration and the schema entry.
2. Rename `StepOrdinal` to `DbOrdinal`.
3. Add `IngredientLineId`, `IngredientLineName` and `IngredientLine`, and the
   `lines` field on `Recipe`, `NewRecipe` and `RecipeUpdate`.
4. Load and write the lines in the store.
5. The editor keeps the lines of a saved recipe and writes them back
   unchanged, so this change loses no line. It shows none yet.
6. Add store tests: create and get keep the lines in order; an update replaces
   them and a kept line keeps its id; a delete takes them with it; a recipe from
   before REC-07 upgrades with no lines; the table refuses a blank name, a
   negative ordinal, and a second line with the same ordinal; a stored blank name
   fails `get`.
7. Add `lines: vec![]` to the existing recipe literals in the tests.

### Change 3 — REC-07: the editor adds, names, moves and removes lines (about 700 lines, review)

1. Move `CanvasChange` to `crate::draft_change::DraftChange`.
2. Add `ingredient_lines.rs` and wire it into the editor.
3. Add the lines to the read view.
4. Add Rust tests in a new `crates/kreatyna/tests/ingredient_lines.rs` (green
   path, validation, red cases), with `test_case` for the moves:
   - ＋ Line adds an empty line at the bottom with a new id.
   - A name change keeps the raw text; Save trims it.
   - Remove removes only that line.
   - Move: down, up, onto itself (nothing), and an unknown id (nothing).
   - Save with an empty line: no save, "Name required" on that line only;
     the next change checks again.
   - A line change makes the editor dirty; changing it back makes it clean.
   - The read view lists the lines in order.

### Change 4 — REC-07: ingredient lines on the Mac (about 600 lines, review of the FFI)

1. Mirror the types in the FFI, and add the id to `uniffi.toml` and
   `Ids.swift`.
2. Try the drag first: build `MacIngredientLines` with the handle, and run
   the UI test's drag. If it does not move the line, use the context-menu
   fallback, and write that down in Validation. The UI test comes in this
   change, because the drag needs it.
3. Build the rest of the list, the read view section, the fixtures and the
   snapshots.

### Change 5 — REC-07: the UI test and the demo (about 200 lines, report)

1. Record the demo per document-apple-delivery: the editor with lines, the
   drag, "Name required", and the read view.

## Checks

| Command | Expected |
|---|---|
| `devenv shell -- cargo fmt --check` | No output. |
| `devenv shell -- cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| `devenv shell -- cargo test --workspace` | All pass; the new store and editor tests among them. |
| `devenv shell -- mac-bindings debug` | Generates the bindings. |
| `devenv shell -- mac-test --ui MacRecipeUITests` | All unit and snapshot tests, and the recipe UI tests, pass. |
| `devenv shell -- ios-app-build` | Builds. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| A line has only a name. | REC-08: amount, known unit and note. |
| A line links to no catalog ingredient. | REC-10, REC-11. |
| Steps do not use lines. | REC-20. |
| No keyboard way to add, remove or move a line. | Not planned; the − button and the context menu cover the mouse-free cases only in the fallback. |
| The list row does not count lines (D-15: title, time, steps). | Not planned. |

## Validation

Change 4:

| Check | Result |
|---|---|
| `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` | Clean. |
| `cargo test --workspace` | 910 pass (7 new in the FFI). |
| `mac-bindings debug` | Generates the bindings. |
| `mac-test --ui MacRecipeUITests` | All unit, snapshot and recipe UI tests pass, with the new `testIngredientLinesAddDragRemoveAndSave`. |
| `ios-app-build` | Builds. |

The drag works on macOS, so the context-menu fallback was not needed. A
system drag (`.draggable` and `.dropDestination`) did not start under
XCUITest: three tries (a drop on the name field, a slow drag with a hold, a
drop on the handle) left the order unchanged. A `DragGesture` on the handle
moves the line, as the canvas nodes do.

The 12 editor snapshots (24 images) changed only in the bottom strip:
"Ingredients Coming soon" became "Ingredients" with ＋ Line, and the canvas
is a few points shorter.
