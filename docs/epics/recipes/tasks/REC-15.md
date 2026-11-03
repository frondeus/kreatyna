# REC-15 — Move steps on the canvas

Tracked in beads: `vibetyna-hrx.15`. Changes: three, listed in Steps.
Dependencies: REC-14 (`vibetyna-hrx.14`), closed. This plan builds on the
REC-14 canvas.

## Open questions

None. You answered the two questions in the REC-15 Inbox thread. They are in
Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-15/Q1 | Does dragging a node select it? | Yes. The dragged node becomes the selected one, as in node editors such as ComfyUI or Figma. |
| REC-15/Q2 | Where does a dropped node land? | Exactly where you drop it, in whole points. No snap to the dot grid. |

### Decided in the plan

You saw these rules in the questions message, with no objection.

| Rule | Follows from |
|---|---|
| Swift draws only the drag in progress. On drop, Swift sends one event, `StepCanvasEvent::Move { id, to }`. | The card's acceptance. |
| A position is never negative. A drag past the left or the top edge stops at 0. Swift's `canvasPoint` already clamps. | D-19, `1pj`. |
| A drop at the position the node already has, on a node that is already selected, changes nothing. Rust does not render. | `CanvasChange::Unchanged`. |
| A click without movement selects, as in REC-14. A drag starts after 4 points of movement. | REC-14. |
| A moved step makes the editor "changed", so Cancel asks "Discard changes?". A step moved back to its saved position does not count. | E-10. `StepCanvas::differs_from` already compares positions. |
| No database change. `update` already writes each step's position. | REC-14. |

## Outcome

You open Shakshuka, which has the steps "Dice onion" and "Fry onion", and
click Edit. You drag "Dice onion" down and to the right. While you drag, the
node follows the pointer above the other nodes. When you release it, it stays
there and becomes the selected step: it gets the accent border, and the
inspector shows its Title. You click Save. You open Pasta, then Shakshuka
again, and click Edit. "Dice onion" is where you dropped it.

## Terms

- **Drag:** you press on a node, move the pointer at least 4 points, and
  release. The release is the **drop**.
- **Drag in progress:** the time between the press and the drop. Only Swift
  knows about it. Rust learns the result on the drop.
- **Canvas point:** a position on the canvas in whole points, with x to the
  right and y down from the top-left corner (REC-14).

## Design

The target is section 2 of the
[recipes mockups](../../../tasks/recipes/mockups.html). It does not show a
drag, so this slice follows common node-editor behavior:

| Moment | What you see |
|---|---|
| Drag in progress | The node follows the pointer and draws above the other nodes. Its border does not change yet. |
| Drop | The node stays at the drop point. It is selected: accent border, and its Title in the inspector. |
| Drop past the left or the top edge | The node stops at x = 0 or y = 0. |
| Drop past the right or the bottom edge | The canvas grows to hold the node, as in REC-14. |

No new mockup is needed: a drop looks like the REC-14 `recipeCanvas`
snapshot with the node in a new place.

## Changes by layer

### db

No change.

### rust (`crates/kreatyna/src/recipes/step_canvas.rs`)

One new event (*sketch*):

```rust
pub enum StepCanvasEvent {
    Add { at: CanvasPoint },
    Select(StepId),
    ClearSelection,
    SetTitle { id: StepId, title: String },
    /// A drop after a drag. Puts the step at `to` and selects it
    /// (REC-15/Q1). Ignored for an id that names no step.
    Move { id: StepId, to: CanvasPoint },
}
```

`StepCanvas::apply` handles it (*sketch*):

```rust
StepCanvasEvent::Move { id, to } => {
    let Some(step) = self.steps.iter_mut().find(|step| step.id == id) else {
        return CanvasChange::Unchanged;
    };
    let placed = step.place(to);
    // Both run: the position and the selection can each change.
    placed.or(self.select(id))
}

impl DraftStep {
    /// Puts the step at `to`, and says whether its position changed.
    fn place(&mut self, to: CanvasPoint) -> CanvasChange { /* … */ }
}

impl CanvasChange {
    /// `Changed` when either is `Changed`.
    fn or(self, other: CanvasChange) -> CanvasChange { /* … */ }
}
```

Each "did it change" result is a `CanvasChange`, not a `bool`
(designing-types). The editor needs no change: `change_canvas` already
renders on `Changed`, `is_dirty` already compares positions, and Save already
writes them.

**Tests** (`crates/kreatyna/tests/recipe_editor.rs`, with the REC-14
fixture):

