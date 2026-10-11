---
name: plan-reviewable-epics
description: Plan a larger feature in this repository as an epic of small vertical slices (SQLite → Rust → UniFFI → SwiftUI in one change), drawn as a dependency graph on a self-contained HTML page, then deliver one slice per jj change. Use when work spans several behaviors or screens; ordinary small fixes need no epic.
---

# Plan reviewable epics

Optimize for the human reviewer's ability to understand one change at a time.
Build a feature gradually through thin vertical slices; never implement the
whole epic in one change.

## Start from an accepted brief

If the intended behavior is still unclear, run
[run-product-discovery](../run-product-discovery/SKILL.md) first and plan from
its brief and mockups. Cite the brief's `D-xx` decisions in slices instead of
restating them.

Inspect the current code and `AGENTS.md`, then write a short architecture
section: components and their responsibilities, state ownership, data flow,
new concepts, and decisions that affect slice boundaries. Stay at the level of
concepts and contracts; function-level design belongs to each slice's plan.

When the user names an unfinished branch or earlier work as a reference, mine it
for lessons. Plan against the current baseline, and do not merge, restore, or
remove that work without the user's instruction.

Ask the user about tensions the brief leaves open (for example a rule that
early slices cannot satisfy yet) before fixing slice boundaries. Record the
answers as epic-level decisions.

## Slice vertically

A slice is one user-visible behavior delivered through every layer it needs in
one change: migration and Diesel models, Rust domain and widget, UniFFI types,
SwiftUI, and tests at each layer (Rust widget tests per
[write-widget-tests](../write-widget-tests/SKILL.md), Swift tests per
[build-swift-presentation](../build-swift-presentation/SKILL.md), and demo
evidence of the running app). After each slice the app builds, runs, and shows
something new.

- **Make it as small as possible while still demoable.** "Create a recipe with
  only a title" is a slice; "the recipe editor" is an epic.
- **Stubs are welcome when they are visible.** A disabled button, a no-op
  toolbar item, or a placeholder section ("Steps — coming soon") lets a slice
  ship a screen before its parts exist. Name every stub on the slice card and
  the slice that replaces it. Never fake business logic: no hard-coded data
  pretending to be persisted, and no Swift-side decisions standing in for Rust.
- **Split by sub-behavior, never by layer.** If a slice is too big, cut the
  behavior thinner (name only → amount and unit → custom units). Do not cut it
  into "the Rust part" and "the Swift part".
- **Introduce each concept in the first slice that needs it.** A table, widget,
  navigation rule, or graph algorithm arrives together with the behavior that
  uses it, not in a preparatory "foundation" slice.
- **Let rules grow.** When a rule spans data that later slices add (for example
  "a saved recipe needs steps"), each slice enforces what exists so far. Name
  the slice that completes the rule, and say what intermediate data may look
  like until then.
- **Two exceptions may be non-vertical:**
  - *Human gates* are steps only the user can perform, such as signing in to
    an account or granting a permission. The card carries a beginner-friendly,
    step-by-step guide. Before adding one, check whether an agent could do the
    step through a reviewable text change instead (for example XcodeGen for
    Xcode targets); the user prefers that.
  - *Tooling* slices add skills, scripts, or test harness pieces that later
    slices need.

  Keep both kinds rare and small.

## Draw the dependency graph

Turn the slices into a directed acyclic graph. An edge means the later slice
needs the earlier one's behavior or contract; do not add edges merely to force
a sequence. Give each slice a stable ID with an epic prefix (`MAC-04`,
`REC-12`), and never reuse an ID; new slices get the next free number, even if
they run earlier. A slice may live in another repository (such as `krucyfiks`);
it gets its own jj change and review there. Show one suggested order, and mark the
ordering choices that are the user's preference rather than dependencies.
Name the critical path.

## Write the epic as an HTML page

Write the epic to `docs/epics/<epic>/epic.html`, because graphs and status read
better visually than in Markdown. Keep it one self-contained static page that
works offline: inline CSS and JS, no external scripts, fonts, or images.

- **One inline data object** holds the epic: slices with `id`, `title`,
  `kind` (`slice`, `gate`, `tooling`), `status`, `deps`, `outcome`, `layers`
  (db, rust, ffi, swift), `stubs`, `acceptance`, `refs` (brief `D-xx`), and an
  optional `plan` link. Everything visual is rendered from it, so the graph,
  cards, and tables cannot disagree.
- **DAG**: inline SVG laid out by dependency depth (column = longest path from
  a root), with arrows for edges and nodes colored by status. Clicking a node
  scrolls to its card. Edges to slices in another epic appear as a labelled
  external node.
- **Status legend**: `planned`, `ready`, `in-progress`, `awaiting-review`,
  `approved`. Implemented is not approved; only the user approves.
- **Sections**: context and goal (link the brief and mockups), architecture,
  epic-level decisions, the graph, slice cards in the suggested order, open
  questions, and out of scope (cite parked `F-xx` items).
- **Themes**: light and dark through CSS tokens on `:root`, following
  `prefers-color-scheme`, plus a toggle.
- Write cards for a reader who knows the app but not the plan. Prefer concrete
  examples ("400 g Mutti tomatoes → tomato") over shorthand.

Render the page in light and dark (for example with headless Chrome) and look
at the screenshots before handing it over. Check that edges match `deps`,
labels don't overlap, and every slice has a card. The user can annotate it with
`/plannotator-annotate docs/epics/<epic>/epic.html`; process the annotations as
in discovery and redraw. A **Looks good** label on a proposal means the user
accepts it: move that open question into the decisions as an `E-xx` entry
(noting the `Q-xx` it came from), and update the slices that cite it.

Keep the epic page and any brief updates in their own jj change, following
[use-of-jj-vcs](../use-of-jj-vcs/SKILL.md). Approval of the epic establishes
direction; it does not remove the review boundary between slices.

## Deliver one slice at a time

Pick the next slice whose dependencies are approved. In its own jj change:

1. Write its implementation plan at `docs/epics/<epic>/tasks/<ID>.md`, following
   [write-slice-plan](../write-slice-plan/SKILL.md). If the plan has open
   questions, stop for the user's answers before implementing. Do not prewrite
   plans for later slices.
2. Implement the slice across its layers, following the layer skills
   ([forward-only-migrations](../forward-only-migrations/SKILL.md),
   [model-effectful-initialization](../model-effectful-initialization/SKILL.md),
   [build-swift-presentation](../build-swift-presentation/SKILL.md)). Never
   hand-edit generated bindings.
3. Run the relevant checks: Rust tests, formatting and Clippy, binding
   generation, and Swift Testing, snapshots, and UI tests for the platforms the
   slice touches. Capture demo evidence per
   [document-ios-delivery](../document-ios-delivery/SKILL.md). Report any check
   that could not run.
4. Set the slice's status in the epic page's data object to `awaiting-review`.
5. Report the change ID, behavior, validation, and remaining stubs, then stop
   for review, even when another slice is technically independent.

Review fixes stay in the same slice's change. Do not combine slices or squash
them into an epic-sized change. If implementation reveals a missing dependency
or larger scope, revise the graph and discuss the new boundary before absorbing
more work into the current slice.
