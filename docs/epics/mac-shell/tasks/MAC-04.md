# MAC-04 — Select an ingredient to see its details

Tracked in beads: `vibetyna-4rn.8`. Change: `nuolursx`. Dependencies: MAC-01 (approved), MAC-02
(approved).

## Open questions

None. The user answered MAC-04/Q1 and MAC-04/Q2 in the Inbox on 2026-10-13
(see Decisions).

## Decisions

### MAC-04/Q1 — The Mac gets its own catalog coordinator now

*Context:* Today Rust ignores list events while a detail is open.
`IngredientCatalog::update` accepts `List` events only on the list screen:

```rust
// real — crates/kreatyna/src/ingredients/catalog.rs
Message::Event(IngredientCatalogEvent::List(scoped)) => {
    if !self.accepts(scoped.scope) || !matches!(self.active, ActiveScreen::List) {
        return Command::empty();
    }
```

It also pauses the list (`self.list.suspend()`) when a detail opens. This is
right for the iPhone, where the detail covers the list. On the Mac, the list
stays visible, so a click on Tomato while Garlic is open must replace the
detail. E-14 says: add a Mac coordinator as soon as a Mac rule differs.

*Decision:* Mac coordinator now (the recommendation). A new Rust widget,
`IngredientSplitCatalog`, uses the same list and detail widgets. The list
always stays live. Swift chooses the layout once at startup: `stack` on the
iPhone, `split` on the Mac. The iPhone keeps `IngredientCatalog` with no
change in behavior. This becomes epic decision E-19.

### MAC-04/Q2 — No row is selected when the app opens

*Decision:* Nothing selected (the recommendation). The detail pane says
"Select an ingredient" until you click a row, as in MAC-01.

### Decided in the plan (sent with the questions, no objection)

- The highlighted row comes from Rust's selection, not from Swift.
- The arrow keys move the selection like a click: each move sends one event.
- ⌘-click on the selected row clears the selection.
- The detail pane uses the native Mac grouped form.
- **Edit** and **Delete** in the detail toolbar are disabled until MAC-06 and
  MAC-07.

## Outcome

You run `run-mac-app.sh --seed apple/Kreatyna/Fixtures/demo-ingredients.json`.
The window shows six rows and "Select an ingredient". You click **Garlic**.
The row is highlighted, and the detail pane shows:

- **Garlic** as the title
- Notes: "Peel the cloves. Approximate values."
- Nutrition per 100 g: Calories 149 kcal, Protein 6.4 g, Carbohydrates 33 g,
  Fat 0.5 g
- Reference units: clove 5 g, bulb 50 g

You click **Basil**. The detail pane now shows Basil: a title and
"Nutrition per 100 g" with "—" for each value. You press ↓. Canned tomatoes
is selected and shown. You ⌘-click Canned tomatoes. The pane returns to
"Select an ingredient". The iPhone app behaves exactly as before.

## Terms

- **Coordinator:** a Rust widget that owns child widgets (list, detail,
  form) and decides which one is active. `IngredientCatalog` is the iPhone's
  coordinator. It keeps a stack of screens.
- **Split layout:** the Mac's columns: the list and the detail side by side.
  Both are visible at the same time.
- **Scope (`CommandScopeId`):** an ID that Rust puts in each view. Swift sends
  it back with each event. When a screen changes, Rust makes a new scope, so
  events and results from the old screen are ignored.
- **Layout (`AppLayout`):** a new value that Swift passes to `app(…)` at
  startup: `stack` or `split`. Rust builds the matching coordinator.
- **Grouped form:** the macOS style of `Form { … }.formStyle(.grouped)`, with
  rounded boxes and section titles, as in System Settings.

## Design

[MAC-04-design.html](MAC-04-design.html) shows the two states: Garlic with
full data, and Basil with a name only. The stubs are marked there.

## Changes by layer

**db: no change.**

### rust — `crates/kreatyna/src/ingredients/split_catalog.rs` (new)

