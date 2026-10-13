# MAC-08 — Search the catalog from the toolbar

Status: approved. Change: `mrszzoyr`. Dependencies: MAC-01 (approved), MAC-04
(approved).

## Open questions

None. The user answered MAC-08/Q1 in the Inbox on 2026-10-13 (see
Decisions).

## Decisions

### MAC-08/Q1 — A search keeps the open detail

*Context:* Garlic is open, and you type "tom". Garlic's row leaves the list.
The Mac coordinator (`IngredientSplitCatalog`, E-19) keeps `selected` and the
detail. The Rust test `the_list_stays_live_while_a_detail_is_open::search`
covers it.

*Decision:* Keep the detail (the recommendation). No Rust change. When you
clear the search, Garlic's row is highlighted again.

### MAC-08/Q2 — The search field sits above the list

*Context:* The first build used `.searchable`, which macOS 27 puts at the
right end of the window toolbar, above the detail. The recipes mockup
(`docs/tasks/recipes/mockups.html`, section 1) puts "Search recipes" at the
top of the list column. The user, reviewing the demo: "We should be
consistent with search bar position. move it above the list".

*Decision:* The Mac search field is at the top of the list column, as in the
mockup. SwiftUI has no `.searchable` placement for the middle column, so the
Mac wraps AppKit's `NSSearchField` (`MacSearchField`). iOS keeps
`.searchable`.

### Decided in the plan (sent with the question, no objection)

- The prompt is "Search ingredients". (The toolbar placement in the first
  plan was replaced by MAC-08/Q2.)
- ⌘F focuses the field if macOS gives that to `.searchable`, and Esc clears
  it. The UI test checks both.
- The Mac reuses the `TextEcho` logic from iOS. Rust's `search` stays the
  source of truth.

## Outcome

The catalog has Tomato, Canned tomatoes, and Garlic. You click the search
field in the toolbar (or press ⌘F) and type "tom". The list shows Canned
tomatoes and Tomato. You type "x". The list says "No matches for “tomx”".
You press Esc. The field clears, and all three rows come back. If Garlic was
open, its detail stays in the pane the whole time.

## Terms

- **`.searchable`:** the SwiftUI modifier that adds a system search field.
  On the Mac it goes in the window toolbar. On iOS it goes in the navigation
  bar.
- **`TextEcho`:** a small Swift type (`Shared/TextEcho.swift`). It remembers
  the texts that Swift sent to Rust and has not seen back yet. So a render
  that is one keystroke behind does not erase what you typed.
- **Search event:** `IngredientListEvent::Search(query)`. The Rust list widget
  stores the query and filters its rows by the ingredient name (`name_key`:
  case folding, trimmed).

## Design

No new mockup. The list states exist since MAC-01: rows, and "No matches for
“tom”" (`ingredientListNoMatches`). The new part is the system search field
in the toolbar, as in Mail or Notes. The demo shows the result.

## Changes by layer

**db, rust, ffi: no change.** The list widget filters on `Search`, and the
split catalog accepts list events while a detail is open. The examples are
Swift.

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `RustSearchable.swift` (new) | `rustSearchText(_:text:send:)`: keeps any search field's text in step with Rust through a `TextEcho`, and sends `.search(query:)`. Moved from `IngredientListScreen`; the logic does not change. `rustSearchable(_:send:)` is `.searchable` plus `rustSearchText`, for iOS. |

*sketch:*

```swift
extension View {
    /// A system search field whose text Rust owns (`IngredientListView.search`).
    func rustSearchable(_ search: String, send: @escaping (IngredientListEvent) -> Void) -> some View {
        modifier(RustSearchable(search: search, send: send))
    }
}

private struct RustSearchable: ViewModifier {
    let search: String
    let send: (IngredientListEvent) -> Void
    @State private var text: String
    @State private var echo = TextEcho()

    func body(content: Content) -> some View {
        content
            .searchable(text: $text, prompt: "Search ingredients")
            .onChange(of: text) { _, text in
                if let query = echo.typed(text, displayed: search) { send(.search(query: query)) }
            }
            .onChange(of: search) { _, search in
                if let text = echo.received(search, showing: text) { self.text = text }
            }
    }
}
```

### swift — iOS and Mac

| File | Change |
|---|---|
| `Kreatyna/IngredientListScreen.swift` | Uses `.rustSearchable(view.search, send: send)` in place of its own `searchText`, `searchEcho`, and two `onChange`. The look does not change; the iOS snapshots must stay the same. |
| `KreatynaMac/MacSearchField.swift` (new) | `NSSearchField` in an `NSViewRepresentable`, with `rustSearchText`. Typing, the clear button, and Esc all send the field's action. Identifier `ingredientSearch`. |
| `KreatynaMac/MacIngredientList.swift` | `MacSearchField` above the list content (MAC-08/Q2). |

### tests

| Layer | File | Tests |
|---|---|---|
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New `filteredCatalog` (search "tom", one row: Canned tomatoes) and `noMatches` (search "tomx", no rows, 3 in total), light and dark: 4 new images. The 10 existing Mac images must not change. |
| UI smoke (Mac) | `KreatynaMacUITests.swift` | New `testSearchNarrowsTheList`: seed Tomato, Canned tomatoes, Garlic. ⌘F, type "tom": Garlic is gone, both tomato rows remain. Type "x": "No matches". Esc: Garlic is back. A second part: open Garlic, search "tom", and the detail stays. |
| Demo capture (Mac) | `MacDemoCaptureUITests.swift` | New `testCaptureMAC08Demo`: seed the demo fixture, open Garlic, search "tom", capture; type "x", capture. |

