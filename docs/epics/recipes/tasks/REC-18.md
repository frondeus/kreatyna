# REC-18 — Connect steps into a graph

Tracked in beads: `vibetyna-hrx.18`. Changes: six, listed in Steps.
Dependencies: REC-14 (`vibetyna-hrx.14`), closed. REC-15, REC-16 and
REC-17 are closed too; this plan builds on their canvas and step model.

## Open questions

None. You answered three questions in the REC-18 Inbox thread. They are in
Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-18/Q1 | How do you remove an arrow? | As for steps: a click selects the arrow; Edit ▸ Delete Connection (⌘⌫) or right-click ▸ Delete Connection removes it. |
| REC-18/Q2 | What do you see when a connection is refused? | It snaps back, and a short line under the canvas says why ("That would make a loop."), until your next change. |
| REC-18/Q3 | Where can you drop the arrow? | Anywhere on the target node. The ○ only shows where the arrow ends. |
| REC-18/Q4 | What are an edge's ends called? (plan review) | `from` and `to`, as the arrow is drawn, not `before` and `after`. The inspector keeps the label **After**. |

## Decided in the plan

| Rule | Follows from |
|---|---|
| An arrow from A's ● to B's ○ means "B comes after A". Rust names the ends `from` and `to` (REC-18/Q4). | D-20, the mockup, your plan review. |
| Rust refuses three connections: a step to itself, a duplicate arrow, and an arrow that closes a loop. Several loose ends are fine. | The epic card, D-22. |
| Deleting a step deletes its arrows. | An arrow needs both ends. |
| The inspector's **After** shows read-only chips: the steps with an arrow into this step, in the order added. | The mockup: "Drawn on the canvas; mirrored here." |
| Save rewrites the arrows with the steps, in the same transaction. Each arrow row points at its two step rows, with `ON DELETE CASCADE`. | Cooldown 3 note: Save deletes and inserts the steps, so step row ids change on every Save. |
| A refused connection changes no step and no arrow. The refusal is not a problem: Save ignores it. | Q2. |
| The cycle check gets property-style tests without a new crate: every set of arrows between 4 steps (4,096 graphs) against a second, simpler check. | The epic card asks for property-style tests. An exhaustive check over small graphs is deterministic. |

## Outcome

You open Shakshuka and click Edit. You press on the ● at the right of
"Dice onion" and drag. A dashed line follows the pointer. You release it
over "Fry onion". An arrow now goes from "Dice onion" to "Fry onion". You
click "Fry onion": the inspector's **After** shows a chip "Dice onion".

You drag from the ● of "Fry onion" to "Dice onion". The line snaps back,
and the line under the canvas says "That would make a loop." It goes away
after your next change.

You click the arrow. It turns the accent color, and the inspector shows
"Dice onion → Fry onion". You press ⌘⌫, and the arrow is gone. You connect
the steps again and press ⌘S. After Edit, the arrow is still there.

## Terms

| Term | Meaning |
|---|---|
| Port | The small circle on a node's side. ○ (left) is the input, ● (right) the output. |
| Edge, connection, arrow | One "comes after" link between two steps. Rust calls it `StepEdge`; the UI calls it a connection; the canvas draws an arrow. |
| Loop (cycle) | Arrows that lead from a step back to itself, such as A → B → A. A recipe with a loop has no first step. |
| Rubber band | The dashed line Swift draws from the ● to the pointer during a drag. Only Swift knows about it. |
| `ON DELETE CASCADE` | A SQLite foreign-key rule: deleting a row also deletes the rows that point at it. |

## Design

The target state is in the epic mockups,
[docs/tasks/recipes/mockups.html](../../../tasks/recipes/mockups.html),
"Editor" screen: the ports, the curved arrows with arrowheads, the dashed
"dragging a connection…" line, and the After chips. No new design page is
needed. The selected arrow and the refusal line follow the existing
selection (accent) and problem (red footnote) styles.

## Changes by layer

### db (`crates/kreatyna-db`)

Migration `2026-11-05-120000-0000_create_recipe_step_edges` (*sketch*):

