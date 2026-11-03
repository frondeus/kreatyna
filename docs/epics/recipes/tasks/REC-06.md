# REC-06 — Search recipes

Tracked in beads: `vibetyna-hrx.6`. Changes: four, listed in Steps.
Dependencies: none on the card. This plan builds on the REC-01 to REC-05 code.

## Open questions

None. Each rule copies the ingredient search on the Mac (MAC-08) or the
catalog list. The one new choice is the type of "no matches", which follows
the designing-types rule; the Decisions table lists it, so you can object to
it before you accept the plan.

## Decisions

### Decided in the plan

| Rule | Follows from |
|---|---|
| The search field sits at the top of the recipe list column, with the prompt "Search recipes". | MAC-08/Q2, the recipes mockup (section 1) |
| Rust keeps the search text. A recipe matches when its title key contains the search key: `name_key(title).contains(name_key(search))`. "sha" and "SHA" both match "Shakshuka". An empty search shows every recipe. | The REC-06 card: "Rust filters by the title key, like the catalog". |
| When recipes exist but none matches, the list says "No matches for “shx”". When no recipe exists, it says "No recipes yet", with or without a search. | MAC-08 |
| A search keeps the open read view or editor. The hidden row is not highlighted. Clearing the search highlights it again. | MAC-08/Q1 |
| A row click opens only a row that the search shows. | The catalog: `contains_visible`. |
| The search never hides the draft row of the create editor. | The catalog: `draft_row` ("The search never hides it"). |
| A reload after Save or Delete keeps the search. | The search lives in the list widget, which lives as long as the section. |
| Esc in the field clears it. No ⌘F. | MAC-08, deviation 1: ⌘F does not reach this field on macOS 27. |
| **No-matches type:** a new `RecipeListStatus::NoMatches` variant, not the catalog's `Ready { rows, total_count }`. `rows` and `total_count` are two fields that can disagree. | designing-types, rule 4 |
| iOS does not change. | E-04 |

## Outcome

The list has Pasta and Shakshuka. You click the search field above the list
and type "sha". The list shows only Shakshuka. You type "x". The list says
"No matches for “shax”". You press Esc. The field clears, and both rows come
back. If Shakshuka was open, its read view stays in the pane the whole time.

## Terms

- **Title key:** `name_key(title)`, the lower-case, trimmed form of a title.
  The store sorts and checks uniqueness by it (REC-02).
- **Recipe list:** the `RecipeList` widget. It loads the recipes and now also
  keeps the search text and filters the rows.
- **`MacSearchField`:** the Mac search field above a list. It wraps AppKit's
  `NSSearchField`, because SwiftUI's `.searchable` cannot sit in the middle
  column of a split view (MAC-08/Q2).
- **`TextEcho` / `rustSearchText`:** the Swift helper that sends each typed
  change to Rust and shows Rust's text, without echo loops (MAC-08).

## Design

The target is section 1 of the
[recipes mockups](../../../tasks/recipes/mockups.html): "Search recipes"
above the list. The field and the "No matches" state copy the Mac ingredient
list (MAC-08, see `docs/epics/mac-shell/tasks/MAC-08-demo/`). No new mockup;
the demo shows the result.

## Changes by layer

### db

No change.

### rust (`crates/kreatyna/src/recipes/list.rs`)

The list keeps every loaded recipe with its title key, and the view shows the
ones that match (*sketch*):

```rust
pub enum RecipeListEvent {
    Refresh,
    /// The search text as typed. Rust filters by its key.
    Search(String),
}

pub enum RecipeListStatus {
    Loading,
    Failed(RecipeLoadError),
    /// The recipes that match the search, by title key. Empty only when no
    /// recipe is saved: "No recipes yet".
    Ready { rows: Vec<RecipeListRow> },
    /// Recipes are saved, but none matches the search.
    NoMatches,
}

pub struct RecipeListView {
    /// The search text as typed.
    pub search: String,
    pub status: RecipeListStatus,
}

// Inside the list.
enum ListState {
    Loading,
    Failed(RecipeLoadError),
    /// Every loaded recipe, by title key. Each entry keeps its key.
    Ready(Vec<Entry>),
}
```

| Event or outcome | Result |
|---|---|
| `Search(text)` | The list keeps `text` and renders. No store call. |
| `LoadFinished(Ok)` | Keeps every recipe; the view filters by the search. |
| `contains(id)` | True only for a row that the search shows. |

`RecipeListStatus::Ready { rows }` keeps its meaning for an empty search, so
the REC-01 to REC-05 tests do not change.

**`section.rs`** — no change: `select` already asks `list.contains(id)`, the
pane does not depend on the search, and `RecipeSectionEvent::List` already
routes `Search`.

### ffi (`crates/kreatyna-ffi/src/app_types/recipes.rs`)

| Change |
|---|
| `RecipeListEvent::Search { query: String }`, as for ingredients. |
| `RecipeListStatus::NoMatches`. |
| `RecipeListView { search, status }`. |

### swift — shared (`apple/Kreatyna/Shared/`)

