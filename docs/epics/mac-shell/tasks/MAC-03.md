# MAC-03 — Counter leaves the app

Tracked in beads: `vibetyna-4rn.7`. Change: `vwrumwyu`.
Dependencies: MAC-11 (approved).

## Open questions

None. The user answered MAC-03/Q1, MAC-03/Q2 and MAC-03/Q3 in review on
2026-10-12 (see Decisions).

## Decisions

### MAC-03/Q1 — Remove the app-level activation scope

*Context:* `AppWidget` has an `activation: CommandScope`. Rust renews it on
each tab switch (`crates/kreatyna/src/app.rs:187`). Each app event carries the
scope that Swift displayed, and Rust ignores the event when the scope is old
(`app.rs:191`). Swift sends the scope back from `RunningAppView.scope`
(`AppEventComposition.swift:7`). Without tabs, nothing renews the scope, so
the check can never reject an event. The ingredient catalog has its own,
second scope (`IngredientCatalogView.scope`). This slice does not change it.

*Decision:* Option A ("Looks good"). Remove the activation scope.
`AppEvent::Ingredients` carries only the catalog event. `AppView` and
`RunningAppView` lose `scope`. REC-01 (recipes epic) adds a section scope back,
together with the section switching that renews it, and with tests that show a
rejected event.

*Why:* introduce each concept in the first slice that needs it. A check that
can never reject anything is not tested behavior.

### MAC-03/Q2 — Rename the database files, without moving data

*Context:* two file names say "counter":

| File | Where | Content |
|---|---|---|
| `Application Support/Kreatyna/counter.sqlite` | `AppDatabaseLocation.swift:27`, pinned by `KreatynaTests.swift:114` | The real app data: ingredients, and today the counter value. |
| `crates/kreatyna-ffi/counter-dev.sqlite` | `devenv.nix:84` (`DATABASE_URL`), `crates/kreatyna-ffi/.gitignore` | A scratch database for the Diesel CLI. It is git-ignored. No code reads it. |

*Decision:* Option B (user: "this one"). Rename the app file to
`kreatyna.sqlite` and the scratch file to `kreatyna-dev.sqlite`. Swift does
not move or delete the old file.

*Consequence:* each existing install (your simulators and devices) starts with
an empty catalog after this slice. The old `counter.sqlite` stays on disk,
unused. You can delete it by hand, or delete and reinstall the app. Option C
(a one-time move in Swift) was rejected: it adds Swift code that makes a data
decision, for data that is only test data today.

### MAC-03/Q3 — Remove the `Clock` capability

*Context:* `Clock` is a Rust capability with one method,
`async fn sleep_ms(&self, ms: u64)` (`crates/kreatyna/src/capabilities.rs:15`).
Only Counter's background tick uses it. It crosses FFI as the `ClockHandler`
foreign trait and the `clock` field of the FFI `AppCapabilities` record
(`crates/kreatyna-ffi/src/lib.rs:115-136`). Swift implements it in
`AppClock.swift`.

*Decision:* Option A (user: "Good"). Remove `Clock` from Rust, FFI and Swift.

*Why:* the recipe brief's cooking timers must survive quitting the app (D-27,
D-42). They need wall-clock deadlines, not a sleep of N milliseconds. The
cooking-timer slice designs its own capability. The `krucyfiks` Counter
example keeps the `sleep_ms` pattern.

## Outcome

- The iOS app opens straight into the ingredient catalog. There is no tab bar
  at the bottom. The list shows "Canned tomatoes", "Cucumber", "Garlic" as
  before. Search, **+**, detail, edit and delete work as before.
- The search field is now at the bottom of the screen. iOS 26 puts it there
  when a screen has no tab bar (found during implementation, see Validation).
- The app stores its data in `Application Support/Kreatyna/kreatyna.sqlite`.
  An existing install starts with an empty catalog: the app no longer opens
  the old `counter.sqlite` (Q2).
- The migration still upgrades old databases. Rust tests show it: a database
  with a counter value of 27 and the ingredient "Canned tomatoes" upgrades
  without errors. "Canned tomatoes" is still there. The `counter` table is gone.
- `kreatyna`, `kreatyna-db` and `kreatyna-ffi` contain no Counter types.
  Counter lives on only in `krucyfiks/examples/counter` (MAC-11).

## Terms

