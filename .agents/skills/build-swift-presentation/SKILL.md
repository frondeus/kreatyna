---
name: build-swift-presentation
description: Build, refactor, test, or review the SwiftUI side of kreatyna. Swift is a dumb presentation layer over the Rust core; its tests cover wiring and rendering, never business logic that Rust tests already own.
---

# Build the Swift presentation layer

The Rust core owns state, validation, navigation, and persistence. Rust tests
cover its state machines and widgets. Swift renders Rust view data and forwards
user intent as Rust events. Keep it dumb. Read `AGENTS.md` for the ownership
rules and the handover requirement.

## Rendering and events

- Render the latest `AppView` from the adapter. Branch only to choose what to
  display for a view variant. Do not derive new business facts from view data.
- Do not mirror domain state in Swift, validate input, decide navigation, or
  change local state optimistically. A tap sends an event; the next render
  shows its result.
- Child views receive their view value and a `send` closure for their own
  local event type. Parent views wrap these events in their own event type,
  using the scope IDs from the displayed view. One central `dispatch` sends the
  result to Rust. Do not create scopes or reuse old ones.
- Project navigation from the Rust route. Derive SwiftUI path, sheet, and
  dialog bindings from the view. Their setters only send the corresponding
  event, such as `Back`, `RequestClose`, or a confirmation `Cancel`. If Rust
  rejects the event, the next render restores the displayed state.
- Swift may own focus, scroll position, animation, transient text presentation,
  and the user-facing text for typed errors and validation reasons.
- If a screen needs a decision that Rust does not make, stop and propose a
  focused Rust change. Do not add the decision in Swift.
- Give every interactive or asserted element a stable accessibility identifier.

## Testing policy

Do not re-test Rust behavior in Swift. Validation, navigation rules,
dirty/discard flows, stale scopes, search and sorting, persistence policy, and
counter arithmetic belong to Rust tests. A Swift test that fails only because
Rust behavior changed is in the wrong layer.

- **Swift Testing:** test adapters and integration. Examples are the render
  callback refreshing the view on the main actor, dispatch and conversion
  errors becoming visible, startup failure and retry forwarding, callback
  ownership without retain cycles, platform paths, logging, and parent
  composition copying the displayed scope IDs. A real-core test may send one
  event to prove a round trip; it must not assert a rule table.
- **Snapshot tests:** build view values by hand in Swift, without the Rust
  core. Take one snapshot per meaningful visual state. Fix the device, color
  scheme, locale, and content size category. Inspect every new image or diff.
  Never record a new baseline just to make a failing test pass.
- **XCTest UI tests:** use a few smoke flows that exercise the real core. Show
  that UI events reach Rust, its views reach the screen, and data persists
  across relaunch. Do not build edge-case matrices in UI tests.

Run the checks for each change and report any that could not run. Inside the
devenv shell, `ios-bindings debug` rebuilds the Rust core and `ios-test` runs
all iOS tests (Swift Testing, snapshots, and UI tests) on a booted simulator.

## Xcode project

Change targets, files, packages, and build settings only in
`apple/Kreatyna/project.yml`, then run `apple-generate`. The generated
`Kreatyna.xcodeproj` is not committed; never edit it. New Swift files in a
target's folder need no project change.
