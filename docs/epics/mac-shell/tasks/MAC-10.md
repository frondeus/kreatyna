# MAC-10 — Generate the Xcode project with XcodeGen

Status: approved. Change: `ompwommq`. Dependencies: MAC-09 (approved).

## Open questions

None. The user answered MAC-10/Q1, MAC-10/Q2, and MAC-10/Q3 in review (see
Decisions).

## Decisions

### MAC-10/Q1 — `apple/Kreatyna/project.yml` generates `apple/Kreatyna/Kreatyna.xcodeproj`

*Context:* XcodeGen writes wrong source-folder paths when the spec and the
project are in different folders (trial run with `--project`). So they must
be in the same folder.

*Decision:* Option A. The spec is `apple/Kreatyna/project.yml`, next to the
generated project. Nothing moves: `$(SRCROOT)` stays `apple/Kreatyna/`, and
the `$(SRCROOT)/../../target` paths, the scripts, and the snapshot folders
stay the same. MAC-00 adds the Mac target to the same spec. This is epic
decision E-15; the MAC-10 card now names `apple/Kreatyna/project.yml`.

### MAC-10/Q2 — Keep `Package.resolved` in version control through `.gitignore` exceptions

*Context:* `Package.resolved` locks the Swift package versions
(swift-snapshot-testing 1.19.6, swift-custom-dump 1.7.3,
swift-issue-reporting 2.1.1, swift-syntax 604.0.0), like `Cargo.lock`. It is
inside the generated `.xcodeproj` folder, which E-13 does not commit.

*Decision:* Option A. Ignore the generated `.xcodeproj` folder, except
`project.xcworkspace/xcshareddata/swiftpm/Package.resolved`. XcodeGen does
not delete this file when it regenerates the project.

### MAC-10/Q3 — Fix the Rust link settings in MAC-10

*Context:* Today only Debug builds for the simulator work. Checked on
2026-10-11:

| Build | Result before MAC-10 |
|---|---|
| Debug, simulator | Works |
| Debug, real iPhone | Fails: `ld: building for 'iOS', but linking in object file … built for 'iOS-simulator'` |
| Release, simulator | Fails: `error: cannot find type 'RustBuffer' in scope` (no bridging header) |

*Decision:* Option B. MAC-10 makes all four combinations of
{Debug, Release} × {simulator, real iPhone} build. The build-settings
comparison then shows these intended differences, listed in "Known
differences".

