# MAC-05 — Create an ingredient from the toolbar

Tracked in beads: `vibetyna-4rn.9`. Change: `kvmxpqvv`. Dependencies: MAC-04
(approved).

## Open questions

None. The user answered MAC-05/Q1 to Q4 in the Inbox on 2026-10-13 (see
Decisions).

## Decisions

### MAC-05/Q1 — A row click on a dirty create form asks first (E-10 now)

*Context:* the list stays visible next to the form. E-10 says that a dirty
form asks "Discard changes?" before it switches. A clean form switches at
once. The epic put E-10 in MAC-06, but the create form is the first form that
can be dirty.

*Decision:* do E-10 in this slice (the recommendation). **Discard Changes**
opens the clicked row. **Keep Editing** keeps the form and forgets the click.
MAC-06 reuses the rule for the edit form.

### MAC-05/Q2 — A highlighted draft row while the create form is open

*Context:* the Mac coordinator reports the highlighted row as `selected`, the
ID of the ingredient in the pane. The create form is not a row. The mockup
([MAC-05-design.html](MAC-05-design.html), section 1) compared three options:
A, no highlight; B, the previous row stays highlighted; C, a draft row.

*Decision:* C, the pattern of macOS Contacts and Notes. While the create form
is open, a draft row is at the top of the list, highlighted. It shows the
typed name, or "New Ingredient" while the name is empty. The search does not
hide it. Save puts the real row at its sorted place; Cancel removes the draft
row. `selected` is `None` while the form is open. A click on the draft row
does nothing. (The Inbox refused the answer card because the label was cut
off; the user's note names C.)

### MAC-05/Q3 — A save keeps the search

*Context:* you search "tom" and create "Basil". The search hides Basil's row.
MAC-08/Q1 decided that a search keeps the open detail.

*Decision:* keep the search (the recommendation). Basil's detail opens. Its
row is hidden until you clear the search; then it is highlighted.

### MAC-05/Q4 — A remove button per unit, and drag to reorder

*Context:* on iOS you swipe a unit to delete it and drag it to reorder. macOS
has no swipe.

*Decision:* each unit has a remove (−) button. You drag a unit by its handle
(≡) to reorder it. If drag does not work inside a grouped `Form` on macOS,
the unit gets Move Up and Move Down in its context menu instead, and
Validation says so (the fallback named in the question).

### MAC-05/Q5 — The snapshot flake waits for its own change

*Context:* 4 Mac list snapshots with an empty detail pane render the
"Ingredients" title a little darker than their baselines. The approved change
before MAC-05 fails the same way, so the cause is on this Mac, not in the
slice (see Validation).

*Decision:* keep MAC-05's baselines (the recommendation). A separate change
after MAC-05 looks for the cause. Until then, `mac-test` reports these 8
cases as failed.

### MAC-05/Q6 — `Scoped::map` lives in krucyfiks

*Context:* code review round 1 (line 220 of `split_catalog.rs`): "It seems
that Scoped needs `.map` method as well." MAC-05 rebuilt a `Scoped` by hand
in 4 places.

*Decision:* add `Scoped::map` to krucyfiks, with a unit test, as its own jj
change in the krucyfiks repo (`qxrqqrtk`, "Scoped::map: change a scoped value
and keep its scope"). The two event sites use it:
`scoped.map(PaneEvent::Detail)`. The two outcome sites in `pane_command` keep
`Scoped::new` after destructuring, because the variant chooses the outer type;
a `map` there would need a dummy `()` value. MAC-05/Q9 replaced that.

### MAC-05/Q7 — Toolbar buttons are icons

*Context:* code review round 1, on `02-validation.png`: "Those cancel and save
buttons. Can we use icons instead?" The detail toolbar mixed text (**Edit**)
and an icon (**Delete**).

*Decision:* Cancel `xmark`, Save `checkmark`, Edit `pencil` (the
recommendation). Each keeps its text label for VoiceOver and a tooltip
(`.help`); Delete gets a tooltip too. Esc and ⌘S do not change.

### MAC-05/Q8 — Cancel and Save share one toolbar capsule

*Context:* demo review: "In details screen there is this neat thing where
edit and delete buttons are grouped together. Shouldn't we use something like
that here as well?" Edit and Delete share a capsule because they are one
`ToolbarItemGroup`. In one group, a Save with `.borderedProminent` still gets
its own capsule on macOS 27; without it, ✕ and ✓ share one.

*Decision:* group them; Save is no longer blue (the recommendation). Both
panes' toolbars look the same. ⌘S still saves.

### MAC-05/Q9 — One `Pane` outcome variant

*Context:* code review round 2 (line 223): "I thought Scoped was getting
`.map`..." `pane_command` rebuilt `Scoped` because the inner value chose the
outer `Detail` or `Form` variant.

*Decision:* option B (the recommendation).
`IngredientSplitCatalogOutcome` is `List(..)` and
`Pane(Scoped<IngredientSplitPaneOutcome>)`, and the public
`IngredientSplitPaneOutcome` has `Detail` and `Form`. `pane_command` is
`.map(IngredientSplitCatalogOutcome::Pane)`; `update` accepts the scope once
and then matches the inner variant. No code in the split catalog rebuilds a
`Scoped`. Outcomes do not cross the FFI, so Swift does not change. This
reshapes the `Detail` outcome from MAC-04.

### MAC-05/Q10 — `return_to`, `clicked_row`, and `discard_destination()`

*Context:* code review round 3 (line 73): "I don't understand difference
between `previous` and `switch_to`." The user asked whether a stack would fit.
A stack does not tell Keep Editing whether its top item is the clicked row
(remove it) or the ingredient that was open at **＋** (keep it); it would need
a flag, and it never holds more than 2 items.

*Decision:* two fields named by when they are set: `return_to` (the
ingredient open at **＋**) and `clicked_row` (the row clicked while "Discard
changes?" is open). `CreatePane::discard_destination()` holds the precedence:
`clicked_row`, then `return_to`, then nothing.

### Decided in the question message (no objection)

- **＋** stays in the list column's toolbar. ⌘N also opens the form.
- The form's toolbar shows **Cancel** and **Save** in place of **Edit** and
  **Delete** (as icons since MAC-05/Q7). Esc is Cancel. ⌘S is Save. The pane title is "New Ingredient".
- **＋** is disabled while a form is open or a delete runs. Rust decides this
  (`can_create`); Swift only disables the button.

## Outcome

Garlic is open. You press **＋**. The detail pane shows the empty "New
Ingredient" form, and the cursor is in Name. A highlighted "New Ingredient"
row appears at the top of the list. You type "Paprika"; the top row now says
"Paprika". You press ⌘S. The top row goes away, Paprika is between Olive oil
and Tomato, highlighted, and the pane shows Paprika's detail. You quit and start the app again: Paprika is
still there.

Other paths:

- Save with an empty name: "Required." appears under Name.
- Save "garlic": "An ingredient with this name already exists." appears at
  the top. The form stays open.
- Cancel on an empty form: Garlic's detail comes back.
- Type "Pap", then click Tomato: "Discard changes?". **Discard Changes** opens
  Tomato. **Keep Editing** keeps "Pap".

## Terms

- **Form widget:** `IngredientForm` (`form.rs`). The Rust widget behind the
  iOS create and edit screens. It holds the draft, validates it on Save,
  stores it, and owns the "Discard changes?" step. It finishes with
  `Saved { ingredient }` or `Discarded`.
- **Dirty:** the draft differs from the draft that the form started with.
  `IngredientFormView.dirty` reports it.
- **Pane scope:** E-19 gives the detail pane a new scope each time its content
  changes. Rust ignores an event or a result that carries an old scope. The
  form gets a new pane scope too.
- **E-10:** the epic rule "a row click on a dirty form asks *Discard
  changes?*; a clean form switches at once".
- **Draft row:** the highlighted row at the top of the list while the create
  form is open (MAC-05/Q2). It is not an ingredient yet; Rust reports only
  the typed name.
- **`.confirmationDialog`:** the SwiftUI modifier for a question with
  buttons. On macOS it appears as a sheet that slides down from the window's
  title bar.

## Design

[MAC-05-design.html](MAC-05-design.html), light and dark. Each section shows
the chosen variant; the options that were not chosen are behind an
"Alternatives" toggle.

1. **＋**: the empty form, with a reference unit, and the draft row
   (MAC-05/Q2: C).
2. Typing "Paprika": the draft row follows the name.
3. Reference units: − button and drag handle (MAC-05/Q4).
4. Validation issues: "Required." and "Enter a number." under their fields.
5. Duplicate name: the save error at the top of the form.
6. Saved: Paprika's detail, its row highlighted.
7. Saved while the search "tom" hides the row (MAC-05/Q3).
8. A row click on a dirty form: the "Discard changes?" sheet (MAC-05/Q1).

The form fields and texts are the iOS ones (`IngredientFormScreen`,
`ErrorText.swift`). Only the layout is Mac: a grouped form, as in the Mac
detail.

## Changes by layer

**db: no change.** The store already saves new ingredients and reports
duplicate names.

### rust — `crates/kreatyna/src/ingredients/split_catalog.rs`

The coordinator learns create and E-10. The list and form widgets do not
change.

*sketch:*

```rust
pub enum IngredientSplitCatalogEvent {
    List(Scoped<IngredientListEvent>),
    Detail(Scoped<IngredientDetailEvent>),
    /// New. Save, Cancel (`RequestClose`), field edits, and the discard question.
    Form(Scoped<IngredientFormEvent>),
    ClearSelection { scope: CommandScopeId },
}

pub enum IngredientSplitCatalogOutcome {
    List(Scoped<IngredientListOutcome>),
    Detail(Scoped<IngredientDetailOutcome>),
    Form(Scoped<IngredientFormOutcome>), // new
}

pub enum IngredientSplitPaneView {
    Empty,
    Detail(IngredientDetailView),
    Create(IngredientFormView), // new
}

pub struct IngredientSplitCatalogView {
    pub list: Scoped<IngredientListView>,
    pub selected: Option<IngredientId>,
    pub pane: Scoped<IngredientSplitPaneView>,
    /// New. False while a form is open or a delete runs.
    pub can_create: bool,
    /// New. The draft row's text while the create form is open: the typed
    /// name, trimmed. Empty means no name yet (Swift shows "New Ingredient").
    pub draft_row: Option<String>,
}

enum Pane {
    Empty,
    Detail(IngredientDetail),
    Create(CreatePane), // new
}

struct CreatePane {
    form: IngredientForm,
    /// The ingredient that was open when ＋ was pressed. Cancel goes back to it.
    return_to: Option<Ingredient>,
    /// The row clicked while "Discard changes?" is open (E-10). Keep Editing clears it.
    clicked_row: Option<Ingredient>,
}

impl CreatePane {
    /// Where Discard goes: the clicked row wins, then `return_to`, then nothing.
    fn discard_destination(&mut self) -> Option<Ingredient>;
}
```

`Pane` stays a `Widget`. Its event and outcome become private enums with a
`Detail` and a `Form` variant. The catalog maps its public `Detail` and
`Form` variants onto them, so the FFI keeps one variant per child.

Rules (what the coordinator does):

| Pane | **＋** (`CreateRequested`) | Row click on R (`OpenDetail`) |
|---|---|---|
| Empty | Opens the form. `return_to = None`. | Opens R. |
| Detail G, no delete running | Opens the form. `return_to = G`. | Opens R. G again: nothing. |
| Detail, delete running or confirmed | Nothing. | Nothing. |
| Form, editing, clean | Nothing. | Opens R. |
| Form, editing, dirty | Nothing. | `clicked_row = R`. Sends `RequestClose` to the form, which asks "Discard changes?". |
| Form, asking, saving, or finished | Nothing. | Nothing. |

| Form outcome | What the coordinator does |
|---|---|
| `Saved { ingredient }` | `list.apply_saved(ingredient)`; reloads if the list was not complete. Shows the new ingredient's detail. `selected` is its ID, also when the search hides its row (MAC-05/Q3). |
| `Discarded` | Shows `discard_destination()`: `clicked_row`, else `return_to`, else the empty pane. |
| The question ends with Keep Editing | The form is editing again. The coordinator sets `clicked_row = None`. |
| Any other | Forwards it to the form. |

Each change of pane content uses `show`, which gives the pane a new scope
(E-19). So late form results, for example a save of a discarded form, are
ignored. `ClearSelection` does nothing while a form is open. While the form
is open, `selected` is `None` and `draft_row` is the form's trimmed name. The
draft row is not a list row, so the search never hides it, and Swift sends
no event for a click on it. The new pane's
form starts with `NutritionBasis::Per100Grams`, as on iOS.

*sketch of one test (`crates/kreatyna/tests/ingredient_split_catalog.rs`):*

```rust
#[tokio::test]
async fn a_row_click_on_a_dirty_form_asks_and_discard_opens_the_row() -> anyhow::Result<()> {
    let mut fixture = SplitFixture::ready(vec![ingredient(1, "Garlic"), ingredient(2, "Tomato")], "").await?;
    fixture.create().await?;
    fixture.type_name("Pap").await?;

    fixture.click_row(id(2)).await?;
    assert_matches!(fixture.form().status, IngredientFormStatus::ConfirmingDiscard(_));

    fixture.confirm_discard().await?;
    assert_eq!(fixture.catalog.view().selected, Some(id(2)));
    assert_matches!(fixture.catalog.view().pane.value, IngredientSplitPaneView::Detail(_));
    Ok(())
}
```

New Rust tests:

| Test | Proves |
|---|---|
| `create_opens_the_form_in_a_new_pane_scope` | **＋** shows an empty form with a new scope; `selected` is `None`; `draft_row` is `Some("")`; `can_create` is false. |
| `the_draft_row_shows_the_typed_name` | Typing "Paprika" sets `draft_row` to `Some("Paprika")`; Save and Cancel set it to `None`. |
| `the_draft_row_ignores_the_search` | With the search "tom", the list hides Garlic, and `draft_row` is still `Some(..)`. |
| `saving_adds_the_row_and_shows_its_detail` | Saved: the row is in the list, its detail is open, it is selected. |
| `a_saved_ingredient_hidden_by_the_search_still_opens` | MAC-05/Q3. |
| `a_duplicate_name_keeps_the_form_with_the_error` | The store's `DuplicateName` stays in the form view. |
| `cancel_on_a_clean_form_returns_to_the_previous_detail` | `return_to = Garlic`. |
| `cancel_on_a_clean_form_with_nothing_before_empties_the_pane` | `return_to = None`. |
| `a_row_click_on_a_clean_form_opens_the_row` | E-10, clean. |
| `a_row_click_on_a_dirty_form_asks_and_discard_opens_the_row` | E-10, Discard. |
| `keep_editing_forgets_the_clicked_row` | E-10, Keep Editing; a later Cancel returns to `return_to`, not the clicked row. |
| `create_is_ignored_while_a_form_is_open_or_a_delete_runs` | `test_case` for both. |
| `a_late_save_result_for_a_discarded_form_is_ignored` | Scope check. |
| `clearing_the_selection_is_ignored_while_a_form_is_open` | ⌘-click does not drop the form. |

The test `create_and_edit_from_the_list_are_not_routed_yet` loses its
"create" case; "edit from the list" stays until MAC-06.

### ffi — `crates/kreatyna-ffi/src/app_types/split_catalog.rs`

| Type | Change |
|---|---|
| `IngredientSplitCatalogEvent` | New `Form { scope: ScopeId, event: IngredientFormEvent }`. |
| `IngredientSplitPaneView` | New `Create { form: IngredientFormView }`. Its conversion becomes `TryFrom`, because the form view's conversion can fail. |
| `IngredientSplitCatalogView` | New `can_create: bool` and `draft_row: Option<String>`. |

Tests in `app_types/tests.rs`: the `Form` event converts with its scope; a
`Create` pane converts.

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `SyncedTextField.swift` | Moved from `Kreatyna/`. No change; it has no iOS-only API. |
| `IngredientText.swift` | Adds `IngredientDraftField.accessibilityIdentifier`, moved from `IngredientFormScreen.identifier(for:)`. iOS uses it; its identifiers stay the same. |
| `AppEventComposition.swift` | `IngredientSplitCatalogView.form(_:)` wraps a form event with the pane scope. |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacIngredientForm.swift` (new) | The grouped form: save error, Name, Notes, Basis, 4 nutrition fields, reference units. Each unit row: drag handle, name, weight, volume, − button. Title "New Ingredient". Toolbar: Cancel (`.cancelAction`, Esc), Save (⌘S), a spinner while saving. `.confirmationDialog("Discard changes?")` with **Discard Changes** and **Keep Editing**. Name gets the focus when the form opens. |
| `MacCatalogView.swift` | The `.create(form)` pane shows `MacIngredientForm`. |
| `MacIngredientList.swift` | **＋** sends `.createRequested`, has ⌘N, and is disabled when `can_create` is false. The draft row is the first row, in italics, with "New Ingredient" for an empty name. While it exists, the list's selection shows it; a click on it sends nothing. |

*sketch:*

```swift
case .create(let form):
    MacIngredientForm(view: form) { send(view.form($0)) }
```

iOS changes only where it uses the moved `SyncedTextField` and the moved
identifiers. Its snapshots must not change.

### tests

| Layer | File | Tests |
|---|---|---|
| Swift Testing | `SharedTests/AdapterPlumbingTests.swift` | `catalog.form(.save)` carries the pane scope. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New `newIngredientForm` (empty, one unit row added, draft row "New Ingredient"), `formWithIssues` (Name "Required.", Calories "Enter a number.", unit "clove" with no measurement), `duplicateName` (save error, draft row "garlic"). Light and dark: 6 new images. |
| UI smoke (Mac) | `KreatynaMacUITests.swift` | `testCreatedIngredientSurvivesRelaunch`: **＋**, the draft row shows "New Ingredient", type "Paprika", ⌘S, see the detail; relaunch with the same database ID; the row is there. `testDirtyFormAsksBeforeOpeningARow`: seed Garlic and Tomato, **＋**, type "Pap", click Tomato, **Keep Editing** (the form stays), click Tomato, **Discard Changes** (Tomato's detail). |
| Demo capture (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureMAC05Demo`: open Garlic, **＋**, capture; Save with an empty name, capture; type "garlic" and Save, capture; type "Paprika" and Save, capture; **＋**, type "Pap", click Tomato, capture the sheet. |

The 12 older list-column snapshots change: **＋** is now enabled. I will
record them again and compare old and new by eye. Only the **＋** button may
differ. `startupFailure` must not change.

### docs

| File | Change |
|---|---|
| `docs/epics/mac-shell/tasks/MAC-05-design.html` (new) | The mockup. |
| `docs/epics/mac-shell/tasks/MAC-05-demo/` (new) | 6 screenshots. The demo text is the Inbox message (process change `yqlqysnq`). |
| `docs/epics/mac-shell/epic.html` | MAC-05 status and plan link. MAC-06 card: E-10 for the create form is done in MAC-05; MAC-06 applies it to the edit form. |

## Steps

1. Add `Form` to the split catalog's event, outcome, and pane; add `CreatePane`.
2. Route `CreateRequested` and the row-click rules from the tables.
3. Handle `Saved`, `Discarded`, and Keep Editing.
4. Add `can_create` and `draft_row` to the view.
5. Write the 14 Rust tests; change the "not routed yet" test.
6. Add the FFI types and their 2 tests.
7. Run `devenv shell -- mac-bindings debug` and `devenv shell -- ios-bindings debug`.
8. Move `SyncedTextField` and the field identifiers to `Shared/`; run `apple-generate`.
9. Add `form(_:)` to `AppEventComposition.swift` and its Swift test.
10. Write `MacIngredientForm`; connect it in `MacCatalogView`, and **＋** and the draft row in `MacIngredientList`.
11. Try drag to reorder; if it does not work, add Move Up and Move Down to the context menu.
12. Add the 3 snapshot tests; record 6 images; look at each one.
13. Record the 12 changed list-column images again; compare old and new.
14. Add the 2 UI tests and the demo capture method.
15. Run checks 1–5.
16. Capture the demo screenshots into `MAC-05-demo/`.
17. Set MAC-05 to `awaiting-review`.
18. Send the demo as an Inbox message, then a guided review of the Rust side, to the MAC-05 thread.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | Passes, with the 14 new split catalog tests and 2 FFI tests. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `devenv shell -- mac-test` | All pass. 6 new snapshots; 12 re-recorded and checked by eye; `startupFailure` unchanged. |
| 4 | `devenv shell -- ios-test BB6851F0-6FB2-48A6-B250-F9179542F68E` | All pass. No iOS snapshot file changes. |
| 5 | `capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests/testCaptureMAC05Demo docs/epics/mac-shell/tasks/MAC-05-demo` | 5 PNG files. |
| 6 | `jj diff --summary` | Rust, FFI, Swift, tests, snapshots, docs. `project.yml` unchanged (new files sit in folders that it already lists). |

## Stubs and exclusions

- **Edit** and **Delete** in the detail stay disabled (MAC-06, MAC-07).
- Edit from the list stays ignored (MAC-06).
- The creation basis is always per 100 g, as on iOS. The stack catalog's
  `creation_basis` is not shared with the split catalog.
- No "Discard changes?" when you quit the app or close the window with a
  dirty form. Neither platform has it today.
- No snapshot of the "Discard changes?" sheet: an off-screen window does not
  show sheets. The UI test and the demo cover it. The MAC-06 card asks for
  this snapshot; MAC-06 will say how it captures it.

## Validation

Run on 2026-10-13, macOS 27.0.1, Xcode 27.0, iPhone 17 Pro simulator.

### Deviations from the plan

1. **No drag to reorder (MAC-05/Q4 fallback).** A unit row with `.onMove`
   inside the grouped `Form` did not move in a UI test: a drag from the
   second unit's handle onto the first, and a slow coordinate drag with a
   1-second press, both left the order unchanged. As the decision says for
   this case, each unit has **Move Up**, **Move Down**, and **Remove** in its
   context menu (right-click). The drag handle is gone, so it does not
   suggest a drag that does nothing. The UI test is
   `testContextMenuReordersUnits`, not `testDraggingAUnitReordersIt`.
2. **The save error is above the form, not in it.** The demo found a bug:
   after "An ingredient with this name already exists.", the first key
   removed the error's form section, the rows below it moved, and Name lost
   the cursor. Rust saved "P" instead of "Paprika". The error is now under
   the title, outside the form. `testCreatedIngredientSurvivesRelaunch`
   types after the error to keep this fixed.
3. **`SyncedTextField` has a `prompt`.** In a Mac grouped form, the field's
   title is its label, so the placeholder ("Required", "Unknown") needs its
   own text. The parameter is optional; iOS does not use it and its
   snapshots did not change.
4. **The form's accessibility identifier is on the form only.** On the whole
   pane it replaced the identifiers of the title and the save error (the
   same effect that MAC-08 noted for the detail title).
5. **Test names.** `a_late_save_result_for_a_discarded_form_is_ignored`
   became `a_late_form_result_from_an_old_pane_scope_is_ignored`: a save
   cannot start while the form is being discarded, so the test replays a
   `Saved` result after the pane moved on.

### Checks

| # | Check | Result |
|---|---|---|
| 1 | `cargo test --workspace` | Passed, 447 tests (431 before): 14 new split catalog tests, 2 new FFI tests. A mutation check (remove "Keep Editing forgets the row"; open `return_to` instead of `clicked_row`) made one test fail each time. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `devenv shell -- mac-test` | 51 passed, 4 demo captures skipped, **8 failed**: `catalogWithRows`, `emptyCatalog`, `filteredCatalog`, `noMatches` (light and dark). See "Snapshot flake" below. 6 new snapshots match. The 12 re-recorded list-column images differ from the approved ones only in the **＋** glyph (a 28 × 28 px box), compared pixel by pixel. `startupFailure` did not change. |
| 4 | `devenv shell -- ios-test BB6851F0-6FB2-48A6-B250-F9179542F68E` | `TEST SUCCEEDED`. No iOS snapshot file changed. |
| 5 | `capture-demo.sh mac …/testCaptureMAC05Demo docs/epics/mac-shell/tasks/MAC-05-demo` | 6 PNG files (one more than planned: the search case of MAC-05/Q3). |
| 6 | `jj diff --summary` | Rust, FFI, Swift, tests, snapshots, docs. `project.yml` unchanged. |

UI tests (all passed): `testCreatedIngredientSurvivesRelaunch`,
`testDirtyFormAsksBeforeOpeningARow` (Esc on a clean form, Keep Editing,
Discard Changes), `testContextMenuReordersUnits`, and the 4 older tests.

### Snapshot flake (not caused by MAC-05)

The 4 list snapshots with an empty detail pane now fail on this Mac: the
"Ingredients" title is a darker grey than in the baselines (only the pixels
of the title text differ). The approved parent change (`xzrzzrtr`, before
MAC-05) fails the same 4 tests in the same way; I ran it in a separate jj
workspace. I recorded these images at 22:00 with the dim title, as in the approved baselines; every run from 22:14 on gives the darker title. During the
search for the cause, these did not change the result: the app's active
state (always inactive), the first responder (always the window), the other
windows of the test host (hidden or not), and the **＋** button (enabled or
disabled). The cause is still open; MAC-05/Q5 moves it to a separate change.

### Review round 1 (2026-10-13)

| Finding | Verdict | Change |
|---|---|---|
| `Scoped` needs `.map` | Confirmed, introduced by MAC-05 | MAC-05/Q6 |
| Icons for Cancel and Save | Partly: a design choice | MAC-05/Q7 |

Checks after round 1:

| Check | Result |
|---|---|
| krucyfiks `cargo test --workspace`, clippy | 187 passed (1 new: `mapping_a_scoped_value_keeps_its_scope`); no warnings. |
| `cargo test --workspace`, clippy (vibetyna) | 447 passed; no warnings. |
| `devenv shell -- mac-test` | `TEST SUCCEEDED`: 60 passed, 0 failed, 4 demo captures skipped. The 4 snapshots of the flake passed in this run, so the flake comes and goes (MAC-05/Q5). |
| Snapshots | 10 re-recorded (4 detail, 6 form). Pixel comparison: every difference is inside the toolbar buttons (y 15–88 of 1200). |
| Demo | Captured again; the screenshots show the icons. |

After MAC-05/Q8 (one capsule for ✕ ✓): the 6 form snapshots were recorded
again; every difference is inside the toolbar buttons. `mac-test`: 51
passed, 4 skipped, and the 8 cases of the flake failed again (MAC-05/Q5); all
UI tests passed. The demo was captured again (one try failed because the
user clicked the test window while it ran).

iOS was not run again: round 1 changed only Mac Swift files and Rust code
that iOS does not use.

### Review round 2 (2026-10-14)

| Finding | Verdict | Change |
|---|---|---|
| Line 223: "I thought Scoped was getting `.map`" | Intended (flagged in round 1) | MAC-05/Q9, option B |

Process: the review now goes as a guided review of the Rust side to the
Inbox, and the demo is the Inbox message (process change `yqlqysnq`). The
`demo.md` file is deleted.

Checks after round 2: `cargo test --workspace` 447 passed; clippy no
warnings. Swift was not run again: the change is Rust-internal (outcomes do
not cross the FFI), and the bindings did not change.

### Review round 3 (2026-10-14)

| Finding | Verdict | Change |
|---|---|---|
| Line 73: the difference between `previous` and `switch_to` is unclear | Confirmed: the names did not say when each is set | MAC-05/Q10 |

Checks after round 3: `cargo test --workspace` 447 passed; clippy no
warnings. Swift did not change.

### Not run

- Release build. No build settings changed.
