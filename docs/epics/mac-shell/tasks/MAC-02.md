# MAC-02 — Run and demo the Mac app from the terminal

Status: approved. Change: `yvxqwqyy`. Dependencies: MAC-01 (approved).

## Open questions

None. The user answered MAC-02/Q1 to MAC-02/Q4 in review on 2026-10-12
(see Decisions).

## Decisions

### MAC-02/Q1 — The run script checks the window and takes no screenshot

*Context:* A screenshot of a Mac window needs the Screen Recording
permission. On this Mac (2026-10-12), the agent's shell does not have it
(`CGPreflightScreenCaptureAccess()` returned `false`).
`CGWindowListCopyWindowInfo` still gives the owner process ID, the layer, and
the size of each window, with no permission.

*Decision:* Option A (Looks good). The script waits until the app process has
an on-screen window. It prints the process ID and the window size, for
example `960 × 600`. The agent cannot see the window content. The UI tests
and the demo capture show the content, because XCTest takes screenshots with
Automation Mode. A screenshot with the permission can come later with no
change to the script interface.

### MAC-02/Q2 — Seeds are JSON text, and the run script reads a JSON file

*Context:* Before MAC-02, `KREATYNA_TEST_SEED` was a list of names with
commas, and Rust saved names only (E-17). MAC-04 shows notes, nutrition,
and reference units, so its tests and demo need full data. The user asked in
review: "can we actually seed from json file?" The sandboxed app cannot read
a file in the repository. `open --env` and the UI test's `launchEnvironment`
pass text to the app, up to about 1 MB for all arguments and the environment
together.

*Decision:* Option A (Looks good). In MAC-02:

- `seed_ingredients(database_path, json)` takes JSON text in place of names.
  This changes E-17.
- The script reads the file and passes its text in `KREATYNA_TEST_SEED`.
  Swift passes the text to Rust and does not parse it.
- Rust parses each item into an `IngredientDraft` and validates it with the
  rules of the ingredient form.
- An item is a name, or an object with the fields of the form.
- A new fixture `apple/Kreatyna/Fixtures/demo-ingredients.json` has full
  data.
- The slice's layers become ffi, swift, and tooling.

The other options were: B, names now and JSON in MAC-04; C, names now and
JSON as a new slice MAC-12.

### MAC-02/Q3 — `document-ios-delivery` becomes `document-apple-delivery`

*Decision:* Option B (Looks good). One skill with shared rules and a section
for each platform. The 2 links change:
`.agents/skills/plan-reviewable-epics/SKILL.md` line 134 and
`docs/epics/ingredient-catalog.md` line 198. Old plans keep the plain-text
name `document-ios-delivery`, because it records what happened then.

### MAC-02/Q4 — Every project skill gets a symlink

*Decision:* Option B (Looks good). `.claude/skills/` gets 7 new symlinks, so
all 11 skills are available by name: `build-swift-presentation`,
`document-apple-delivery`, `forward-only-migrations`,
`model-effectful-initialization`, `run-macos-app`, `use-of-jj-vcs`, and
`write-widget-tests`.

## Outcome

From the repository root, inside devenv, you run:

```sh
.agents/skills/run-macos-app/scripts/run-mac-app.sh --seed apple/Kreatyna/Fixtures/demo-ingredients.json
```

The script builds the Rust core and the Mac app. It quits a running copy of
the app, then opens the new build. The Kreatyna window comes to the front and
shows Canned tomatoes, Cucumber, Garlic, Olive oil, and Tomato. Rust saved
Garlic with its nutrition and the unit "clove ≈ 5 g". MAC-04 shows these
fields when you click the row. The script prints:

```text
==> Kreatyna is running (pid 51234) on macOS 27.0.1
    window: 960 × 600
    database: isolated, seeded from demo-ingredients.json (5 items)
```

Without `--seed`, the window shows your real database.

For a slice demo, the agent runs one command. It runs the slice's capture UI
test and saves its screenshots as PNG files in the demo folder:

```sh
.agents/skills/document-apple-delivery/scripts/capture-demo.sh mac \
  KreatynaMacUITests/MacDemoCaptureUITests docs/epics/mac-shell/tasks/MAC-01-demo
```

## Terms

