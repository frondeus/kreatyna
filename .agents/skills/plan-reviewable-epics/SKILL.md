---
name: plan-reviewable-epics
description: Plan larger features and feature sets in this repository through high-level architecture, a DAG of small reviewable tasks, and a separate implementation plan and jj change for each task. Use when the work spans multiple concepts or components; ordinary small fixes do not need an epic plan.
---

# Plan reviewable epics

Optimize for the human reviewer's ability to understand one change at a time.
Build larger features gradually; do not implement the whole epic in one change.

## Agree on architecture first

If the intended behavior is still unclear, run
[run-product-discovery](../run-product-discovery/SKILL.md) first and plan from
its brief.

Start with the intended behavior, scope, and non-goals. Inspect the current code
and project agreement, then describe the major components, their responsibilities,
state ownership, interfaces, and data flow. Identify new concepts and unresolved
decisions that affect task boundaries. Keep this discussion at the architectural
level rather than designing every function in advance.

When the user identifies an unfinished branch or earlier implementation as a
reference, inspect it for lessons and reusable ideas. Plan against the current
baseline. Reference work is not implicitly approved implementation; do not merge,
restore, squash, or remove that branch without the user's instruction.

## Split the epic into a dependency graph

Turn the architecture into small, atomic tasks forming a directed acyclic graph
(DAG). Record each task's stable ID, concrete outcome, scope, direct dependencies,
and acceptance criteria. Show the dependency edges and a valid execution order;
do not invent dependencies merely to force independent tasks into a chain.

Each task should introduce one coherent behavior or concept that can be reviewed
without understanding the entire epic. Include the tests and minimal integration
needed to verify that task. Aim for a buildable, testable state after each task.
If a task cannot be verified until several later tasks land, revisit its boundary
or extract the small contract it needs first.

For example, persistence schema and Diesel models may form one task, each widget
another, and Rust navigation a separate task introducing that concept. These are
examples of boundaries, not a required decomposition for every feature. Avoid both
giant changes spanning unrelated concepts and arbitrary one-file tasks that split
a single coherent change.

Keep Rust implementation and Rust-side bindings separate from Swift integration
tasks. Swift tasks depend on review and approval of their Rust contracts, and
remain subject to the project's explicit Swift handover requirement.

Present the architecture and DAG for human review before implementation. Approval
of the epic establishes direction; it does not remove the review boundary between
individual tasks.

## Plan and deliver one task at a time

For the next task whose dependencies have been approved, write its own
implementation plan before coding. Cover the relevant files and types, contracts
or state transitions, implementation steps, meaningful validation, and explicit
scope exclusions. Resolve open decisions that change the agreed architecture or
task boundary. Keep detailed plans for later tasks deferred until their contracts
are sufficiently settled.

Each task gets its own `jj` change, following
[use-of-jj-vcs](../use-of-jj-vcs/SKILL.md). Keep its implementation plan and relevant
epic-status updates with that task. Do not combine separate tasks into one change,
or squash completed tasks together to present the whole epic as one review.
Review fixes belong to the same logical task and follow the agreed review-round
workflow.

Implement only the selected task, run its relevant checks, and report its change
ID, behavior, validation, and remaining limitations. Stop for human review before
starting another task, even when another DAG node is technically independent.
Track planned, ready, in-progress, awaiting-review, and approved tasks separately
so implemented work is not mistaken for approved work.

If implementation reveals a missing dependency or a larger scope, revise the DAG
and discuss the changed boundary before absorbing another task into the current
change. Preserve the small review surface as the epic evolves.
