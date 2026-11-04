# REC-16 — Delete a step

Tracked in beads: `vibetyna-hrx.16`. Changes: three, listed in Steps.
Dependencies: REC-14 (`vibetyna-hrx.14`), closed. REC-15 (`vibetyna-hrx.15`)
is closed too; this plan builds on its canvas.

## Open questions

None. You answered REC-16/Q1 in the REC-16 Inbox thread. It is in Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-16/Q1 | Where does the keyboard go when you click a node? | A click puts the cursor in the step's Title, as today. ⌘⌫ deletes the selected step, and so do the menus. |

## Decided in the plan

| Rule | Follows from |
|---|---|
| ⌘⌫ lives in the menu bar: **Edit ▸ Delete Step**, enabled while a step is selected. A menu item shows the shortcut, and macOS gives a menu's ⌘ keys to the menu before the Title field gets them. Cost: in the step's Title field, ⌘⌫ no longer deletes to the start of the line. | REC-16/Q1. The app has no menu bar items yet; `vibetyna-tl6` plans Editor ▸ Add Step the same way. |
| Delete asks nothing. The step goes at once. | The card: "It disappears". The editor is a draft: Cancel asks "Discard changes?" and restores every step. Undo is out of the epic. |
| A deleted selected step leaves nothing selected. The inspector shows "Select a step". | Figma and ComfyUI. Rust never picks a step for you. |
| Right-click ▸ Delete Step on an unselected node deletes that node. The selection stays. | The card: "right-click it". |
| A deleted saved step makes the editor "changed": Cancel asks. Delete a step you added since the last Save, and the editor is unchanged again. | E-10. `StepCanvas::differs_from` already compares the steps. |
| No database change. `update` already deletes the step rows and writes the new list. | REC-14. The test `an_update_replaces_the_steps_and_a_kept_step_keeps_its_id` already drops a step. |
| "Anything that refers to it" is the selection today. Connections (REC-18) remove their own edges then. | The card. |

## Outcome

You open Shakshuka, which has the steps "Dice onion" and "Fry onion", and
click Edit. You click "Dice onion" and press ⌘⌫. The node disappears, and
the inspector says "Select a step". You right-click "Fry onion" and choose
**Delete Step**. The canvas is empty. You click Save. The row says
"Shakshuka" with no step count, and the read view lists no steps.

## Terms

- **⌘⌫:** Command and the Delete key on a Mac keyboard (Backspace on other
  keyboards). Finder uses it for Move to Trash.
- **Keyboard focus:** the view that gets the keys you press. Swift owns it
  (REC-14).
- **Focused value:** a SwiftUI value that a view publishes while its window
  is active. A menu bar command reads it, so the command can act on the open
  editor.

## Design

The target is section 2 of the
[recipes mockups](../../../tasks/recipes/mockups.html). It does not show a
delete. This slice adds one context menu item on a node:

| Moment | What you see |
|---|---|
| Right-click a node | A menu with **Delete Step** (trash icon). The canvas menu with Add Step does not open. |
| Edit menu | **Delete Step ⌘⌫**, after the Cut, Copy, and Paste group. It is disabled when no step is selected or no editor is open. |
| Delete | The node disappears. If it was selected, the inspector says "Select a step". |

