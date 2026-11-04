---
name: run-introspection
description: Run the introspection step of a delivery cycle in kreatyna. Read every Plannotator comment the user made since the last introspection, find the mistakes that repeat, and turn each into a lint, a check, or a skill rule. Use at the mol-cycle "introspection" step, after the cooldown, or when the user asks what you keep getting wrong.
---

# Run an introspection

The user's review comments show what you get wrong. A mistake that the user
corrects twice is a missing rule, or a rule that you did not apply. The
introspection finds these and fixes the cause, so the user does not correct
the same thing a third time (the user, 2026-11-04).

It is a step of `mol-cycle`: four feature slices, the bugs, the full suite,
the cooldown, then the introspection. Talk to the user in an Inbox thread
named "Introspection <cycle>".

## 1. Gather the comments

The cursor is the time of the newest comment the last introspection read.
Read it from beads memory, then gather everything after it:

```bash
bd memories introspection-cursor
python3 -I .agents/skills/run-introspection/scripts/gather-comments.py \
  --since <cursor> --out <scratchpad>/introspection-<cycle>
```

The script reads, and never changes, these sources:

| File | Source |
|---|---|
| `inbox.md` | Every Inbox message by the user, by thread, with each agent message as one line of context |
| `annotations.md` | Comments on attached files and images |
| `annotate.md` | `plannotator annotate` and plan-review feedback (before the Inbox) |
| `code-reviews.md` | `plannotator review` feedback in the Claude Code transcripts |

Read all four files completely. Do not sample them.

## 2. Sort the corrections

Skip approvals (👍, "lgtm") and answers that take your recommendation. For
each other comment, note:

- **Theme.** Use one short name, for example "bool for a mode" or "a helper
  that already exists".
- **Date and place.** Give the slice or change, the file, and the line.
- **Covered?** Did a skill or memory hold this rule on that date?
  - *Yes*: you had the rule and did not apply it. The fix is a check that runs
    without you: a lint, a test, or a step in a checklist that you must
    complete.
  - *No*: the rule was missing. The fix is the rule.

A theme with two or more comments repeats. Also count a comment where the user
asks for a skill, a rule, or a bead "for the other places": the user had to
tell you to codify it.

Also look for themes that stopped. A theme that has no comment after its fix
shows that the fix works. Report it, and do not change it.

## 3. Choose the fix

For each repeating theme, choose the strongest fix that is possible:

1. **A lint.** Use a Clippy lint in `[workspace.lints]`, with its settings in
   `clippy.toml`. A machine check never forgets. Count the warnings that it
   gives today (`cargo clippy --workspace --all-targets -- -W clippy::<name>`).
2. **A check in a script or a test**, when no lint exists.
3. **A rule in a skill**, with an example from the comment. Put it in the
   skill that is active when the mistake happens.
4. **A memory** (`bd remember`), for a fact that no skill owns.

Do not propose a fix for a comment that is only a decision about a feature.

## 4. Report, then fix

Send one Inbox message in the "Introspection <cycle>" thread with:

- the count of comments read, by source, and the period they cover;
- a table of the repeating themes: count, dates, covered or not;
- the themes that stopped after an earlier fix;
- one `:::question` block for each proposed fix, with the warning count for a
  lint.

Make each approved fix in its own reviewed jj change. Fixes to Rust code need a
guided review. File each fix as a bead with the `introspection` label, and
link it with `discovered-from: <the introspection step>`.

## 5. Move the cursor

When every approved fix is closed, write the newest comment time that the
script printed:

```bash
bd remember --key introspection-cursor "<time>"
```

Then close the step.
