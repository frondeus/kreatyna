# Ingredient catalog — S01 demo

These screens come from the running app on the **iPhone 17 Pro simulator,
iOS 26.3.1** (region Poland, English UI). The real Rust core and a fresh SQLite
database were used. `DemoCaptureUITests` captured them with
`TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1`; every step goes through the normal UI
and event path. Screens are scaled to 1000 px high.

Some states are too brief or cannot be reached with a healthy database:
loading, pending operations, and storage failures. They are shown with the
deterministic snapshot baselines in the [last section](#states-shown-with-snapshot-fixtures).
Those images use hand-built view data, not the running app.

## Create, validate, and browse

| Start | Action | Result |
| --- | --- | --- |
| Fresh database | Launch | Empty catalog ![](demo/01-empty-catalog.png) |
| Empty catalog | Tap **+** | Blank form in a sheet ![](demo/02-create-form.png) |
| Blank form | Save with no name, then enter a name and calories `1,2,3` and save | Rust reports *Required*, then *Enter a number* for calories. The draft is kept. ![](demo/03-validation.png) |
| Validation error | Calories `149`, **Add Unit** `clove` = 5 g | Filled form with a reference unit ![](demo/04-filled-form.png) |
| Filled form | Save, then create *Cucumber* and *Canned tomatoes* | Sorted catalog ![](demo/05-catalog.png) |

## Duplicate name and dirty discard

| Start | Action | Result |
| --- | --- | --- |
| Catalog with *Garlic* | Create `garlic` and save | The store rejects the case-folded duplicate; the draft is kept ![](demo/06-duplicate-name.png) |
| Dirty form | Tap **Cancel** | Rust asks before discarding ![](demo/07-discard-confirmation.png) |
| Discard confirmation | **Discard Changes** | Back on the unchanged catalog |

## Search

| Start | Action | Result |
| --- | --- | --- |
| Catalog | Search `cu` | Only *Cucumber* ![](demo/08-search.png) |
| Search active | Extend to `cuzz` | No-results state ![](demo/09-search-no-matches.png) |

## Detail, edit, delete

| Start | Action | Result |
| --- | --- | --- |
| Catalog | Tap *Garlic* | Pushed detail; unknown nutrition shows `—` ![](demo/10-detail.png) |
| Detail | **Edit**, append ` bulb` | Edit form over the detail ![](demo/11-edit-from-detail.png) |
| Edit form | Save | Returns to its origin, the updated detail ![](demo/12-edited-detail.png) |
| Detail | Tap the trash button | Delete confirmation ![](demo/13-delete-confirmation.png) |
| Confirmation | **Delete** | Back on the list without the ingredient ![](demo/14-after-delete.png) |

## Persistence and tabs

| Start | Action | Result |
| --- | --- | --- |
| Two ingredients saved | Terminate and relaunch with the same database | Catalog restored from SQLite ![](demo/15-after-relaunch.png) |
| Catalog | **Counter** tab, **Increase** | Counter still works under Rust-owned tabs ![](demo/16-counter-tab.png) |

## States shown with snapshot fixtures

These baselines live in
`apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/`.
They were reviewed visually before they were committed.

| State | Snapshot |
| --- | --- |
| Startup in progress | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/initializing.1.png) |
| Startup failed, Retry | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/initializationFailed.1.png) |
| List loading | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/list-name-status.loading.png) |
| List load failed, Retry | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/list-name-status.failed.png) |
| Refresh failed over loaded rows | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/list-name-status.refreshFailed.png) |
| Saving | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/formSaving.1.png) |
| Field and unit validation | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/formValidationIssues.1.png) |
| Deleting | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/detail-name-deletion.deleting.png) |
| Delete failed | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/detail-name-deletion.deleteFailed.png) |
| Dispatch rejected by the adapter | ![](../../../apple/Kreatyna/KreatynaTests/Snapshots/__Snapshots__/ScreenSnapshotTests/adapterError.1.png) |

## Checks

All runs used the iPhone 17 Pro simulator on iOS 26.3.1 with Xcode 27.0.

- `KreatynaTests` passed, covering Swift Testing adapter and plumbing tests and
  20 snapshot tests. The snapshots passed in two consecutive runs after
  recording.
- `KreatynaUITests` passed: three real-core smoke flows and the launch tests.
- `DemoCaptureUITests` passed and produced the screenshots above.

Not shown: a save failure caused by storage. Its banner uses the same component
as the duplicate-name error.