- **Skill:** a folder in `.agents/skills/` with a `SKILL.md` file. An agent
  reads it before a task of that kind. Claude Code finds it through a
  symlink in `.claude/skills/`.
- **Sandbox container:** the folder where a sandboxed Mac app keeps its files:
  `~/Library/Containers/pl.frondeus.Kreatyna/Data/`.
- **`open`:** the macOS command that starts an app like a double-click does.
  `open --env A=b` adds an environment variable for that launch.
- **Screen Recording permission:** a macOS privacy permission. Without it, a
  program cannot read the pixels of other apps' windows.
- **Automation Mode:** the macOS mode that lets XCTest control the desktop
  and take screenshots. It is on without a prompt since MAC-00/Q1.
- **Result bundle (`.xcresult`):** the folder where `xcodebuild test` saves
  results and attachments. `xcrun xcresulttool export attachments` copies the
  attachments out, with a `manifest.json` that lists their names.
- **Capture test:** a UI test that only takes demo screenshots. It is skipped
  unless `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1` is set. iOS has
  `DemoCaptureUITests`, and the Mac has `MacDemoCaptureUITests`.
- **Seed:** ingredients that Rust saves into an isolated test database before
  the app starts (E-17). Only Debug builds seed.
- **serde, serde_json:** the standard Rust libraries that read JSON into Rust
  structs. `#[derive(Deserialize)]` generates the reader for a struct.

## Design

No UI change. The app looks as in MAC-01
([demo](MAC-01-demo/demo.md)).

## Changes by layer

**db, rust (`kreatyna`): no change.**

### ffi — `crates/kreatyna-ffi/src/seed.rs`

`Cargo.toml` adds `serde = { features = ["derive"] }` and `serde_json` to
`[dependencies]`.

The seed format (*sketch*). An item is a name, or an object. Unknown fields
are an error, so a typo such as `"calorie"` fails. Numbers are JSON numbers.
Rust writes each number as text into the draft, and the form's rules check
it. `basis` is `"100g"` (default) or `"100ml"`.

```json
[
  "Cucumber",
  {
    "name": "Garlic",
    "notes": "Peel the cloves.",
    "calories": 149, "protein": 6.4, "carbohydrates": 33, "fat": 0.5,
    "units": [{ "name": "clove", "grams": 5 }]
  },
  {
    "name": "Olive oil", "basis": "100ml", "calories": 820, "fat": 91,
    "units": [{ "name": "tablespoon", "millilitres": 15 }]
  }
]
```

```rust
// sketch
#[uniffi::export]
pub fn seed_ingredients(database_path: String, json: String) -> Result<(), SeedError>

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SeedError {
    #[error("database startup failed: {error:?}")]
    Startup { error: AppInitializationError },
    /// The text is not a JSON array of names and ingredient objects.
    #[error("invalid seed JSON: {message}")]
    InvalidJson { message: String },          // new
    /// Item `index` (from 0) does not pass the ingredient form's rules.
    #[error("invalid ingredient {index}: {name:?}")]
    InvalidIngredient { index: u32, name: String }, // was InvalidName { name }
    #[error("ingredient save failed: {error:?}")]
    Save { error: IngredientSaveError },
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SeedIngredient {
    name: String,
    #[serde(default)] notes: String,
    #[serde(default)] basis: SeedBasis,   // "100g" | "100ml"
    calories: Option<serde_json::Number>,
    protein: Option<serde_json::Number>,
    carbohydrates: Option<serde_json::Number>,
    fat: Option<serde_json::Number>,
    #[serde(default)] units: Vec<SeedUnit>, // name, grams?, millilitres?
}

impl From<SeedIngredient> for IngredientDraft { /* numbers → text */ }
```

Rust parses the text to a `serde_json::Value` array first. Then it reads each
item alone. So an error names the item: `item 2: unknown field "calorie"`.
Rust parses all items before it opens the database. A bad item saves
nothing. Names that already exist are skipped, as before.

Tests in `seed/tests.rs` (*sketch*):

