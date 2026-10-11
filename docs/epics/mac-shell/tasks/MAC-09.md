# MAC-09 — Rename the `ios/` folder to `apple/`

Status: awaiting review. Change: `qttlvxyx`. Dependencies: none.

## Open questions

None. The user answered MAC-09/Q1 and MAC-09/Q2 in review (see Decisions).

## Decisions

### MAC-09/Q1 — Rename `target/generated/ios/` to `target/generated/apple/` now

*Context:* The devenv script `ios-bindings` (`devenv.nix`, line 104) writes
the generated Swift bindings to
`target/generated/ios/<rust-target>/<profile>/`. The Xcode project file
`project.pbxproj` points to this folder twice: line 36 (the Swift file
`kreatyna_ffi.swift`) and line 444 (the bridging header
`kreatyna_ffiFFI.h`). `target/` is ignored by version control.

*Decision:* Option B. Rename the output folder to `target/generated/apple/`
in MAC-09. Edit `devenv.nix` line 104 and `project.pbxproj` lines 36 and 444.
The moved `project.pbxproj` is a rename plus a two-line edit. MAC-01 may
still change the layout when it adds the macOS build.

### MAC-09/Q2 — Delete the committed Xcode per-user files and ignore them

*Context:* Two files of Xcode's personal state are in version control:

- `ios/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/xcuserdata/frondeus.xcuserdatad/UserInterfaceState.xcuserstate`
  (which panes and files were open in Xcode),
- `ios/Kreatyna/Kreatyna.xcodeproj/xcuserdata/frondeus.xcuserdatad/xcschemes/xcschememanagement.plist`
  (scheme order and visibility for one user).

They do not change the build. Xcode writes them again when you open the
project.

*Decision:* Option B. Delete both files in MAC-09. Add `xcuserdata/` to
`.gitignore`, so that Xcode's new copies stay out of version control.

## Outcome

The Xcode project and the Swift sources are in `apple/Kreatyna/`. The
generated bindings are in `target/generated/apple/`. The iOS app builds,
tests, and runs as before. The app on screen does not change. Later slices
add the Mac app (MAC-00) next to the iOS app in `apple/`.

## Terms

- **Rename in jj:** jj does not store a "rename" record. It compares the old
  and new files. When a file's content is identical or almost identical,
  `jj diff --summary` shows it as `R` (renamed).
- **`project.pbxproj`:** the text file inside `Kreatyna.xcodeproj` that lists
  targets, files, and build settings.
- **`$(SRCROOT)`:** an Xcode variable. It is the folder that contains the
  `.xcodeproj`. Today it is `<repo>/ios/Kreatyna`. After MAC-09 it is
  `<repo>/apple/Kreatyna`.
- **DerivedData:** Xcode's build output folder. Our scripts use
  `target/xcode/DerivedData`. Xcode's own default is in
  `~/Library/Developer/Xcode/DerivedData/` and is named after the project
  path.
- **`xcuserdata`:** a folder where Xcode keeps the state of one user, for
  example open tabs. It is personal, not project configuration.
- **Snapshot baseline:** a reference PNG in a `__Snapshots__` folder. A
  snapshot test renders a view and compares it with this PNG. The
  SnapshotTesting library finds the folder next to the test source file.

## Design

No UI change, so there is no mockup. After MAC-09 the app looks the same as
[05-catalog.png](../../ingredient-catalog/demo/05-catalog.png).

## Changes by layer

**db, rust, ffi crates: no change.** No Rust code refers to the `ios/` folder
(`rg` over `crates/` finds nothing). `cargo test` runs only as a regression
check. Only the devenv script that writes the bindings changes.

**Relative paths stay correct.** All paths in `project.pbxproj` are
relative. The folder depth does not change:

```text
before:  <repo>/ios/Kreatyna/   + ../../target  →  <repo>/target
after:   <repo>/apple/Kreatyna/ + ../../target  →  <repo>/target
```

**All references** (full `rg` result; `target/`, `.devenv/`, `.jj/`, and
`.git/` excluded):

| File | Line(s) | Change |
|---|---|---|
| `devenv.nix` | 104 | `generated/ios/` → `generated/apple/` (Q1) |
| `apple/Kreatyna/Kreatyna.xcodeproj/project.pbxproj` | 36, 444 | `generated/ios/` → `generated/apple/` (Q1) |
| `apple/Kreatyna/Kreatyna.xcodeproj/**/xcuserdata/**` | 2 files | Delete (Q2) |
| `.gitignore` | new line | `xcuserdata/` (Q2) |
| `.agents/skills/run-ios-simulator/scripts/run-in-simulator.sh` | 94 | `-project ios/…` → `-project apple/…` |
| `docs/epics/ingredient-catalog/demo.md` | 59, 64–73 | `ios/Kreatyna/…` → `apple/Kreatyna/…` (11 snapshot links) |
| `docs/epics/mac-shell/epic.html` | 196, 209, 260, 336–337 | No change. These lines describe the rename itself. |

The devenv scripts keep their names (`ios-build-device`,
`ios-build-simulator`, `ios-bindings`). They build for iOS.

Examples (sketch):