| Term | Meaning |
|---|---|
| Counter | A demo widget: a number with Increase, Decrease, Reset, and a background tick every 1000 ms. It is the app's second tab today. |
| `AppWidget` | The top Rust widget. It holds the child widgets and routes events and outcomes to them. |
| Widget | A `krucyfiks` trait with `init`, `update(Message)` and `view`. `update` changes state and returns a `Command`. |
| Command | Async work that a widget returns. The runtime runs it. It can request effects and produce outcomes. |
| Capability | A trait such as `Render` or `IngredientStore`. A widget calls it. The platform (Swift or SQLite) answers it. |
| `CommandScope` | An ID that a widget renews when a screen changes. Events and outcomes carry it. The widget ignores those with an old ID. |
| Forward-only migration | A new SQL file that changes the schema. It is never undone. `down.sql` stays empty. |
| `schema.rs` | Diesel's Rust description of the tables. Queries compile against it. |
| UniFFI | The tool that generates Swift code from Rust types. The generated Swift is never edited by hand. |
| `RunningAppView` | The FFI record that Swift gets when the app has started. Today it has `tab`, `scope`, `ingredients` and `counter`. |
| Snapshot test | A test that renders a SwiftUI view to a PNG and compares it with a stored baseline PNG. |
| Baseline | The stored PNG that a snapshot test compares against. A changed baseline needs a visual review. |
| XcodeGen | It generates the Xcode project from `apple/Kreatyna/project.yml`. The project includes whole folders, so deleting a Swift file needs no YAML edit. |
| jj change | One reviewable unit of work in jj (the version control tool used here). This slice is change `vwrumwyu`. |

## Design

[`MAC-03-design.html`](MAC-03-design.html) shows the iOS catalog before and
after: the tab bar disappears, and the search field moves to the bottom.
[`MAC-03-demo/demo.md`](MAC-03-demo/demo.md) shows the running app. The reference
screenshot of the screen before this slice is
`docs/epics/ingredient-catalog/demo/05-catalog.png`.

## Changes by layer

The file lists come from `rg -n -i "counter|AppTab|composab|clock|CLIENT_CAPABILITY|sleep_ms|tick"`
over the repository on 2026-10-11 (`target/` and `docs/` excluded), and from
`jj file list`. Each hit is in one of the tables below, or in "Stubs and
exclusions".

### db

| File | Change |
|---|---|
| `crates/kreatyna-db/migrations/2026-10-11-200000-0000_drop_counter/up.sql` | New. `DROP TABLE counter;` |
| `crates/kreatyna-db/migrations/2026-10-11-200000-0000_drop_counter/down.sql` | New. Empty (forward-only policy). |
| `crates/kreatyna-db/src/schema.rs` | Remove the `counter` table block. Remove `counter` from `allow_tables_to_appear_in_same_query!`. Edit by hand, as before: the file differs on purpose from `diesel print-schema` (for example `local_id -> BigInt`). |
| `crates/kreatyna-db/src/sqlite.rs` | Remove `impl CounterDatabaseMut for SqliteDatabase` (lines 35–75), its imports (lines 6–8, 12), the `DatabaseEffect::Counter` variant and its worker arm (lines 119–123). |
| `crates/kreatyna-db/src/ingredient_models/tests.rs` | Rewrite two migration tests (below). Remove `counter` from the `schema` import (line 13). |
| `crates/kreatyna-db/src/ingredient_store/tests.rs` | Remove the `CounterDatabaseMut` import (line 4), `db.set_count(27)` (line 79), and both `count()` asserts (lines 138, 152). |

`DatabaseEffect` keeps one variant (*sketch*). REC-01 adds a recipe store
variant, so the enum stays:

```rust
pub enum DatabaseEffect {
    Ingredient(Envelope<IngredientStoreRequest>),
}
```

The old migration `2026-09-15-101056-0000_create_counter` stays. Diesel
records applied migrations by name. A deleted migration would break the
upgrade path.

Migration tests after the change. `schema.rs` has no `counter` table, so the
tests write and read it with raw SQL (*sketch*):

