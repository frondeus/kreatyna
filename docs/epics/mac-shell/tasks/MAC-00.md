# MAC-00 — Add the macOS app target

Status: approved. Change: `qqkvvsly`. Dependencies: MAC-10 (approved).

## Open questions

None. The user accepted the recommendations for MAC-00/Q1 and MAC-00/Q2 in
review on 2026-10-12 (see Decisions).

## Decisions

### MAC-00/Q1 — Allow Automation Mode without a prompt, once

*Context:* On macOS, an XCTest UI test controls the real desktop. macOS
allows this only in **Automation Mode**. On this Mac (2026-10-12),
`automationmodetool` reported "Automation Mode is disabled. This device
requires user authentication to enable Automation Mode." An agent cannot
answer a password or Touch ID prompt.

*Decision:* Option A. You run
`sudo automationmodetool enable-automationmode-without-authentication` one
time (step 13). After that, `mac-test` runs the Mac UI tests with no prompt,
also when an agent starts it. During a UI test run, the test moves the app
window to the front and takes keyboard input for some seconds. To undo it,
run `sudo automationmodetool disable-automationmode-without-authentication`.

So `mac-test` always runs the UI tests, and it has no `-skip-ui` option.

### MAC-00/Q2 — The Mac app uses the bundle identifier `pl.frondeus.Kreatyna`

*Context:* A sandboxed Mac app stores all its files in
`~/Library/Containers/<bundle identifier>/Data/`. MAC-01 puts the SQLite
database there, so a later change of the identifier leaves the data behind.
The iOS app already uses `pl.frondeus.Kreatyna`.

*Decision:* Option A. The Mac app uses the same identifier as the iOS app.
Its container is `~/Library/Containers/pl.frondeus.Kreatyna/Data/`. This keeps
universal purchase and future sync (D-02) open. The test bundles use
`pl.frondeus.KreatynaMacTests` and `pl.frondeus.KreatynaMacUITests`.

## Outcome

`mac-test` builds the new Mac app and runs its tests. Opening the built app
shows a small window with a globe and "Hello, world!", and **Kreatyna** in
the menu bar. The app runs in the macOS sandbox: its home folder is
`~/Library/Containers/pl.frondeus.Kreatyna/Data`. Nothing talks to Rust yet.
The iOS app builds, tests, and runs exactly as before.

## Terms

- **Target:** one product that Xcode builds. After MAC-00 there are six: the
  iOS app `Kreatyna` with `KreatynaTests` and `KreatynaUITests`, and the Mac
  app `KreatynaMac` with `KreatynaMacTests` and `KreatynaMacUITests`.
- **Scheme:** a named recipe for `xcodebuild`: what to build and which tests
  to run. MAC-00 adds the scheme `KreatynaMac`.
- **XcodeGen and `project.yml`:** `apple-generate` runs XcodeGen. It reads
  `apple/Kreatyna/project.yml` and writes `Kreatyna.xcodeproj`. Agents edit
  only the YAML (E-13).
- **Setting group:** a named block of build settings in `project.yml` that
  several targets include. MAC-00 adds one, `macOS`, so the three Mac targets
  do not repeat the same lines.
- **App Sandbox:** a macOS security mode. The app sees only its own container
  folder, unless it has extra permissions ("entitlements"). The build setting
  `ENABLE_APP_SANDBOX: YES` makes Xcode add the entitlement
  `com.apple.security.app-sandbox` to the signature.
- **Ad-hoc signing ("Sign to Run Locally"):** a code signature with no Apple
  developer certificate. `CODE_SIGN_IDENTITY: "-"` selects it. The app runs
  on this Mac only. This Mac has no signing certificate
  (`security find-identity -v -p codesigning` finds 0), so this is the only
  option today.
- **Hardened runtime:** a macOS mode that only Apple notarisation requires.
  It also blocks loading code from a different signer. Ad-hoc signatures have
  no signer ("Team ID"), so the test bundle cannot load into the app (see
  Changes by layer).
- **Automation Mode:** the macOS mode that lets XCTest UI tests control the
  mouse and keyboard. See MAC-00/Q1.
- **Test host:** the app that a unit-test bundle loads into. Unit tests then
  run inside the real, sandboxed app process.

## Design