No new mockup is needed: after a delete, the canvas looks like the REC-14
`recipeCanvas` snapshot with one node less.

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
    Move { id: StepId, to: CanvasPoint },
    /// Edit ▸ Delete Step (⌘⌫) on the selected node, or right-click ▸
    /// Delete Step. Removes the
    /// step; a deleted selected step leaves nothing selected. Ignored for an
    /// id that names no step.
    Delete(StepId),
}
```

`StepCanvas::apply` handles it (*sketch*):

```rust
StepCanvasEvent::Delete(id) => {
    let Some(index) = self.steps.iter().position(|step| step.id == id) else {
        return CanvasChange::Unchanged;
    };
    self.steps.remove(index);
    // `selected` must always name a step in `steps`.
    if self.selected == Some(id) {
        self.selected = None;
    }
    CanvasChange::Changed
}
```

The editor needs no change: `change_canvas` renders on `Changed`, `is_dirty`
compares the steps, and Save writes the list without the deleted step.

**Tests** (`crates/kreatyna/tests/recipe_editor.rs`, with the REC-14
fixture):

| Test | It checks |
|---|---|
| `deleting_the_selected_step_removes_it_and_clears_the_selection` | The view, and one render. |
| `deleting_an_unselected_step_keeps_the_selection` | Right-click on the other node. |
| `a_canvas_event_that_changes_nothing_does_nothing` | New case: delete an unknown step. |
| `save_writes_the_steps_without_the_deleted_one` | `written_steps` has only the kept step. |
| `deleting_the_untitled_step_lets_save_pass` | Save refuses an untitled step (REC-14/Q2); you delete it; Save writes. |
| `cancel_after_changing_a_saved_step_asks` | New case: delete it. |
| `deleting_an_added_step_keeps_the_editor_unchanged` | Add, then delete: Cancel discards at once. |
| `edits_steps_save_and_cancel_are_ignored_while_saving`, `canvas_events_are_ignored_while_discard_is_asked` | New Delete cases. |

### ffi (`crates/kreatyna-ffi/src/app_types/recipes.rs`)

`StepCanvasEvent::Delete { id: StepId }`, and its conversion to the core
event. One conversion case in `app_types/tests.rs`.

### swift

`MacStepCanvas.swift`:

- Each node gets a context menu with **Delete Step**. It sends
  `.delete(id:)` for that node.

`MacRecipeEditor.swift`:

- The editor publishes a focused value `deleteStep` while a step is
  selected. It is the action that sends `.canvas(.delete(id:))` for the
  selected step's id, which Swift reads from Rust's view. With no selection,
  the value is absent.

`KreatynaMacApp.swift` and a new `MacEditCommands.swift`:

- `CommandGroup(after: .pasteboard)` adds **Delete Step** with
  `.keyboardShortcut(.delete, modifiers: .command)`. It reads the focused
  value and is disabled without it.

The inspector does not change: a click still puts the cursor in Title
(REC-16/Q1).

**Unit test** (`MacStepCanvasTests`): none. The new Swift code is wiring;
the UI test covers it.

**UI test** `testDeleteStepsAndSave` (`MacRecipeUITests`):

1. Create Shakshuka with "Dice onion" and "Fry onion". Save. Click Edit.
2. Check that Edit ▸ Delete Step is disabled: nothing is selected.
3. Click "Dice onion". The cursor is in its Title. Press ⌘⌫. Check that one
   node is left and the inspector says "Select a step".
4. Right-click "Fry onion". Choose Delete Step. Check that no node is left.
5. Save. Check that the read view lists no steps.

**Demo** `testCaptureREC16Demo`: `01-edit-menu.png` (Edit ▸ Delete Step with
"Dice onion" selected), `02-context-menu.png` (Delete Step on "Fry onion"),
`03-after-delete.png`.

**Snapshots:** no new snapshot. The card asks for a snapshot of the context
menu, but a menu is a window of its own, and the snapshot tests render one
view. The demo's `02-context-menu.png` shows the menu instead, as REC-14's
`01-context-menu.png` did.

### docs

The Validation section of this plan, after the last change.

## Steps

### Change 1 — REC-16: plan (this change)

1. Write this plan. Set `plan: "tasks/REC-16.md"` on the epic card, and the
   bead's spec.
2. Send it, with REC-16/Q1, to the REC-16 thread. Record the answer.

### Change 2 — REC-16: the editor deletes steps (about 200 lines, review)

1. Add `StepCanvasEvent::Delete` and handle it in `StepCanvas::apply`.
2. Write the editor tests.
3. Add the FFI event and its test.
4. Run `mac-bindings debug`, so the Mac build stays green.
5. Run checks 1 to 3 and check 7. Send the guided review.

### Change 3 — REC-16: delete nodes on the Mac (about 300 lines, report)

1. Add the node context menu.
2. Add the `deleteStep` focused value and the Edit ▸ Delete Step command.
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
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. No image changes. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh mac …/testCaptureREC16Demo docs/epics/recipes/tasks/REC-16-demo` | `01-edit-menu.png`, `02-context-menu.png`, `03-after-delete.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub | Replaced by |
|---|---|
| The inspector shows Duration, Instructions, and After as disabled placeholders. | REC-17, REC-18 |
| No connections, so a delete removes only the step. | REC-18 |

- No question before a delete, and no undo (the epic excludes undo).
- No delete of several nodes at once: there is no multiple selection.
- No Delete button in the inspector or the toolbar.
- No other menu bar items. Editor ▸ Add Step stays in `vibetyna-tl6`.
- No recipe screens on iOS (E-04).

## Validation

Not run yet.