How (agent's design, within the decision):

- **One copy of the Swift bindings.** The bindings that UniFFI generates
  from the device library and from the simulator library are byte-identical
  (`diff -r` on 2026-10-11). So `ios-bindings` writes them once, to
  `target/generated/apple/swift/`. The project uses that one folder for the
  Swift file and the bridging header in every configuration. MAC-01 can use
  the same folder for the Mac target (E-12).
- **One Rust library per SDK and configuration.** A new build setting
  `RUST_PROFILE` is `debug` in Debug and `release` in Release. The linker
  flags select the library with an SDK condition:

  ```yaml
  # sketch
  RUST_PROFILE: debug        # in configs.Debug; "release" in configs.Release
  OTHER_LDFLAGS[sdk=iphonesimulator*]:
    - $(inherited)
    - $(SRCROOT)/../../target/aarch64-apple-ios-sim/$(RUST_PROFILE)/libkreatyna_ffi.a
    - -lsqlite3
  OTHER_LDFLAGS[sdk=iphoneos*]:
    - $(inherited)
    - $(SRCROOT)/../../target/aarch64-apple-ios/$(RUST_PROFILE)/libkreatyna_ffi.a
    - -lsqlite3
  SWIFT_OBJC_BRIDGING_HEADER: $(SRCROOT)/../../target/generated/apple/swift/kreatyna_ffiFFI.h
  ```
- **Risk:** a Release build for the simulator may also build for `x86_64`
  (Intel). The Rust library is `arm64` only. If the link step fails for that
  reason, exclude `x86_64` for the simulator SDK
  (`EXCLUDED_ARCHS[sdk=iphonesimulator*]: x86_64`) and record it in
  Validation.

## Outcome

You never edit Xcode files by hand. To change the Xcode project, an agent
edits `project.yml`, a readable text file. A script generates
`Kreatyna.xcodeproj` from it. The iOS app builds, tests, and runs as before
on the simulator. It now also builds for a real iPhone and in Release. For
example, MAC-00 adds the Mac app as about 30 lines of YAML.

## Terms

- **XcodeGen:** a command-line tool. It reads a YAML file (`project.yml`) and
  writes an Xcode project (`.xcodeproj`). The YAML is the source; the
  `.xcodeproj` is output, like files in `target/`.
- **Target:** one product that Xcode builds. Today there are three: the app
  `Kreatyna`, the unit tests `KreatynaTests` (Swift Testing and snapshots),
  and the UI tests `KreatynaUITests` (XCTest).
- **Scheme:** a named recipe for `xcodebuild`: which target to build and
  which test targets to run. `xcodebuild -scheme Kreatyna` uses it. Today
  Xcode creates the scheme automatically. XcodeGen writes it as a file.
- **Build setting:** one key and value that controls the build, for example
  `IPHONEOS_DEPLOYMENT_TARGET = 26.2`. `xcodebuild -showBuildSettings` prints
  all of them for one target and configuration.
- **Configuration:** a set of build settings. The project has `Debug` and
  `Release`.
- **Synced folder:** a folder that Xcode reads as it is on disk. A new
  `.swift` file in it is part of the target with no project edit. Today's
  project uses synced folders; the generated project uses them too.
- **Setting presets:** default settings that XcodeGen adds by itself. They
  differ from Xcode's template (for example `gnu++14` instead of `gnu++20`).
  The spec turns them off and lists today's settings explicitly.
- **`Package.resolved`:** the lock file for Swift packages, like
  `Cargo.lock`.

## Design

No UI change, so there is no mockup. After MAC-10 the app looks the same as
[05-catalog.png](../../ingredient-catalog/demo/05-catalog.png).

## Changes by layer

**db, rust, ffi: no Rust code change.** The Rust crates do not change. The
devenv script that generates the bindings changes its output folder. The
examples below are YAML, Nix, and shell.

**tooling — `apple/Kreatyna/project.yml` (new).** It describes the project,
the three targets, the package, and the scheme. Shortened sketch:

```yaml
# sketch
name: Kreatyna
options:
  settingPresets: none          # no XcodeGen defaults; settings below are explicit
  developmentLanguage: en
packages:
  SnapshotTesting:
    url: https://github.com/pointfreeco/swift-snapshot-testing
    from: 1.18.0
settings:
  base:                          # copied from today's project-level settings
    IPHONEOS_DEPLOYMENT_TARGET: "26.2"
    SDKROOT: iphoneos
    CLANG_CXX_LANGUAGE_STANDARD: gnu++20
    GCC_C_LANGUAGE_STANDARD: gnu17
    LOCALIZATION_PREFERS_STRING_CATALOGS: YES
    # ... the other CLANG_*/GCC_* lines from today
  configs:
    Debug: { RUST_PROFILE: debug, GCC_OPTIMIZATION_LEVEL: 0, SWIFT_OPTIMIZATION_LEVEL: -Onone }
    Release: { RUST_PROFILE: release, SWIFT_COMPILATION_MODE: wholemodule, VALIDATE_PRODUCT: YES }
targets:
  Kreatyna:
    type: application
    platform: iOS
    sources:
      - path: Kreatyna
        type: syncedFolder
      - path: ../../target/generated/apple/swift/kreatyna_ffi.swift
        optional: true           # exists only after `ios-bindings`
    settings:
      base:
        PRODUCT_BUNDLE_IDENTIFIER: pl.frondeus.Kreatyna
        SWIFT_DEFAULT_ACTOR_ISOLATION: MainActor
        SWIFT_OBJC_BRIDGING_HEADER: $(SRCROOT)/../../target/generated/apple/swift/kreatyna_ffiFFI.h
        OTHER_LDFLAGS[sdk=iphonesimulator*]: [...]   # see MAC-10/Q3
        OTHER_LDFLAGS[sdk=iphoneos*]: [...]
        # ... INFOPLIST_KEY_* and the other app settings from today
    scheme:
      testTargets:
        - name: KreatynaTests
          parallelizable: true
        - name: KreatynaUITests
          parallelizable: true
  KreatynaTests:
    type: bundle.unit-test
    platform: iOS
    sources: [{ path: KreatynaTests, type: syncedFolder }]
    dependencies: [{ target: Kreatyna }, { package: SnapshotTesting }]
  KreatynaUITests:
    type: bundle.ui-testing
    platform: iOS
    sources: [{ path: KreatynaUITests, type: syncedFolder }]
    dependencies: [{ target: Kreatyna }]
```

`parallelizable: true` keeps today's behavior: Xcode's automatic scheme runs
tests on simulator clones ("Clone 2 of iPhone 17 Pro" in the MAC-09 log).

**tooling — `devenv.nix`.**

- Add `pkgs.xcodegen` to `packages` (version 2.44.1 in the pinned
  nixpkgs).
- Change `ios-bindings`: generate the bindings once, from the simulator
  library, into `$CARGO_TARGET_DIR/generated/apple/swift`. Remove the
  per-target loop.
- Add the script `apple-generate`: run `xcodegen generate --spec
  apple/Kreatyna/project.yml --quiet`.
- Add the script `ios-test`: run `apple-generate`, then `xcodebuild test`
  for the `Kreatyna` scheme in the clean environment, with
  `-derivedDataPath target/xcode/DerivedData`. It takes the destination as
  an argument (default: the first booted iOS simulator).

Sketch:

```nix
scripts.apple-generate = {
  description = "Generate the Xcode project from apple/Kreatyna/project.yml";
  exec = ''
    cd ${lib.escapeShellArg config.devenv.root}
    xcodegen generate --spec apple/Kreatyna/project.yml --quiet
  '';
};
```

**tooling — `run-in-simulator.sh`.** Before `xcodebuild`, run
`apple-generate`. If it is not on `PATH`, stop with "run inside the devenv
shell". Update the step list in `run-ios-simulator/SKILL.md`.

**version control — `.gitignore` and tracked files.**

- Ignore the generated project, except `Package.resolved` (MAC-10/Q2):

  ```gitignore
  # Generated by XcodeGen from apple/Kreatyna/project.yml (E-13).
  /apple/Kreatyna/Kreatyna.xcodeproj/*
  !/apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/
  /apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/*
  !/apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/xcshareddata/
  /apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/xcshareddata/*
  !/apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/xcshareddata/swiftpm/
  /apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/xcshareddata/swiftpm/*
  !/apple/Kreatyna/Kreatyna.xcodeproj/project.xcworkspace/xcshareddata/swiftpm/Package.resolved
  ```
- Stop tracking `project.pbxproj` and `contents.xcworkspacedata`
  (`jj file untrack`). The files stay on disk; the script writes them again.

**docs and agent rules.**

- `AGENTS.md`: replace "Until then, change the project only in ways
  `xcodebuild` can verify" with: "Edit `apple/Kreatyna/project.yml`. Never
  edit the generated `.xcodeproj`. Run `apple-generate` after each edit."
- `.agents/skills/build-swift-presentation/SKILL.md`: add the same rule, and
  name `ios-test` as the command for Swift tests.
- Epic page: add E-15 (MAC-10/Q1), and change the MAC-10 card to
  `apple/Kreatyna/project.yml`.

**Known differences in the build-settings comparison** (to record in
Validation):

| Setting | Before | After | Why |
|---|---|---|---|
| `OTHER_LDFLAGS[arch=*]` (Debug) | simulator library only | removed | Replaced by the per-SDK settings (MAC-10/Q3). |
| `OTHER_LDFLAGS[sdk=iphonesimulator*]`, `[sdk=iphoneos*]` | — | library for that SDK and `$(RUST_PROFILE)` | MAC-10/Q3. |
| `SWIFT_OBJC_BRIDGING_HEADER` | Debug only, `…/aarch64-apple-ios-sim/debug/…` | all configurations, `…/generated/apple/swift/…` | MAC-10/Q3. |
| `RUST_PROFILE` | — | `debug` / `release` (all targets; it is a project setting) | New helper setting (MAC-10/Q3). |
| `EXCLUDED_ARCHS[sdk=iphonesimulator*]` | — | `x86_64`; so `ARCHS` is `arm64` instead of `arm64 x86_64` | The Rust simulator library is `arm64` only. Without it, Release for the simulator failed at link (MAC-10/Q3 risk). |
| Object IDs in `project.pbxproj` | Xcode's IDs | XcodeGen's IDs | Not a build setting; nobody reads them. |
| Scheme | Automatic | Shared file `Kreatyna.xcscheme` | Same targets and test targets. |

## Steps

1. Record the "before" state: for each target (`Kreatyna`, `KreatynaTests`,
   `KreatynaUITests`) and each configuration (`Debug`, `Release`), save
   `xcodebuild -showBuildSettings -sdk iphonesimulator` to
   `target/xcode/mac10-settings/before-<target>-<config>.txt`.
2. Add `pkgs.xcodegen`, the two scripts, and the `ios-bindings` change to
   `devenv.nix`. Reload the devenv shell.
3. Write `apple/Kreatyna/project.yml`. Copy each setting from today's
   `project.pbxproj`. Write the link settings from MAC-10/Q3.
4. Run `ios-bindings debug` and `apple-generate`.
5. Save the "after" state the same way as step 1, as `after-*.txt`.
6. Compare before and after. Fix `project.yml` until the only differences
   are in the "Known differences" table.
7. Add the `.gitignore` rules. Untrack the two generated files.
8. Update `run-in-simulator.sh`, its `SKILL.md`, `AGENTS.md`,
   `build-swift-presentation/SKILL.md`, and the epic page.
9. Run the checks below.
10. Fill in **Validation**. Set MAC-10 to `awaiting-review` in the epic page.

## Checks

| Command | Expected result |
|---|---|
| `xcodegen --version` in the devenv shell | `2.44.1` |
| `apple-generate` twice in a row | The second run changes no file. |
| `diff` of `before-*.txt` and `after-*.txt` (paths normalized) | Only the rows in "Known differences". |
| `xcodebuild -list -project apple/Kreatyna/Kreatyna.xcodeproj` | Targets `Kreatyna`, `KreatynaTests`, `KreatynaUITests`; scheme `Kreatyna`. |
| `jj file list \| rg 'xcodeproj'` | Only `Package.resolved`. |
| `rm apple/Kreatyna/Kreatyna.xcodeproj/project.pbxproj && apple-generate` | The project is back; this simulates a fresh checkout. |
| `ios-bindings debug` then `ios-test` | All 50 tests pass. No new or changed files under `__Snapshots__`. |
| `xcodebuild build`, Debug, `generic/platform=iOS`, `CODE_SIGNING_ALLOWED=NO` | Succeeds (failed before MAC-10). |
| `ios-bindings release`, then `xcodebuild build`, Release, `generic/platform=iOS Simulator` and `generic/platform=iOS`, `CODE_SIGNING_ALLOWED=NO` | Both succeed (failed before MAC-10). |
| `run-in-simulator.sh --skip-rust` | The app opens on the ingredient catalog. |
| `cargo test --workspace` | Pass (regression only). |

## Stubs and exclusions

- No stubs.
- No Mac target (MAC-00) and no shared Swift folder (E-12, MAC-01).
- No code signing for a real iPhone. The device builds run with
  `CODE_SIGNING_ALLOWED=NO`; installing on a phone needs a signing team
  later.
- No new Swift code and no new tests. The existing tests prove the
  generated project.
- No `xcodegen` cache (`--use-cache`). Generation takes less than one
  second.

## Validation

Run on 2026-10-11, Xcode 27.0, iPhone 17 Pro simulator (iOS 26.3).

- `xcodegen --version`: 2.44.1.
- `apple-generate` twice: identical `project.pbxproj` and scheme (`shasum`).
- Build-settings comparison (3 targets × Debug/Release, simulator SDK): only
  the rows in "Known differences". `EXCLUDED_ARCHS` also changes the derived
  values `ARCHS`, `ARCHS_BASE`, `ONLY_ARCH`, and removes the `*_x86_64` path
  settings.
- `xcodebuild -list`: targets `Kreatyna`, `KreatynaTests`, `KreatynaUITests`;
  scheme `Kreatyna`.
- `jj file list | rg xcodeproj`: only `Package.resolved` (unchanged).
- Fresh checkout: `project.pbxproj` deleted, then `ios-test` regenerated it,
  and all 50 tests passed. No snapshot file changed.
- Builds that failed before MAC-10 now succeed (`CODE_SIGNING_ALLOWED=NO`):
  Debug for a device, Release for the simulator (after the `x86_64`
  exclusion), Release for a device.
- The bindings generated from the release and debug libraries are
  byte-identical (`diff -r`), so one copy in `target/generated/apple/swift/`
  is correct.
- `run-in-simulator.sh --skip-rust` (now with `apple-generate`): the app opens
  on the ingredient catalog.
- `cargo test --workspace`: 509 passed, 0 failed.

**Flaky test found (not caused by MAC-10).** After the first green run, two
more full `ios-test` runs each failed one test:
`ScreenSnapshotTests/counter(name:view:)` with the argument `"loading"`.
SnapshotTesting reported a 99.497 % pixel match (99.5 % is required). The
difference image shows only the animated activity indicator. The test
renders a spinner, and the captured animation frame depends on the CPU load.
The load comes from UI tests that run at the same time on a parallel
simulator clone. MAC-09 used the same parallel setup (Xcode's automatic
scheme). `-only-testing:KreatynaTests` passed 42 of 42 tests in two runs.
MAC-10 did not change the baseline or the test. MAC-03 deletes the Counter
screen and this snapshot. Other snapshots that show a spinner (for example
`list-name-status.loading`) passed in every run.