| Test | It checks |
|---|---|
| `a_move_puts_the_step_at_the_point_and_selects_it` (cases: nothing selected, another step selected, the same step selected) | The view, and one render. |
| `a_move_to_the_same_point_of_the_selected_step_does_nothing` | No render. Joins `a_canvas_event_that_changes_nothing_does_nothing` as a case. |
| `a_move_to_the_same_point_selects_an_unselected_step` | Selection only, and one render. |
| `a_move_of_an_unknown_step_does_nothing` | A case of the same "changes nothing" test. |
| `save_writes_the_moved_position` | `written_steps` has the new position. |
| `cancel_after_moving_a_saved_step_asks` | A case of the existing Cancel tests. |
| `moving_a_step_back_to_its_saved_point_keeps_the_editor_unchanged` | A case of the existing "unchanged" test. |
| `moves_are_ignored_while_saving_or_while_discard_is_asked` | Cases of the two existing "ignored" tests. |

### ffi (`crates/kreatyna-ffi/src/app_types/recipes.rs`)

`StepCanvasEvent::Move { id: StepId, to: CanvasPoint }`, and its conversion
to the core event. One conversion test in `app_types/tests.rs`.

### swift (`apple/Kreatyna/KreatynaMac/MacStepCanvas.swift`)

- Each node gets a `DragGesture(minimumDistance: 4)` in the canvas's
  coordinate space, next to its tap gesture.
- `@State private var drag: NodeDrag?` holds the dragged node's id and the
  translation. While it is set, the node draws at its position plus the
  translation, last in the `ZStack`, so it is on top.
- On drop, Swift sends `.move(id:, to: Self.canvasPoint(position +
  translation))`. Swift keeps drawing the node at the drop point until the
  next view arrives, so the node does not jump back for one frame.
- Swift makes no decision: Rust selects, and the clamp in `canvasPoint` only
  keeps the value in the range of `UInt32`.

**UI test** `testDragStepAndReopen` (`MacRecipeUITests`):

1. Create Shakshuka with one step, "Dice onion". Save.
2. Click Edit. Drag the node by (200, 120) points with
   `press(forDuration:thenDragTo:)`.
3. Check that the inspector shows "Dice onion": the node is selected.
4. Save. Open Pasta, then Shakshuka. Click Edit.
5. Check that the node's frame moved by (200, 120) from where it was in step 2.

**Demo** `testCaptureREC15Demo`: `01-before-drag.png`, `02-after-drop.png`
(selected, inspector), `03-reopened.png`.

**Snapshots:** no new snapshot. A dropped node looks like `recipeCanvas`, and
a drag in progress cannot be held still in a snapshot test.

### docs

The Validation section of this plan, after the last change.

## Steps

### Change 1 — REC-15: plan (this change)

1. Write this plan. Set `plan: "tasks/REC-15.md"` on the epic card, and the
   bead's spec.
2. Send it to the REC-15 thread.

### Change 2 — REC-15: the editor moves steps (about 250 lines, review)

1. Add `StepCanvasEvent::Move` and handle it in `StepCanvas::apply`.
2. Write the editor tests.
3. Add the FFI event and its test.
4. Run `mac-bindings debug`, so the Mac build stays green.
5. Run checks 1 to 3 and check 7. Send the guided review.

### Change 3 — REC-15: drag nodes on the Mac (about 350 lines, report)

1. Add the drag gesture to `MacStepCanvas`.
2. Add the UI test and the demo method.
3. Run checks 4, 5 and 7.
4. Capture the demo (check 6).
5. Fill in Validation in this plan.
6. Send the demo and the report.

## Checks

Test scope: Rust in full for each change. On the Mac, all unit and snapshot
tests, and the UI tests of the Recipes section only (`--ui MacRecipeUITests`).

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. No image changes. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh mac …/testCaptureREC15Demo docs/epics/recipes/tasks/REC-15-demo` | `01-before-drag.png`, `02-after-drop.png`, `03-reopened.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub | Replaced by |
|---|---|
| Nodes cannot be deleted. | REC-16 |
| The inspector shows Duration, Instructions, and After as disabled placeholders. | REC-17, REC-18 |
| No ports and no connections on the canvas. | REC-18 |

- No snap to the dot grid (REC-15/Q2).
- No move from the keyboard (for example the arrow keys). Add Step from the
  keyboard is filed as `vibetyna-tl6`; a keyboard move would join it.
- No multiple selection, and no move of several nodes at once.
- No auto-scroll while a drag reaches the edge of the visible canvas.
- No auto-arrange (D-19, F-02).
- No recipe screens on iOS (E-04).

## Validation

Not run yet.
