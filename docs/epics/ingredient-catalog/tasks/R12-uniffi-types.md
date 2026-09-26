# R12 — UniFFI app events and views

Status: awaiting review. Change: `mtrtrxrk`. Companion krucyfiks change:
`qpmqyrzn`. R11 is approved.

## Contract

Expose the Rust app's event and view tree as plain UniFFI records and enums. A
fixed-width `ScopeId` carries each Rust-issued tab or screen activation identity.
Concrete nested event variants reconstruct the core's `Scoped<T>` values; the
core continues to check the supplied identity and active state. Conversion never
allocates or substitutes a scope.

Ingredient IDs cross as canonical UUID text. Quantities cross as exact decimal
text, nullable values stay nullable, editable draft strings remain untouched,
and collection indices and counts use checked `u64`/`usize` conversions. Loading,
validation, persistence failure, confirmation, navigation, and counter readiness
states remain typed.

The companion krucyfiks change gives `CommandScopeId` a `u64` representation and
lossless conversions. It remains a process-local comparison value: reconstructing
one does not create a scope or grant access to one. Scope cancellation and latest
command behavior do not change.

## Swift and runtime boundary

Swift child views will emit local event values. Their parent composition closures
capture the scope IDs from the displayed view and wrap those events before one
central dispatch call. The bridge introduces no runtime-backed event targets or
per-screen dispatch objects.

This task defines data and checked conversions only. R13 owns the app facade,
central dispatch, database and capability workers, startup failure, rendering,
and shutdown. Preserve the existing CounterFfi entry point. Do not edit generated
bindings or add Swift implementation.

## Validation

Test scope round trips, nested scope preservation, stale-event rejection through
the existing widgets, all catalog screen shapes and both edit origins, counter
readiness, typed loading and write failures, checked IDs and indices, exact
decimal representation, nullable values, and unchanged draft text.

Run framework, core, and FFI tests; Clippy with warnings denied; formatting
checks; both configured iOS builds; and Swift binding generation. Inspect the
generated Swift declarations for nested event variants and scope records. Report
unrelated formatting failures or unavailable platform checks separately.

Validation completed on 2026-09-26:

- `cargo test --workspace --offline` in krucyfix
- `cargo clippy --workspace --all-targets --offline -- -D warnings` in krucyfix
- `cargo fmt --all -- --check` in krucyfix
- `cargo test -p kreatyna -p kreatyna-ffi --offline`
- `cargo clippy -p kreatyna -p kreatyna-ffi --all-targets --offline -- -D warnings`
- `devenv shell ios-bindings -- debug` for device, simulator, and Swift bindings

The generated device and simulator Swift files match and expose `ScopeId` as a
`UInt64` record plus nested scoped app/catalog event cases. The workspace-wide
vibetyna formatting check still reports only the existing import-order differences
in `tests/background.rs` and `tests/composition.rs`.
