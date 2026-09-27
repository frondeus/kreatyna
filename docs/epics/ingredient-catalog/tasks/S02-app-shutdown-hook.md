# S02 — Application shutdown hook propagation

Status: awaiting review. Change: `mtqllwxk`. Depends on approved krucyfiks change
`mxxtlnuy`.

## Contract

Forward the generic `Widget::begin_shutdown` hook through `AppShell`,
`AppWidget`, and `IngredientCatalog`. Counter stops its recurring clock command;
finite commands already returned by any child remain owned by the runtime and
continue to completion. Existing command scopes remain valid so accepted saves,
deletes, loads, writes, and their follow-up outcomes can drain.

`AppShell::can_begin_shutdown` reports false while operational preparation is in
progress, forwards readiness after the app exists, and stores no runtime lifecycle
state. The runtime continues driving preparation outcomes until the query permits
it to invoke the shutdown hook exactly once. The prepared app's initialization
command is not polled before the hook is invoked.

This change does not close or join the database worker, report persistence
failures, track foreign callbacks, or expose the final FFI shutdown API. Those
belong to S03–S05. Counter write ordering remains unchanged.

## Validation

Verify that the app-level hook stops Counter's recurring command, an accepted
Counter write still completes, and an ingredient save already in progress still
applies its result and follow-up render during shutdown preparation.
