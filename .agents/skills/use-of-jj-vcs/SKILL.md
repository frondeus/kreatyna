---
name: use-of-jj-vcs
description: Use for any task that edits files in this repository. Its version history uses jj, and each change requires human review before another change begins.
---

# Use of JJ VCS

This repository uses Jujutsu (`jj`) for version history. The human always reviews code changes.

Before editing any repository files for a new task:

1. Inspect the history with `jj`, `jj log`, and `jj log -r "::@" -n 10`. Check `jj status` and the current change. Do not stack a new change on an empty change; confirm it has no work, then abandon it with `jj abandon <change-id>`. Do not leave empty changes you created in the log.
2. If a previous change is awaiting human review, do not start another change. Wait for approval or address review feedback in that same change.
3. Run `jj new` to create a change for the task.
4. Run `jj describe -m "<concise description>"` to describe it.
5. Only then edit files.

## Size limit: under 1,000 lines

The user reviews every change, so each change stays under 1,000 lines of
diff. Size is the insertions plus the deletions that this command reports:

```bash
jj diff -r @ --stat '~.beads/issues.jsonl' | tail -1
```

The beads export does not count, and images count as 0. Plan the split before
you write code: a slice plan lists its changes with estimated sizes (see
[write-slice-plan](../write-slice-plan/SKILL.md)). Measure again before each
report. If a change is over the limit, split it with `jj split` (set
`JJ_EDITOR=true`) into atomic changes: each one builds and passes every check
alone. Cut at a file or layer boundary, for example the database and store,
then the Rust widgets, then the FFI and Swift. A file that only moves counts
twice, so prefer editing a file in place to renaming it. Each change in the
series gets its own report or review, in order.

When the change is ready, run `bd export -o .beads/issues.jsonl` so the change records the current beads. Inspect the log again, remove any empty change you created, and report the change identifier, the work done, and the checks run for human review. Stop there. Run another `jj new` only after the human approves the current change.

Exception: a change that edits no Rust (`.rs`) files gets no review (see "Code review: count the Rust files first" in [use-plannotator-inbox](../use-plannotator-inbox/SKILL.md)). Send the report, and you may start the next change without waiting for approval.