```rust
#[test]
fn empty_database_migrates_and_rerun_is_harmless() {
    let mut db = SqliteConnection::establish(":memory:").unwrap();
    db.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
    assert_eq!(db.run_pending_migrations(MIGRATIONS).unwrap().len(), 3);
    assert!(db.run_pending_migrations(MIGRATIONS).unwrap().is_empty());
    assert!(!table_exists(&mut db, "counter"));
    assert_eq!(i::table.count().get_result::<i64>(&mut db).unwrap(), 0);
}

#[test]
fn counter_database_upgrades_and_keeps_ingredients() {
    // 1. A file database runs the first two migrations only (counter, ingredients).
    // 2. Insert counter value 27 with `sql_query`, and the ingredient "Canned tomatoes".
    // 3. `run_pending_migrations` returns 1 (drop_counter).
    // 4. The counter table is gone. The ingredient row is unchanged ("1.2300" calories).
    // 5. Reopen the file: no pending migrations, the ingredient row is still there.
}
```

`table_exists` is a small test helper that reads `sqlite_master`. The second
test replaces `counter_only_database_migrates_and_reopens_without_data_loss`.

### rust — `crates/kreatyna`

| File | Change |
|---|---|
| `src/lib.rs` | Remove `Counter`, `CounterStatus`, `CounterView`, `CounterEvent`, `CounterOutcome`, `Counter::test_fixture`, the `CounterCapabilities` re-export, `pub mod composability`, and `AppTab` from the `app` re-export. Keep `CLIENT_CAPABILITY`: the ingredient capabilities use it. |
| `src/capabilities.rs` | Keep only `Render`. Remove `Clock` (Q3), `CounterDatabase`, `DatabaseError`, `CounterEffect`, `CounterCapabilities`, `CounterEffectStream`, the compile-fail doctest, and `mod tests`. |
| `src/capabilities/tests.rs` | Delete. MAC-11 moved these 17 tests to `krucyfiks/macro-tests`. |
| `src/composability.rs` | Delete. MAC-11 moved it to `krucyfiks/examples/counter`. |
| `src/app.rs` | Rewrite as a thin wrapper around the catalog (sketch below). |
| `tests/counter.rs`, `tests/background.rs`, `tests/composition.rs` | Delete. MAC-11 moved them. |
| `tests/app.rs` | Keep 3 tests, add 1 (table below). |
| `Cargo.toml` | Remove `async-stream`, `async-trait` and `tracing` if `cargo check --workspace --all-targets` passes without them. Only Counter code uses them today. |

`AppWidget` after the change (*sketch*, Q1). It no longer renders by
itself: only tab switches did that. So `AppCapabilities.render`,
`AppEffect::Render` and the `app_lifetime` scope go. `ingredient_lifetime`
stays: it cancels catalog commands when the app is dropped.

```rust
#[derive(Debug)]
pub enum AppEvent {
    Ingredients(IngredientCatalogEvent),
}

#[derive(Debug)]
pub enum AppOutcome {
    Ingredients(Scoped<IngredientCatalogOutcome>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppView {
    pub ingredients: IngredientCatalogView,
}

#[derive(Clone)]
pub struct AppCapabilities {
    pub ingredients: IngredientCapabilities,
}

#[derive(Debug, krucyfiks::Effect)]
pub enum AppEffect {
    Ingredients(IngredientEffect),
}

pub struct AppWidget {
    ingredients: IngredientCatalog,
    ingredient_lifetime: CommandScope,
    initialized: bool,
}
```

`update` routes `AppEvent::Ingredients(event)` to the catalog. It routes
`AppOutcome::Ingredients(outcome)` to the catalog when
`ingredient_lifetime.accepts(outcome.scope)`. `init` runs once.
`can_begin_shutdown` and `begin_shutdown` delegate to the catalog.

`tests/app.rs` (12 tests today):

| Test | Fate | Why |
|---|---|---|
| `initialization_is_concurrent_and_runs_once` (2 cases) | Replace with `initialization_loads_the_catalog_once` | One child is left. Nothing is concurrent. |
| `tab_events_require_the_current_activation_and_ready_counter` | Delete | No tabs, no Counter, no activation scope. |
| `navigation_draft_and_confirmation_survive_tab_switches` | Delete | No tab switches. Catalog tests cover navigation and drafts. |
| `pending_save_finishes_while_hidden` (2 cases) | Delete | The catalog cannot be hidden. |
| `pending_delete_finishes_while_hidden` (2 cases) | Delete | The catalog cannot be hidden. |
| `counter_departure_cancels_ticks_but_preserves_accepted_writes` | Delete | Counter rule. |
| `shutdown_hook_stops_counter_background_but_preserves_an_accepted_write` | Delete | Counter rule. |
| `shutdown_hook_preserves_an_accepted_ingredient_save_and_follow_up` | Keep | App shutdown still delegates to the catalog. |
| `app_drop_cancels_unpolled_initialization` | Keep | Still true. |
| — | Add `catalog_events_and_outcomes_route_through_the_app` | Shows the app routing: search "gar" updates `view().ingredients.list.search`; `CreateRequested` opens the create form. |

