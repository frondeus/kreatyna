# R07 — Ingredient detail widget

Status: planned. Dependencies: R03 and R07a. Implementation follows R07a approval.

## Contract

Add a public IngredientDetail widget constructed with IngredientCapabilities and
a complete Ingredient. Initialization is empty; the record is already available.
Its view retains the complete record and typed deletion state: idle, confirmation
with optional IngredientDeleteError, deleting, or deleted. Exact quantities,
unknown values, basis, notes, and unit order are preserved.

Events request editing, request deletion, or route a confirmation child's event.
Outcomes carry edit intent with the complete ingredient, a tagged confirmation
decision, a command-ID-tagged deletion result, or successful deletion with the
ingredient ID. Parent-facing outcomes returned to the widget do nothing.

## Confirmation composition and deletion

The detail owns a Confirmation child only while asking for a decision. Opening
it renders; cancelling removes it, clears the error, and renders idle state.
Tag each child instance and its mapped outcomes with a monotonically increasing,
checked generation. Accept a decision only for the active instance. Consuming a
decision removes that instance before any asynchronous work; ignore obsolete or
duplicate decisions. This prevents a queued decision from a previous prompt
affecting a reopened prompt.

Confirming sets deleting state synchronously, renders, then calls store.delete
with the displayed ingredient ID. Use LatestCommandSlot and CommandExt::latest
for command identity and cancellation, but ignore interaction events while
deleting rather than replacing pending writes. Reject stale and duplicate
deletion completions.

Failure creates a fresh confirmation and retains the typed error in the parent;
the user can retry directly or cancel. Never retry automatically. Success enters
terminal deleted state, renders, then emits the parent deletion outcome once.
Edit intent is available only while idle. Dropping the widget cancels pending
command work, without promising rollback of a submitted deletion.

## Tests and delivery

Use the effect driver to verify initial complete view, edit intent, confirmation
routing/cancel, render-before-delete order, correct ID, both typed errors,
retry/cancel after failure, terminal success, suppressed repeated actions,
stale completion and obsolete confirmation decisions, and drop cancellation.
Use named test_case cases and a local fixture owning widget and effects below
tests; access fields directly when observing intermediate states.

Create a separate R07 jj change with implementation, tests, and this plan.
Run core tests, Clippy with warnings denied, FFI compilation, and formatting
checks; report unrelated formatting failures separately. Mark awaiting review
only after validation. No database, framework, navigation, coordinator, runtime,
bindings, or Swift changes.