```sql
-- "to" comes after "from" (REC-18). An edge goes with either step.
CREATE TABLE recipe_step_edges (
    from_local_id BIGINT NOT NULL
        REFERENCES recipe_steps (local_id) ON DELETE CASCADE,
    to_local_id BIGINT NOT NULL
        REFERENCES recipe_steps (local_id) ON DELETE CASCADE,
    PRIMARY KEY (from_local_id, to_local_id),
    CHECK (from_local_id <> to_local_id)
);
```

The table has no `recipe_local_id`: both ends belong to the recipe through
their steps. `down.sql` stays empty (forward-only migrations).

`recipe_store.rs`:

- `insert_edges(connection, step_ids: &HashMap<StepId, LocalStepId>, edges)`
  writes after `insert_steps`. An edge whose step is not in the written list
  fails the save with `Storage`.
- `update_in_transaction` deletes the steps, as today. The cascade deletes
  their edges. Then it inserts the steps and the edges.
- `record` reads the edges whose `from` step belongs to the recipe, joined
  to both steps for their remote ids. An edge whose `to` step belongs to
  another recipe, or a set of edges with a loop, fails `get` with `Storage`.

### rust (`crates/kreatyna/src/recipes`)

New module `graph.rs` (*sketch*):

```rust
/// "`to` comes after `from`": an arrow from the ● of `from` to the ○ of
/// `to`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StepEdge {
    pub from: StepId,
    pub to: StepId,
}

/// Why Rust refuses a connection (REC-18/Q2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectRefusal {
    /// "A step cannot come after itself."
    SameStep,
    /// "These steps are already connected."
    Duplicate,
    /// "That would make a loop."
    Loop,
}

/// A recipe's edges: never a self-loop, a duplicate, or a loop. Only
/// `connect` and `from_saved` make one, so a value always passes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StepEdges(Vec<StepEdge>);

impl StepEdges {
    /// Adds `edge`, or says why not.
    pub fn connect(&mut self, edge: StepEdge) -> Result<(), ConnectRefusal>;
    /// The edges the store read. Fails for any edge `connect` would refuse.
    pub fn from_saved(edges: Vec<StepEdge>) -> Result<Self, ConnectRefusal>;
    /// Removes `edge`. Says whether it was there.
    pub(crate) fn disconnect(&mut self, edge: StepEdge) -> CanvasChange;
    /// Removes every edge of `step`.
    pub(crate) fn remove_step(&mut self, step: StepId);
    /// The edges in the order connected.
    pub fn iter(&self) -> impl Iterator<Item = &StepEdge>;
}
```

The loop check: `edge` closes a loop when `edge.from` can be reached from
`edge.to` along the existing edges. A depth-first search with a visited
set does it in time linear in the edges.

`model.rs`: `Recipe`, `NewRecipe` and `RecipeUpdate` gain
`edges: StepEdges`. Every place that builds one is updated: `editor.rs`,
`recipe_models.rs`, and the tests in `app.rs`, `recipe_detail.rs`,
`recipe_editor.rs`, `recipe_section.rs`, and the store tests.

`step_canvas.rs`:

- `StepCanvas` keeps `edges: StepEdges` and a `Selection` instead of
  `selected: Option<StepId>`:

  ```rust
  enum Selection {
      None,
      Step(StepId),
      Connection(StepEdge),
  }
  ```

  Only the canvas's own methods change it, so a selected step or edge always
  exists.
- New events (*sketch*):

  ```rust
  /// A drop of a ● drag on a node (REC-18/Q3). Adds the edge, or keeps the
  /// refusal for the view. Selects nothing new.
  Connect { from: StepId, to: StepId },
  /// A click on an arrow. Ignored for an edge that does not exist.
  SelectConnection(StepEdge),
  /// Edit ▸ Delete Connection (⌘⌫), or right-click ▸ Delete Connection.
  DeleteConnection(StepEdge),
  ```

- `Delete(StepId)` also removes the step's edges.
- The refusal: `Connect` that is refused stores `refusal: Some(…)`. Any
  later event that changes the canvas clears it (REC-18/Q2). An event that
  changes nothing keeps it.
- `differs_from` compares the edges as a set, so the order connected does
  not make the editor dirty.

