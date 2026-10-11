---
name: run-product-discovery
description: Run a pre-production discovery with the user when they have a feature idea but not yet a settled expectation. Ask focused question rounds, record decisions in a brief, and sketch HTML mockups under docs/tasks/<topic>/. Ends with an accepted brief and final mockups; produces no epic, task DAG, or code.
---

# Run product discovery

Treat the user as a client who does not yet know exactly what they want. Your
job is to help them find out, not to design the implementation. Discovery ends
with a brief the user agrees with and final mockups that show it. Splitting the
work into epics is a separate step that happens only when the user asks, using
[plan-reviewable-epics](../plan-reviewable-epics/SKILL.md).

## Ground the conversation

Read just enough of the current code and docs to ask grounded questions, and
tell the user in two or three sentences what exists today. Note constraints from
`AGENTS.md` that limit the options (Rust owns logic, the user sets up Xcode,
persistence is in Rust). Do not survey implementation details the user hasn't
asked about.

## Ask in rounds

Use the structured question tool, at most four questions per round. Order the
rounds by how much each answer reshapes the rest:

1. **Shape**: when and why the feature is used, how data gets in, what the
   central concept buys the user.
2. **Data**: what the core entities hold, how they link to existing ones, and
   how structured each field needs to be.
3. **Interaction**: layouts, editing gestures, edge states.

Write every option so the user can choose without knowing the codebase. Say
what each option costs or trades away ("big UI investment", "needs network"),
and recommend one when the choice is clear. Use option previews (ASCII layouts
or data samples) when the user is comparing shapes rather than words.

Read the answers literally:

- Free-text answers override the offered options and often pivot the plan
  (for example "start on macOS, iPhone later"). Reflect the pivot back in one
  sentence and adjust the next round around it.
- A skipped question stays open. It is not a "no".
- When two answers pull against each other ("cooking at the stove is #1" vs.
  "the Mac comes first"), ask about the tension directly instead of choosing.
- When the user picks an expensive option, ask how it should arrive in stages.
  Record their answer even if they keep the expensive path.

Keep the MVP line separate from the roadmap: ask which version they would
actually *use*, and treat everything after it as a ladder of later rungs.
Every "not in the MVP" answer goes into a **parked features** list (`F-xx`,
with the question it came from). Nothing the user wants later is lost, and the
MVP stays small.

## Review the mockups with the user

After the first rounds, let the user annotate the mockups directly (for
example with `/plannotator-annotate docs/tasks/<topic>/mockups.html`).
Annotations answer questions in bulk and raise new ones. Process every
annotation:

- An answer becomes a `D-xx` entry; "not in MVP" becomes an `F-xx` entry.
- A partial answer gets its decided part logged, and the rest becomes a new,
  narrower `Q-xx`.
- "What's this?" or "I don't understand" means the mockup failed. Relabel the
  element or rewrite the note in plain words with a concrete example, and
  reply in chat with the explanation.
- An answer with a consequence the user may not have seen (for example "show
  total time" when steps run in parallel) becomes a new question with
  visualised options.

Then redraw the mockups to show the decided state. Drop rejected variants,
remove controls that were parked, and keep `Q-xx` badges only on what is still
open.

## Record in docs/tasks/<topic>/

Under plan mode, ask the questions first and write files only after the plan is
approved. Then keep two files current after every round:

- **`brief.md`**: status line, a short vision, a decision log (`D-xx`, decision
  plus one-line why, grouped by topic), a roadmap ladder marked as a
  hypothesis, open questions (`Q-xx`), parked features (`F-xx`), and
  non-goals. Answered questions move into the decision log; IDs are never
  reused.
- **`mockups.html`**: one self-contained static page that works offline, with no
  external scripts or fonts. Rules:
  - Use one realistic example (one recipe, one ingredient set) throughout. Drive
    the repeated pieces from a small inline data object so every screen agrees.
  - Use native-looking platform chrome for the target platform.
  - Tag each screen **MVP** or **Later**. Later screens exist only to check that
    the MVP model won't block them.
  - Put `Q-xx` badges on the exact element a question is about, linking to a
    status list at the bottom (open questions and parked features). Close
    each screen with "Open" and "Decided" boxes that cite `Q-xx`, `D-xx` and
    `F-xx`.
  - Write notes for someone who doesn't know the plan. Avoid shorthand like
    "rung 6" without saying what it means and showing an example.
  - Show open choices as variants A/B side by side, each with a one-line "why".
  - Support light and dark themes through CSS tokens, plus a theme toggle.

Render the page (light and dark) and look at the screenshots before you hand it
over. Check that badges don't cover content and that the derived views agree
with the data.

Keep these files in their own jj change, following
[use-of-jj-vcs](../use-of-jj-vcs/SKILL.md). Update that same change as rounds
progress; do not create a change per round.

## Stay out of bounds

- No implementation, migrations, bindings, or Swift code.
- No Xcode automation; `AGENTS.md` reserves initial setup for the user.
- Do not settle technical choices the user hasn't asked to decide (storage
  format, crates, AI backend). Note them as later design work.
- Do not write the epic or the task DAG, and do not start one on your own.
  Keep the roadmap high level: what comes first, and an unordered list of
  candidates after it.

## Close the loop

When the open questions run dry, run one last round on behaviour nobody has
looked at yet. Ask about deleting data other records link to, what must be
filled in before saving, uniqueness rules, and what comes after the MVP.
Accept "I'll decide after using it" as an answer and record it.

## Finish

Discovery is done when the user says the brief matches what they want. Make
the mockups final: they show only the decided state, with deferred questions
listed. Mark the brief's status as accepted, and stop. Epic planning starts
only when the user asks for it.