If ⌘F does not focus the field, the UI test clicks the field instead, and
the plan's Validation says so.

### docs

| File | Change |
|---|---|
| `docs/epics/mac-shell/tasks/MAC-08-demo/demo.md` (new) | 2 screenshots: filtered list with Garlic still open, no matches. |
| `docs/epics/mac-shell/epic.html` | MAC-08 status. |

## Steps

1. Create `Shared/RustSearchable.swift` with the logic from `IngredientListScreen`.
2. Change `IngredientListScreen` to use it.
3. Add `.rustSearchable` to `MacIngredientList`.
4. Add the 2 snapshot tests; record the 4 images; look at each one.
5. Add the UI test and the demo capture method.
6. Run checks 1–3.
7. Capture the demo and write `MAC-08-demo/demo.md`.
8. Set MAC-08 to `awaiting-review`.
9. Send `demo.md` to the MAC-08 Inbox thread, then open `plannotator review`.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `devenv shell -- mac-test` | All pass. 10 existing Mac snapshots unchanged; 4 new ones match. |
| 2 | `devenv shell -- ios-test <udid>` | All pass. No iOS snapshot changes, so the move to `RustSearchable` changed nothing visible. |
| 3 | `cargo test --workspace` | Passes, unchanged (no Rust change; it proves nothing broke in the bindings build). |
| 4 | `capture-demo.sh mac …/testCaptureMAC08Demo docs/epics/mac-shell/tasks/MAC-08-demo` | `01-search.png`, `02-no-matches.png`. |
| 5 | `jj diff --summary` | Swift, tests, snapshots, and docs only. No Rust, no `project.yml`. |

## Stubs and exclusions

- Search filters by name only, as on iOS. No search in notes or units.
- No search history, no tokens, no scopes.
- **＋**, **Edit**, and **Delete** stay disabled (MAC-05, MAC-06, MAC-07).

## Validation

Run on 2026-10-13, macOS 27.0.1, Xcode 27.0, iPhone 17 Pro simulator.

### Deviations from the plan

1. **No ⌘F.** On macOS 27, ⌘F did not focus the first `.searchable` field
   (the UI test measured `hasKeyboardFocus` 0 after ⌘F, 1 after a click). The
   final `NSSearchField` has no ⌘F either. As the plan says for this case,
   the UI test and the demo click the field. A Find command (⌘F) needs a menu
   command and focus state; it is not in this slice.
2. **Snapshot harness.** In light mode, the search text was white on white.
   The field editor follows the app's appearance, not the window's, and the
   test host runs in this Mac's dark mode. `snapshot(…)` now also sets
   `NSApp.appearance` for the duration of a snapshot. The real app is not
   affected.
3. **Stale baselines re-recorded.** The search field changes the toolbar of
   every catalog snapshot. The 8 older catalog images (`catalogWithRows`,
   `emptyCatalog`, `detailWithFullData`, `detailWithNameOnly`, light and
   dark) still passed, because the tolerance (`precision: 0.995`,
   `perceptualPrecision: 0.98`) hid the new field. I deleted them, recorded
   them again, and compared old and new by eye: the only difference is the
   search field in the toolbar. `startupFailure` has no toolbar field and
   did not change.
4. **UI test seed.** The test seeds Garlic with a clove unit and checks
   "clove" to prove the detail stays. The title has no reliable
   accessibility element.

5. **Placement changed in review (MAC-08/Q2).** The field moved from the
   toolbar to the top of the list column, with a new `MacSearchField`. All
   12 list-column snapshots were recorded again and checked by eye. An
   off-screen window gives its first key view (now the search field) focus,
   which showed the search text as selected; the snapshot helper now calls
   `window.makeFirstResponder(nil)` before the capture. A UI test showed that
   the launched app focuses nothing, so this matches the app.

### Checks

| # | Check | Result |
|---|---|---|
| 1 | `mac-test` | After MAC-08/Q2: `TEST SUCCEEDED`, 50 passed, 0 failed, 3 capture tests skipped. 14 Mac snapshots: 4 new, 8 re-recorded, 2 unchanged (`startupFailure`). |
| 2 | `ios-test BB6851F0-…` | After MAC-08/Q2: `TEST SUCCEEDED`, 0 failed. No iOS snapshot file changed. |
| 3 | `cargo test --workspace` | Passed (431). No Rust change. |
| 4 | `capture-demo.sh mac …/testCaptureMAC08Demo docs/epics/mac-shell/tasks/MAC-08-demo` | `01-search.png`, `02-no-matches.png`. |
| 5 | `jj diff --summary` | Swift, tests, snapshots, docs. No Rust, no `project.yml`. |

UI test `testSearchNarrowsTheList` (passes with both placements): open Garlic, click the field, type
"tom" (Garlic hidden, both tomato rows shown, detail stays), type "x" ("No
matches"), Esc (all rows back). Passed.

### Not run

- Release build. No build settings changed.