The view (*sketch*). `StepCanvasView` becomes a struct. Its old enum is
renamed `StepNodesView` and gets a third variant:

```rust
pub struct StepCanvasView {
    pub nodes: StepNodesView,
    /// In the order connected. Swift draws an arrow for each.
    pub connections: Vec<StepEdge>,
    /// The last refused connection, until the next change.
    pub refusal: Option<ConnectRefusal>,
}

pub enum StepNodesView {
    Unselected { steps: Vec<StepNodeView> },
    Selected { selected: StepNodeView, others: Vec<StepNodeView> },
    /// The inspector shows "Dice onion → Fry onion".
    ConnectionSelected { connection: StepConnectionView, steps: Vec<StepNodeView> },
}

pub struct StepConnectionView {
    pub edge: StepEdge,
    pub from: StepLabel,
    pub to: StepLabel,
}

// StepNodeView gains:
    /// The inspector's After: the steps with an arrow into this one, in the
    /// order added.
    pub after: Vec<StepAfterView>, // { id: StepId, label: StepLabel }
```

**Tests:**

| File | It checks |
|---|---|
| `tests/step_graph.rs` (new) | `connect` refuses a self-loop, a duplicate, and loops of 2, 3 and 4 steps; it accepts a diamond and loose ends. `from_saved` refuses the same cases. The exhaustive test: for every set of the 12 possible edges between 4 steps, adding them one by one with `connect` succeeds exactly when a topological sort (Kahn's algorithm) of the set succeeds. |
| `tests/recipe_editor.rs` | Connect adds an arrow and the target's After. Each refusal leaves the steps and arrows, shows the refusal, and the next change clears it. A click on an arrow selects it; Delete Connection removes it. Deleting a step removes its arrows and their After chips. Save writes the edges. Cancel asks after a connect and after a disconnect; connecting and then disconnecting the same edge keeps the editor unchanged. The new events are ignored for unknown ids, while saving, and while Discard is asked. |
| `recipe_store/tests.rs` | Create and update keep the edges, also when the steps' row ids change. A deleted step takes its edges. An edge to an unknown step fails the save and changes nothing. A recipe from before REC-18 has no edges. The table refuses a self-loop and a duplicate. Rows with a loop fail `get`. |

### ffi (`crates/kreatyna-ffi`)

- `StepEdge`, `ConnectRefusal`, `StepConnectionView`, `StepAfterView` as
  UniFFI types; the `StepCanvasView` struct and the `StepNodesView` enum.
- The three new events.
- Conversion tests in `app_types/tests.rs`.

### swift

`MacStepCanvas.swift`:

- Each node gets ○ on its left and ● on its right, as in the mockup.
- A drag that starts on a ● draws the rubber band (dashed, accent) to the
  pointer. On the drop, Swift finds the node under the pointer and sends
  `.connect(from:to:)`. A drop on empty canvas sends nothing.
- Arrows are curves from the ● of `from` to the ○ of `to`, with an
  arrowhead, under the nodes. A click within 5 points of a curve sends
  `.selectConnection`. The selected arrow is accent and thicker.
- An arrow's context menu has Delete Connection.
- Under the canvas, the refusal line in red footnote text:
  - `SameStep`: "A step cannot come after itself."
  - `Duplicate`: "These steps are already connected."
  - `Loop`: "That would make a loop."

`MacEditCommands.swift`: `DeleteStepAction` becomes `DeleteSelectionAction`
with a title: the item reads "Delete Step" or "Delete Connection". ⌘⌫ stays.

`MacStepInspector.swift`:

- After: read-only chips with the step labels, or "None" in secondary text.
  The hint "Drawn on the canvas."
- For a selected arrow: "Connection", then "Dice onion → Fry onion".

**Unit tests** (`MacStepCanvasTests`): the curve's end points for two
nodes; the hit test of a point near and far from a curve; the node under a
drop point.

**Snapshots** (`MacScreenSnapshotTests`, light and dark):

- New `recipeCanvasConnected`: four steps, a diamond of arrows, and a
  selected step with two After chips.
- New `recipeConnectionDrag`: the rubber band from a ● to a point (the drag
  state comes in through an initializer meant for tests).
- New `recipeConnectionSelected`: an accent arrow and the inspector.
- New `recipeConnectionRefused`: the loop line under the canvas.
- Changed: every snapshot that shows a node gets the two ports, and the
  inspector's After becomes real. Each one is checked by eye.

**UI test** `testConnectStepsAndSave` (`MacRecipeUITests`): add "Dice
onion" and "Fry onion"; drag from Dice onion's ● to Fry onion; select Fry
onion and see the After chip; drag back for the loop line; save; Edit; the
arrow is still there; click it, press ⌘⌫, and it is gone.

**Demo** `testCaptureREC18Demo`: `01-connected.png` (arrows and After),
`02-loop-refused.png`, `03-connection-menu.png` (the arrow's context menu,
cropped to the window).

### docs

The Validation section of this plan, after the last change.

## Steps

### Change 1 — REC-18: plan (this change)

1. Write this plan. Set `plan: "tasks/REC-18.md"` on the slice in
   `epic.html`, and the bead's spec.
2. Send it to the REC-18 thread.

### Change 2 — REC-18: recipes store step edges (about 650 lines, review)

1. Add `graph.rs`: `StepEdge`, `ConnectRefusal`, `StepEdges`.
2. Write `tests/step_graph.rs`, the exhaustive test among them.
3. Add `edges` to `Recipe`, `NewRecipe` and `RecipeUpdate`. Update every
   place that builds one.
4. Add the migration, the schema, the edge rows, `insert_edges`, and the
   load.
5. The editor keeps the loaded edges and writes them back unchanged.
6. Write the store tests. Run checks 1 to 3 and check 7. Send the guided
   review.

### Change 3 — REC-18: the canvas view carries connections (about 450 lines, review)

1. Rename the view enum to `StepNodesView`; add the `StepCanvasView`
   struct with `connections` and `refusal`, and `after` on each node.
2. Update the editor tests (`.canvas` becomes `.canvas.nodes` where they
   compare nodes).
3. Add the FFI types and tests. Swift reads `canvas.nodes`; it draws no
   arrows yet.
4. Run `mac-bindings debug`, checks 1 to 4, and check 7. Send the guided
   review.

### Change 4 — REC-18: the editor connects and disconnects steps (about 700 lines, review)

1. Add `Selection`, the three events, the refusal, and the step delete's
   edge removal.
2. Add `ConnectionSelected` to the view.
3. Write the editor tests.
4. Add the FFI events and tests. Keep Swift building: `ConnectionSelected`
   shows "Select a step" until change 5.
5. Run checks 1 to 4 and check 7. Send the guided review.

### Change 5 — REC-18: arrows and ports on the Mac (about 700 lines, report)

1. Add the ports, the rubber band, and `.connect`.
2. Draw the arrows; add the hit test, the selection, and the context menu.
3. Add Delete Connection to the Edit menu.
4. Add After and the connection view to the inspector, and the refusal
   line.
5. Add the unit tests. Record the new and changed snapshots, and check each
   one by eye.
6. Run checks 4, 5 and 7. Send a short report with the snapshots.

### Change 6 — REC-18: the UI test and the demo (about 200 lines, report)

1. Add `testConnectStepsAndSave` and `testCaptureREC18Demo`.
2. Run checks 1 to 7. Capture the demo (check 6).
3. Fill in Validation in this plan.
4. Send the demo and the report.

## Checks

Test scope: Rust in full for each change. On the Mac, all unit and snapshot
tests, and the UI tests of the Recipes section only (`--ui MacRecipeUITests`).

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. Only the image changes listed above. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh mac …/testCaptureREC18Demo docs/epics/recipes/tasks/REC-18-demo` | `01-connected.png`, `02-loop-refused.png`, `03-connection-menu.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

- The read view does not show the graph or stages. REC-19 groups the steps
  by stage.
- No auto-arrange and no edge routing around nodes: arrows are plain curves
  and may cross nodes (D-19, F-02).
- No keyboard way to connect steps. Connecting needs the pointer, as in
  node editors.
- The After chips are read-only. You change connections on the canvas only.
- Total time and scheduling come with REC-21.
- iOS shows no recipes (E-04).

## Validation

Not run yet.
