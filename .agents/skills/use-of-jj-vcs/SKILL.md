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

When the change is ready, inspect the log again, remove any empty change you created, and report the change identifier, the work done, and the checks run for human review. Stop there. Run another `jj new` only after the human approves the current change.