No mockup. The window is Xcode's SwiftUI app template ("Hello, world!" under
a globe icon). MAC-01 replaces it with the three-column catalog shown in
[mockups.html](../../../tasks/recipes/mockups.html).

## Changes by layer

**db, rust, ffi: no change.** MAC-00 has no Rust code. The examples below are
YAML, Nix, and Swift.

### tooling — `apple/Kreatyna/project.yml`

Today the project-level settings are for iOS (*real*):

```yaml
settings:
  base:
    IPHONEOS_DEPLOYMENT_TARGET: "26.2"
    SDKROOT: iphoneos
```

MAC-00 keeps them. The iOS targets do not change. The Mac targets override
`SDKROOT` through a new setting group (*sketch*):

```yaml
settingGroups:
  macOS:
    SDKROOT: macosx
    MACOSX_DEPLOYMENT_TARGET: "26.2"   # same OS generation as the iOS target
    CODE_SIGN_STYLE: Automatic
    CODE_SIGN_IDENTITY: "-"            # ad-hoc: no certificate on this Mac
    ENABLE_HARDENED_RUNTIME: NO        # see "Why no hardened runtime" below
    CURRENT_PROJECT_VERSION: 1
    MARKETING_VERSION: "1.0"
    GENERATE_INFOPLIST_FILE: YES
    SWIFT_APPROACHABLE_CONCURRENCY: YES
    SWIFT_UPCOMING_FEATURE_MEMBER_IMPORT_VISIBILITY: YES
    SWIFT_VERSION: "5.0"

targets:
  KreatynaMac:
    type: application
    platform: macOS
    sources:
      - path: KreatynaMac
        type: syncedFolder
    settings:
      groups: [macOS]
      base:
        ASSETCATALOG_COMPILER_APPICON_NAME: AppIcon
        ASSETCATALOG_COMPILER_GLOBAL_ACCENT_COLOR_NAME: AccentColor
        COMBINE_HIDPI_IMAGES: YES
        ENABLE_APP_SANDBOX: YES
        ENABLE_PREVIEWS: YES
        LD_RUNPATH_SEARCH_PATHS: ["$(inherited)", "@executable_path/../Frameworks"]
        PRODUCT_BUNDLE_IDENTIFIER: pl.frondeus.Kreatyna   # MAC-00/Q2: same as iOS
        PRODUCT_NAME: Kreatyna         # menu bar and Finder show "Kreatyna"
        STRING_CATALOG_GENERATE_SYMBOLS: YES
        SWIFT_DEFAULT_ACTOR_ISOLATION: MainActor
        SWIFT_EMIT_LOC_STRINGS: YES
    scheme:
      testTargets:
        - name: KreatynaMacTests
          parallelizable: true
        - name: KreatynaMacUITests
          parallelizable: true

  KreatynaMacTests:
    type: bundle.unit-test
    platform: macOS
    sources:
      - path: KreatynaMacTests
        type: syncedFolder
    dependencies:
      - target: KreatynaMac
    settings:
      groups: [macOS]
      base:
        BUNDLE_LOADER: $(TEST_HOST)
        PRODUCT_BUNDLE_IDENTIFIER: pl.frondeus.KreatynaMacTests
        PRODUCT_NAME: $(TARGET_NAME)
        TEST_HOST: $(BUILT_PRODUCTS_DIR)/Kreatyna.app/Contents/MacOS/Kreatyna

  KreatynaMacUITests:
    type: bundle.ui-testing
    platform: macOS
    sources:
      - path: KreatynaMacUITests
        type: syncedFolder
    dependencies:
      - target: KreatynaMac
    settings:
      groups: [macOS]
      base:
        PRODUCT_BUNDLE_IDENTIFIER: pl.frondeus.KreatynaMacUITests
        PRODUCT_NAME: $(TARGET_NAME)
        TEST_TARGET_NAME: KreatynaMac
```

The settings come from Xcode 27's "macOS App" template
(`COMBINE_HIDPI_IMAGES`, `LD_RUNPATH_SEARCH_PATHS`, `ENABLE_APP_SANDBOX`,
`SDKROOT`) and from today's iOS targets (Swift settings). The agent
leaves out these template settings:

