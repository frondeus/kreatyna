# R06 — Ingredient list widget

Status: awaiting review. Change: `llyullvx`. R05 is approved.

Add a Rust `IngredientList` with its own event, outcome, and view types. It loads
complete records through the ingredient store capability, keeps them private,
and exposes sorted ID/name rows. Search matches Unicode-folded ingredient names
only, retaining the user's exact query text. An empty catalog and a filtered list
with no matches remain distinct through the total count.

Initial load and refresh render their pending states before requesting the store.
Refresh keeps cached rows and query visible, including after a failed request.
The same refresh event retries failed loads. A command ID identifies the pending result so stale
outcomes cannot replace newer data; the command slot cancels work when the widget
is dropped. Create, detail, and edit intents are outcomes for a later parent to
handle. Selection uses only currently visible records and carries complete data.

`Widget::init` now takes `&mut self` in the shared krucyfix change `nuzpozyw`,
so the widget starts its tracked load directly. `Command::latest` and
`LatestCommandSlot` in that shared change replace older loads and own both
cancellation and generation checking. Completed results remain valid until
replacement or explicit stop. New refreshes cancel earlier refreshes; queued
results from older generations are ignored.

Use the existing effect driver to test load and render order, errors/retry, search
and sort, refresh/cache behavior, stale results, cancellation, and navigation
intents. Run core tests, FFI compilation, Clippy, and formatting checks. R10 owns
catalog coordination and updates after writes; this task adds no database,
navigation, runtime, binding, or Swift implementation.

Validation: all core tests, including 17 list-widget scenarios, pass. Clippy
with warnings denied, FFI compilation, and changed-file formatting pass. The
krucyfix workspace tests, Clippy, and formatting pass with its shared API change.
Workspace formatting still reports existing import-order differences in
unrelated counter and FFI files.