*sketch:*

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitCatalogEvent {
    List(Scoped<IngredientListEvent>),
    Detail(Scoped<IngredientDetailEvent>),
    /// ⌘-click on the selected row.
    ClearSelection { scope: CommandScopeId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitCatalogOutcome {
    List(Scoped<IngredientListOutcome>),
    Detail(Scoped<IngredientDetailOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientSplitPaneView {
    /// "Select an ingredient".
    Empty,
    Detail { scope: CommandScopeId, detail: IngredientDetailView },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngredientSplitCatalogView {
    pub list: IngredientListView,
    pub list_scope: CommandScopeId,
    /// The highlighted row. Swift does not keep its own selection.
    pub selected: Option<IngredientId>,
    pub pane: IngredientSplitPaneView,
}

pub struct IngredientSplitCatalog {
    caps: IngredientCapabilities,
    list: IngredientList,
    list_scope: CommandScope,   // never replaced: the list is always live
    pane: Pane,                 // Empty | Detail(IngredientDetail)
    pane_scope: CommandScope,   // replaced on each selection change
}
```

Two scopes instead of one: the list keeps one scope for its whole life,
because it never leaves the window. The pane gets a new scope each time the
selection changes, so a late event or result for Garlic's detail is ignored
after you click Tomato.

Rules:

| Situation | Rust does |
|---|---|
| Launch | Loads the list. `selected` is `None`, `pane` is `Empty` (MAC-04/Q2). |
| `List(Select(id))`, row visible | The list sends `OpenDetail`. The coordinator opens a new `IngredientDetail` in a new pane scope and renders. |
| `Select` of the selected row | Nothing. The detail keeps its state. |
| `Select` while the detail is confirming or running a delete | Nothing. The delete finishes first. (Only reachable after MAC-07.) |
| `List(Search)`, `List(Refresh)` with a detail open | The list handles them. The detail stays, also when the search hides its row. |
| `ClearSelection` | The pane becomes `Empty` in a new pane scope. Ignored while a delete runs. |
| Detail outcome `Deleted { id }` | Removes the row from the cached list (or reloads), and the pane becomes `Empty`. |
| List outcomes `CreateRequested`, `OpenEdit` | Ignored until MAC-05 and MAC-06. **＋** and **Edit** are disabled in Swift. |
| Detail outcome `OpenEdit` | Ignored until MAC-06. |
| Shutdown | Like `IngredientCatalog`: the list and an open detail both drain. |

`Deleted` handling is in this slice because the detail widget can produce
it. The coordinator must route every outcome. Swift cannot reach it before
MAC-07 enables **Delete**.

The list never pauses in this coordinator: `suspend` and `resume` are not
called.

`mod.rs` exports the new types. `IngredientList::contains_visible` and
`apply_deleted` are `pub(crate)` already, so the new widget can use them.

### rust — `crates/kreatyna/src/app.rs`

*sketch:*

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLayout { Stack, Split }

pub enum AppEvent {
    Ingredients(IngredientCatalogEvent),
    SplitIngredients(IngredientSplitCatalogEvent),   // new
}

pub enum IngredientsView {                          // new
    Stack(IngredientCatalogView),
    Split(IngredientSplitCatalogView),
}

pub struct AppView { pub ingredients: IngredientsView }

impl AppWidget {
    pub fn new(caps: AppCapabilities, layout: AppLayout) -> Self
}
```

`AppWidget` holds one of the two coordinators. An event for the other
layout is ignored (`Command::empty()`). `test_fixture()` keeps `Stack`, so
the existing app tests do not change. A new `test_fixture_split()` is for
the split tests.

### ffi — `crates/kreatyna-ffi`

| File | Change |
|---|---|
| `src/lib.rs` | `app(caps, database_path, layout: AppLayout)`. |
| `src/app_shell.rs` | `AppSetup` gets `layout`. `prepare_app` passes it to `AppWidget::new`. |
| `src/app_types/app.rs` | `AppLayout` (uniffi enum). `AppEvent::SplitIngredients { event }`. `RunningAppView { ingredients: IngredientsView }`. |
| `src/app_types/split_catalog.rs` (new) | `IngredientSplitCatalogEvent`, `IngredientSplitCatalogView`, `IngredientSplitPaneView`, with conversions like `catalog.rs`. |
| `src/app_types/tests.rs` | Conversion tests for the new types. |
| `src/app_runtime_tests.rs`, `src/app_shell/tests.rs` | Pass `AppLayout::Stack`. One new test starts with `Split` and reaches a ready split view. |

*sketch (Swift sees):*

```swift
enum IngredientsView { case stack(catalog: IngredientCatalogView), split(catalog: IngredientSplitCatalogView) }
func app(caps: AppCapabilities, databasePath: String, layout: AppLayout) -> AppFfi
```

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `AppModel.swift` | `init(layout: AppLayout = .platform, databasePath: String? = nil)`. Passes `layout` to `app(…)`. |
| `AppLayout+Platform.swift` (new) | `static let platform`: `.split` on macOS, `.stack` on iOS. A platform fact, not a business rule. |
| `AppEventComposition.swift` | `IngredientSplitCatalogView.list(_:)`, `.detail(_:)`, `.clearSelection`; `RunningAppView.splitIngredients(_:)`. |
| `IngredientText.swift` (new) | The measurement text (`5 g`, `14 g · 15 ml`) moves here from `IngredientDetailScreen.swift`, so both apps format units the same way. `NutritionBasis.label` ("per 100 g") is already shared in `ErrorText.swift`. |

### swift — iOS (`apple/Kreatyna/Kreatyna/`)

| File | Change |
|---|---|
| `ContentView.swift` | `case .running(let app)` reads `case .stack(let catalog) = app.ingredients`. A `split` view cannot occur on iOS; the view shows nothing for it. |
| `IngredientDetailScreen.swift` | Uses the shared measurement text. Its look does not change. |

The iOS snapshots must not change. `ios-test` proves it.

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacRootView.swift` | Reads `case .split(let catalog)`, and sends `app.splitIngredients(_)`. |
| `MacCatalogView.swift` | Takes `IngredientSplitCatalogView`. The detail column shows `MacIngredientDetail` or the placeholder, from `pane`. |
| `MacIngredientList.swift` | `List(selection:)` with a binding. *get:* `view.selected`. *set:* a new ID sends `.select(id:)`; `nil` sends `clearSelection`. Swift does not store the selection. |
| `MacIngredientDetail.swift` (new) | Grouped `Form`: title, Notes (if any), Nutrition per 100 g / 100 ml, Reference units (if any). Toolbar: **Edit** and **Delete**, both disabled. |

*sketch:*

```swift
List(selection: Binding(
    get: { view.selected },
    set: { id in send(id.map { .list(.select(id: $0)) } ?? .clearSelection) }
)) { ForEach(rows, id: \.id) { row in Text(row.name).tag(row.id) } }
```

Accessibility identifiers: `ingredientDetail`, `ingredientNotes`,
`ingredientUnits`, `editIngredient`, `deleteIngredient`.

### tests

| Layer | File | Tests |
|---|---|---|
| Rust widget | `crates/kreatyna/tests/ingredient_split_catalog.rs` (new) | One test or test case per row of the rules table, plus: a stale detail event after a selection change is ignored; a list event with the list scope is accepted while a detail is open; a row that the search hides cannot be selected. |
| Rust app | `crates/kreatyna/tests/app.rs` | The split layout shows `IngredientsView::Split`; a stack event to a split app is ignored, and the reverse. |
| FFI | `app_types/tests.rs`, `app_runtime_tests.rs` | Conversions; a split app reaches ready. |
| Swift Testing | `SharedTests/AdapterPlumbingTests.swift` | The split event helpers keep the scopes from the view. `AppLayout.platform` is `.split` in the Mac test host and `.stack` in the iOS one. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New: `detailWithFullData`, `detailWithNameOnly`, light and dark: 4 new images. The 6 MAC-01 images must stay the same. |
| UI smoke (Mac) | `KreatynaMacUITests.swift` | New `testSelectingRowsShowsTheirDetail`: seeds Garlic (with a clove unit) and Basil; clicks Garlic and sees "clove" and "5 g"; clicks Basil and sees no "clove". |
| Demo capture (Mac) | `MacDemoCaptureUITests.swift` | New method `testCaptureMAC04Demo`: seeds the fixture; captures Garlic, then Basil. |

### fixture

`apple/Kreatyna/Fixtures/demo-ingredients.json` gets a name-only item,
`"Basil"`. The Rust test `demo_fixture_is_valid` then expects 6 names.

### docs

| File | Change |
|---|---|
| `docs/epics/mac-shell/epic.html` | New E-19 (layouts and the split coordinator). MAC-04 status. MAC-05 and MAC-07 layers gain `rust` and `ffi`: create and delete must also be routed through the split coordinator. |
| `docs/epics/mac-shell/tasks/MAC-04-demo/demo.md` (new) | Garlic and Basil screenshots, macOS version, window size, appearance. |

## Steps

1. Add `split_catalog.rs` and export it in `ingredients/mod.rs`.
2. Write `tests/ingredient_split_catalog.rs`, then make it pass.
3. Add `AppLayout` and `IngredientsView` to `app.rs`; update `tests/app.rs`.
4. Add `"Basil"` to the fixture; update `demo_fixture_is_valid`.
5. Add the FFI types and the `layout` parameter; update the FFI tests.
6. Run Rust checks 1–2.
7. Run `mac-bindings debug` and `ios-bindings debug`.
8. Add the shared Swift files and change `AppModel`.
9. Update the iOS `ContentView` and `IngredientDetailScreen`.
10. Update the Mac views and add `MacIngredientDetail.swift`.
11. Add the snapshot tests, record the 4 new images, and look at each one.
12. Add the UI smoke test and the demo capture method.
13. Run checks 3–6.
14. Capture the demo with `capture-demo.sh` and write `MAC-04-demo/demo.md`.
15. Update `epic.html`.
16. Set MAC-04 to `awaiting-review`.
17. Send `demo.md` to the MAC-04 Inbox thread, then open `plannotator review`.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All tests pass, with the new split catalog tests. |
| 2 | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` | Clean. |
| 3 | `devenv shell -- mac-test` | All pass. The 6 MAC-01 snapshots are unchanged; 4 new snapshots match. |
| 4 | `devenv shell -- ios-test <udid>` | All pass. No iOS snapshot changes. |
| 5 | `run-mac-app.sh --seed apple/Kreatyna/Fixtures/demo-ingredients.json` | The window opens with the isolated database. |
| 6 | `capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests/testCaptureMAC04Demo docs/epics/mac-shell/tasks/MAC-04-demo` | Saves `01-garlic.png` and `02-basil.png`. They match the design. |
| 7 | `jj diff --summary` | Only files from this plan. No migration. No `project.yml` change, because the new Swift files are in folders that the targets already include. |

## Stubs and exclusions

- **Edit** in the detail toolbar is disabled (MAC-06). The split coordinator
  ignores `OpenEdit`.
- **Delete** in the detail toolbar is disabled (MAC-07). The coordinator
  already handles `Deleted`, but Swift cannot start a delete.
- **＋** stays disabled (MAC-05). The coordinator ignores `CreateRequested`.
- The search field does not exist yet (MAC-08). The coordinator already
  keeps the detail when a search hides its row; MAC-08 shows it.
- No context menu on rows.
- No change to iPhone behavior. iPad is not in scope.
- No multi-window support.

## Validation

Run on 2026-10-13, macOS 27.0.1, Xcode 27.0, iPhone 17 Pro simulator.

### Deviations from the plan

1. **Pane scope.** The sketch put the scope inside
   `IngredientSplitPaneView::Detail`. The code has a `pane_scope` field next
   to `pane`. `ClearSelection` needs the pane scope, and a single field is
   simpler for Swift (`view.clearSelection`, `view.detail(_:)`).
2. **Keyboard focus.** A click selected a row but did not give the list
   keyboard focus, so ↓ did nothing. The UI test found it (the focused element
   was the window). `MacIngredientList` now has a `@FocusState` that is set
   to `true` each time Rust's `selected` changes. This is presentation state,
   which Swift may own (AGENTS.md). A local copy of the selection did not
   fix it, and `.focused` without the `onChange` did not either.
3. **No shutdown test.** The planned rules row "Shutdown" has no test of its
   own. `IngredientDetail` never delays shutdown, so such a test cannot fail;
   the split catalog forwards `can_begin_shutdown` and `begin_shutdown` like
   `IngredientCatalog`.
4. **Extra rules.** Two rules came out of the tests and are in the code:
   `ClearSelection` with nothing selected does nothing, and `ClearSelection`
   is ignored while a delete runs.
5. **Accessibility identifier.** The list has `ingredientList`, so tests can
   tell it apart from the sidebar list.
6. **`IngredientsView` helpers.** Swift gets `app.ingredients.stack` and
   `.split` (optional accessors) in `AppEventComposition.swift`, instead of a
   `switch` in each view and test.

### Checks

| # | Check | Result |
|---|---|---|
| 1 | `cargo test --workspace` | Passed. `kreatyna`: 352 tests, including 18 new split catalog tests and 2 new app tests. `kreatyna-ffi`: 43 (3 new). `kreatyna-db`: 29. |
| 2 | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` | Clean. Three single-instance enums have `#[allow(clippy::large_enum_variant)]` with a reason, like the existing FFI view enums. |
| 3 | `mac-test` | `TEST SUCCEEDED`: 46 passed, 2 capture tests skipped. The 6 MAC-01 snapshots are unchanged. The 4 new snapshots were recorded on the first run, checked by eye, and matched on the second run. |
| 4 | `ios-test BB6851F0-…` | `TEST SUCCEEDED`: 57 passed, 1 capture test skipped. No iOS snapshot changed. |
| 5 | `run-mac-app.sh --skip-rust --seed apple/Kreatyna/Fixtures/demo-ingredients.json` | `pid 96690`, macOS 27.0.1, `isolated, seeded from demo-ingredients.json (6 items)`. |
| 6 | `capture-demo.sh mac …/testCaptureMAC04Demo docs/epics/mac-shell/tasks/MAC-04-demo` | `01-garlic.png`, `02-basil.png`. They match the design. Garlic's row has the focused (blue) highlight. |
| 7 | `jj diff --summary` | Files from this plan, plus `MAC-04-demo/`. No migration. No `project.yml` change. |

UI smoke test `testSelectingRowsShowsTheirDetail`: click Garlic (clove 5 g
shown), click Basil (clove gone), ↓ (Garlic again), ⌘-click Garlic ("Select
an ingredient"). Passed.

### Not run

- Release build. No build settings changed.

### Code review, round 1 (2026-10-13)

The user's 4 comments, discussed in the Inbox before any code changed:

| ID | Comment | Decision | Change |
|---|---|---|---|
| R1 | `split_catalog.rs:40`: use `Scoped<IngredientListView>` and the same for the pane | Yes, for the split catalog | `IngredientSplitCatalogView { list: Scoped<IngredientListView>, selected, pane: Scoped<IngredientSplitPaneView> }`. Over UniFFI, which has no generics: `ScopedIngredientListView` and `ScopedIngredientSplitPaneView` records (`scope`, `value`). Swift reads `view.list.value`, and the event helpers take `view.list.scope` and `view.pane.scope`. This replaces deviation 1. |
| R2 | `split_catalog.rs:56`: the same pairing in the widget state | Private `ScopedChild<W>` | `ScopedChild<W> { widget, scope: CommandScope }` with `replace`, `accepts`, `command`, and `view`. Replacing the child replaces its scope, which cancels its commands. |
| R3 | `app.rs:12–13`: two endpoints instead of `AppLayout` | Keep `AppLayout` | None. |

Checks after the changes: `cargo test --workspace` passed (counts as
above); fmt and Clippy clean; `mac-test` `TEST SUCCEEDED` (46 passed, 2
capture tests skipped, snapshots unchanged); `ios-test` `TEST SUCCEEDED`
(0 failed). The demo screenshots were not captured again: the views draw
the same values.

### Code review, round 2 (2026-10-13)

| ID | Comment | Decision | Change |
|---|---|---|---|
| Q1 | `split_catalog.rs:49`: move `ScopedChild` to its own module | `crates/kreatyna/src/scoped_child.rs`, crate-private, own unit tests | New module with 6 unit tests in `scoped_child/tests.rs`: `accept` for the current and a foreign scope, `replace` gives a new scope, `view` carries the scope, `forward` updates and scopes, `forward` ignores an old scope, `replace` cancels the old commands. |
| Q2 | `split_catalog.rs:269`: the repeated "check the scope, then update" pattern | `accept` + `forward` on `ScopedChild`, `Pane` becomes a `Widget` | `accept(Scoped<T>) -> Option<T>` and, for a `Widget` child, `forward(Scoped<Event>)`. Event arms are one `forward(…).map(…)` each; outcome arms use `accept`, then the coordinator's own handling. `Pane` implements `Widget` (an empty pane ignores messages), so `show`, shutdown, and detail events no longer `match` on it. `ClearSelection` compares its scope with `pane.scope()`. |

Checks after the changes: `cargo test --workspace` passed (431 tests); fmt
and Clippy clean; `mac-bindings debug` and `mac-test` `TEST SUCCEEDED` (46
passed). Not run again: `ios-test`, because the iPhone uses the stack layout
and nothing in it or in the FFI types changed.


### Code review, round 3 (2026-10-13)

| ID | Comment | Decision | Change |
|---|---|---|---|
| R4 | `app.rs:76`: use `ScopedChild` for `ingredients` and `ingredient_lifetime` | `ScopedChild<Ingredients>`, and `Ingredients` becomes a `Widget` | `AppWidget { ingredients: ScopedChild<Ingredients>, initialized }`. `Ingredients` implements `Widget` (event `AppEvent`, outcome the new `IngredientsOutcome { Stack, Split }`), so the layout `match` lives in one place. `AppOutcome` has one variant, `Ingredients(Scoped<IngredientsOutcome>)`, checked with `accept`. App events carry no scope, so they use `widget.update`, not `forward`. `AppOutcome` is not exported over FFI; Swift does not change. |

Checks after the changes: `cargo test --workspace` passed (431 tests); fmt
and Clippy clean; `mac-bindings` and `ios-bindings` rebuilt; `mac-test`
`TEST SUCCEEDED` (46 passed); `ios-test` `TEST SUCCEEDED` (0 failed).