| Template setting | Why MAC-00 leaves it out |
|---|---|
| `ENABLE_USER_SELECTED_FILES: readonly` | The app opens no user files. Rust keeps the database in the container. |
| `REGISTER_APP_GROUPS: YES` | The app uses no app groups. |
| `ENABLE_HARDENED_RUNTIME: YES` | See below. |

**Why no hardened runtime.** The agent tried the template settings in a
throwaway project on 2026-10-12. With the hardened runtime and ad-hoc signing,
`xcodebuild test` failed:

```text
code signature in '…/KreatynaMacTests' not valid for use in process:
mapping process and mapped file (non-platform) have different Team IDs
```

With `ENABLE_HARDENED_RUNTIME: NO`, the same test passed, and the app ran in
its container (`HOME=…/Library/Containers/<bundle id>/Data`). Only
notarisation needs the hardened runtime. Notarisation is out of scope for this
epic.

**Release builds.** Xcode builds a Mac Release app for `arm64` and `x86_64`.
MAC-00 has no Rust library, so this works. MAC-01 links the Rust library and
must then exclude `x86_64` or build Rust for both. MAC-00 does not decide this.

### tooling — `devenv.nix`

A new script `mac-test` mirrors `ios-test`. It needs no simulator. The minimal
environment that `ios-test` uses moves to a shared `let` binding, so both
scripts use one copy (*sketch*):

```nix
let
  # Xcode needs nothing from the Nix shell. A minimal environment keeps Nix
  # compiler variables (LD, CC, SDKROOT, NIX_LDFLAGS) out of its link step.
  xcodeEnv = ''
    clean_env=(env -i HOME="$HOME" USER="$USER" PATH=/usr/bin:/bin:/usr/sbin:/sbin
      DEVELOPER_DIR="$DEVELOPER_DIR")
  '';
in
{
  scripts.mac-test = {
    description = "Generate the Xcode project and run all macOS tests: mac-test";
    exec = ''
      set -euo pipefail
      cd ${lib.escapeShellArg config.devenv.root}
      apple-generate
      ${xcodeEnv}
      mkdir -p target/xcode
      echo "==> Testing on this Mac (log: target/xcode/mac-test.log)"
      if ! "''${clean_env[@]}" xcodebuild test \
          -project apple/Kreatyna/Kreatyna.xcodeproj -scheme KreatynaMac \
          -destination "platform=macOS,arch=arm64" \
          -derivedDataPath target/xcode/DerivedData \
          >target/xcode/mac-test.log 2>&1; then
        grep -E "error:|failed|TEST FAILED" target/xcode/mac-test.log >&2 || tail -40 target/xcode/mac-test.log >&2
        exit 1
      fi
      grep -E "Test run with|TEST SUCCEEDED" target/xcode/mac-test.log || true
    '';
  };
}
```

Both scripts share `target/xcode/DerivedData`. Xcode keeps Mac products in
`Build/Products/Debug/` and simulator products in
`Build/Products/Debug-iphonesimulator/`, so they do not overwrite each other.

### swift — new folders in `apple/Kreatyna/`

| File | Content |
|---|---|
| `KreatynaMac/KreatynaMacApp.swift` | `@main` app with one `WindowGroup { ContentView() }`. |
| `KreatynaMac/ContentView.swift` | Template view: globe image and `Text("Hello, world!")` with `.accessibilityIdentifier("greeting")`. |
| `KreatynaMac/Assets.xcassets/` | `AppIcon` (macOS sizes, no images yet) and `AccentColor`, like the iOS catalog. |
| `KreatynaMacTests/SandboxTests.swift` | One Swift Testing test (below). |
| `KreatynaMacUITests/KreatynaMacUITests.swift` | One launch test (below). |

`ContentView` exists in both apps. The apps are separate Swift modules, so the
names do not conflict. MAC-01 replaces the Mac `ContentView`.

Swift Testing sketch. It guards the sandbox acceptance item, and MAC-01 relies
on the container for the database path:

```swift
// sketch
import Foundation
import Testing

struct SandboxTests {
    @Test func appRunsInItsSandboxContainer() throws {
        let bundleID = try #require(Bundle.main.bundleIdentifier)
        #expect(bundleID == "pl.frondeus.Kreatyna")
        #expect(NSHomeDirectory().hasSuffix("/Library/Containers/\(bundleID)/Data"))
    }
}
```

