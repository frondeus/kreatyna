# MAC-07 — Delete an ingredient with confirmation

Tracked in beads: `vibetyna-4rn.12`. Change: `sytozknl`. Dependencies: MAC-04
(approved), Cooldown 1 (`vibetyna-979`, approved).

## Open questions

None. The user answered MAC-07/Q1 in the Inbox on 2026-10-31 (see
Decisions).

## Decisions

### MAC-07/Q1 — Swift writes the dialog text

*Context:* The epic card says "the confirmation dialog shows whatever the
Rust confirmation widget says". Rust's `ConfirmationView` has no text: it is
`AwaitingDecision` or `Resolved { decision }`. The detail view adds the
delete error (`IngredientDeleteError`). iOS and the Mac "Discard changes?"
dialog already write their words in Swift.

*Decision:* A, the recommendation. Swift writes "Delete Garlic?", "This
cannot be undone." and the button labels. Rust decides when the dialog
shows, what each button does, and which error shows. No Rust change.

### Decided in the plan (sent with the question, no objection)

- A failed delete reopens the dialog (Rust does this). The dialog message
  shows the error, for example "The ingredient could not be deleted from
  storage.", as on iOS.
- While Rust deletes, the pane shows a "Deleting…" overlay, as on iOS.
- No ⌘⌫ shortcut. In the search field, ⌘⌫ deletes to the start of the line;
  a toolbar shortcut would take it over.

## Outcome

The catalog has Garlic and Tomato. You click Garlic. You click **Delete**
(the trash icon) in the detail toolbar. A dialog asks "Delete Garlic?" with
"This cannot be undone.", **Delete** and **Cancel**. **Cancel** closes it,
and Garlic stays open. **Delete** removes the Garlic row, and the pane shows
"Select an ingredient". You quit and open the app again: Garlic is still
gone.

## Terms

- **Split catalog:** `IngredientSplitCatalog`, the Mac coordinator in Rust
  (E-19). It owns the list and one pane: empty, detail, create form, or edit
  form.
- **Detail deletion:** `IngredientDetailDeletion`, the part of the detail
  view that says where a delete is: `idle`, `confirming(confirmation:error:)`,
  `deleting`, or `deleted`.
- **`confirmationDialog`:** the SwiftUI modifier for a question with
  buttons. macOS draws it as a sheet, in a separate window, on top of the
  main window.
- **Snapshot test:** a test that renders a view off screen and compares the
  image with a recorded baseline (`swift-snapshot-testing`).

## Design

No new mockup. The dialog is the system sheet, as in
[03-discard-question.png](MAC-06-demo/03-discard-question.png) from MAC-06.
The empty pane after a delete is the "Select an ingredient" placeholder from
MAC-04. The demo shows both.

## Changes by layer

**db, rust, ffi: no change.** MAC-04 built and tested the delete in the
split catalog. The relevant Rust tests are in
`crates/kreatyna/tests/ingredient_split_catalog.rs`:

| Test | What it proves |
|---|---|
| `deleting_the_selected_ingredient_removes_its_row_and_empties_the_pane` | Confirm removes the row and shows the empty pane. |
| `the_selection_is_kept_while_a_delete_is_being_confirmed` | The row stays highlighted while the dialog is open. |
| `a_failed_delete_keeps_the_selection` | A failure keeps the detail and the row. |
| `create_is_ignored_while_a_form_is_open_or_a_delete_runs` | **＋** is locked during a delete. |
| `edit_is_ignored_while_a_delete_is_being_confirmed` | **Edit** is locked during a delete. |

*real* — the detail view that Swift renders (`crates/kreatyna/src/ingredients/detail.rs`):

```rust
pub enum IngredientDetailDeletion {
    Idle,
    Confirming {
        confirmation: ConfirmationView,
        error: Option<IngredientDeleteError>,
    },
    Deleting,
    Deleted,
}
```

The examples below are Swift, because only Swift changes.

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `IngredientText.swift` | New `DeleteText` with the dialog words, used by iOS and the Mac. |

*sketch:*

