# Project working agreement

## Architecture

Build a native iOS application with SwiftUI and UniFFI bindings to a handwritten
Elm/Crux-inspired Rust core. User interactions become Rust events; Rust updates
application state and supplies view data for SwiftUI to render.

Rust owns application state, business rules, validation, and navigation decisions.
Swift renders view data and forwards events without duplicating business logic.
Swift may own transient presentation details such as keyboard focus, scroll
position, and animation progress.

Swift executes platform operations requested by Rust and returns their results
through the core's event/effect interface. Business decisions stay in Rust.

## Ownership and collaboration

- The user exclusively maintains Rust implementation, Rust tests, Cargo
  configuration, and Rust-side binding definitions. Agents may inspect these and
  provide read-only guidance, but must not modify them. Describe required Rust
  changes for the user to implement.
- After explicit handover of the Swift side, agents maintain SwiftUI views, Swift
  integration code, Apple platform adapters, Xcode configuration, and Swift-side
  tests. Do not assume that handover has happened.
- The user manually creates and configures the initial Xcode project. Provide
  beginner-friendly, step-by-step guidance without automating setup. The user is
  new to iOS development and is using Xcode 26.2 with the iOS SDK.

## Initial Xcode choices

- Interface: SwiftUI.
- Language: Swift.
- Testing System: Swift Testing with XCTest UI Tests.
- Storage: None. Persistence is implemented in Rust.

## Persistence

Rust owns SQLite access through Diesel, including schema, migrations, queries,
transactions, and persistence policy. All of this remains user-maintained.

Do not introduce SwiftData, Core Data, or direct database access from Swift.
Swift may supply platform-specific filesystem paths when needed.

## Bindings

Use UniFFI for the Rust-to-Swift interface. Never hand-edit generated bindings.
When the interface needs Rust changes, describe the required contract and let
the user implement those changes.

## Swift-side testing

Agents must maintain:

- Meaningful Swift Testing unit and integration tests for adapters and integration
  behavior.
- XCTest end-to-end UI tests for critical flows exercising the real Rust core.
- Deterministic visual snapshot tests for the presentation layer, configured
  separately from Xcode's initial testing-system selection.

Keep snapshot inputs and rendering conditions controlled. Review visual
differences before accepting new baselines; do not update snapshots merely to
make failing tests pass. Run checks appropriate to each Swift-side change and
report any checks that could not run.

## Live debugging direction

Plan a development-only RPC interface and CLI/library for inspecting Rust state
in the live application and exercising the same event path used by the UI.
Rust implementation remains user-owned. The protocol and transport are future
design work; do not treat them as already specified or implemented.

## Current scope

This agreement documents the intended architecture and responsibilities. App
scaffolding, persistence implementation, bindings, test infrastructure, and the
debugging interface are later tasks. Creating this document does not authorize
automating the user's initial Xcode setup or taking over Rust development.