XCTest UI sketch:

```swift
// sketch
import XCTest

final class KreatynaMacUITests: XCTestCase {
    @MainActor
    func testLaunchShowsWindow() throws {
        let app = XCUIApplication()
        app.launch()
        XCTAssertTrue(app.staticTexts["greeting"].waitForExistence(timeout: 5))
    }
}
```

### docs

- `.agents/skills/build-swift-presentation/SKILL.md`: the last paragraph of
  "Testing policy" names `ios-test`. Add one sentence for `mac-test`.
- `docs/epics/mac-shell/epic.html`: MAC-00 `status` and `plan`.
- `AGENTS.md`: "Xcode 26.2" becomes "Xcode 27.0", the version on this Mac
  (with macOS 27.0.1). Done at the user's request on 2026-10-12.

References found with
`rg -n "ios-test|xcodebuild|-scheme|KreatynaMac" --glob '!target/**'`
(2026-10-12), excluding plan files:

| File | Change in MAC-00 |
|---|---|
| `devenv.nix:123-158` (`ios-test`) | Uses the shared `xcodeEnv` binding. No behavior change. |
| `.agents/skills/build-swift-presentation/SKILL.md` | Add `mac-test`. |
| `apple/Kreatyna/KreatynaUITests/DemoCaptureUITests.swift:6` | None (iOS only). |
| `docs/epics/mac-shell/epic.html:213,376-380` | Status and plan only. |

## Steps

1. Record the iOS build settings before the change. Run it in the minimal
   environment that `ios-test` uses:
   `xcodebuild -showBuildSettings -project apple/Kreatyna/Kreatyna.xcodeproj -scheme Kreatyna -destination "generic/platform=iOS Simulator" > target/xcode/ios-settings-before.txt`.
2. Add the `macOS` setting group to `apple/Kreatyna/project.yml`.
3. Add the `KreatynaMac` target and its scheme.
4. Add the `KreatynaMacTests` and `KreatynaMacUITests` targets.
5. Set `PRODUCT_BUNDLE_IDENTIFIER` for `KreatynaMac` to `pl.frondeus.Kreatyna` (MAC-00/Q2).
6. Create `apple/Kreatyna/KreatynaMac/` with `KreatynaMacApp.swift`, `ContentView.swift`, and `Assets.xcassets`.
7. Create `apple/Kreatyna/KreatynaMacTests/`.
8. Add `SandboxTests.swift`.
9. Create `apple/Kreatyna/KreatynaMacUITests/` with the launch test.
10. Move the minimal Xcode environment in `devenv.nix` to the `xcodeEnv` binding.
11. Add the `mac-test` script to `devenv.nix`.
12. Reload the shell with `direnv reload`, so `mac-test` exists.
13. Ask the user to run `sudo automationmodetool enable-automationmode-without-authentication` (MAC-00/Q1).
14. Run `apple-generate`.
15. Run the checks below.
16. Add the `mac-test` sentence to `build-swift-presentation`.
17. Set MAC-00 to `awaiting-review` in `epic.html`.
18. Fill in **Validation**.

## Checks

| # | Command | Expected result |
|---|---|---|
| 1 | `apple-generate` | Exit 0, no output. |
| 2 | `xcodebuild -list -project apple/Kreatyna/Kreatyna.xcodeproj` | Targets: `Kreatyna`, `KreatynaTests`, `KreatynaUITests`, `KreatynaMac`, `KreatynaMacTests`, `KreatynaMacUITests`. Schemes: `Kreatyna`, `KreatynaMac`. |
| 3 | Step 1's command into `target/xcode/ios-settings-after.txt`, then `diff` both files | No difference. |
| 4 | `mac-test` | `TEST SUCCEEDED`. 1 Swift Testing test and 1 UI test pass. |
| 5 | `codesign -d --entitlements - target/xcode/DerivedData/Build/Products/Debug/Kreatyna.app` | Lists `com.apple.security.app-sandbox`. `Signature=adhoc`. |
| 6 | `open target/xcode/DerivedData/Build/Products/Debug/Kreatyna.app` | A window shows "Hello, world!". The menu bar shows **Kreatyna**. |
| 7 | `xcodebuild build -project apple/Kreatyna/Kreatyna.xcodeproj -scheme KreatynaMac -configuration Release -destination "platform=macOS"` (in the clean environment) | `BUILD SUCCEEDED`. |
| 8 | `ios-bindings debug`, then `ios-test` on a booted simulator | Same result as at the end of MAC-03: all iOS tests pass. |