| File | Change |
|---|---|
| `RustSearchable.swift` | `rustSearchText` takes `send: (String) -> Void`, so the recipe list can use it. The iOS ingredient list passes `{ send(.search(query: $0)) }`. |

### swift — Mac (`apple/Kreatyna/KreatynaMac/`)

| File | Change |
|---|---|
| `MacSearchField.swift` | Takes `prompt`, `identifier`, and `send: (String) -> Void`. |
| `MacIngredientList.swift` | Passes "Search ingredients" and `ingredientSearch`, as today. |
| `MacRecipeList.swift` | The field above the list: "Search recipes", `recipeSearch`. The `.noMatches` case shows "No matches for “…”" (`recipeListNoMatches`). |

### tests

| Layer | File | Tests |
|---|---|---|
| Rust widget | `tests/recipe_list.rs` | "sha" and "SHA" show only Shakshuka. An empty search shows every row. A search with no match gives `NoMatches`; with no recipes it gives `Ready { rows: [] }`. A search does not call the store. A reload keeps the search. A search during the first load applies to its result. |
| Rust widget | `tests/recipe_section.rs` | A search keeps the open read view and its id in `selected`. A click on a row that the search hides is ignored. The draft row stays during a search. |
| ffi | `app_types/tests.rs` | `Search { query }` converts; `NoMatches` and `search` convert. |
| Swift adapter | `SharedTests/AdapterPlumbingTests.swift` | `list(.search(query:))` routes to the recipes list. |
| Snapshots (Mac) | `MacScreenSnapshotTests.swift` | New: `searchedRecipeList` ("sha", one row) and `recipeListNoMatches` ("shx"), light and dark: 4 new images. The field changes every recipe list column, so I record the 14 recipe images again (7 tests, light and dark) and compare each with the old one by eye. MAC-08 found that the tolerance can hide a new field, so I delete them before the run. |
| UI smoke (Mac) | `MacRecipeUITests.swift` | `testSearchNarrowsRecipes`: create Pasta and Shakshuka; click the field; type "sha": one row; type "x": "No matches"; Esc: two rows. |
| Demo (Mac) | `MacDemoCaptureUITests.swift` | `testCaptureREC06Demo`: `01-search-sha`, `02-no-matches`. |

### docs

| File | Change |
|---|---|
| `docs/epics/recipes/epic.html` | REC-06 card: `plan: "tasks/REC-06.md"`. |
| `docs/epics/recipes/tasks/REC-06-demo/` (new) | The 2 demo screenshots. |

## Steps

The slice is about 1,000 lines of diff without images, so it is four jj
changes, each under 1,000 lines. You review each change with Rust before the
next one starts. A change with no `.rs` file gets a report and no review.

### Change 1 — REC-06: plan (about 250 lines, no review)

This plan, the REC-06 card on `epic.html`, and the beads.

### Change 2 — REC-06: the recipe list filters by search (about 400 lines, review)

1. Add `Search`, `NoMatches`, `search`, and the internal `ListState`.
2. Make `contains` use the visible rows.
3. Extend `tests/recipe_list.rs` and `tests/recipe_section.rs`.
4. Map `NoMatches` and ignore `search` in the FFI, so the change builds alone. Change 3 adds them to the FFI.
5. Run checks 1 to 3 and check 7.
6. Send the guided review.

### Change 3 — REC-06: search in the FFI (about 150 lines, review)

1. Add the FFI fields and cases, and their tests.
2. Run `mac-bindings debug`.
3. Update the Swift call sites so the build stays green. The Mac shows the
   empty state for `.noMatches` until change 4.
4. Run checks 1 to 5 and check 7.
5. Send the guided review.

### Change 4 — REC-06: search recipes on the Mac (about 250 lines, report)

1. Generalize `rustSearchText` and `MacSearchField`.
2. Add the field and the no-matches state to `MacRecipeList`.
3. Add the adapter test and the snapshot tests. Delete the 14 recipe images,
   record all images, and compare the old and new images by eye.
4. Add the UI test and the demo method.
5. Run checks 4, 5 and 7.
6. Capture the demo (check 6).
7. Fill in Validation in this plan.
8. Send the demo and the report to the REC-06 Inbox thread.

## Checks

Test scope: Rust in full for each change. On the Mac, all unit and snapshot
tests, and the UI tests of the Recipes section only (`--ui MacRecipeUITests`).
The full suite runs once per cycle, in the cycle's full-suite step.

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. 4 new images and 14 recorded again, each checked by eye. No ingredient image changes. |
| 5 | `devenv shell -- ios-app-build` | Builds. iOS is parked, so only the build runs. |
| 6 | `capture-demo.sh mac …/testCaptureREC06Demo docs/epics/recipes/tasks/REC-06-demo` | `01-search-sha.png`, `02-no-matches.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| The search matches titles only, not ingredient lines or steps. | Not planned in this epic. |

- No ⌘F (MAC-08, deviation 1).
- The search text is not saved: it is empty after a relaunch, as for
  ingredients.
- No recipe screens on iOS (E-04).

## Validation

Not run yet.