```diff
# devenv.nix
-        output="$CARGO_TARGET_DIR/generated/ios/$target/$profile"
+        output="$CARGO_TARGET_DIR/generated/apple/$target/$profile"

# project.pbxproj, line 444
-"$(SRCROOT)/../../target/generated/ios/aarch64-apple-ios-sim/debug/kreatyna_ffiFFI.h"
+"$(SRCROOT)/../../target/generated/apple/aarch64-apple-ios-sim/debug/kreatyna_ffiFFI.h"

# run-in-simulator.sh, line 94
-    -project ios/Kreatyna/Kreatyna.xcodeproj -scheme Kreatyna \
+    -project apple/Kreatyna/Kreatyna.xcodeproj -scheme Kreatyna \
```

Expected `jj diff --summary` (shortened):

```text
R {ios => apple}/Kreatyna/Kreatyna/ContentView.swift
... (47 more R lines for unchanged files)
R {ios => apple}/Kreatyna/Kreatyna.xcodeproj/project.pbxproj   (or D + A)
D ios/Kreatyna/Kreatyna.xcodeproj/.../UserInterfaceState.xcuserstate
D ios/Kreatyna/Kreatyna.xcodeproj/.../xcschememanagement.plist
M .gitignore
M .agents/skills/run-ios-simulator/scripts/run-in-simulator.sh
M devenv.nix
M docs/epics/ingredient-catalog/demo.md
M docs/epics/mac-shell/epic.html
A docs/epics/mac-shell/tasks/MAC-09.md
```

## Steps

1. Close the Kreatyna project in Xcode, if it is open.
2. Run `mv ios apple` from the repository root.
3. Delete the two `xcuserdata` files. Add `xcuserdata/` to `.gitignore`.
4. Edit `generated/ios/` to `generated/apple/` in `devenv.nix` and
   `project.pbxproj`.
5. Edit line 94 of `run-in-simulator.sh`.
6. Edit the 11 paths in `docs/epics/ingredient-catalog/demo.md`.
7. Run the checks below.
8. Fill in **Validation**. Set MAC-09 to `awaiting-review` in the epic page.
9. Tell the user: open `apple/Kreatyna/Kreatyna.xcodeproj` in Xcode from now
   on. The old entry in Xcode's recent projects list no longer works.

## Checks

| Command | Expected result |
|---|---|
| `rg -n --hidden -g '!target' -g '!.devenv' -g '!.jj' -g '!.git' '(^\|[^A-Za-z_])ios/'` | Only the `epic.html` lines and this plan. |
| `jj diff --summary` | Only the lines listed above. |
| `jj file list \| rg xcuserdata` | Nothing. |
| `cargo test --workspace` | Pass (regression only). |
| `ios-bindings debug` | Writes the bindings to `target/generated/apple/`. |
| `xcodebuild build` and `xcodebuild test` with `-project apple/Kreatyna/Kreatyna.xcodeproj -scheme Kreatyna -derivedDataPath target/xcode/DerivedData` on a simulator, in the clean environment of `run-in-simulator.sh` | The build succeeds. Swift Testing, snapshot, and UI tests pass. |
| `jj diff --summary` after the tests | No new or changed files under `__Snapshots__`. A new file means that SnapshotTesting did not find a baseline. |
| `.agents/skills/run-ios-simulator/scripts/run-in-simulator.sh --skip-rust` | The app opens in the simulator. The screenshot shows the ingredient catalog. |

## Stubs and exclusions

- No stubs.
- No XcodeGen (MAC-10), no Mac target (MAC-00), no Counter removal (MAC-03).
- No rename of the `ios-*` devenv scripts or the `run-ios-simulator` skill.
  They work with iOS, not with the folder.
- The old `target/generated/ios/` folder stays on disk until `target/` is
  cleaned. Nothing reads it after MAC-09.

## Validation

Run on 2026-10-11, Xcode 27.0, iPhone 17 Pro simulator (iOS 26.3).

- `jj diff --summary`: 48 `R` lines (all moved files, including the edited
  `project.pbxproj`), 2 `D` lines (`xcuserdata`), and `M`/`A` only for the
  files in the table.
- `rg` for `ios/`: only `epic.html` (lines that describe the rename) and
  this plan.
- `jj file list | rg xcuserdata`: nothing.
- `cargo test --workspace`: all tests pass.
- `ios-bindings debug` (in a fresh `devenv shell`, because the loaded shell
  still had the old script): writes the bindings to `target/generated/apple/`.
- Before the Xcode build, the old `target/generated/ios/` was moved to
  `target/generated/ios.stale-before-MAC-09/`. So the build cannot read old
  bindings by mistake.
- `xcodebuild test` (clean environment, `target/xcode/DerivedData`): 50
  tests pass, 0 fail. The build log refers only to `generated/apple`.
- Snapshot baselines: no new or changed files under `__Snapshots__`.
- `run-in-simulator.sh --skip-rust`: the app opens on the ingredient catalog.

Note for the user: the devenv scripts change with `devenv.nix`. Reload the
shell (leave and enter the folder again, or run `direnv reload`) before you
run `ios-bindings`.