| Test | Checks |
|---|---|
| `seeds_names_into_new_database` | `["Garlic", "Tomato"]` saves 2 names (was a `Vec`). |
| `seeds_all_fields` | Garlic's notes, nutrition, basis, and "clove = 5 g" read back from the store. |
| `seeds_millilitre_basis` | Olive oil has `Per100Millilitres` and "tablespoon = 15 ml". |
| `second_seed_skips_existing_names` | Unchanged, with JSON. |
| `blank_name_is_rejected` | `InvalidIngredient { index: 0, name: "  " }`. |
| `invalid_number_is_rejected` | `"protein": -1` gives `InvalidIngredient { index: 0, … }`. |
| `invalid_json_is_rejected` | `[`, `{}`, and `["Garlic", 5]` give `InvalidJson`. Nothing is saved. |
| `unknown_field_is_rejected` | `{"name": "Garlic", "calorie": 1}` gives `InvalidJson`, with `item 0` and `calorie` in the message. |
| `demo_fixture_is_valid` | `include_str!` of `apple/Kreatyna/Fixtures/demo-ingredients.json` seeds 5 ingredients. The fixture cannot go stale. |
| `unopenable_path_reports_startup_error` | Unchanged. |

### swift

| File | Change |
|---|---|
| `Shared/TestSeed.swift` | `names(environment:) -> [String]` becomes `json(environment:) -> String?`. It returns the text of `KREATYNA_TEST_SEED` only when `KREATYNA_TEST_DATABASE_ID` is set. |
| `Shared/AppModel.swift` | Calls `seedIngredients(databasePath:json:)`. On an error, it also logs Rust's message with `Logger` (subsystem `pl.frondeus.Kreatyna`, category `seed`). The window text stays "Test data could not be added." |
| `SharedTests/KreatynaTests.swift` | `seededIngredientAppearsInList` passes `#"["Garlic"]"#`. `TestSeedTests` checks the text, and that no text is returned without a database ID. |
| `KreatynaMacUITests/KreatynaMacUITests.swift`, `MacDemoCaptureUITests.swift` | `launch(seed:)` keeps its `[String]` parameter. It encodes the names as a JSON array with `JSONEncoder`. |
| `Fixtures/demo-ingredients.json` (new) | Garlic, Tomato, Canned tomatoes, Cucumber (per 100 g), and Olive oil (per 100 ml), with notes, nutrition, and units. Approximate values. Not part of any Xcode target. |

```swift
// sketch — Shared/TestSeed.swift
static func json(environment: [String: String] = ProcessInfo.processInfo.environment) -> String? {
    guard environment["KREATYNA_TEST_DATABASE_ID"] != nil else { return nil }
    return environment["KREATYNA_TEST_SEED"]
}
```

### tooling — new skill `.agents/skills/run-macos-app/`

| File | Content |
|---|---|
| `SKILL.md` | When to use the skill, the steps of the script, how to read its output, troubleshooting. Same form as `run-ios-simulator`. |
| `scripts/run-mac-app.sh` | The run script. |
| `scripts/window-info.swift` | Prints the size of the on-screen windows of one process ID. |

`run-mac-app.sh [--skip-rust] [--seed FILE.json]` does these steps:

1. With `--seed`, it checks that the file is JSON (`jq -e 'type == "array"'`).
   Rust checks the content when the app starts.
2. Unless `--skip-rust`, it runs `mac-bindings debug`. Xcode links the
   prebuilt Rust library and has no cargo build phase.
3. It runs `apple-generate`.
4. It builds the `KreatynaMac` scheme in Debug, with `xcodebuild` in a
   minimal environment. It uses DerivedData in `target/xcode/DerivedData`,
   like `mac-test`. The log goes to `target/xcode/mac-build.log`.
5. It quits a running Mac copy of Kreatyna. It finds the copy by the path
   `Kreatyna.app/Contents/MacOS/Kreatyna`. iOS simulator apps have no
   `Contents/MacOS` in their path, so the script never stops them.
6. It opens `target/xcode/DerivedData/Build/Products/Debug/Kreatyna.app`.
   With `--seed`, it adds the two variables:

   ```sh
   # sketch
   open_args=()
   if [ -n "$seed_file" ]; then
     open_args+=(--env "KREATYNA_TEST_DATABASE_ID=$(uuidgen)"
                 --env "KREATYNA_TEST_SEED=$(cat "$seed_file")")
   fi
   open "${open_args[@]}" "$app"
   ```

