# MAC-06 — Edit an ingredient from its detail

Tracked in beads: `vibetyna-4rn.11`. Change: "MAC-06: edit an ingredient from
its detail". Dependencies: MAC-05 (`vibetyna-4rn.9`, closed).

## Open questions

None. The user answered MAC-06/Q1 in the Inbox thread "MAC-06" before this
plan.

## Decisions

### MAC-06/Q1 — The edited row keeps its saved name until Save

*Context:* while the create form is open, a draft row shows the name as you
type (MAC-05/Q2). For edit, the row already exists.

*Decision:* the edited row stays highlighted and shows the saved name until
Save (the recommendation). This is how Contacts and Notes work. No draft row
appears while the edit form is open.

### Decided in the question message (no objection)

| Rule | Follows from |
| --- | --- |
| **＋** is disabled while the edit form is open. | MAC-05: `can_create` is false while a form is open. |
| Clicking the row that you edit does nothing. | Clicking the selected row keeps the detail (today's rule). |
| A search that hides the edited row keeps the form open. | MAC-08/Q1. |
| After Save, the row moves to its sorted place and stays highlighted. The search stays. | MAC-05/Q3. |
| Esc is Cancel, and ⌘S is Save. Edit gets no new shortcut. | MAC-05; the epic excludes extra shortcuts. |
| The pane title is the saved name, for example "Garlic", until Save. | MAC-06/Q1. |

## Outcome

Garlic is open. You click **Edit** (✎). The detail pane shows the form with
Garlic's values, and the title "Garlic". The Garlic row stays highlighted.

- You change the name to "Black Garlic" and press ⌘S. The pane shows the
  "Black Garlic" detail. The row now reads "Black Garlic", at its sorted
  place, highlighted.
- You change the name and press Esc. The app asks "Discard changes?".
  **Discard Changes** shows the Garlic detail as it was saved. **Keep
  Editing** keeps the form.
- You change the name and click Tomato. The same question appears. **Discard
  Changes** opens Tomato. **Keep Editing** keeps the form, and a later Cancel
  goes back to Garlic.
- With no changes, Esc or a click on Tomato leaves the form at once.

## Terms

| Term | Meaning |
| --- | --- |
| Pane | The right column of the Mac window. It shows nothing, a detail, or a form. |
| Pane scope | An ID that Rust gives the pane's content. Each new content gets a new scope, so late events for old content are ignored (E-19). |
| Dirty form | A form whose values differ from the values it opened with. The form widget reports this as `dirty`. |
| E-10 | The epic rule: clicking another row while a form is dirty asks "Discard changes?", the same as Cancel. A clean form switches at once. |
| `IngredientSplitCatalog` | The Rust widget for the Mac window. It owns the list and the pane, and decides what the pane shows. |

## Design

No new mockup. The edit form is the create form from MAC-05
([MAC-05-design.html](MAC-05-design.html), and the demo
[MAC-05-demo](MAC-05-demo/)), with two differences:

1. The title is the saved name ("Garlic"), not "New Ingredient".
2. The list has no draft row. The edited row stays highlighted.

## Changes by layer

### db

None. `IngredientStore::save` already takes an `IngredientWrite` for an
existing ID. The iOS app uses it to save edits.

### rust (`crates/kreatyna/src/ingredients/split_catalog.rs`)

Today the split catalog ignores both edit outcomes (*real*):

```rust
// Edit arrives with MAC-06.
IngredientDetailOutcome::OpenEdit { .. } => Command::empty(),
```

1. Rename `CreatePane` to `FormPane`. Its fields stay: `form`, `return_to`,
   `clicked_row`, and `discard_destination()`. For edit, `return_to` is the
   saved ingredient, so Cancel returns to it.
2. Add a pane variant for the edit form. It keeps the saved ingredient for
   the highlight and the title.

   ```rust
   // sketch
   enum Pane {
       Empty,
       Detail(IngredientDetail),
       Create(FormPane),
       /// `saved` is the ingredient as stored. The row and the title show it
       /// until Save (MAC-06/Q1).
       Edit { saved: Ingredient, pane: FormPane },
   }

   pub enum IngredientSplitPaneView {
       Empty,
       Detail(IngredientDetailView),
       Create(IngredientFormView),
       Edit { saved_name: String, form: IngredientFormView },
   }
   ```

3. `Pane`'s `init`, `can_begin_shutdown`, `begin_shutdown`, `update`, and
   `view` get an `Edit` arm, the same as `Create`.
4. `handle_detail_outcome`: `OpenEdit { ingredient }` opens the edit form,
   if the detail still shows that ingredient and no delete runs. The form
   starts from `IngredientDraft::from(&ingredient)`, as on iOS.
5. The catalog view while the edit form is open:

   | Field | Value |
   | --- | --- |
   | `selected` | `Some(saved.id)` |
   | `draft_row` | `None` |
   | `can_create` | `false` |

6. `select` (a row click) treats `Edit` as `Create`, with one more rule: a
   click on the edited row does nothing. A dirty form asks first (E-10).
7. `handle_form_outcome` accepts outcomes from `Create` and from `Edit`.
   `Saved` calls `apply_saved` (it replaces the row with the same ID) and
   shows the new detail. `Discarded` goes to `discard_destination()`.
8. `IngredientListOutcome::OpenEdit` stays ignored: no Mac list action sends
   it. The comment changes from "arrives with MAC-06" to say so.

Widget tests in `crates/kreatyna/tests/ingredient_split_catalog.rs`. The test
`edit_from_the_detail_is_not_routed_yet` is replaced. New tests (*sketch*
names):

| Test | Checks |
| --- | --- |
| `edit_opens_the_form_in_a_new_pane_scope` | New scope; `Edit` view with `saved_name` "Garlic"; `selected` is Garlic; no draft row; `can_create` false. |
| `the_edited_row_keeps_its_saved_name_until_save` | Type "Black Garlic"; the row and `saved_name` still read "Garlic". |
| `saving_an_edit_updates_the_row_and_shows_its_detail` | Save "Black Garlic"; the row is renamed and sorted; the detail shows it; `selected` stays. |
| `cancel_on_a_clean_edit_returns_to_the_detail` | Cancel with no change shows the Garlic detail. |
| `discard_on_a_dirty_edit_returns_to_the_saved_detail` | Type, Cancel, Discard Changes: the Garlic detail with its saved values. |
| `a_row_click_on_an_edit_form` (parameterized: clean, dirty) | Clean: Tomato opens at once. Dirty: the question appears; Discard Changes opens Tomato. |
| `keep_editing_on_an_edit_forgets_the_clicked_row` | Click Tomato, Keep Editing, Cancel, Discard Changes: Garlic, not Tomato. |
| `clicking_the_edited_row_does_nothing` | The view does not change. |
| `edit_is_ignored_while_a_delete_is_being_confirmed` | The detail keeps its delete question; no form opens. |

### ffi (`crates/kreatyna-ffi/src/app_types/split_catalog.rs`)

1. Add the variant `Edit { saved_name: String, form: IngredientFormView }`
   to `IngredientSplitPaneView`, and its arm in `TryFrom`.
2. Add a conversion test in `app_types/tests.rs`.

### swift

1. `KreatynaMac/MacIngredientDetail.swift`: the view takes
   `send: (IngredientDetailEvent) -> Void`. **Edit** sends `.edit` and is
   enabled. **Delete** stays disabled (MAC-07).
2. `KreatynaMac/MacIngredientForm.swift`: the view takes `title: String`.
   The create form passes "New Ingredient".
3. `KreatynaMac/MacCatalogView.swift`: a new case.

   ```swift
   // sketch
   case .edit(let savedName, let form):
       MacIngredientForm(view: form, title: savedName) { send(view.form($0)) }
           .id(view.pane.scope)
   ```

4. The list needs no change: it highlights `selected`, and the edited row
   is `selected`.

Swift tests:

1. Snapshot `editIngredientForm` (light and dark): Garlic's values, title
   "Garlic", the Garlic row highlighted.
2. Snapshot `discardConfirmation` (light and dark): the "Discard changes?"
   dialog over a dirty edit form. macOS shows this dialog as a sheet, in a
   separate window. If the snapshot cannot capture the sheet, the demo UI
   test captures it instead, and Validation says so.
3. UI test `testEditedNameAppearsInTheList`: select Garlic, click Edit,
   change the name to "Black Garlic", press ⌘S. Expect the row "Black Garlic"
   and the detail title "Black Garlic".
4. Demo capture (`MacDemoCaptureUITests`): the edit form, the discard
   question, and the saved result, into `docs/epics/mac-shell/tasks/MAC-06-demo/`.

### docs

1. This plan's Validation section.
2. No epic decision changes: MAC-06/Q1 applies to this slice only.

## Steps

1. Rename `CreatePane` to `FormPane` in `split_catalog.rs`.
2. Add `Pane::Edit` and `IngredientSplitPaneView::Edit`.
3. Add the `Edit` arms to the `Pane` widget methods.
4. Route `IngredientDetailOutcome::OpenEdit` to a new `open_edit` method.
5. Update `selected`, `draft_row`, `can_create`, and `select` for `Edit`.
6. Let `handle_form_outcome` accept both form panes.
7. Write the widget tests, and delete `edit_from_the_detail_is_not_routed_yet`.
8. Add the FFI variant and its conversion test.
9. Run `devenv shell -- mac-bindings debug`.
10. Change `MacIngredientDetail`, `MacIngredientForm`, and `MacCatalogView`.
11. Add the snapshots, and review each new image before you accept it.
12. Add the UI test and the demo capture.
13. Run every check, and fill in Validation.

## Checks

| Command | Expected result |
| --- | --- |
| `cargo fmt --all -- --check` | Clean. |
| `cargo test --workspace` | All pass, including the new widget tests and the FFI conversion test. |
| `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| `devenv shell -- mac-bindings debug` | `IngredientSplitPaneView.edit` is in the Swift bindings. |
| `devenv shell -- mac-test` | All Mac tests pass, including the new snapshots and the UI test. |
| `devenv shell -- ios-bindings debug`, then an iOS build | iOS still builds. iOS does not use the split catalog. iOS tests are not run (iOS is parked). |

## Stubs and exclusions

- **Delete** in the detail stays disabled: MAC-07 (`vibetyna-4rn.12`).
- `IngredientListOutcome::OpenEdit` stays ignored: no Mac list action sends
  it.
- Quit with a dirty form does not ask: `vibetyna-3rv`.
- Double-click a row to edit, and a keyboard shortcut for Edit: not planned.

## Validation

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Clean. |
| `cargo test --workspace` | 463 passed, 0 failed. New: 12 widget test cases and 1 FFI conversion test. |
| `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| `devenv shell -- mac-bindings debug` | `IngredientSplitPaneView.edit(savedName:form:)` is in the Swift bindings. |
| `devenv shell -- mac-test` | Succeeded: 64 passed. The 5 demo-capture UI tests skip, as before. |
| `devenv shell -- ios-bindings debug`, then `xcodebuild build-for-testing` for `generic/platform=iOS Simulator` | Succeeded. iOS tests were not run (iOS is parked). |
| Demo capture | 4 screenshots in [MAC-06-demo](MAC-06-demo/), macOS 27.0.1, dark appearance. |

Differences from the plan:

1. **No `discardConfirmation` snapshot.** The snapshot showed only the dimmed
   form: macOS draws "Discard changes?" in its own sheet window, which the
   off-screen capture does not include. The demo screenshot
   [03-discard-question.png](MAC-06-demo/03-discard-question.png) shows the
   dialog, as the plan allowed. The snapshot `editIngredientForm` covers the
   form under it.
2. **Two detail snapshots changed** (`detailWithFullData` and
   `detailWithNameOnly`, light and dark). The **Edit** pencil is now enabled,
   not grey. That is the only difference: about 25 pixels per image. I checked
   each image before I accepted it.
3. **The UI test finds the detail title by its text.** The pane's
   `ingredientDetail` identifier replaces the title's `ingredientName`
   identifier, so the test looks for a text "Black Garlic" that is not a row.
4. Two more widget tests than the plan lists:
   `clicking_the_edited_row_does_nothing` runs for a clean and a dirty form,
   and `a_search_that_hides_the_edited_row_keeps_the_form` checks the
   MAC-08/Q1 rule.