Result: 4 tests. The `Fixture` loses `counter`, `switch`, and the Counter
branch in `initialize`.

Sketch of the new test (*sketch*):

```rust
#[tokio::test]
async fn catalog_events_and_outcomes_route_through_the_app() -> anyhow::Result<()> {
    let mut f = Fixture::new();
    f.initialize().await?;
    let command = f.list(IngredientListEvent::Search("gar".into()));
    f.render(command).await?;
    assert_eq!(f.app.view().ingredients.list.search, "gar");
    f.create().await?;
    assert!(matches!(
        f.app.view().ingredients.screen,
        IngredientCatalogScreenView::Create(_)
    ));
    Ok(())
}
```

### ffi — `crates/kreatyna-ffi`

| File | Change |
|---|---|
| `src/lib.rs` | Remove `ClockHandler`, `ClockHandlerShim` and the `clock` field of `AppCapabilities` (Q3). Change the `start` doc comment from "render and clock handlers" to "the render handler". |
| `src/app_shell.rs` | `AppSetup` loses `clock`. `prepare_app` creates only the ingredient store client. `DatabaseWorker.close_handles` has one handle. |
| `src/app_shell/tests.rs` | Use the ingredient store only (table below). |
| `src/app_types.rs` | Remove `mod counter` and `pub use counter::*`. |
| `src/app_types/counter.rs` | Delete. |
| `src/app_types/app.rs` | Remove `AppTab`, `AppEvent::SelectTab`, `AppEvent::Counter`. `RunningAppView` keeps only `ingredients` (Q1). |
| `src/app_types/support.rs` | No change. `count()` still converts list counts and draft indexes. |
| `src/app_types/tests.rs` | Rewrite two tests, delete one (table below). |
| `src/app_runtime_tests.rs` | Remove `TestClock` and the `clock` fields. Replace `ready.tab` and `SelectTab` uses. Remove the `app.counter.status` assert. |

FFI shapes after the change (*sketch*, Q1):

```rust
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum AppEvent {
    Ingredients { event: IngredientCatalogEvent },
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct RunningAppView {
    pub ingredients: IngredientCatalogView,
}

#[derive(uniffi::Record)]
pub struct AppCapabilities {
    render: Arc<dyn RenderHandler>,
}
```

FFI tests:

| Test | Change |
|---|---|
| `app_shell/tests.rs` `close_and_wait_finishes_while_clients_are_retained` | Check only the ingredient client: `list()` returns `Transport` after close. |
| `app_shell/tests.rs` `admitted_requests_from_both_capabilities_finish_after_close` | Rename to `admitted_requests_finish_after_close`. Admit two ingredient `list()` requests instead of one counter and one ingredient request. |
| `app_shell/tests.rs` `drop_requests_close_without_aborting_the_worker` | Use the ingredient store client. |
| `app_types/tests.rs` `nested_event_conversion_preserves_both_scope_ids` | Rename to `nested_event_conversion_preserves_the_catalog_scope`. Check only the catalog scope (29). |
| `app_types/tests.rs` `converted_stale_event_is_rejected_by_existing_app_routing` | Make the catalog scope old: open the create form, then send a list event with the scope read before. Expect an empty command and an unchanged screen. |
| `app_types/tests.rs` `counter_view_preserves_readiness` (2 cases) | Delete. |
| `app_runtime_tests.rs` `app_ffi_initializes_before_exposing_the_running_app` | Remove the `ready.tab` assert. After shutdown, dispatch an ingredient search event and expect `Terminated`. |
| `app_runtime_tests.rs` `graceful_shutdown_before_initialization_is_polled_drains_prepared_children` | Remove the `app.counter.status` assert. The ingredient list assert stays. |

### swift — `apple/Kreatyna`

`project.yml` needs no edit: the targets include the folders `Kreatyna`,
`KreatynaTests` and `KreatynaUITests`.

