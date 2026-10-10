# S01 — Swift integration and presentation

Status: awaiting review. Change: `nqouwrvs`. The user handed over the Swift
side for this task.

## Contract

Replace the old Counter-only Swift adapter with one over `AppFfi`. Swift
renders `AppView` and forwards `AppEvent` values, following
[build-swift-presentation](../../../../.agents/skills/build-swift-presentation/SKILL.md).
It adds no business, validation, or navigation decisions.

- `AppModel` owns the facade, holds the latest `AppView`, refreshes it from
  the render callback on the main actor, and shows dispatch failures. The
  render callback references the model weakly, so there is no ownership cycle
  (P08).
- The database stays at `Application Support/Kreatyna/counter.sqlite`, so
  existing data survives. UI tests keep their per-run database directory.
- Child views send local events through closures. Parents wrap them in the
  scope IDs of the displayed view.
- Initialization renders progress, failure with Retry, or the running app.
- Native tabs select Ingredients or Counter through `SelectTab`.
- The list is the navigation root. A detail is pushed. Forms appear as
  sheets that cannot be dismissed interactively. Back, Cancel, and dialog
  dismissal only send events.
- Swift maps typed load, save, delete, startup, and validation errors to text.
- Text fields and search show typed text at once. `TextEcho` keeps
  keystrokes that Rust has not rendered yet, and it accepts values that
  Rust changes on its own.

## Tests

- Swift Testing: adapter round trip, error visibility, startup failure,
  callback ownership, database location, logging, and scope composition.
- Snapshot tests (swift-snapshot-testing) over hand-built view values for
  each visual state of the startup, list, detail, form, and counter screens.
- XCTest smoke flows on the real core: create, edit, and delete; relaunch
  persistence; both tabs.
- `DemoCaptureUITests` captures the demo screenshots. It runs only with
  `TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1`.

Rust tests own all behavior rules. Swift does not repeat them.

## Exclusions

No Rust behavior changes, no shutdown completion work (S03–S05), and no
hand-edited generated bindings.

## Delivery

`docs/epics/ingredient-catalog/demo.md` with simulator screenshots, following
`document-ios-delivery`.
