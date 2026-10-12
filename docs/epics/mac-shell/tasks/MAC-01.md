# MAC-01 — The Mac window shows the ingredient catalog

Status: approved. Change: `wsstwmoz`. Dependencies: MAC-00 (approved).

## Open questions

None. The user answered MAC-01/Q1 and MAC-01/Q2 in review on 2026-10-12
(see Decisions).

## Decisions

### MAC-01/Q1 — A Rust function seeds ingredients for UI tests

*Context:* The MAC-01 card asks for a UI smoke test that sees a seeded row.
The Mac app cannot create ingredients until MAC-05 (**＋** is disabled). The
UI test process cannot write into the app's sandbox container, because macOS
protects other apps' containers. iOS UI tests create rows through the UI.

*Decision:* Option B. `kreatyna-ffi` exports
`seed_ingredients(database_path:names:)`. Rust opens and migrates the
database and saves each name through the existing SQLite store, with the
same validation as the form. In Debug builds, the shared `AppModel` calls it
before the core starts, when the launch environment has both
`KREATYNA_TEST_DATABASE_ID` and `KREATYNA_TEST_SEED` (for example
`Garlic,Tomato`). Later Mac slices (MAC-04 to MAC-08) can seed their UI tests
the same way. This is epic decision E-17.

Details (agent's design, within the decision):

- **Only names.** The seed creates name-only ingredients, like a form with
  only a name. MAC-04 can extend the input if its UI test needs notes or
  units ("clove ≈ 5 g").
- **Existing names are skipped,** so a relaunch with the same seed does not
  fail on `DuplicateName`. Rust owns this rule.
- **It ships in Release.** One bindings file serves Debug and Release. If the
  Release library had no `seed_ingredients` symbol, the Release link would
  fail. Swift calls it only in Debug.
- **Overlap.** The planned development RPC (`AGENTS.md`, "Live debugging
  direction") may replace this function later.
- **Seeding only with an isolated database.** Without
  `KREATYNA_TEST_DATABASE_ID`, Swift ignores `KREATYNA_TEST_SEED`. A seed can
  never write into your real database.

### MAC-01/Q2 — Tests never open the real database

*Context:* Mac tests run in the app's real sandbox container
`~/Library/Containers/pl.frondeus.Kreatyna/Data/`. Before MAC-01, the unit
test host would open and migrate the real database, and UI tests would leave
a `UITests/<UUID>/` folder next to it for each run.

*Decision:* Option A. In Debug builds only, the shared
`AppDatabaseLocation` changes:

- When the app runs as a unit-test host (the environment has
  `XCTestConfigurationFilePath`), the path is `:memory:`.
- With `KREATYNA_TEST_DATABASE_ID`, the database is in the container's
  temporary folder: `<NSTemporaryDirectory>/KreatynaUITests/<UUID>/`. It
  survives a relaunch. macOS deletes old temporary files by itself.

The same change applies to iOS, where it is harmless. This is epic decision
E-18.

### Design review — no emojis

The user asked to drop the emojis from
[MAC-01-design.html](MAC-01-design.html). The page now shows text only. The
Mac views follow the page: the sidebar item, the empty states, and the
"Select an ingredient" placeholder have no icons. The failure states keep no
icon either.

## Outcome

`mac-bindings debug` builds the Rust core for the Mac. `mac-test` then runs
the Mac tests. Opening the app starts the real Rust core with its own SQLite
database at
`~/Library/Containers/pl.frondeus.Kreatyna/Data/Library/Application Support/Kreatyna/kreatyna.sqlite`.
The window has three columns:

| Column | Content |
|---|---|
| Sidebar | One static item, **Ingredients**. |
| List | The catalog from Rust, for example "Canned tomatoes", "Cucumber", "Garlic". On a new Mac it says "No ingredients yet". |
| Detail | "Select an ingredient". |

The toolbar shows **＋**, disabled. If startup fails, for example because the
database cannot be opened, the window shows "Kreatyna could not start", the
reason, and **Retry**. The iOS app looks and works as before.

## Terms

- **Shared source folder:** `apple/Kreatyna/Shared/`. `project.yml` lists it
  in both app targets, so each app compiles the same files (E-12). It holds
  the Swift adapter over Rust, not views.
- **Swift adapter:** the Swift code between the views and Rust: `AppModel`
  (holds the Rust core and its latest view), the render callback, logging,
  the database path, and event wrapping. It has no business rules.
- **Bindings:** the files that UniFFI generates from the Rust library:
  `kreatyna_ffi.swift` and the C header `kreatyna_ffiFFI.h`. They are in
  `target/generated/apple/swift/`. Nobody edits them by hand.
- **Static library:** `libkreatyna_ffi.a`, the compiled Rust core. Xcode
  links it into the app. Each platform needs its own copy:
  `aarch64-apple-ios`, `aarch64-apple-ios-sim`, and now
  `aarch64-apple-darwin` (macOS on Apple Silicon).
- **`NavigationSplitView`:** the SwiftUI view for a Mac window with two or
  three columns: sidebar, content, and detail.
- **`Window` scene:** a SwiftUI scene with exactly one window. **File → New
  Window** does not exist, and the app quits when the window closes. A
  `WindowGroup` allows many windows.
- **Snapshot test:** a test that renders a view from hand-built view values
  and compares the image with a stored baseline PNG.
- **Test host:** the app that a unit-test bundle loads into. On the Mac, it
  is the real, sandboxed app.
- **Seed:** test data that exists before the app starts. Here: ingredients
  that `seed_ingredients` saves into an isolated test database (E-17).
- **Automation Mode:** the macOS mode that XCTest UI tests need. It is
  enabled without a password on this Mac since MAC-00/Q1.

## Design

[MAC-01-design.html](MAC-01-design.html) shows the target window in light
and dark mode: catalog with rows, empty catalog, list failure, and startup
failure. The layout follows the main window in
[mockups.html](../../../tasks/recipes/mockups.html#main), with only the
**Ingredients** section and no recipe items.

## Changes by layer

**db: no change.** The seed uses the existing schema and store.

### rust/ffi — `seed_ingredients` (MAC-01/Q1)

A new file `crates/kreatyna-ffi/src/seed.rs` with its tests in
`crates/kreatyna-ffi/src/seed/tests.rs` (*sketch*):

```rust
/// Saves name-only ingredients for UI tests and demos (E-17).
///
/// Rust opens and migrates the database, validates each name like the form
/// does, and skips names that already exist. Swift calls this only in Debug
/// builds, and only for an isolated test database.
#[uniffi::export]
pub fn seed_ingredients(database_path: String, names: Vec<String>) -> Result<(), SeedError> {
    RUNTIME.block_on(async move {
        let mut database = SqliteDatabase::new(&database_path).await?;
        for name in names {
            let mut draft = IngredientDraft::new(NutritionBasis::Per100Grams);
            draft.name = name.clone();
            let write = IngredientWrite::try_from(&draft)
                .map_err(|_| SeedError::InvalidName { name })?;
            match database.save(write).await {
                Ok(_) | Err(capabilities::IngredientSaveError::DuplicateName) => {}
                Err(error) => return Err(SeedError::Save { error: error.into() }),
            }
        }
        Ok(())
    })
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SeedError {
    #[error("database startup failed: {error:?}")]
    Startup { error: AppInitializationError },
    #[error("invalid ingredient name: {name}")]
    InvalidName { name: String },
    #[error("ingredient save failed: {error:?}")]
    Save { error: IngredientSaveError },
}
```

`AppInitializationError` and `IngredientSaveError` are the existing FFI
types; their `From` conversions exist. The function blocks the calling thread
for a few milliseconds. Swift calls it once, before the core starts.

Rust tests (*sketch*), with a temporary database file like the store tests in
`crates/kreatyna-db/src/ingredient_store/tests.rs`:

| Test | Checks |
|---|---|
| `seeds_names_into_new_database` | Seed `["Garlic", "Tomato"]`, reopen with `SqliteDatabase::new`, `list()` has both names. |
| `second_seed_skips_existing_names` | Seed `["Garlic"]` twice, then `["garlic", "Tomato"]`: 2 ingredients, no error. |
| `blank_name_is_rejected` | Seed `["  "]` returns `SeedError::InvalidName`. |
| `unopenable_path_reports_startup_error` | A path in a missing folder returns `SeedError::Startup { error: DatabaseOpen }`. |

The bindings change, so both `mac-bindings` and `ios-bindings` must run
after this change. Otherwise one platform's library has no
`seed_ingredients` symbol and its link fails.

### tooling — build the Rust library for macOS

`devenv.nix` gets a macOS variant of the existing iOS build. Today
`iosSetup` sets `IPHONEOS_DEPLOYMENT_TARGET`, and `iosBuild target sdk`
builds one target with Xcode's compiler and the SDK's SQLite (*real*):

```nix
iosBuild = target: sdk: iosSetup + ''
  export SDKROOT="$(/usr/bin/xcrun --sdk ${sdk} --show-sdk-path)"
  ...
  cargo build --locked -p kreatyna-ffi --lib --target ${target} "''${profile_args[@]}"
'';
```

The change (*sketch*):

| Name | Change |
|---|---|
| `iosSetup` → `appleSetup` | Also exports `MACOSX_DEPLOYMENT_TARGET="''${MACOSX_DEPLOYMENT_TARGET:-26.2}"`, the Mac target's value. |
| `iosBuild` → `appleBuild` | No logic change. It already works for any Apple target and SDK. |
| `generateBindings` (new `let`) | The UniFFI command that `ios-bindings` runs today, with the library path as a parameter. |
| `mac-build [debug\|release]` (new script) | `appleBuild "aarch64-apple-darwin" "macosx"`. |
| `mac-bindings [debug\|release]` (new script) | Runs `mac-build`, then `generateBindings` on `target/aarch64-apple-darwin/<profile>/libkreatyna_ffi.a`. |
| `ios-bindings` | Uses `generateBindings`. No behavior change. |

Both binding scripts write to the same `target/generated/apple/swift/`
folder. MAC-10 showed that the bindings are identical for the device and the
simulator libraries. Check 3 confirms the same for the macOS library.

Building with an explicit `--target aarch64-apple-darwin` puts the library in
`target/aarch64-apple-darwin/`. It does not touch `target/debug/`, which
`cargo test` uses.

### tooling — `apple/Kreatyna/project.yml`

| Where | Change |
|---|---|
| `Kreatyna` sources | Add `- path: Shared` (`type: syncedFolder`). |
| `KreatynaMac` sources | Add `Shared` and the generated `kreatyna_ffi.swift`, like the iOS target. |
| `KreatynaMac` settings | `SWIFT_OBJC_BRIDGING_HEADER` and `OTHER_LDFLAGS[sdk=macosx*]` (below). |
| `macOS` setting group | `EXCLUDED_ARCHS[sdk=macosx*]: x86_64`. |
| `KreatynaTests`, `KreatynaMacTests` sources | Add `- path: SharedTests` (`type: syncedFolder`). |
| `KreatynaMacTests` dependencies | Add `- package: SnapshotTesting`. |

Link settings (*sketch*, same pattern as iOS):

```yaml
SWIFT_OBJC_BRIDGING_HEADER: $(SRCROOT)/../../target/generated/apple/swift/kreatyna_ffiFFI.h
OTHER_LDFLAGS[sdk=macosx*]:
  - $(inherited)
  - $(SRCROOT)/../../target/aarch64-apple-darwin/$(RUST_PROFILE)/libkreatyna_ffi.a
  - -lsqlite3
```

**Why `x86_64` is excluded.** Xcode builds Mac Release apps for Apple Silicon
and Intel (MAC-00 check 7: `x86_64 arm64`). The Rust library is built for
Apple Silicon only, so the Intel link step would fail. The simulator already
has the same exclusion (MAC-10). Intel Macs are not a target of this epic.

**One shared folder in two targets.** The agent tested it on 2026-10-12 with
XcodeGen 2.44.1 in a throwaway project: both apps built and compiled the
shared file. XcodeGen writes two group objects for the same folder; one has
a `TEMP_…` identifier. Xcode accepts it, and the generated project is not
committed, so this has no effect on reviews.

### swift — shared adapter (`apple/Kreatyna/Shared/`)

These files move from `apple/Kreatyna/Kreatyna/`. Only `AppModel` and
`AppDatabaseLocation` change content:

| File | Content |
|---|---|
| `AppModel.swift` | `AppModel`, `AppRenderer`. Gains the Debug-only seed call (MAC-01/Q1). |
| `AppDatabaseLocation.swift` | The database path. Changed for MAC-01/Q2 (below). |
| `AppEventComposition.swift` | Scope wrapping of child events; `IngredientFormEvent.move`. |
| `ErrorText.swift` | User-facing text for Rust's typed errors. |
| `RustLogger.swift` | `RustLogger`, `RustLogging`. |
| `TestSeed.swift` (new) | Debug-only reading of `KREATYNA_TEST_SEED` (MAC-01/Q1). |
| `TextEcho.swift` (new) | `TextEcho`, moved out of `SyncedTextField.swift`. It is adapter logic, and MAC-08's search field needs it. `SyncedTextField` stays in the iOS folder. |

The logging subsystem stays `pl.frondeus.Kreatyna`, which is also the Mac
bundle identifier.

`AppDatabaseLocation.path` gets one more parameter, so tests can set the
environment (MAC-01/Q2, *sketch*):

```swift
static func path(
    fileManager: FileManager = .default,
    applicationSupportDirectory: URL? = nil,
    environment: [String: String] = ProcessInfo.processInfo.environment
) throws -> String {
    #if DEBUG
    if environment["XCTestConfigurationFilePath"] != nil { return ":memory:" }
    if let value = environment["KREATYNA_TEST_DATABASE_ID"], let id = UUID(uuidString: value) {
        // Next to, not inside, the real data. macOS cleans the temporary folder.
        let directory = fileManager.temporaryDirectory
            .appendingPathComponent("KreatynaUITests", isDirectory: true)
            .appendingPathComponent(id.uuidString, isDirectory: true)
        try fileManager.createDirectory(at: directory, withIntermediateDirectories: true)
        return directory.appendingPathComponent("kreatyna.sqlite").path
    }
    #endif
    // ... unchanged: Application Support/Kreatyna/kreatyna.sqlite
}
```

A new shared file `TestSeed.swift` reads the seed request, and `AppModel`
calls it after it has the path and before `app(...)` (MAC-01/Q1, *sketch*):

```swift
#if DEBUG
enum TestSeed {
    /// Names from KREATYNA_TEST_SEED, only for an isolated UI-test database.
    static func names(environment: [String: String] = ProcessInfo.processInfo.environment) -> [String] {
        guard environment["KREATYNA_TEST_DATABASE_ID"] != nil,
              let value = environment["KREATYNA_TEST_SEED"] else { return [] }
        return value.split(separator: ",").map(String.init)
    }
}
#endif

// In AppModel.init, after the path is known:
#if DEBUG
let seed = TestSeed.names()
if !seed.isEmpty {
    do { try seedIngredients(databasePath: path, names: seed) } catch {
        errorMessage = "Test data could not be added."
    }
}
#endif
```

Swift only splits the text. Rust validates and saves the names.

### swift — Mac views (`apple/Kreatyna/KreatynaMac/`)

| File | Content |
|---|---|
| `KreatynaMacApp.swift` | `Window("Kreatyna", id: "main") { ContentView() }`. Calls `RustLogging.start()` in `init`, like iOS. |
| `ContentView.swift` | Replaces the template. Owns `@State var model = AppModel()` and shows `MacRootView`. |
| `MacRootView.swift` | `.initializing` → "Starting…"; `.initializationFailed` → error and **Retry**; `.running` → `MacCatalogView`. Red adapter error text at the bottom. |
| `MacCatalogView.swift` | `NavigationSplitView`: static sidebar with a text-only **Ingredients** item, `MacIngredientList`, and the "Select an ingredient" detail placeholder. No icons (design review). |
| `MacIngredientList.swift` | The list column for each `IngredientListStatus`, and the disabled **＋** toolbar button. |

The `Mac` prefix keeps the Mac views easy to tell apart from the iOS views
with similar jobs (`RootView`, `IngredientCatalogScreen`).

**Why a `Window` scene.** A `WindowGroup` lets you open a second window. Each
window would create its own `AppModel`, so two Rust cores would use one
database file. Multiple windows are out of scope for the epic.

`MacIngredientList` (*sketch*). Rows are text only; MAC-04 makes them
selectable:

```swift
struct MacIngredientList: View {
    let view: IngredientListView
    let send: (IngredientListEvent) -> Void

    var body: some View {
        content
            .navigationTitle("Ingredients")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button("Add Ingredient", systemImage: "plus") {}
                        .disabled(true) // MAC-05
                        .accessibilityIdentifier("addIngredient")
                }
            }
    }

    @ViewBuilder private var content: some View {
        switch view.status {
        case .loading:
            ProgressView("Loading ingredients…")
        case .failed(let error):
            // "Ingredients unavailable", error.message, Retry → send(.refresh)
        case .ready(let rows, let totalCount, let refresh):
            List(rows, id: \.id) { row in
                Text(row.name).accessibilityIdentifier("ingredientRow")
            }
            // overlays (text only, no icons): "No ingredients yet" when totalCount == 0,
            // a search "no matches" view when rows are empty and totalCount > 0,
            // and the refresh failure row with Retry, like iOS.
        }
    }
}
```

Events: **Retry** in the list sends `send(view.list(.refresh))`, wrapped with
the displayed scope by the shared `AppEventComposition`. **Retry** after a
startup failure calls `model.retryInitialization()`. Swift sends nothing else
in MAC-01.

### swift — tests

**Shared adapter tests (`apple/Kreatyna/SharedTests/`).** These files move
from `KreatynaTests/` with no content change, so both platforms run them:
`KreatynaTests.swift` (`MainActorTests`, `AppModelTests`,
`AppDatabaseLocationTests`), `AdapterPlumbingTests.swift`,
`RustLoggerTests.swift`, `Fixtures.swift`. They already cover the card's
Swift Testing items:

| Card item | Existing test |
|---|---|
| Render callback reaches the main actor | `rendererDeliversBackgroundCallbackBeforeReturning`, `dispatchedEventReturnsThroughRenderCallback` |
| Startup failure and Retry are forwarded | `startupFailureRendersAndRetryIsForwarded` |

Both test targets import the app as `@testable import Kreatyna`. That works
on the Mac because MAC-00 named the Mac product `Kreatyna`.

`AppDatabaseLocationTests` gets two tests for MAC-01/Q2 (*sketch*):

```swift
@Test func unitTestHostUsesMemoryDatabase() throws {
    let path = try AppDatabaseLocation.path(environment: ["XCTestConfigurationFilePath": "/x"])
    #expect(path == ":memory:")
}

@Test func uiTestDatabaseIsInTemporaryFolder() throws {
    let id = UUID()
    let path = try AppDatabaseLocation.path(environment: ["KREATYNA_TEST_DATABASE_ID": id.uuidString])
    #expect(path.hasPrefix(FileManager.default.temporaryDirectory.path))
    #expect(path.hasSuffix("KreatynaUITests/\(id.uuidString)/kreatyna.sqlite"))
}
```

`createsStableDirectory` passes an empty environment, so it keeps testing
the Application Support path while it runs inside a test host.

A new `TestSeedTests` suite checks `TestSeed.names` (*sketch*):

| Test | Checks |
|---|---|
| `readsNamesForIsolatedDatabase` | Both variables set, `"Garlic,Tomato"` → `["Garlic", "Tomato"]`. |
| `ignoresSeedWithoutIsolatedDatabase` | Only `KREATYNA_TEST_SEED` set → `[]`. |

One real-core test, `seededIngredientAppearsInList`, seeds `["Garlic"]` into
a temporary database file, starts `AppModel`, and waits for a "Garlic" row.
It proves the Swift-to-Rust call and its effect; Rust tests own the rules.

**Mac snapshots (`KreatynaMacTests/Snapshots/MacScreenSnapshotTests.swift`).**
Fixed conditions: window size 960 × 600 points, US English, light and dark
appearance. One image per state and appearance, 6 images:

| Test | View |
|---|---|
| `catalogWithRows` | `MacRootView` running with `Fixtures.rows` |
| `emptyCatalog` | running, `.ready(rows: [], totalCount: 0, refresh: .idle)` |
| `startupFailure` | `.initializationFailed(error: .databaseOpen)` |

The tests build view values by hand from `Fixtures`; they do not start Rust.
Risk: an off-screen `NSHostingView` may draw the sidebar material wrong. If
the images show that, the helper puts the view in a real `NSWindow` before it
renders, as the iOS helper does with `drawHierarchyInKeyWindow`. The agent
records this in Validation.

**Mac UI tests (`KreatynaMacUITests/`).**

| Test | Checks |
|---|---|
| `testLaunchShowsSeededCatalog` (replaces MAC-00's `testLaunchShowsWindow`) | Launch with a new `KREATYNA_TEST_DATABASE_ID` and `KREATYNA_TEST_SEED=Garlic,Tomato`. Rows "Garlic" and "Tomato" exist, and `addIngredient` is not enabled. |
| `testLaunchShowsEmptyCatalog` | Launch with a new `KREATYNA_TEST_DATABASE_ID` and no seed. `ingredientListEmpty` exists. |
| `MacDemoCaptureUITests` | Skipped unless `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1`, like iOS. Attaches window screenshots: seeded catalog (light), seeded catalog (dark, launch argument `-AppleInterfaceStyle Dark`), empty catalog. |

The MAC-00 test `SandboxTests` stays in `KreatynaMacTests`.

### docs

| File | Change |
|---|---|
| `docs/epics/mac-shell/tasks/MAC-01-demo/demo.md` (new) | Screenshots of the running Mac window, macOS version, window size, and the checks. |
| `.agents/skills/build-swift-presentation/SKILL.md` | "`mac-bindings debug` rebuilds the Rust core for the Mac." Mention `Shared/` and `SharedTests/`. |
| `apple/Kreatyna/project.yml:114` comment | "Generated by `ios-bindings` or `mac-bindings`". |
| `docs/epics/mac-shell/epic.html` | MAC-01 status and plan. New epic decisions E-17 (test seeding) and E-18 (tests never open the real database). |

References found with
`rg -n --hidden "ios-bindings|ios-build-simulator|ios-build-device|ios-test|mac-test"`
(2026-10-12, without `target/`, plan files and the ingredient-catalog epic):

| File | Change in MAC-01 |
|---|---|
| `devenv.nix:93-113` | Renamed `let` bindings; new scripts. `ios-*` scripts keep their names. |
| `apple/Kreatyna/project.yml:114` | Comment only. |
| `.agents/skills/build-swift-presentation/SKILL.md:56-58` | Add `mac-bindings`. |
| `.agents/skills/run-ios-simulator/SKILL.md`, `scripts/run-in-simulator.sh` | None. `ios-bindings` keeps its name and behavior. |

## Steps

1. Run `ios-bindings debug` and `ios-test`, and note the counts (48 passed, 1 skipped expected).
2. In `devenv.nix`, rename `iosSetup` to `appleSetup` and `iosBuild` to `appleBuild`.
3. Add `MACOSX_DEPLOYMENT_TARGET` to `appleSetup`.
4. Add `generateBindings`, `mac-build`, and `mac-bindings`. Make `ios-bindings` use `generateBindings`.
5. Run `direnv reload`.
6. Write the four `seed_ingredients` tests in `crates/kreatyna-ffi/src/seed/tests.rs`.
7. Write `crates/kreatyna-ffi/src/seed.rs` until the tests pass.
8. Run `cargo fmt`, `cargo clippy --workspace --all-targets`, and `cargo test --workspace`.
9. Move the five adapter files to `apple/Kreatyna/Shared/` with `mv`.
10. Change `AppDatabaseLocation` for MAC-01/Q2.
11. Move `TextEcho` from `SyncedTextField.swift` to `Shared/TextEcho.swift`.
12. Add `Shared/TestSeed.swift` and the seed call in `AppModel`.
13. Move the four test files to `apple/Kreatyna/SharedTests/`.
14. Add the Q2 tests, `TestSeedTests`, and `seededIngredientAppearsInList`.
15. Edit `project.yml` as in "Changes by layer".
16. Run `apple-generate`.
17. Run `ios-bindings debug`, then `ios-test`. Expect the step 1 counts plus the new shared tests.
18. Run `mac-bindings debug`.
19. Write `KreatynaMacApp.swift` and `ContentView.swift`.
20. Write `MacRootView.swift`, `MacCatalogView.swift`, and `MacIngredientList.swift`.
21. Build the Mac scheme and open the app once to see the real window.
22. Add `MacScreenSnapshotTests` and record the 6 baselines.
23. Look at each new image in light and dark. Fix the views or the helper until they match the design page.
24. Run `mac-test` again, so the snapshots compare against the baselines.
25. Replace `testLaunchShowsWindow` with the two UI smoke tests.
26. Add `MacDemoCaptureUITests`.
27. Capture the demo and export the screenshots to `MAC-01-demo/`.
28. Write `MAC-01-demo/demo.md`.
29. Update the skill, the `project.yml` comment, and the epic page (status, E-17, E-18).
30. Run all checks.
31. Set MAC-01 to `awaiting-review` and fill in **Validation**.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. MAC-03's counts, plus 4 tests in `kreatyna_ffi` (28 → 32). |
| 1a | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` | No output, no warnings. |
| 2 | `mac-bindings debug` | Prints "Built …/aarch64-apple-darwin/debug/libkreatyna_ffi.a" and "Generated Swift bindings". |
| 3 | Copy `target/generated/apple/swift`, run `ios-bindings debug`, `diff -r` both | No difference. |
| 4 | `xcodebuild -list -project apple/Kreatyna/Kreatyna.xcodeproj` | Same 6 targets and 2 schemes as MAC-00. |
| 5 | `mac-test` | `TEST SUCCEEDED`. Shared adapter tests, `SandboxTests`, 6 snapshots, and the 2 UI smoke tests pass. The demo test is skipped. |
| 6 | `ios-test <udid>` | `TEST SUCCEEDED`. 48 passed + the new shared tests, 1 skipped. No iOS snapshot changed. |
| 7 | `mac-build release`, then `xcodebuild build … -scheme KreatynaMac -configuration Release` in the clean environment | `BUILD SUCCEEDED`. `lipo -archs` prints `arm64`. |
| 8 | `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1 xcodebuild test … -only-testing:KreatynaMacUITests/MacDemoCaptureUITests` | Passes. Screenshots are in the result bundle. |
| 9 | `open target/xcode/DerivedData/Build/Products/Debug/Kreatyna.app` | The window shows the three columns with an empty catalog (a new real container). The user confirms by looking. |
| 9a | `ls ~/Library/Containers/pl.frondeus.Kreatyna/Data/Library/Application\ Support/Kreatyna/` after `mac-test` | Only `kreatyna.sqlite` from check 9, if any. No `UITests` folder. |
| 10 | `jj diff --summary` | Adapter and test files show as moves (delete + add with the same content). No generated file is in the change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| The sidebar has one static **Ingredients** item with no selection logic. | REC-01 (Rust-owned sidebar sections). |
| The detail pane always says "Select an ingredient". Rows are not selectable. | MAC-04. |
| **＋** is disabled. The empty catalog says "Ingredients you add appear here." | MAC-05. |
| The seed creates name-only ingredients. | MAC-04, if its UI test needs details. |
| `seed_ingredients` is test tooling in the shipped library. | The development RPC, when it is designed (`AGENTS.md`). |
| No search field. | MAC-08. |
| No `run-macos-app` skill. The agent runs the app with `open`. | MAC-02. |
| No Intel (`x86_64`) Mac build. | Not planned. |
| `cargo` does not build the macOS library from `mac-test`; run `mac-bindings` first, as on iOS. | Not planned. |
| No app icon images. | Not planned. |
| One window only; no multiple windows. | Out of scope for the epic. |

## Validation

Run on 2026-10-12. Mac: macOS 27.0.1, Xcode 27.0 (27A266a), Apple Silicon,
Automation Mode without a prompt (MAC-00/Q1). Simulator: iPhone 17 Pro
(`BB6851F0-…`), booted by the agent without the Simulator window.

### Deviations from the plan

1. **A Debug-only sandbox exception for Mac snapshots.** The first snapshot
   run failed: "You don't have permission to save the file
   “MacScreenSnapshotTests” in the folder “__Snapshots__”". The unit-test
   host is the sandboxed app, so it cannot read or write the repository. The
   new file `apple/Kreatyna/Config/KreatynaMac-Debug.entitlements` allows
   read and write access to `$(SRCROOT)/KreatynaMacTests/Snapshots/` only.
   `project.yml` uses it only in the Debug configuration
   (`CODE_SIGN_ENTITLEMENTS`). The Release app does not have it (check 7).
   The sandbox stays on.
2. **Snapshots render in an off-screen window.** An `NSHostingView` alone drew
   no list and no sidebar. The helper now puts the view in an `NSWindow` at
   (-10 000, -10 000) and captures the window's frame view, so the title bar
   and toolbar are included. The window is never key, so controls show their
   inactive look (for example, a gray **Retry**).
3. **Catalog snapshots hide the sidebar.** The sidebar material draws only
   through the window server, so it was blank in the captures (white in dark
   mode). `MacCatalogView` gets an `init` parameter `columnVisibility`
   (default `.all`, kept in `@State`, so the user can still toggle the
   sidebar). The snapshots pass `.doubleColumn`. The demo screenshots show
   the real sidebar.
4. **Window size.** The startup-failure window first shrank to its content.
   The `Window` scene now has `.defaultSize(width: 960, height: 600)` and a
   minimum of 800 × 500 points. The snapshot helper gives the view a fixed
   960 × 600 frame. Images are 1920 × 1304 pixels with a toolbar and
   1920 × 1264 without one (content plus title bar, at 2x).
5. **Demo in the system appearance only.** The launch argument
   `-AppleInterfaceStyle Light` did not change the appearance on macOS 27, so
   the demo has 2 screenshots in this Mac's dark mode instead of 3. Light mode
   is in the snapshots.
6. **"No matches" text.** `ContentUnavailableView.search` shows an icon. The
   Mac list uses a text-only "No matches for “…”" (design review: no icons).
   MAC-08 shows it for the first time.
7. **Review fix: temporary databases use `tempfile`.** Code review found
   that the seed tests used `IngredientId::new()` only to make a unique file
   name. The same pattern existed in 4 older helpers. By the user's choice,
   all 5 now use `tempfile::tempdir()` (a dev-dependency of `kreatyna-db` and
   `kreatyna-ffi`; version 3.27.0 was already in `Cargo.lock` through UniFFI):
   `kreatyna-db/src/ingredient_store/tests.rs`,
   `kreatyna-ffi/src/app_runtime_tests.rs` (`TemporaryDatabase`,
   `RecoverableDatabase`), `kreatyna-ffi/src/app_shell/tests.rs`, and
   `kreatyna-ffi/src/seed/tests.rs`. The folder is deleted on drop, so the
   hand-written `Drop` implementations are gone. Rust test counts did not
   change.
8. **Shared test count.** The plan expected 2 new shared tests for Q2; there
   are 5 (2 for Q2, 2 `TestSeedTests`, and `seededIngredientAppearsInList`).

### Checks

| # | Command | Result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass: `app` 4, `confirmation` 17, `ingredient_catalog` 36, `ingredient_detail` 36, `ingredient_domain` 16, `ingredient_draft` 39, `ingredient_form` 92, `ingredient_list` 46, `ingredient_navigation` 46, `kreatyna_db` 29, `kreatyna_ffi` 32 (was 28), doctests 1. |
| 1a | `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` | Clean, after `cargo fmt` rewrapped one line in `seed.rs`. |
| 1b | Checks 1 and 1a again after the review fix (deviation 7) | Same counts, all pass. Clean. |
| 2 | `mac-bindings debug` | Built `target/aarch64-apple-darwin/debug/libkreatyna_ffi.a` (`arm64`), generated the bindings. |
| 3 | Bindings from `mac-bindings`, then from `ios-bindings`, `diff -r` | Identical. Both contain `seedIngredients`. |
| 4 | `xcodebuild -list` | Same 6 targets and 2 schemes as MAC-00. |
| 5 | `mac-test` | `TEST SUCCEEDED`. 32 tests: 31 passed, 1 skipped (`testCaptureMacWindowDemo`, as designed). Includes the shared adapter tests, `SandboxTests`, the 6 snapshots, and both UI smoke tests. |
| 6 | `ios-test BB6851F0-…` | `TEST SUCCEEDED`. 53 passed (48 + 5 new shared tests), 1 skipped. No iOS snapshot changed. |
| 7 | `mac-build release`, then `xcodebuild build … -configuration Release` | `BUILD SUCCEEDED`. `lipo -archs`: `arm64`. No snapshot entitlement in the Release signature. |
| 8 | `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1 xcodebuild test … MacDemoCaptureUITests` | Passed. 2 screenshots in `MAC-01-demo/`. |
| 9 | `open …/Debug/Kreatyna.app` | The app runs. It created `kreatyna.sqlite` in the real container's `Application Support/Kreatyna/`. The agent cannot see the screen; the demo screenshots show the same build. |
| 9a | `ls` of the real container after `mac-test` | Before check 9: no `Kreatyna` folder at all. After check 9: only `kreatyna.sqlite`. No `UITests` folder. |
| 10 | `jj diff --summary` | Moved files show as renames (`R`). `AppDatabaseLocation.swift` shows as delete + add, because most of its content changed. No generated file is in the change. |

### Snapshot review

The agent looked at all 6 new baselines. The rows, the empty state, the
detail placeholder, the title, the disabled **＋**, and the startup failure
with **Retry** match the design page in light and dark, apart from
deviations 2 and 3.