| File | Change |
|---|---|
| `Kreatyna/CounterScreen.swift` | Delete. |
| `Kreatyna/AppClock.swift` | Delete (Q3). |
| `Kreatyna/ContentView.swift` | `.running(let app)` shows `IngredientCatalogScreen` directly. Delete `AppTabsView`. |
| `Kreatyna/AppEventComposition.swift` | Remove `counter(_:)`. `ingredients(_:)` returns `.ingredients(event: event)` (Q1). |
| `Kreatyna/AppModel.swift` | Remove the `clock` parameter. Create `AppCapabilities(render: renderer)`. |
| `Kreatyna/AppDatabaseLocation.swift` | Return `…/Kreatyna/kreatyna.sqlite` instead of `counter.sqlite` (Q2). |
| `KreatynaTests/Fixtures.swift` | `running(screen:)` loses `tab` and `counter`. |
| `KreatynaTests/AdapterPlumbingTests.swift` | Remove the `app.counter(.increase)` expectation. Expect `.ingredients(event: .list(scope: ScopeId(value: 2), event: .createRequested))`. |
| `KreatynaTests/KreatynaTests.swift` | `dispatchedEventReturnsThroughRenderCallback`: send a list search for "gar" and wait until `ingredients.list.search == "gar"`. `dispatchAfterShutdownIsVisible`: send the same search event. `createsStableDirectory`: expect `Kreatyna/kreatyna.sqlite`, and remove the comment at line 113 (Q2). |
| `KreatynaTests/RustLoggerTests.swift` | No change. `"counter"` there is only an example log target. |
| `KreatynaTests/Snapshots/ScreenSnapshotTests.swift` | Delete the `counter` test. Rename `runningIngredientsTab` to `runningCatalog`. Update both `Fixtures.running` calls. |
| `KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/` | Delete `counter-name-view.loading.png`, `counter-name-view.ready.png`, `runningIngredientsTab.1.png`. Re-record `adapterError.1.png` and record `runningCatalog.1.png` (both lose the tab bar). |
| `KreatynaUITests/KreatynaUITests.swift` | Delete `testTabsKeepTheirState`. |
| `KreatynaUITests/DemoCaptureUITests.swift` | Delete the Counter part (lines 100–105, `16-counter-tab`). |

Swift sketch for the root view (*sketch*):

```swift
case .running(let app):
    IngredientCatalogScreen(view: app.ingredients) { send(app.ingredients($0)) }
```

Snapshot review rule: only `adapterError` and `runningCatalog` may change.
Both must show the same catalog as before, without the tab bar. Every other
baseline must pass without re-recording. Baselines come from the iOS 26.3
simulator.

### tooling

| File | Change |
|---|---|
| `devenv.nix:84` | `DATABASE_URL` points to `crates/kreatyna-ffi/kreatyna-dev.sqlite` (Q2). |
| `crates/kreatyna-ffi/.gitignore` | Ignore `/kreatyna-dev.sqlite` and `/kreatyna-dev.sqlite-*` instead of the `counter-dev` names. |

### docs

| File | Change |
|---|---|
| `docs/epics/mac-shell/epic.html` | MAC-03 `status` and `plan`; MAC-10 and MAC-11 are marked approved in this change. |
| `docs/epics/mac-shell/tasks/MAC-03.md` | This plan. Validation after implementation. |
| `docs/epics/mac-shell/tasks/MAC-03-design.html` | Before and after design. |
| `docs/epics/mac-shell/tasks/MAC-03-demo/` | `demo.md` and three simulator screenshots. |
| `docs/epics/ingredient-catalog/R13-ffi-problems.md` | P07 (lines 41, 62, 140–146): mark it resolved by removal in MAC-03, with a pointer to the warning in `krucyfiks/examples/counter`. |

## Steps

Database:

1. Create the folder `crates/kreatyna-db/migrations/2026-10-11-200000-0000_drop_counter`.
2. Write `DROP TABLE counter;` in its `up.sql`. Leave its `down.sql` empty.
3. Remove the `counter` table from `crates/kreatyna-db/src/schema.rs`.
4. Rewrite the two migration tests in `ingredient_models/tests.rs`.
5. Remove the Counter calls from `ingredient_store/tests.rs`.

Rust core:

6. Reduce `crates/kreatyna/src/capabilities.rs` to `Render`.
7. Delete `capabilities/tests.rs`, `composability.rs`, and the three Counter test files.
8. Rewrite `crates/kreatyna/src/app.rs` as in the sketch.
9. Remove Counter from `crates/kreatyna/src/lib.rs`.
10. Rewrite `crates/kreatyna/tests/app.rs` to its 4 tests.
11. Remove unused dependencies from `crates/kreatyna/Cargo.toml`.
12. Remove the Counter store from `crates/kreatyna-db/src/sqlite.rs`.

