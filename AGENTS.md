# Project working agreement

## Communication

All communication with the user goes through the Plannotator Inbox: questions,
plans, review requests, reports, and blockers. Follow
[use-plannotator-inbox](.agents/skills/use-plannotator-inbox/SKILL.md). The
chat is only for a short line that points to the Inbox message. Code review
depends on the Rust files a change edits: none means no review, and otherwise
a guided review of the Rust side and the demos goes to the Inbox (see
"Code review: count the Rust files first" in that skill). The user does not
review Swift.

## Work tracking

Beads (`bd`) owns status and dependencies for epics, slices, and bugs. Epic
pages and plans in `docs/epics/` are design documents: context, decisions,
mockups, and the implementation plan of each slice. They do not record status.
A slice runs through the `mol-slice` formula: start it with
[start-slice.sh](.agents/skills/plan-reviewable-epics/scripts/start-slice.sh),
and `bd ready` shows its next step. Before a jj change is finished, run
`bd export -o .beads/issues.jsonl`, so jj records the beads with the code.

## Architecture

Build a native Apple application, macOS first and iOS second, with SwiftUI and
UniFFI bindings to a handwritten
Elm/Crux-inspired Rust core. User interactions become Rust events; Rust updates
application state and supplies view data for SwiftUI to render.

Rust owns application state, business rules, validation, and navigation decisions.
Swift renders view data and forwards events without duplicating business logic.
Swift may own transient presentation details such as keyboard focus, scroll
position, and animation progress.

Swift executes platform operations requested by Rust and returns their results
through the core's event/effect interface. Business decisions stay in Rust.

## Ownership and collaboration

- Agents may maintain Rust implementation, Rust tests, Cargo configuration, and
  Rust-side binding definitions as part of requested work.
- The Swift side was permanently handed over on 2026-10-11. Agents maintain
  SwiftUI views, Swift integration code, Apple platform adapters, Xcode
  configuration (including new targets), and Swift-side tests on macOS and iOS,
  as part of the vertical slices they deliver.
- The user prefers not to work in Xcode. XcodeGen generates the Xcode project
  from `apple/Kreatyna/project.yml`, so agents change it as reviewable text.
  Edit `project.yml`, never the generated `.xcodeproj` (it is not committed),
  and run `apple-generate` after each edit. The user is new to Apple
  development and is using Xcode 27.0.

## Initial Xcode choices

- Interface: SwiftUI.
- Language: Swift.
- Testing System: Swift Testing with XCTest UI Tests.
- Storage: None. Persistence is implemented in Rust.

## Persistence

Rust owns SQLite access through Diesel, including schema, migrations, queries,
transactions, and persistence policy.

Do not introduce SwiftData, Core Data, or direct database access from Swift.
Swift may supply platform-specific filesystem paths when needed.

## Bindings

Use UniFFI for the Rust-to-Swift interface. Never hand-edit generated bindings.
Agents may update Rust-side binding definitions when the interface changes.

## Swift-side testing

Agents must maintain:

- Meaningful Swift Testing unit and integration tests for adapters and integration
  behavior.
- XCTest end-to-end UI tests for critical flows exercising the real Rust core.
- Deterministic visual snapshot tests for the presentation layer, configured
  separately from Xcode's initial testing-system selection.

Keep snapshot inputs and rendering conditions controlled. Review visual
differences before accepting new baselines; do not update snapshots merely to
make failing tests pass. Run checks appropriate to each Swift-side change and
report any checks that could not run.

## Live debugging direction

Plan a development-only RPC interface and CLI/library for inspecting Rust state
in the live application and exercising the same event path used by the UI.
The protocol and transport are future design work; do not treat them as already
specified or implemented.

## Current scope

This agreement documents the intended architecture and responsibilities. App
scaffolding, persistence implementation, bindings, test infrastructure, and the
debugging interface are later tasks. The user created the initial Xcode
project by hand; later Xcode changes belong to agents (see above).

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:1105d646 -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/core-concepts/sync-concepts.md for details and anti-patterns.

## Agent Context Profiles

The managed Beads block is task-tracking guidance, not permission to override repository, user, or orchestrator instructions.

- **Conservative (default)**: Use `bd` for task tracking. Do not run git commits, git pushes, or Dolt remote sync unless explicitly asked. At handoff, report changed files, validation, and suggested next commands.
- **Minimal**: Keep tool instruction files as pointers to `bd prime`; use the same conservative git policy unless active instructions say otherwise.
- **Team-maintainer**: Only when the repository explicitly opts in, agents may close beads, run quality gates, commit, and push as part of session close. A current "do not commit" or "do not push" instruction still wins.

## Session Completion

This protocol applies when ending a Beads implementation workflow. It is subordinate to explicit user, repository, and orchestrator instructions.

1. **File issues for remaining work** - Create beads for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **Handle git/sync by active profile**:
   ```bash
   # Conservative/minimal/default: report status and proposed commands; wait for approval.
   git status

   # Team-maintainer opt-in only, unless current instructions forbid it:
   git pull --rebase
   git push
   git status
   ```
5. **Hand off** - Summarize changes, validation, issue status, and any blocked sync/commit/push step

**Critical rules:**
- Explicit user or orchestrator instructions override this Beads block.
- Do not commit or push without clear authority from the active profile or the current user request.
- If a required sync or push is blocked, stop and report the exact command and error.
<!-- END BEADS INTEGRATION -->

<!-- BEGIN BEADS CODEX SETUP: generated by bd setup codex -->
## Beads Issue Tracker

Use Beads (`bd`) for durable task tracking in repositories that include it. Use the `beads` skill at `.agents/skills/beads/SKILL.md` (project install) or `~/.agents/skills/beads/SKILL.md` (global install) for Beads workflow guidance, then use the `bd` CLI for issue operations.

### Quick Reference

```bash
bd ready                # Find available work
bd show <id>            # View issue details
bd update <id> --claim  # Claim work
bd close <id>           # Complete work
bd prime                # Refresh Beads context
```

### Rules

- Use `bd` for all task tracking; do not create markdown TODO lists.
- Run `bd prime` when Beads context is missing or stale. Codex 0.129.0+ can load Beads context automatically through native hooks; use `/hooks` to inspect or toggle them.
- Keep persistent project memory in Beads via `bd remember`; do not create ad hoc memory files.

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/core-concepts/sync-concepts.md for details and anti-patterns.
<!-- END BEADS CODEX SETUP -->