Check 6 needs a person to look at the screen. The agent records whether it
saw the window, or that it could not.

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| The window shows the template "Hello, world!" view. | MAC-01 (three-column catalog). |
| The Mac app does not link the Rust library or the bindings. | MAC-01. |
| No shared source folder between iOS and Mac (E-12). | MAC-01. |
| No SnapshotTesting dependency and no snapshots for Mac. | MAC-01. |
| No `run-macos-app` skill and no Mac demo screenshots. | MAC-02. |
| `AppIcon` has no images; macOS shows a generic icon. | Not planned yet. |
| No hardened runtime, no certificate signing, no notarisation. | Out of scope for the epic. |

## Validation

Run on 2026-10-12. Mac: macOS 27.0.1, Xcode 27.0 (27A266a), Apple Silicon.
Simulator: iPhone 17 Pro (`BB6851F0-…`). The agent booted it without the
Simulator window for check 8.

### Deviations from the plan

1. **Two more settings in the Mac test targets.** `KreatynaMacTests` and
   `KreatynaMacUITests` also set `STRING_CATALOG_GENERATE_SYMBOLS: NO` and
   `SWIFT_EMIT_LOC_STRINGS: NO`. The iOS test targets set the same values.
2. **Check 3 compares only the settings part.** The raw `diff` showed one
   difference: Xcode printed the resolved Swift packages
   (`swift-custom-dump`, `swift-issue-reporting`) in a different order before
   the settings. The settings below the `Build settings` line are identical.
3. **`AppIcon` uses the classic macOS sizes** (16 to 512 points, at 1x and
   2x, idiom `mac`), with no images. Xcode 27 builds it with no warning.
4. **`automationmodetool` still says "Automation Mode is disabled".** That is
   correct: XCTest turns the mode on for each test run. After your command,
   it also says "This device DOES NOT REQUIRE user authentication to enable
   Automation Mode", and `mac-test` showed no prompt.

### Checks

| # | Command | Result |
|---|---|---|
| 1 | `devenv shell -- apple-generate` | Exit 0, no output. |
| 2 | `xcodebuild -list -project apple/Kreatyna/Kreatyna.xcodeproj` | Targets `Kreatyna`, `KreatynaMac`, `KreatynaMacTests`, `KreatynaMacUITests`, `KreatynaTests`, `KreatynaUITests`. Schemes `Kreatyna`, `KreatynaMac`. |
| 3 | `xcodebuild -showBuildSettings … -scheme Kreatyna` before and after, then `diff` | Settings identical (see deviation 2). |
| 4 | `devenv shell -- mac-test` | `TEST SUCCEEDED`. `SandboxTests/appRunsInItsSandboxContainer()` and `KreatynaMacUITests.testLaunchShowsWindow()` passed. No authentication prompt. |
| 5 | `codesign -dv --entitlements - …/Debug/Kreatyna.app` | `Identifier=pl.frondeus.Kreatyna`, `Signature=adhoc`, `com.apple.security.app-sandbox` present. |
| 6 | `open target/xcode/DerivedData/Build/Products/Debug/Kreatyna.app` | The process `Kreatyna` runs. The agent cannot see the screen; the UI test (check 4) found the "Hello, world!" text in the window. |
| 7 | `xcodebuild build … -scheme KreatynaMac -configuration Release -destination "platform=macOS"` | `BUILD SUCCEEDED`. The binary contains `x86_64 arm64`. |
| 8 | `ios-bindings debug`, then `ios-test BB6851F0-…` | `TEST SUCCEEDED`. 48 passed, 1 skipped (`DemoCaptureUITests`), the same as MAC-03. |

### Not run

- No Rust checks: MAC-00 changes no Rust code.
- No snapshots and no demo screenshots: the window is the Xcode template.
  MAC-01 adds both, and MAC-02 adds Mac screenshot tooling.