FFI:

13. Delete `app_types/counter.rs`, and update `app_types.rs` and `app_types/app.rs`.
14. Remove `ClockHandler` and the `clock` field from `kreatyna-ffi/src/lib.rs`.
15. Remove `clock` and the counter client from `app_shell.rs`.
16. Update `app_shell/tests.rs`, `app_types/tests.rs` and `app_runtime_tests.rs`.
17. Run Rust checks 1–4.
18. Run `ios-bindings` (check 5) to regenerate the Swift bindings.

Swift:

19. Delete `CounterScreen.swift` and `AppClock.swift`.
20. Update `ContentView.swift` and `AppEventComposition.swift`.
21. Update `AppModel.swift` and `AppDatabaseLocation.swift`.
22. Update `Fixtures.swift`, `AdapterPlumbingTests.swift`, `KreatynaTests.swift` and `ScreenSnapshotTests.swift`.
23. Delete the three old PNG files and `adapterError.1.png`.
24. Delete `testTabsKeepTheirState` and the Counter part of `DemoCaptureUITests.swift`.

Tooling, checks and close:

25. Rename the scratch database in `devenv.nix` and `crates/kreatyna-ffi/.gitignore`.
26. Delete the local files `crates/kreatyna-ffi/counter-dev.sqlite*`. They are no longer
    ignored, so jj would otherwise record them in this change.
27. Run `ios-test` (check 6). It records the two new baselines.
28. Open both new PNG files and compare them with the design page.
29. Run `ios-test` again (check 7).
30. Capture demo screenshots of the catalog without the tab bar (check 8).
31. Run the search check (check 9).
32. Mark P07 as resolved in `R13-ffi-problems.md`.
33. Fill in Validation in this plan.
34. Set MAC-03 `status` to `awaiting-review` in the epic data object.
35. Report the change ID, then stop for review.

## Checks