7. It waits at most 15 seconds for the process and an on-screen window
   (layer 0). Then it prints the process ID, the macOS version
   (`sw_vers -productVersion`), the window size, and the database. If no
   window appears, it prints the app's log from the last minute
   (`log show --predicate 'subsystem == "pl.frondeus.Kreatyna"' --last 1m`)
   and exits with status 1.

The `SKILL.md` says: if the window shows "Test data could not be added.",
read Rust's message with `log show --predicate 'subsystem ==
"pl.frondeus.Kreatyna" AND category == "seed"' --last 5m`.

`window-info.swift` uses `CGWindowListCopyWindowInfo`. It reads only the
owner process ID, the layer, and the bounds. These need no permission
(MAC-02/Q1):

```swift
// sketch
let pid = Int32(CommandLine.arguments[1])!
let windows = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements],
                                         kCGNullWindowID) as? [[String: Any]] ?? []
for window in windows where window[kCGWindowOwnerPID as String] as? Int32 == pid
    && window[kCGWindowLayer as String] as? Int == 0 {
    let bounds = window[kCGWindowBounds as String] as! [String: Double]
    print("\(Int(bounds["Width"]!)) × \(Int(bounds["Height"]!))")
}
```

### tooling — `document-ios-delivery` becomes `document-apple-delivery`

Rename the folder (MAC-02/Q3). Then rewrite `SKILL.md`:

