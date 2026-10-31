# R07a — Reusable confirmation widget

Tracked in beads: `vibetyna-flx.7`. Change: `pyzppluw`. R06 is approved.

## Contract

Public `confirmation` module provides `Confirmation`, `ConfirmationEvent`,
`ConfirmationOutcome`, and `ConfirmationView`. Construction starts at
`AwaitingDecision`; initialization returns an empty command.

The first Confirm or Cancel event synchronously records
`Resolved { decision }` and emits Confirmed or Cancelled once. Further events
and outcomes passed back to the widget do nothing, even before the first command
is polled. Reopening requires a new instance; there is no reset operation.

The widget has no capabilities. Parents own presentation, action context,
pending operations, errors, and retry policy. Parents render when opening a
confirmation and when handling its decision. Swift supplies displayed wording.

## Validation and delivery

Named test_case scenarios cover initial state, empty initialization, both decisions,
repeated and opposite events before and after polling, and ignored outcomes in
both states. No fixture abstraction is needed.

Run core tests, Clippy with warnings denied, FFI compilation, and formatting
checks. Report unrelated formatting failures separately. Stop for review before
R07; no framework, persistence, navigation, runtime, bindings, or Swift changes.

Validation: all core tests pass, including five confirmation scenarios. Core and
FFI Clippy with warnings denied, FFI compilation, and changed-file formatting
pass. Workspace formatting reports existing import-order differences in
kreatyna/tests/background.rs, kreatyna/tests/composition.rs, and
kreatyna-ffi/src/lib.rs; these unrelated files were left unchanged.