Run all commands from the repository root, inside the devenv shell.

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. Counts in the table below. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --all --check` | No output. |
| 4 | `cargo check --workspace --all-targets` after step 11 | Builds without `async-stream`, `async-trait` and `tracing` in `kreatyna`. |
| 5 | `ios-bindings` | "Generated Swift bindings in …". The generated `kreatyna_ffi.swift` has no `Counter`, `AppTab` or `ClockHandler`. |
| 6 | `ios-test` (first run) | Swift Testing and UI tests pass. SnapshotTesting records `runningCatalog.1.png` and `adapterError.1.png` and reports them as new. |
| 7 | `ios-test` (second run) | All tests pass with no new recordings. |
| 8 | `KREATYNA_CAPTURE_DEMO=1` demo capture, per `document-ios-delivery` | Screenshots 01–15 of the catalog without the tab bar. No `16-counter-tab`. |
| 9 | `rg -n -i -e counter -e AppTab -e ClockHandler -e sleep_ms crates apple` | Only these hits: the old migration folder `create_counter`, the new `drop_counter` migration, the two migration tests, the log target strings in `kreatyna-ffi/src/logging/tests.rs` and `RustLoggerTests.swift`. |

Rust test counts, before (run on 2026-10-11) and expected after:

| Test binary | Before | After |
|---|---|---|
| `kreatyna` unit tests | 17 | 0 |
| `tests/app.rs` | 12 | 4 |
| `tests/background.rs` | 23 | removed |
| `tests/composition.rs` | 31 | removed |
| `tests/counter.rs` | 35 | removed |
| `tests/confirmation.rs` and the 7 `tests/ingredient_*.rs` | 17, 36, 36, 16, 39, 92, 46, 46 | unchanged |
| `kreatyna_db` unit tests | 29 | 29 |
| `kreatyna_ffi` unit tests | 30 | 28 |
| `kreatyna` doctests | 4 | 1 (the `Grams`/`Millilitres` compile-fail doctest) |

## Stubs and exclusions

No stubs.

This slice does not:

- add a Mac target, Mac views or a Mac database location (MAC-00, MAC-01);
- add sidebar sections or a section scope (REC-01);
- delete the old `create_counter` migration (forward-only policy);
- change the ingredient catalog widgets, their scopes, or their tests;
- change `IngredientCatalogScreen` or any catalog view;
- fix `crates/kreatyna-db/diesel.toml`, which names a wrong schema path
  (`src/database/schema.rs`). Nothing runs `diesel print-schema` today;
- rename `CLIENT_CAPABILITY` (it means channel capacity);
- edit historical documents that mention Counter: the ingredient-catalog
  plans, `ingredient-catalog/demo.md` and its `16-counter-tab.png`, the
  recipes brief, and the MAC-09 to MAC-11 plans;
- change the "counter arithmetic" example in the `build-swift-presentation`
  skill, or "the existing counter migration" in `forward-only-migrations`
  (that migration still exists);
- move, copy or delete the old `counter.sqlite` file (Q2).

## Validation

Run on 2026-10-12. Simulator: iPhone 17 Pro, iOS 26.3. Xcode in
`/Applications/Xcode.app`.

### Deviations from the plan

1. **The search field moved to the bottom.** The snapshot review (step 28)
   showed a second visual change, not only the missing tab bar. iOS 26 shows
   `.searchable` at the bottom of an iPhone screen that has no tab bar. With
   the tab bar, it was under the title. The existing list baselines
   (`list-name-status.*.png`) already render without a tab bar and already
   show the field at the bottom. So the new `runningCatalog.1.png` matches
   them, and no other baseline changed. The agent kept the system default and
   updated the design page and Outcome. To keep the field under the title
   instead, `IngredientListScreen` needs
   `placement: .navigationBarDrawer(displayMode: .always)`. That would also
   change the 6 list baselines.
2. **The agent's shell had an old devenv environment.** It predated MAC-10:
   `apple-generate` and `ios-test` were missing, and `ios-bindings` wrote to
   the old `target/generated/ios/...` folder. The agent ran the scripts through
   `devenv shell -- <script>`, which uses the current `devenv.nix`. Your
   shell is not affected if direnv reloads it.
3. **`AppEffect` needs the `krucyfiks::Effect` trait import** in
   `tests/app.rs` for `any_route()`. The old file imported it too.
4. **Two ingredient requests in one task.** `IngredientStoreClient` is not
   `Clone`. `admitted_requests_finish_after_close` sends both `list()`
   requests from one task with `futures::join!`.

### Checks

| # | Command | Result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. `kreatyna` unit 0, `app` 4, `confirmation` 17, `ingredient_catalog` 36, `ingredient_detail` 36, `ingredient_domain` 16, `ingredient_draft` 39, `ingredient_form` 92, `ingredient_list` 46, `ingredient_navigation` 46, `kreatyna_db` 29, `kreatyna_ffi` 28, `kreatyna` doctests 1. Matches the expected counts. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --all --check` | No output. |
| 4 | `cargo check --workspace --all-targets` | Builds without `async-stream`, `async-trait` and `tracing` in `kreatyna`. `Cargo.lock` only loses `async-stream` and `async-stream-impl`. |
| 5 | `devenv shell -- ios-bindings` | Bindings in `target/generated/apple/swift`. No `Counter`, `AppTab` or `ClockHandler` in `kreatyna_ffi.swift`. |
| 6 | `devenv shell -- ios-test <udid>` (first run) | 46 passed. `runningCatalog()` and `adapterError()` failed because they recorded new baselines, as expected. |
| — | Snapshot review | Old and new PNGs compared side by side. Changes: the tab bar is gone, and the search field moved to the bottom (deviation 1). The list rows, title, **+** and error text are unchanged. |
| 7 | `devenv shell -- ios-test <udid>` (second run) | `TEST SUCCEEDED`. 48 passed, `DemoCaptureUITests` skipped (as designed). |
| 8 | `DemoCaptureUITests` with `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1` | Passed. 15 screenshots, no `16-counter-tab`. Three are in `MAC-03-demo/`. |
| 9 | `rg -n -i -e counter -e AppTab -e ClockHandler -e sleep_ms crates apple` | Only the expected hits: the two migrations, the two migration tests, and the log target strings. |

### Not run

- The Mac app: it does not exist yet (MAC-00).
- An upgrade on a real device or an installed simulator app: the app now
  opens `kreatyna.sqlite`, so an old `counter.sqlite` is not opened (Q2). The
  Rust test `counter_database_upgrades_and_keeps_ingredients` covers the
  migration.
- The light theme of `MAC-03-design.html` was not rendered. No Chrome is
  installed, and Quick Look used the system's dark theme.