- **Shared rules** (from today's text): choose scenarios from the
  acceptance, use controlled data, label fixtures, never show a mockup as a
  capture, `demo.md` with start, action, and result, record checks, read
  every image.
- **Data:** seed with JSON (E-17). A demo that needs full ingredient data
  uses `apple/Kreatyna/Fixtures/` or its own fixture there.
- **iOS:** state the simulator device and the iOS version. Capture with
  `DemoCaptureUITests`.
- **Mac:** state the macOS version, the window size in points, and the
  appearance (light or dark). Capture with `MacDemoCaptureUITests`. The
  window uses the Mac's system appearance; the snapshot tests cover the other
  appearance (MAC-01 deviation). A failure that the app cannot produce on
  demand goes under "Not captured from the running app", with the snapshot or
  test that covers it.
- **Capture:** each slice adds a test method to the capture test class. The
  method attaches window screenshots with the file names of the demo
  (`01-catalog`). `capture-demo.sh` exports them.

New `scripts/capture-demo.sh <mac|ios> <test identifier> <output folder> [--udid UDID]`:

1. It runs `apple-generate`.
2. It runs `xcodebuild test -only-testing:<test identifier>` with
   `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1`, in a minimal environment. It saves
   the result bundle at `target/xcode/demo.xcresult` and deletes an old one
   first. `mac` uses the `KreatynaMac` scheme on this Mac. `ios` uses the
   `Kreatyna` scheme on the booted simulator or on `--udid`.
3. It runs `xcrun xcresulttool export attachments` into
   `target/xcode/demo-attachments/`.
4. It reads `manifest.json` with `jq`. It copies each attachment to
   `<output folder>/<attachment name>.png`.
5. It prints each copied file.

The script does not build Rust, like `mac-test` and `ios-test`. The
`SKILL.md` says to run `mac-bindings` or `ios-bindings` first.

### tooling — `.claude/skills/`

Add symlinks `../../.agents/skills/<name>` (MAC-02/Q4) for the 7 missing
skills: `build-swift-presentation`, `document-apple-delivery`,
`forward-only-migrations`, `model-effectful-initialization`, `run-macos-app`,
`use-of-jj-vcs`, and `write-widget-tests`.

### docs

| File | Change |
|---|---|
| `.agents/skills/plan-reviewable-epics/SKILL.md` line 134 | Link `document-apple-delivery`. |
| `docs/epics/ingredient-catalog.md` line 198 | Link `document-apple-delivery`. |
| `apple/Kreatyna/KreatynaMacUITests/MacDemoCaptureUITests.swift` doc comment | Name `capture-demo.sh` as the way to run it. |
| `apple/Kreatyna/KreatynaUITests/DemoCaptureUITests.swift` doc comment | The same. Only the comment changes. |
| `docs/epics/mac-shell/tasks/MAC-01-demo/demo.md` "To capture again" | Use `capture-demo.sh`. |
| `docs/epics/mac-shell/epic.html` | E-17 text (JSON seeds). MAC-02 layers, outcome, acceptance, and status. |

Old plans keep their plain-text name `document-ios-delivery`.

## Steps

1. Add `serde` and `serde_json` to `crates/kreatyna-ffi/Cargo.toml`.
2. Change `seed_ingredients` and `SeedError` in `seed.rs`.
3. Update and add the Rust tests in `seed/tests.rs`.
4. Create `apple/Kreatyna/Fixtures/demo-ingredients.json`.
5. Run the Rust checks 1–2.
6. Regenerate the bindings with `mac-bindings debug` and `ios-bindings debug`.
7. Change `TestSeed.swift` and `AppModel.swift`.
8. Update the Swift tests and the 2 Mac UI test files.
9. Create `window-info.swift` in `.agents/skills/run-macos-app/scripts/`.
10. Create `run-mac-app.sh` there, with `--skip-rust` and `--seed`.
11. Make the script executable.
12. Write `.agents/skills/run-macos-app/SKILL.md`.
13. Rename `.agents/skills/document-ios-delivery/` to
    `.agents/skills/document-apple-delivery/`.
14. Rewrite its `SKILL.md`.
15. Create `scripts/capture-demo.sh` in that skill, and make it executable.
16. Add the 7 symlinks in `.claude/skills/`.
17. Update the docs from the docs table.
18. Run the checks 3–15.
19. Set MAC-02 to `awaiting-review` in `epic.html`.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test -p kreatyna-ffi` | All tests pass, with the new seed tests. |
| 2 | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` | Clean. |
| 3 | `devenv shell -- mac-test` | All Mac tests pass. |
| 4 | `devenv shell -- ios-test` on the booted iPhone 17 Pro | All iOS tests pass. |
| 5 | `bash -n` on the 2 new scripts | No syntax errors. |
| 6 | `devenv shell -- .agents/skills/run-macos-app/scripts/run-mac-app.sh` | Builds. Opens the app with the real database. Prints the process ID, macOS 27.0.1, a window size, and `database: real`. |
| 7 | Run check 6 again while the app runs | The old process ends. A new process ID is printed. Only one Mac Kreatyna process runs (`pgrep -f Kreatyna.app/Contents/MacOS/Kreatyna`). |
| 8 | `run-mac-app.sh --skip-rust --seed apple/Kreatyna/Fixtures/demo-ingredients.json` | Opens the app. Prints `seeded from demo-ingredients.json (5 items)`. The real `kreatyna.sqlite` has an unchanged modification time. No `seed` error in the log. |
| 9 | `run-mac-app.sh --skip-rust --seed <file with "not json">` | Prints that the file is not a JSON array and exits with status 1, before the build. |
| 10 | `run-mac-app.sh --bad-option` | Prints the usage text and exits with status 2. |
| 11 | `capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests <scratchpad folder>` | Saves `01-catalog.png` and `02-empty-catalog.png`. They show the same content as the files in `MAC-01-demo/`. |
| 12 | `capture-demo.sh ios KreatynaUITests/DemoCaptureUITests <scratchpad folder>` on the booted iPhone 17 Pro | Saves the iOS demo screenshots with their attachment names. |
| 13 | `ls -l .claude/skills/` and `cat .claude/skills/*/SKILL.md >/dev/null` | 12 symlinks (11 here, and `use-plannotator-inbox` from change `krkvmrxp`). Every symlink resolves. |
| 14 | `rg -n --hidden "document-ios-delivery" .agents docs/epics/ingredient-catalog.md` | No links remain. |
| 15 | `jj diff --summary` | The files from this plan only. No `project.yml` change. |

Checks 3 and 6–11 open windows on your screen and take focus for a moment.

## Stubs and exclusions

- No screenshot from `run-mac-app.sh` (MAC-02/Q1).
- No demo for MAC-02. The slice changes no visible app behavior. The run
  output and the capture results go into Validation.
- The detail pane does not show the seeded notes, nutrition, or units until
  MAC-04.
- No seed from the iOS run script. `run-ios-simulator` is unchanged.
- No rename of `run-ios-simulator` or of the `ios-*` devenv scripts.
- No shared helper file for the two run scripts. Each copies the
  `clean_env` lines, like the devenv scripts.
- No Release build, signing, or notarization from the script.
- No light/dark switch for Mac capture tests. The window uses the system
  appearance.
- No new devenv script. The skill scripts call the existing `mac-bindings`,
  `ios-bindings`, and `apple-generate`.

## Validation

Run on 2026-10-13, macOS 27.0.1, Xcode 27.0, iPhone 17 Pro simulator.

### Deviations from the plan

1. **Skill count.** Q4 said "11 today and `run-macos-app`". There were 10
   skills; the agent counted `.` and `..` in an `ls -la` listing. The 7
   symlinks are as planned, so all 11 MAC-02 skills have one. The 12th skill,
   `use-plannotator-inbox`, came from the process change `krkvmrxp` and has
   its own symlink there.
2. **Item errors.** The sketch used `#[serde(untagged)]` for `SeedItem`. An
   untagged enum hides the inner error ("did not match any variant"), so the
   unknown-field test failed. `SeedItem::read` now checks the JSON kind
   first: a string is a name, an object is read as `SeedIngredient` with its
   own error, and anything else is an error that names the kind.
3. **Fixture.** It also has "Olive oil" per 100 ml, and Garlic has a second
   unit, "bulb ≈ 50 g". Olive oil's tablespoon has both 15 ml and 14 g.
4. **UI test helper.** `launch(seed:)` in the 2 Mac UI test files now
   `throws`, because `JSONEncoder().encode` throws.
5. **Change ID.** The plan was started in `krkvmrxp`. The new Inbox process
   was split into that change, so MAC-02 is now `yvxqwqyy`.

### Checks

| # | Check | Result |
|---|---|---|
| 1 | `cargo test -p kreatyna-ffi` | Passed: 40 tests, including 12 seed tests (3 of them `invalid_json_is_rejected` cases). |
| 2 | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` | Clean. |
| 3 | `mac-test` | `TEST SUCCEEDED`: 38 passed, 0 failed, 1 skipped (`MacDemoCaptureUITests`, as designed). |
| 4 | `ios-test BB6851F0-…` | `TEST SUCCEEDED`: 53 passed, 0 failed, 1 skipped (`DemoCaptureUITests`). |
| 5 | `bash -n` on both scripts | No errors. |
| 6 | `run-mac-app.sh` | Built, opened. `pid 78594`, macOS 27.0.1, window `900 × 552` (the size macOS restored from an earlier run), `database: real`. |
| 7 | `run-mac-app.sh --skip-rust` while the app ran | "Quitting the running Kreatyna", new `pid 78710`, 1 Mac Kreatyna process. |
| 8 | `run-mac-app.sh --skip-rust --seed apple/Kreatyna/Fixtures/demo-ingredients.json` | `isolated, seeded from demo-ingredients.json (5 items)`. The real `kreatyna.sqlite` kept its time (Oct 12 10:04:48). The isolated database has 5 ingredients and 6 units (`clove` 5 g, `tablespoon` 14 g / 15 ml, …). No `seed` log errors. |
| 9 | `--seed` with a file that holds `not json` | "Seed file is not a JSON array", exit status 1, no build. |
| 10 | `--bad-option` | Usage text, exit status 2. |
| 11 | `capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests` | Saved `01-catalog.png` and `02-empty-catalog.png`. `01-catalog.png` shows the same 4 rows as in `MAC-01-demo/`, seeded through the new JSON path. |
| 12 | `capture-demo.sh ios KreatynaUITests/DemoCaptureUITests --udid BB6851F0-…` | Saved 15 screenshots (`01-empty-catalog` … `15-after-relaunch`). `10-detail.png` shows Garlic with 149 kcal and clove 5 g. Without `--udid`, the script stopped with "No booted iOS simulator", because `ios-test` had shut the simulator down. |
| 13 | `.claude/skills/` | 12 symlinks. `cat .claude/skills/*/SKILL.md` reads all of them. |
| 14 | `rg -n --hidden "document-ios-delivery" .agents docs/epics/ingredient-catalog.md apple` | No matches. |
| 15 | `jj diff --summary` | Only files from this plan. Old `document-ios-delivery/SKILL.md` shows as deleted and the new skill as added. No `project.yml` change. |

The agent cannot see the window opened by checks 6–8 (MAC-02/Q1). Checks 11
and 12 show the content.

### Not run

- Release build. The slice changes no build settings; `serde_json` is a
  normal dependency of `kreatyna-ffi`.
