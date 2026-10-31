# R13 — App runtime baseline

Tracked in beads: `vibetyna-flx.15`. Change: `uqmquulk`. R12 and its companion krucyfiks
change are approved.

## Baseline

Expose `AppWidget` through UniFFI by following the existing `CounterFfi` runtime
adapter directly. `AppFfi` owns `Arc<Runtime<AppWidget>>` and its runtime task
handle. Construction is synchronous, starts SQLite in a detached task, and starts
the widget runtime immediately. `shutdown` signals the runtime without waiting;
dropping the facade signals shutdown and aborts the runtime task.

The app reuses the existing render and clock callback interfaces. It exposes the
R12 app view and event tree through `view` and `dispatch`. Checked event and view
conversion are the only boundary behavior that has no counter equivalent.
The ingredient catalog chooses its initial nutrition basis. It is not platform
configuration supplied through UniFFI or app-level constructor configuration;
a future settings capability can replace this local default.

One SQLite connection services the counter and ingredient capability inboxes.
This combined worker is the minimum database adaptation needed by the composed
app; each inbox otherwise uses the same generated-client and detached-worker
pattern as `CounterFfi`.

## Deliberate boundary

This change does not attempt to improve startup error reporting, task ownership,
shutdown completion, database draining, or unexpected worker termination. Those
concerns will be recorded and discussed separately, then addressed one at a time
only after agreeing on solutions that fit krucyfiks.

Keep `CounterFfi` available. Add no Swift screens, schema changes, business-rule
changes, or framework redesign in this baseline.

## Validation

- Core and FFI tests pass, including the focused public-facade baseline test.
- Clippy passes for all kreatyna and kreatyna-ffi targets with warnings denied.
- iOS device and simulator libraries compile, and UniFFI generates identical
  Swift declarations for both targets. The generated API exposes synchronous
  `app`, `view`, `dispatch`, and `shutdown` entry points.
- The formatting check reports only the existing import-order differences in
  `crates/kreatyna/tests/background.rs` and
  `crates/kreatyna/tests/composition.rs`; this change leaves them untouched.