```swift
/// The delete dialog's words. Rust decides when it shows (MAC-07/Q1).
enum DeleteText {
    static func title(_ name: String) -> String { "Delete \(name)?" }
    static func message(_ error: IngredientDeleteError?) -> String {
        error?.message ?? "This cannot be undone."
    }
}
```

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacIngredientDetail.swift` | **Delete** sends `.requestDelete`. **Edit** and **Delete** are disabled when `view.deletion` is not `.idle`. A `confirmationDialog` shows while Rust awaits a decision: **Delete** (`confirmDelete`) sends `.confirmation(event: .confirm)`, **Cancel** (`cancelDelete`) sends `.confirmation(event: .cancel)`. A "Deleting…" overlay (`ingredientDeleting`) shows during `.deleting`. |

*sketch:*

```swift
.confirmationDialog(
    DeleteText.title(ingredient.name),
    isPresented: Binding(get: { awaitingDeleteDecision }, set: { _ in }),
    titleVisibility: .visible
) {
    Button("Delete", role: .destructive) { send(.confirmation(event: .confirm)) }
        .accessibilityIdentifier("confirmDelete")
    Button("Cancel", role: .cancel) { send(.confirmation(event: .cancel)) }
        .accessibilityIdentifier("cancelDelete")
} message: {
    Text(DeleteText.message(deleteError))
}
```

The binding's setter does nothing. Rust's view opens and closes the dialog.
Esc on the sheet runs the **Cancel** button, so it sends `.cancel`.

### swift — iOS (`apple/Kreatyna/Kreatyna/`)

| File | Change |
|---|---|
| `IngredientDetailScreen.swift` | Uses `DeleteText` in place of its own two strings. Nothing visible changes. |

### tests

| Layer | File | Tests |
|---|---|---|
| Fixtures | `SharedTests/Fixtures.swift` | `splitCatalog(list:selected:form:)` gets a `deletion` parameter (default `.idle`). |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New `deletingIngredient` (Garlic with the "Deleting…" overlay), light and dark: 2 new images. `detailWithFullData` and `detailWithNameOnly` change, because the trash icon is no longer grey: 4 images to record again and check by eye. |
| UI smoke (Mac) | `KreatynaMacUITests.swift` | New `testDeletedIngredientStaysGoneAfterRelaunch`: seed Garlic and Tomato. Open Garlic, click **Delete**, click **Cancel**: Garlic's detail stays. Click **Delete**, click **Delete** in the dialog: the Garlic row is gone and the placeholder shows. Relaunch on the same database: Tomato is there, Garlic is not. |
| Demo capture (Mac) | `MacDemoCaptureUITests.swift` | New `testCaptureMAC07Demo`: demo fixture, open Garlic, click **Delete**, capture `01-delete-question`; confirm, capture `02-after-delete`. |

**No snapshot of the dialog.** MAC-06 showed that the off-screen snapshot
does not include the sheet window. The demo screenshot `01-delete-question`
shows the dialog instead, as for "Discard changes?". The UI cannot cause a
failed delete; the Rust tests cover that path, and `DeleteText.message`
picks the error text.

### docs

| File | Change |
|---|---|
| `docs/epics/mac-shell/epic.html` | MAC-07 card: `plan: "tasks/MAC-07.md"`, layers `swift` only, acceptance reworded for MAC-07/Q1 and the dialog snapshot. |
| `docs/epics/mac-shell/tasks/MAC-07-demo/` (new) | The 2 demo screenshots. |

## Steps

1. Add `DeleteText` to `Shared/IngredientText.swift`.
2. Use `DeleteText` in `IngredientDetailScreen.swift`.
3. Enable **Delete** in `MacIngredientDetail.swift` and add the dialog and the overlay.
4. Add the `deletion` parameter to `Fixtures.splitCatalog`.
5. Add the `deletingIngredient` snapshot test.
6. Delete the 4 `detailWith…` baselines and record them again.
7. Look at each new and re-recorded image.
8. Add the UI smoke test and the demo capture method.
9. Run the checks.
10. Capture the demo into `MAC-07-demo/`.
11. Update the MAC-07 card in `epic.html`.
12. Send the report to the MAC-07 Inbox thread.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `devenv shell -- mac-test` | All pass. 2 new and 4 re-recorded images, each checked by eye. Other Mac images unchanged. |
| 2 | iOS `xcodebuild build-for-testing` (generic iOS Simulator) | Builds. iOS is parked (mac-first), so only the build runs. |
| 3 | `jj diff -r @ --name-only 'glob:**/*.rs'` | Empty: no Rust file changed. |
| 4 | `capture-demo.sh mac …/testCaptureMAC07Demo docs/epics/mac-shell/tasks/MAC-07-demo` | `01-delete-question.png`, `02-after-delete.png`. |

## Stubs and exclusions

- No ⌘⌫ shortcut and no **Delete** item in the Edit menu.
- No delete from the list (context menu or swipe). Delete is in the detail
  toolbar only.
- No undo. The dialog says "This cannot be undone."
- Recipes that use a deleted ingredient: REC-13.

## Validation

Run on 2026-10-31, macOS 27, Xcode 27.0.

### Deviations from the plan

1. **The dialog is an alert, not a sheet.** macOS 27 draws the
   `confirmationDialog` as a centered alert over the window
   ([01-delete-question.png](MAC-07-demo/01-delete-question.png)). It is
   still a separate window, so the off-screen snapshot cannot capture it.
2. **The toolbar trash icon is not red.** `role: .destructive` colors the
   dialog's **Delete** button, not the toolbar button. macOS draws toolbar
   icons in one color.

### Checks

| # | Check | Result |
|---|---|---|
| 1 | `mac-test` | First run recorded 6 images and failed, as expected. Second run: `TEST SUCCEEDED`, 72 passed, 0 failed, 6 capture tests skipped. New: `deletingIngredient` light and dark. Re-recorded: `detailWithFullData` and `detailWithNameOnly`, light and dark. Each was checked by eye: the only difference is the trash icon, now enabled. |
| 2 | iOS `xcodebuild build-for-testing` | `TEST BUILD SUCCEEDED`. |
| 3 | `jj diff -r @ --name-only 'glob:**/*.rs'` | Empty. |
| 4 | `capture-demo.sh mac …/testCaptureMAC07Demo …/MAC-07-demo` | `01-delete-question.png`, `02-after-delete.png`. |

UI test `testDeletedIngredientStaysGoneAfterRelaunch` passed: Cancel keeps
Garlic open; Delete removes the row and shows "Select an ingredient"; after a
relaunch on the same database, Tomato is there and Garlic is not.

### Not run

- iOS tests (iOS is parked; only the build ran).
- A failed delete in the UI. The UI cannot cause one; the Rust tests cover it.
