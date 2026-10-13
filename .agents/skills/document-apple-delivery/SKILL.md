---
name: document-apple-delivery
description: Produce reviewable evidence for a completed iOS or Mac app feature, including an annotated demo, screenshots from the running app, optional videos, and relevant test results. Use for Swift-side UI deliveries, not Rust-only or planning changes.
---

# Document an Apple delivery

Use the feature's acceptance criteria to choose scenarios. Keep the project's
Swift testing agreement in `AGENTS.md`; this skill describes how to show the
finished behavior to a reviewer.

## Shared rules

Run the integrated app and capture its actual screens. Use controlled data
and state the platform, OS version, and device or window size, so a reviewer
can understand and reproduce the result. When a failure state needs a
fixture or injected response, label that setup and exercise the normal UI and
event path. Do not present a static mockup as a running-app capture.

Do not write a `demo.md` file. The demo is the Inbox message itself: write
everything you want to say in its `body`, and embed each screenshot there
with `![what it shows](docs/epics/<epic>/tasks/<ID>-demo/01-catalog.png)`.
Also list every embedded image in `attachments`, so the Inbox keeps it and
the user can comment on it. For each scenario, state the starting condition,
action, and visible result, then the image. Cover the main flows and each
relevant user-visible edge state from the acceptance criteria, such as empty,
loading, validation, failure/retry, pending, cancellation, and persistence
when applicable. Use short videos when motion or a multi-step interaction is
clearer than still images; attach them, as a supplement to the screenshots.

A state that the running app cannot produce on demand (for example a storage
failure) goes under "Not captured from the running app", with the snapshot
or test that covers it.

Run the relevant Swift Testing, deterministic snapshot, and real-core XCTest
UI checks required by the project. Inspect the captured images and any
snapshot diffs before accepting them. Record the checks and any unavailable
evidence in the demo message; never imply that an unrun check passed. Check
that every embedded image path exists before you send the message.

Send the demo message to the slice's Inbox thread. When the change edits Rust
files, end it with one question: "Does the app behave as the demo shows?".
The user annotates the demo there, and a guided review of the Rust side
follows in the same thread. When it edits no Rust files, the demo is part of
the report, for information only, without the question. See "Code review:
count the Rust files first" in
[use-plannotator-inbox](../use-plannotator-inbox/SKILL.md).

## Data

Seed data through Rust (E-17): set `KREATYNA_TEST_DATABASE_ID` to a new UUID
and `KREATYNA_TEST_SEED` to a JSON array of names or ingredient objects. A
demo that needs full ingredient data (notes, nutrition, units) uses
`apple/Kreatyna/Fixtures/demo-ingredients.json`, or its own fixture in that
folder. The seed format is in
[run-macos-app](../run-macos-app/SKILL.md#seed-files).

## Capture tests

Screenshots come from capture tests: UI tests that run only with
`TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1` and attach screenshots named like the
demo files (`01-catalog`).

| Platform | Class | Screenshot |
|---|---|---|
| iOS | `KreatynaUITests/DemoCaptureUITests` | `XCUIScreen.main.screenshot()` of the simulator |
| Mac | `KreatynaMacUITests/MacDemoCaptureUITests` | `app.windows.firstMatch.screenshot()` of the window only |

Each slice adds a test method for its demo. Then export the screenshots:

```sh
devenv shell -- mac-bindings debug    # or ios-bindings debug
.agents/skills/document-apple-delivery/scripts/capture-demo.sh mac \
  KreatynaMacUITests/MacDemoCaptureUITests docs/epics/<epic>/tasks/<ID>-demo
```

`capture-demo.sh <mac|ios> <test identifier> <output folder> [--udid UDID]`
runs `apple-generate`, runs the test with the capture variable, exports the
attachments from `target/xcode/demo.xcresult`, and copies each one to
`<output folder>/<attachment name>.png`. It does not build Rust. A
`<test identifier>` can name one method:
`KreatynaMacUITests/MacDemoCaptureUITests/testCaptureMacWindowDemo`.

## iOS

State the simulator device and the iOS version. `ios` uses the booted
simulator, or `--udid`. Boot one with `xcrun simctl boot <UDID>` first.

## Mac

State the macOS version (`sw_vers -productVersion`), the window size in
points, and the appearance (light or dark). The window uses the Mac's
system appearance; the snapshot tests cover the other appearance. A UI test
moves the window to the front and takes focus for some seconds. The agent's
shell has no Screen Recording permission, so `screencapture` cannot replace
the capture tests.
