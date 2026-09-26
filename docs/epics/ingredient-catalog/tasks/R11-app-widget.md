# R11 — App widget and Rust-owned tabs

Status: approved. Change: `yrxqlxol`. R10 and R09a are approved.

## Composition contract

Add `AppWidget`, retaining `IngredientCatalog` and `Counter` under Rust-owned
Ingredients and Counter tabs. Ingredients is selected initially. The public app
event, outcome, and view types nest child contracts, expose both retained child
views, and include the selected tab, current activation scope, and counter
loading/readiness state.

Compose ingredient, counter, and app-render capabilities under `AppCapabilities`
and `AppEffect`. The test fixture combines their effect streams using the existing
channel capacities. Initialize both children concurrently exactly once and accept
their completions in either order.

## Lifetimes and tab behavior

Each actual tab activation gets a fresh interaction scope. Only events carrying
the current scope and targeting the selected tab are forwarded. Child commands
use separate app-owned lifetime scopes, so accepted saves, deletions, renders,
and counter writes may finish while their tab is hidden. Dropping the app cancels
all outstanding commands.

Tab changes retain both child widgets and render once. Leaving Counter sends its
existing `StopBackground` event synchronously, invalidating queued ticks; return
does not restart the background automatically. Selecting the current tab does
nothing. Counter owns and exposes its loading/readiness state. App routing uses
that state to disable interactions until initialization has been accepted,
preventing a delayed load from overwriting user input.

Ingredient navigation, search, drafts, confirmations, pending operations, and
the catalog's internal screen scopes remain owned by `IngredientCatalog`.

## Validation and delivery

Use effect-driver tests for both initialization orders, single initialization,
tab rendering, branch and stale-scope rejection, readiness gating, retained
ingredient state, hidden save/delete success and failure, counter background
cancellation and explicit restart, accepted writes after departure, and app-drop
cancellation.

Run core tests, Clippy with warnings denied, FFI compilation, and formatting
checks. Report unrelated formatting failures separately. Runtime wiring, UniFFI
types, database changes, and Swift integration remain later tasks.

Implementation validation completed on 2026-09-26:

- `cargo test -p kreatyna --offline`
- `cargo test -p kreatyna-ffi --offline`
- `cargo clippy -p kreatyna -p kreatyna-ffi --all-targets --offline -- -D warnings`
- targeted `rustfmt --check` for every R11 Rust file

The workspace-wide formatting check still reports only the existing import-order
differences in `tests/background.rs`, `tests/composition.rs`, and
`kreatyna-ffi/src/lib.rs`.
